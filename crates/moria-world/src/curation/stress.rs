use std::collections::BTreeSet;

use serde::Serialize;

use crate::{
    AIR, AabbQ8, BiomeId, ColumnCoord, ObjectKind, ObjectSpatialIndex, TOPSOIL, VoxelCoord,
    WorldIdentity, WorldPointQ8, biome_at, evaluate_base_voxel, evaluate_column,
};

const RADIUS_Q8: i32 = 3 * 256;
const CELL_Q8: i32 = 32 * 256;
const BRICK_Q8: i32 = 16 * 64;
const SURFACE_CELL_EDGE_VOXELS: i32 = 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct CurationStressTarget {
    pub center: VoxelCoord,
    pub broad_dependency_candidates: u16,
    pub exact_dependency_ids: u16,
    pub dependency_bricks: u16,
    pub changed_bricks: u16,
}

pub(super) fn select_radius_three_stress_target(
    index: &ObjectSpatialIndex<'_>,
    identity: &WorldIdentity,
) -> Option<CurationStressTarget> {
    surface_centers(identity)
        .filter(|&center| eligible_surface_center(index, identity, center))
        .filter_map(|center| score(index, center))
        .max_by_key(|target| {
            (
                target.broad_dependency_candidates,
                target.exact_dependency_ids,
                target.dependency_bricks,
                target.changed_bricks,
                std::cmp::Reverse(target.center),
            )
        })
}

/// Enumerates every 4 m surface cell in canonical coordinate order.  The
/// terrain/dressing pipeline owns anchors at this resolution, so a stress
/// target must be one of these actual surface cells rather than an object
/// anchor chosen as a proxy.
fn surface_centers(identity: &WorldIdentity) -> impl Iterator<Item = VoxelCoord> + '_ {
    let bounds = identity.bounds;
    let min_x = bounds.min().x.div_euclid(64);
    let max_x = (bounds.max_exclusive().x - 1).div_euclid(64);
    let min_z = bounds.min().z.div_euclid(64);
    let max_z = (bounds.max_exclusive().z - 1).div_euclid(64);
    (min_x..=max_x)
        .step_by(SURFACE_CELL_EDGE_VOXELS as usize)
        .flat_map(move |x| {
            (min_z..=max_z)
                .step_by(SURFACE_CELL_EDGE_VOXELS as usize)
                .map(move |z| {
                    let y = (evaluate_column(identity, ColumnCoord { x, z }).surface_y_q8 - 1)
                        .div_euclid(64);
                    VoxelCoord::new(x, y, z)
                })
        })
}

fn eligible_surface_center(
    index: &ObjectSpatialIndex<'_>,
    identity: &WorldIdentity,
    center: VoxelCoord,
) -> bool {
    if biome_at(
        identity,
        ColumnCoord {
            x: center.x,
            z: center.z,
        },
    ) != BiomeId::Forest
    {
        return false;
    }
    let Some(bounds) = edit_bounds(center) else {
        return false;
    };
    exact_members(index, bounds)
        .iter()
        .any(|&member| index.placements()[member as usize].kind != ObjectKind::Ruin)
        && eligible_dressing_anchor(identity, center)
}

/// The terrain evaluator owns the immutable topsoil surface. A surface-cell
/// center is a valid dressing anchor only at that upward-facing surface; an
/// object anchor is unrelated to dressing ownership and cannot stand in for it.
fn eligible_dressing_anchor(identity: &WorldIdentity, center: VoxelCoord) -> bool {
    let surface = evaluate_column(
        identity,
        ColumnCoord {
            x: center.x,
            z: center.z,
        },
    );
    surface.surface_y_q8 > center.y * 64
        && surface.surface_y_q8 <= (center.y + 1) * 64
        && evaluate_base_voxel(identity, center).material == TOPSOIL
        && evaluate_base_voxel(identity, VoxelCoord::new(center.x, center.y + 1, center.z)).material
            == AIR
}

fn score(index: &ObjectSpatialIndex<'_>, center: VoxelCoord) -> Option<CurationStressTarget> {
    let bounds = edit_bounds(center)?;
    let exact = exact_members(index, bounds);
    Some(CurationStressTarget {
        center,
        broad_dependency_candidates: u16::try_from(exact.len()).unwrap_or(u16::MAX),
        exact_dependency_ids: u16::try_from(exact.len()).unwrap_or(u16::MAX),
        dependency_bricks: exact
            .iter()
            .copied()
            .map(|member| bricks(index.records()[member as usize].dependency_bounds))
            .max()
            .unwrap_or(0),
        changed_bricks: changed_sphere_bricks(center),
    })
}

fn edit_bounds(center: VoxelCoord) -> Option<AabbQ8> {
    let point = WorldPointQ8::new(center.x * 64, center.y * 64, center.z * 64);
    AabbQ8::new(
        WorldPointQ8::new(
            point.x - RADIUS_Q8,
            point.y - RADIUS_Q8,
            point.z - RADIUS_Q8,
        ),
        WorldPointQ8::new(
            point.x + RADIUS_Q8 + 1,
            point.y + RADIUS_Q8 + 1,
            point.z + RADIUS_Q8 + 1,
        ),
    )
    .ok()
}

fn exact_members(index: &ObjectSpatialIndex<'_>, bounds: AabbQ8) -> Vec<u32> {
    index
        .dependency_cells()
        .iter()
        .filter(|cell| keys(bounds).contains(&cell.key))
        .flat_map(|cell| cell.members.iter().copied())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter(|&member| overlap(index.records()[member as usize].dependency_bounds, bounds))
        .collect()
}

fn keys(bounds: AabbQ8) -> BTreeSet<crate::DependencyGridCellKey> {
    let min_x = bounds.min.x.div_euclid(CELL_Q8);
    let max_x = (bounds.max_exclusive.x - 1).div_euclid(CELL_Q8);
    let min_z = bounds.min.z.div_euclid(CELL_Q8);
    let max_z = (bounds.max_exclusive.z - 1).div_euclid(CELL_Q8);
    (min_x..=max_x)
        .flat_map(|x| (min_z..=max_z).map(move |z| (x, z)))
        .filter_map(|(x, z)| i16::try_from(x).ok().zip(i16::try_from(z).ok()))
        .map(|(x, z)| crate::DependencyGridCellKey { x, z })
        .collect()
}

fn overlap(left: AabbQ8, right: AabbQ8) -> bool {
    left.min.x < right.max_exclusive.x
        && left.max_exclusive.x > right.min.x
        && left.min.y < right.max_exclusive.y
        && left.max_exclusive.y > right.min.y
        && left.min.z < right.max_exclusive.z
        && left.max_exclusive.z > right.min.z
}

fn bricks(bounds: AabbQ8) -> u16 {
    let extent = |min: i32, max: i32| (max - 1).div_euclid(BRICK_Q8) - min.div_euclid(BRICK_Q8) + 1;
    u16::try_from(
        i64::from(extent(bounds.min.x, bounds.max_exclusive.x))
            * i64::from(extent(bounds.min.y, bounds.max_exclusive.y))
            * i64::from(extent(bounds.min.z, bounds.max_exclusive.z)),
    )
    .unwrap_or(u16::MAX)
}

/// Counts only bricks containing voxels changed by the radius-three sphere,
/// rather than every brick touched by its enclosing AABB.
fn changed_sphere_bricks(center: VoxelCoord) -> u16 {
    let radius_voxels = RADIUS_Q8.div_euclid(64);
    let radius_squared = i64::from(RADIUS_Q8).pow(2);
    let mut changed = BTreeSet::new();
    for x in center.x - radius_voxels..=center.x + radius_voxels {
        for y in center.y - radius_voxels..=center.y + radius_voxels {
            for z in center.z - radius_voxels..=center.z + radius_voxels {
                let dx = i64::from((x - center.x) * 64);
                let dy = i64::from((y - center.y) * 64);
                let dz = i64::from((z - center.z) * 64);
                if dx * dx + dy * dy + dz * dz <= radius_squared {
                    changed.insert((x.div_euclid(16), y.div_euclid(16), z.div_euclid(16)));
                }
            }
        }
    }
    u16::try_from(changed.len()).unwrap_or(u16::MAX)
}

#[cfg(test)]
mod tests {
    use crate::{
        AIR, ObjectIndexConfig, RegionConfig, TOPSOIL, VoxelCoord, WorldBounds, WorldIdentity,
        WorldPointQ8, build_object_index, derive_manifest, evaluate_base_voxel,
    };

    use super::{
        eligible_surface_center, score, select_radius_three_stress_target, surface_centers,
    };

    #[test]
    fn selects_the_maximum_from_the_complete_terrain_and_dressing_oracle() {
        let config: RegionConfig = ron::de::from_bytes(include_bytes!(
            "../../../../assets/config/product_one_region.ron"
        ))
        .unwrap();
        let manifest = derive_manifest(
            config.seed,
            &config,
            &ron::de::from_bytes(include_bytes!("../../../../assets/stamps/ruin_p1.ron")).unwrap(),
        )
        .unwrap();
        let identity = WorldIdentity::new(
            manifest.seed,
            manifest.parameters_digest,
            WorldBounds::new(
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
            .unwrap(),
        );
        let index = build_object_index(
            &manifest.objects,
            &ObjectIndexConfig::from_configs(&config.objects, 1_024),
        )
        .unwrap();

        let oracle = surface_centers(&identity)
            .filter(|&center| eligible_surface_center(&index, &identity, center))
            .filter_map(|center| score(&index, center))
            .max_by_key(|target| {
                (
                    target.broad_dependency_candidates,
                    target.exact_dependency_ids,
                    target.dependency_bricks,
                    target.changed_bricks,
                    std::cmp::Reverse(target.center),
                )
            });

        assert_eq!(select_radius_three_stress_target(&index, &identity), oracle);
        let target = select_radius_three_stress_target(&index, &identity).unwrap();
        assert_eq!(
            evaluate_base_voxel(&identity, target.center).material,
            TOPSOIL
        );
        assert_eq!(
            evaluate_base_voxel(
                &identity,
                VoxelCoord::new(target.center.x, target.center.y + 1, target.center.z),
            )
            .material,
            AIR
        );
    }
}
