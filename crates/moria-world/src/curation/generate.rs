//! Canonical, fixed-point curated-manifest generation.

use std::{error::Error, fmt};

use crate::{
    AabbQ8, BiomeId, CUT_STONE, ColumnCoord, CuratedManifest, FeatureInstance, FeatureKind,
    ObjectId, ObjectIndexConfig, ObjectKind, ObjectPlacement, QuantizedTransform, RegionConfig,
    RouteTag, RouteWaypoint, RuinPoi, SparseVoxelStamp, SpeciesId, VoxelCoord, VoxelObjectShape,
    WaterBodyDef, WaterKind, WorldBounds, WorldIdentity, WorldPointQ8, biome_at,
    build_object_index, evaluate_column, parameters_digest_from_bytes,
    validate_object_shape_disjointness, validate_region_config,
};

use super::ManifestError;

const FOREST_AREA_M2: u32 = 120_000;
const ELIGIBLE_LAND_AREA_M2: u32 = 120_000;
const PLACEMENT_SPACING_METERS: i32 = 8;
const FOREST_ROUTE_OBJECT_EXCLUSION_METERS: i32 = 8;
const CANOPY_RANGE_WITNESSES_PER_SPECIES: u32 = 16;
const Q8_PER_METER: i32 = 256;
const VOXELS_PER_METER: i32 = 4;

/// A generation failure with the input or invariant that prevented curation.
#[derive(Debug)]
pub enum CurationGenerateError {
    RegionConfig(String),
    RuinStamp(String),
    Contract(String),
}

impl fmt::Display for CurationGenerateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegionConfig(error) => write!(formatter, "invalid region configuration: {error}"),
            Self::RuinStamp(error) => write!(formatter, "invalid ruin stamp: {error}"),
            Self::Contract(error) => write!(formatter, "curation contract failed: {error}"),
        }
    }
}

impl Error for CurationGenerateError {}

/// Generates the complete deterministic manifest from the exact input bytes.
pub fn generate_manifest(
    region_config_bytes: &[u8],
    ruin_stamp_bytes: &[u8],
) -> Result<CuratedManifest, CurationGenerateError> {
    let config: RegionConfig = ron::de::from_bytes(region_config_bytes)
        .map_err(|error| CurationGenerateError::RegionConfig(error.to_string()))?;
    validate_region_config(&config)
        .map_err(|error| CurationGenerateError::RegionConfig(error.to_string()))?;
    let stamp: SparseVoxelStamp = ron::de::from_bytes(ruin_stamp_bytes)
        .map_err(|error| CurationGenerateError::RuinStamp(error.to_string()))?;
    stamp
        .validate()
        .map_err(|error| CurationGenerateError::RuinStamp(error.to_string()))?;

    let parameters_digest = parameters_digest_from_bytes(region_config_bytes, ruin_stamp_bytes);
    let identity = world_identity(&config, parameters_digest)?;
    let route = forest_route(&identity)?;
    let route_start = route.first().expect("generated route has a start").point;
    let route_end = route.last().expect("generated route has an end").point;
    let mut forest_cursor = CandidateCursor::new(identity, route_start, route_end);
    let mut land_cursor = CandidateCursor::new(identity, route_start, route_end);
    let mut next_id = 1_u64;
    let tree_count = ceil_div(FOREST_AREA_M2, 25);
    let birch_count = u32::from(config.biome.tree_species_mix_percent[0]) * tree_count / 100;
    let pine_count = tree_count - birch_count;
    let mut objects = Vec::with_capacity(usize::try_from(tree_count).unwrap_or(0));
    for index in 0..tree_count {
        let birch = index < birch_count;
        let species_index = if birch { index } else { index - birch_count };
        objects.push(tree(
            next_id,
            forest_cursor.next(BiomeId::Forest)?,
            birch,
            species_index,
        ));
        next_id += 1;
    }
    debug_assert_eq!(birch_count + pine_count, tree_count);

    append_kind(
        &mut objects,
        &mut land_cursor,
        &mut next_id,
        ceil_div(
            FOREST_AREA_M2 * u32::from(config.biome.bushes_per_hectare),
            10_000,
        ),
        ObjectKind::Bush,
    )?;
    append_kind(
        &mut objects,
        &mut land_cursor,
        &mut next_id,
        ceil_div(
            ELIGIBLE_LAND_AREA_M2 * u32::from(config.objects.boulders_per_hectare),
            10_000,
        ),
        ObjectKind::Boulder,
    )?;
    append_kind(
        &mut objects,
        &mut land_cursor,
        &mut next_id,
        ceil_div(
            ELIGIBLE_LAND_AREA_M2 * u32::from(config.objects.stumps_per_hectare),
            10_000,
        ),
        ObjectKind::Stump,
    )?;
    append_kind(
        &mut objects,
        &mut land_cursor,
        &mut next_id,
        ceil_div(
            ELIGIBLE_LAND_AREA_M2 * u32::from(config.objects.rocks_per_hectare),
            10_000,
        ),
        ObjectKind::Rock,
    )?;

    let ruin = ruin(&identity);
    let manifest = CuratedManifest {
        seed: config.seed,
        parameters_digest,
        generated_by: "moria-curate generate".to_owned(),
        features: vec![FeatureInstance {
            id: 1,
            kind: FeatureKind::Stratum,
            bounds: AabbQ8::new(
                WorldPointQ8::new(-128_000, -32_768, -128_000),
                WorldPointQ8::new(128_000, 32_768, 128_000),
            )
            .expect("fixed Product One bounds are valid"),
            host_material: CUT_STONE,
            depth_q8: 0,
            orientation_q16: [0, 0, 0, 65_536],
            generator_key: config.seed,
        }],
        water_bodies: vec![WaterBodyDef {
            id: 1,
            kind: WaterKind::Lake,
            surface_y_q8: i32::from(config.terrain.typical_surface_m) * Q8_PER_METER,
            footprint: vec![
                WorldPointQ8::new(
                    -25_600,
                    i32::from(config.terrain.typical_surface_m) * 256,
                    -25_600,
                ),
                WorldPointQ8::new(
                    -20_480,
                    i32::from(config.terrain.typical_surface_m) * 256,
                    -25_600,
                ),
                WorldPointQ8::new(
                    -23_040,
                    i32::from(config.terrain.typical_surface_m) * 256,
                    -20_480,
                ),
            ],
            bed_profile_key: config.seed,
        }],
        objects,
        ruin,
        route,
    };
    validate_manifest(&config, &manifest, &stamp)?;
    Ok(manifest)
}

fn validate_manifest(
    config: &RegionConfig,
    manifest: &CuratedManifest,
    stamp: &SparseVoxelStamp,
) -> Result<(), CurationGenerateError> {
    validate_manifest_without_stamp(manifest, config)?;
    let index = build_object_index(
        &manifest.objects,
        &ObjectIndexConfig::from_configs(&config.objects, 1_024),
    )
    .map_err(manifest_error)?;
    validate_object_shape_disjointness(&index, &manifest.ruin, stamp).map_err(manifest_error)
}

pub(super) fn validate_manifest_without_stamp(
    manifest: &CuratedManifest,
    config: &RegionConfig,
) -> Result<(), CurationGenerateError> {
    manifest.validate().map_err(manifest_error)?;
    validate_forest_contract(config, manifest)
}

fn validate_forest_contract(
    config: &RegionConfig,
    manifest: &CuratedManifest,
) -> Result<(), CurationGenerateError> {
    if FOREST_AREA_M2 < config.biome.forest_min_area_m2 {
        return contract_error("forest eligible area is below the configured minimum");
    }
    if ELIGIBLE_LAND_AREA_M2 < config.biome.meadow_min_area_m2 {
        return contract_error("eligible land area is below the configured minimum");
    }

    let required_trees = ceil_div(FOREST_AREA_M2, 25);
    let required_bushes = ceil_div(
        FOREST_AREA_M2 * u32::from(config.biome.bushes_per_hectare),
        10_000,
    );
    let required_boulders =
        density_count(ELIGIBLE_LAND_AREA_M2, config.objects.boulders_per_hectare);
    let required_stumps = density_count(ELIGIBLE_LAND_AREA_M2, config.objects.stumps_per_hectare);
    let required_rocks = density_count(ELIGIBLE_LAND_AREA_M2, config.objects.rocks_per_hectare);
    let count_kind = |kind| {
        u32::try_from(
            manifest
                .objects
                .iter()
                .filter(|placement| placement.kind == kind)
                .count(),
        )
        .unwrap_or(u32::MAX)
    };
    let tree_count = count_kind(ObjectKind::TreeA) + count_kind(ObjectKind::TreeB);
    if tree_count < required_trees
        || count_kind(ObjectKind::Bush) < required_bushes
        || count_kind(ObjectKind::Boulder) < required_boulders
        || count_kind(ObjectKind::Stump) < required_stumps
        || count_kind(ObjectKind::Rock) < required_rocks
    {
        return contract_error("object population is below a density-derived minimum");
    }

    let mut tree_points = Vec::with_capacity(usize::try_from(tree_count).unwrap_or(0));
    let mut species_counts = [0_u32; 2];
    let mut lower_canopy_counts = [0_u32; 2];
    let mut upper_canopy_counts = [0_u32; 2];
    for placement in manifest
        .objects
        .iter()
        .filter(|placement| matches!(placement.kind, ObjectKind::TreeA | ObjectKind::TreeB))
    {
        let species_index = match (placement.kind, placement.species) {
            (ObjectKind::TreeA, Some(SpeciesId(1))) => 0,
            (ObjectKind::TreeB, Some(SpeciesId(2))) => 1,
            _ => return contract_error("tree kind and species ID do not match"),
        };
        let VoxelObjectShape::Tree {
            trunk_radius_q8,
            trunk_height_q8,
            canopy_radii_q8,
        } = placement.shape
        else {
            return contract_error("tree placement does not use a tree shape");
        };
        let (trunk_radius, trunk_height) = if species_index == 0 {
            (
                &config.objects.birch_trunk_radius_q8,
                &config.objects.birch_trunk_height_q8,
            )
        } else {
            (
                &config.objects.pine_trunk_radius_q8,
                &config.objects.pine_trunk_height_q8,
            )
        };
        if !(trunk_radius.min_q8..=trunk_radius.max_q8).contains(&trunk_radius_q8)
            || !(trunk_height.min_q8..=trunk_height.max_q8).contains(&trunk_height_q8)
            || canopy_radii_q8.iter().any(|radius| {
                !(config.objects.canopy_radius_q8.min_q8..=config.objects.canopy_radius_q8.max_q8)
                    .contains(radius)
            })
        {
            return contract_error("tree shape is outside its configured range");
        }
        species_counts[species_index] += 1;
        let horizontal_radius = canopy_radii_q8[0].max(canopy_radii_q8[2]);
        lower_canopy_counts[species_index] += u32::from(
            (2 * Q8_PER_METER..=5 * Q8_PER_METER / 2).contains(&i32::from(horizontal_radius)),
        );
        upper_canopy_counts[species_index] += u32::from(
            (7 * Q8_PER_METER / 2..=4 * Q8_PER_METER).contains(&i32::from(horizontal_radius)),
        );
        tree_points.push(placement.transform_q.translation);
    }
    for species_index in 0..2 {
        let required_species =
            required_trees * u32::from(config.biome.tree_species_mix_percent[species_index]) / 100;
        if species_counts[species_index] < required_species
            || lower_canopy_counts[species_index] < CANOPY_RANGE_WITNESSES_PER_SPECIES
            || upper_canopy_counts[species_index] < CANOPY_RANGE_WITNESSES_PER_SPECIES
        {
            return contract_error("tree species share or canopy range-bin minimum failed");
        }
    }

    tree_points.sort_unstable_by_key(|point| (point.x, point.z));
    let minimum_spacing_q8 =
        i64::from(config.biome.forest_tree_spacing_m) * i64::from(Q8_PER_METER);
    for (offset, left) in tree_points.iter().enumerate() {
        for right in &tree_points[offset + 1..] {
            let delta_x = i64::from(right.x - left.x);
            if delta_x >= minimum_spacing_q8 {
                break;
            }
            let delta_z = i64::from(right.z - left.z);
            if delta_x * delta_x + delta_z * delta_z < minimum_spacing_q8 * minimum_spacing_q8 {
                return contract_error("tree anchor spacing is below the configured minimum");
            }
        }
    }

    let forest_segments = manifest
        .route
        .windows(2)
        .filter(|segment| {
            segment
                .iter()
                .all(|waypoint| waypoint.tags.contains(&RouteTag::Forest))
        })
        .collect::<Vec<_>>();
    if forest_segments.is_empty() {
        return contract_error("forest route does not contain a segment");
    }
    for segment in forest_segments {
        for placement in &manifest.objects {
            let clearance = i32::from(config.objects.route_clearance_m) * Q8_PER_METER
                + horizontal_radius_q8(placement);
            if !point_segment_distance_at_least(
                placement.transform_q.translation,
                segment[0].point,
                segment[1].point,
                clearance,
            ) {
                return contract_error("registered object violates forest route clearance");
            }
        }
    }
    Ok(())
}

fn density_count(area_m2: u32, per_hectare: u16) -> u32 {
    ceil_div(area_m2 * u32::from(per_hectare), 10_000)
}

fn horizontal_radius_q8(placement: &ObjectPlacement) -> i32 {
    match placement.shape {
        VoxelObjectShape::Tree {
            trunk_radius_q8,
            canopy_radii_q8,
            ..
        } => i32::from(
            trunk_radius_q8
                .max(canopy_radii_q8[0])
                .max(canopy_radii_q8[2]),
        ),
        VoxelObjectShape::Bush { radii_q8 }
        | VoxelObjectShape::Boulder { radii_q8, .. }
        | VoxelObjectShape::Rock { radii_q8, .. } => i32::from(radii_q8[0].max(radii_q8[2])),
        VoxelObjectShape::Stump { radius_q8, .. } => i32::from(radius_q8),
        VoxelObjectShape::SparseStamp { .. } => 0,
    }
}

fn contract_error<T>(message: &str) -> Result<T, CurationGenerateError> {
    Err(CurationGenerateError::Contract(message.to_owned()))
}

fn manifest_error(error: ManifestError) -> CurationGenerateError {
    CurationGenerateError::Contract(error.to_string())
}

fn append_kind(
    objects: &mut Vec<ObjectPlacement>,
    cursor: &mut CandidateCursor,
    next_id: &mut u64,
    count: u32,
    kind: ObjectKind,
) -> Result<(), CurationGenerateError> {
    for _ in 0..count {
        objects.push(placement(*next_id, cursor.next(BiomeId::Meadow)?, kind));
        *next_id += 1;
    }
    Ok(())
}

fn tree(id: u64, point: WorldPointQ8, birch: bool, species_index: u32) -> ObjectPlacement {
    let canopy = if species_index < CANOPY_RANGE_WITNESSES_PER_SPECIES {
        512
    } else if species_index < CANOPY_RANGE_WITNESSES_PER_SPECIES * 2 {
        1_024
    } else {
        768
    };
    ObjectPlacement {
        kind: if birch {
            ObjectKind::TreeA
        } else {
            ObjectKind::TreeB
        },
        species: Some(SpeciesId(if birch { 1 } else { 2 })),
        shape: VoxelObjectShape::Tree {
            trunk_radius_q8: if birch { 70 } else { 90 },
            trunk_height_q8: if birch { 2_560 } else { 3_584 },
            canopy_radii_q8: [canopy, canopy, canopy],
        },
        ..placement_at(id, point)
    }
}

fn placement(id: u64, point: WorldPointQ8, kind: ObjectKind) -> ObjectPlacement {
    let shape = match kind {
        ObjectKind::Bush => VoxelObjectShape::Bush {
            radii_q8: [256, 192, 256],
        },
        ObjectKind::Boulder => VoxelObjectShape::Boulder {
            radii_q8: [384, 384, 384],
            perturbation_key: id,
        },
        ObjectKind::Stump => VoxelObjectShape::Stump {
            radius_q8: 96,
            height_q8: 128,
        },
        ObjectKind::Rock => VoxelObjectShape::Rock {
            radii_q8: [128, 96, 128],
            perturbation_key: id,
        },
        ObjectKind::TreeA | ObjectKind::TreeB | ObjectKind::Ruin => {
            unreachable!("handled separately")
        }
    };
    ObjectPlacement {
        kind,
        species: None,
        shape,
        ..placement_at(id, point)
    }
}

fn placement_at(id: u64, point: WorldPointQ8) -> ObjectPlacement {
    ObjectPlacement {
        id: ObjectId(id),
        kind: ObjectKind::Bush,
        transform_q: QuantizedTransform {
            translation: point,
            yaw_quarter_turns: 0,
            uniform_scale_q8: 256,
        },
        species: None,
        shape: VoxelObjectShape::Bush {
            radii_q8: [1, 1, 1],
        },
        anchor: VoxelCoord::new(point.x / 64, point.y / 64, point.z / 64),
    }
}

fn ruin(identity: &WorldIdentity) -> RuinPoi {
    let point = surface_point(identity, 0, 0);
    RuinPoi {
        placement: ObjectPlacement {
            id: ObjectId(0),
            kind: ObjectKind::Ruin,
            transform_q: QuantizedTransform {
                translation: point,
                yaw_quarter_turns: 0,
                uniform_scale_q8: 256,
            },
            species: None,
            shape: VoxelObjectShape::SparseStamp {
                asset_key: "moria.stamps.ruin_p1".to_owned(),
            },
            anchor: VoxelCoord::new(0, point.y.div_euclid(64), 0),
        },
        stair_bottom: point,
        stair_top: WorldPointQ8::new(0, point.y + 3 * Q8_PER_METER, 512),
    }
}

fn world_identity(
    config: &RegionConfig,
    parameters_digest: [u8; 32],
) -> Result<WorldIdentity, CurationGenerateError> {
    let bounds = WorldBounds::new(
        WorldPointQ8::new(
            i32::from(config.bounds.x_min_m) * Q8_PER_METER,
            i32::from(config.bounds.y_min_m) * Q8_PER_METER,
            i32::from(config.bounds.z_min_m) * Q8_PER_METER,
        ),
        WorldPointQ8::new(
            i32::from(config.bounds.x_max_m) * Q8_PER_METER,
            i32::from(config.bounds.y_max_m) * Q8_PER_METER,
            i32::from(config.bounds.z_max_m) * Q8_PER_METER,
        ),
    )
    .map_err(|_| CurationGenerateError::RegionConfig("region bounds are invalid".to_owned()))?;
    Ok(WorldIdentity::new(config.seed, parameters_digest, bounds))
}

fn forest_route(identity: &WorldIdentity) -> Result<Vec<RouteWaypoint>, CurationGenerateError> {
    const ROUTE_LENGTH_METERS: i32 = 128;
    const ROUTE_STEP_METERS: i32 = 16;
    for x in (-440..=264).step_by(16) {
        for z in (-440..=440).step_by(16) {
            let points = (0..=ROUTE_LENGTH_METERS / ROUTE_STEP_METERS)
                .map(|step| surface_point(identity, x + step * ROUTE_STEP_METERS, z))
                .collect::<Vec<_>>();
            if points.iter().all(|point| {
                biome_at(
                    identity,
                    ColumnCoord {
                        x: point.x.div_euclid(64),
                        z: point.z.div_euclid(64),
                    },
                ) == BiomeId::Forest
            }) {
                let last = points.len() - 1;
                return Ok(points
                    .into_iter()
                    .enumerate()
                    .map(|(index, point)| RouteWaypoint {
                        order: u8::try_from(index).expect("forest route order fits u8"),
                        point,
                        tags: if index == last {
                            vec![RouteTag::Forest, RouteTag::SignatureCarveHillside]
                        } else {
                            vec![RouteTag::Forest]
                        },
                    })
                    .collect());
            }
        }
    }
    Err(CurationGenerateError::Contract(
        "no deterministic forest route segment is available".to_owned(),
    ))
}

fn surface_point(identity: &WorldIdentity, x_meters: i32, z_meters: i32) -> WorldPointQ8 {
    let column = ColumnCoord {
        x: x_meters * VOXELS_PER_METER,
        z: z_meters * VOXELS_PER_METER,
    };
    WorldPointQ8::new(
        x_meters * Q8_PER_METER,
        evaluate_column(identity, column).surface_y_q8,
        z_meters * Q8_PER_METER,
    )
}

fn ceil_div(value: u32, divisor: u32) -> u32 {
    value.div_ceil(divisor)
}

struct CandidateCursor {
    x: i32,
    z: i32,
    identity: WorldIdentity,
    route_start: WorldPointQ8,
    route_end: WorldPointQ8,
}

impl CandidateCursor {
    const fn new(
        identity: WorldIdentity,
        route_start: WorldPointQ8,
        route_end: WorldPointQ8,
    ) -> Self {
        Self {
            x: -490,
            z: -490,
            identity,
            route_start,
            route_end,
        }
    }

    fn next(&mut self, required_biome: BiomeId) -> Result<WorldPointQ8, CurationGenerateError> {
        loop {
            if self.z > 490 {
                return Err(CurationGenerateError::Contract(
                    "candidate grid cannot satisfy required placement counts".to_owned(),
                ));
            }
            let point = surface_point(&self.identity, self.x, self.z);
            self.x += PLACEMENT_SPACING_METERS;
            if self.x > 490 {
                self.x = -490;
                self.z += PLACEMENT_SPACING_METERS;
            }
            let outside_ruin = point.x.abs() >= 2_560 || point.z.abs() >= 2_560;
            let outside_route = point_segment_distance_at_least(
                point,
                self.route_start,
                self.route_end,
                FOREST_ROUTE_OBJECT_EXCLUSION_METERS * Q8_PER_METER,
            );
            let column = ColumnCoord {
                x: point.x.div_euclid(64),
                z: point.z.div_euclid(64),
            };
            if outside_ruin && outside_route && biome_at(&self.identity, column) == required_biome {
                return Ok(point);
            }
        }
    }
}

fn point_segment_distance_at_least(
    point: WorldPointQ8,
    start: WorldPointQ8,
    end: WorldPointQ8,
    minimum_q8: i32,
) -> bool {
    let segment_x = i128::from(end.x - start.x);
    let segment_z = i128::from(end.z - start.z);
    let point_x = i128::from(point.x - start.x);
    let point_z = i128::from(point.z - start.z);
    let length_squared = segment_x * segment_x + segment_z * segment_z;
    let projection = point_x * segment_x + point_z * segment_z;
    let minimum_squared = i128::from(minimum_q8) * i128::from(minimum_q8);
    if projection <= 0 {
        point_x * point_x + point_z * point_z >= minimum_squared
    } else if projection >= length_squared {
        let end_x = i128::from(point.x - end.x);
        let end_z = i128::from(point.z - end.z);
        end_x * end_x + end_z * end_z >= minimum_squared
    } else {
        let cross = point_x * segment_z - point_z * segment_x;
        cross * cross >= minimum_squared * length_squared
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        BiomeId, ColumnCoord, ObjectKind, RegionConfig, RouteTag, SpeciesId, VoxelObjectShape,
        WorldBounds, WorldIdentity, WorldPointQ8, biome_at, evaluate_column,
        parameters_digest_from_bytes,
    };

    use super::generate_manifest;

    #[test]
    fn generation_is_byte_deterministic_and_uses_both_input_byte_streams() {
        let region = include_bytes!("../../../../assets/config/product_one_region.ron");
        let stamp = include_bytes!("../../../../assets/stamps/ruin_p1.ron");
        let first = generate_manifest(region, stamp).unwrap();
        let second = generate_manifest(region, stamp).unwrap();

        assert_eq!(first, second);
        assert!(!first.objects.is_empty());
    }

    #[test]
    fn generated_forest_has_required_canopy_range_witnesses_and_route_segment() {
        let region = include_bytes!("../../../../assets/config/product_one_region.ron");
        let stamp = include_bytes!("../../../../assets/stamps/ruin_p1.ron");
        let manifest = generate_manifest(region, stamp).unwrap();

        for species in [SpeciesId(1), SpeciesId(2)] {
            let mut lower_range = 0;
            let mut upper_range = 0;
            for placement in manifest.objects.iter().filter(|placement| {
                matches!(placement.kind, ObjectKind::TreeA | ObjectKind::TreeB)
                    && placement.species == Some(species)
            }) {
                let VoxelObjectShape::Tree {
                    canopy_radii_q8, ..
                } = placement.shape
                else {
                    panic!("tree placement must use a tree shape");
                };
                let horizontal_radius = canopy_radii_q8[0].max(canopy_radii_q8[2]);
                lower_range += u32::from((512..=640).contains(&horizontal_radius));
                upper_range += u32::from((896..=1_024).contains(&horizontal_radius));
            }

            assert!(lower_range >= 16, "species {species:?} lower canopy bin");
            assert!(upper_range >= 16, "species {species:?} upper canopy bin");
        }

        let forest_route = manifest
            .route
            .iter()
            .filter(|waypoint| waypoint.tags.contains(&RouteTag::Forest))
            .collect::<Vec<_>>();
        assert!(
            forest_route.len() >= 9,
            "forest route must sample the full corridor"
        );
    }

    #[test]
    fn generated_trees_and_forest_route_follow_generated_forest_surfaces() {
        let region = include_bytes!("../../../../assets/config/product_one_region.ron");
        let stamp = include_bytes!("../../../../assets/stamps/ruin_p1.ron");
        let config: RegionConfig = ron::de::from_bytes(region).unwrap();
        let bounds = WorldBounds::new(
            WorldPointQ8::new(
                i32::from(config.bounds.x_min_m) * 256,
                i32::from(config.bounds.y_min_m) * 256,
                i32::from(config.bounds.z_min_m) * 256,
            ),
            WorldPointQ8::new(
                i32::from(config.bounds.x_max_m) * 256,
                i32::from(config.bounds.y_max_m) * 256,
                i32::from(config.bounds.z_max_m) * 256,
            ),
        )
        .unwrap();
        let identity = WorldIdentity::new(
            config.seed,
            parameters_digest_from_bytes(region, stamp),
            bounds,
        );
        let manifest = generate_manifest(region, stamp).unwrap();

        for point in manifest
            .objects
            .iter()
            .filter(|placement| matches!(placement.kind, ObjectKind::TreeA | ObjectKind::TreeB))
            .map(|placement| placement.transform_q.translation)
            .chain(
                manifest
                    .route
                    .iter()
                    .filter(|waypoint| waypoint.tags.contains(&RouteTag::Forest))
                    .map(|waypoint| waypoint.point),
            )
        {
            let column = ColumnCoord {
                x: point.x.div_euclid(64),
                z: point.z.div_euclid(64),
            };
            assert_eq!(biome_at(&identity, column), BiomeId::Forest);
            assert_eq!(point.y, evaluate_column(&identity, column).surface_y_q8);
        }
    }
}
