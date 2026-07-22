//! Bounded upright-capsule observations against authoritative voxel truth.

use crate::{VoxelCoord, WorldBounds, WorldPointQ8};

use super::{QueryError, QueryLimitKind, QueryMask, WorldHit, WorldRead, WorldSample};

/// Minimum supported capsule radius (0.125 m).
pub const MIN_CAPSULE_RADIUS_Q8: u16 = 32;
/// Maximum supported capsule radius (0.5 m).
pub const MAX_CAPSULE_RADIUS_Q8: u16 = 128;
/// Maximum supported upright half-segment (0.75 m).
pub const MAX_CAPSULE_HALF_SEGMENT_Q8: u16 = 192;
/// Maximum Euclidean sweep displacement (12 m).
pub const MAX_SWEEP_DISPLACEMENT_Q8: u16 = 3_072;
/// Maximum conservative voxel candidates for a sweep.
pub const MAX_SWEEP_CANDIDATE_TESTS: u16 = 8_192;
/// Maximum conservative voxel candidates for an overlap.
pub const MAX_OVERLAP_CANDIDATE_TESTS: u16 = 512;
/// Maximum contacts returned by a capsule query.
pub const MAX_QUERY_HITS: u16 = 512;

const Q16_MAX: i64 = 65_535;
const VOXEL_EDGE_Q8_I64: i64 = 64;

/// A signed Q8 displacement vector.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vec3Q8 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3Q8 {
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

/// An upright Q8 capsule centered on a vertical line segment.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CapsuleQ8 {
    pub center: WorldPointQ8,
    pub radius_q8: u16,
    pub half_segment_q8: u16,
}

impl CapsuleQ8 {
    #[must_use]
    pub const fn new(center: WorldPointQ8, radius_q8: u16, half_segment_q8: u16) -> Self {
        Self {
            center,
            radius_q8,
            half_segment_q8,
        }
    }
}

/// Collision classes selected by a query mask.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MatchedQueryMask {
    Solid,
    Water,
}

/// A quantized collision normal, sorted by axis then sign.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorldNormal {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

/// The safe endpoint and sorted contacts of a capsule sweep.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SweepResult {
    pub safe_fraction_q16: u16,
    pub end_capsule: CapsuleQ8,
    pub hits: Vec<WorldHit>,
}

impl WorldRead<'_, '_> {
    /// Returns all authoritative voxel contacts for an upright capsule.
    pub fn overlap_capsule(
        &self,
        capsule: CapsuleQ8,
        mask: QueryMask,
    ) -> Result<Vec<WorldHit>, QueryError> {
        let bounds = self.ready_bounds()?;
        validate_capsule(capsule, bounds, mask)?;
        let candidates = candidate_range(capsule, capsule, bounds)?;
        if candidates.count() > u32::from(MAX_OVERLAP_CANDIDATE_TESTS) {
            return Err(QueryError::LimitExceeded(
                QueryLimitKind::SweepCandidateWork,
            ));
        }

        let mut hits = Vec::with_capacity(usize::from(MAX_QUERY_HITS));
        for voxel in candidates.iter() {
            let sample = self.sample_voxel(voxel)?;
            if let Some(class) = matched_class(sample, mask)
                && capsule_overlaps_voxel(capsule, voxel)
            {
                push_hit(&mut hits, hit_for(capsule, voxel, sample, class))?;
            }
        }
        sort_hits(&mut hits);
        Ok(hits)
    }

    /// Sweeps an upright capsule and returns the largest collision-free Q16 fraction.
    pub fn sweep_capsule(
        &self,
        capsule: CapsuleQ8,
        displacement: Vec3Q8,
        mask: QueryMask,
    ) -> Result<SweepResult, QueryError> {
        let bounds = self.ready_bounds()?;
        validate_capsule(capsule, bounds, mask)?;
        if squared_length(displacement) > i128::from(MAX_SWEEP_DISPLACEMENT_Q8).pow(2) {
            return Err(QueryError::LimitExceeded(QueryLimitKind::SweepDisplacement));
        }
        let end = moved_capsule(capsule, displacement, Q16_MAX)?;
        validate_capsule_bounds(end, bounds)?;
        let candidates = candidate_range(capsule, end, bounds)?;
        if candidates.count() > u32::from(MAX_SWEEP_CANDIDATE_TESTS) {
            return Err(QueryError::LimitExceeded(
                QueryLimitKind::SweepCandidateWork,
            ));
        }

        let mut impact = None;
        let mut hits = Vec::with_capacity(usize::from(MAX_QUERY_HITS));
        let mut result_count_exceeded = false;
        for voxel in candidates.iter() {
            let sample = self.sample_voxel(voxel)?;
            let Some(class) = matched_class(sample, mask) else {
                continue;
            };
            let Some(fraction) = first_overlap_fraction(capsule, displacement, voxel) else {
                continue;
            };
            match impact {
                None => {
                    impact = Some(fraction);
                    let contact = moved_capsule(capsule, displacement, fraction)?;
                    record_impact_hit(
                        &mut hits,
                        &mut result_count_exceeded,
                        hit_for(contact, voxel, sample, class),
                    );
                }
                Some(current) if fraction < current => {
                    impact = Some(fraction);
                    hits.clear();
                    result_count_exceeded = false;
                    let contact = moved_capsule(capsule, displacement, fraction)?;
                    record_impact_hit(
                        &mut hits,
                        &mut result_count_exceeded,
                        hit_for(contact, voxel, sample, class),
                    );
                }
                Some(current) if fraction == current => {
                    let contact = moved_capsule(capsule, displacement, fraction)?;
                    record_impact_hit(
                        &mut hits,
                        &mut result_count_exceeded,
                        hit_for(contact, voxel, sample, class),
                    );
                }
                Some(_) => {}
            }
        }

        if result_count_exceeded {
            return Err(QueryError::LimitExceeded(QueryLimitKind::ResultCount));
        }

        let safe_fraction = impact.map_or(Q16_MAX, |value| value.saturating_sub(1));
        let end_capsule = moved_capsule(capsule, displacement, safe_fraction)?;
        sort_hits(&mut hits);
        Ok(SweepResult {
            safe_fraction_q16: safe_fraction as u16,
            end_capsule,
            hits,
        })
    }
}

#[derive(Clone, Copy)]
struct CandidateRange {
    min: VoxelCoord,
    max: VoxelCoord,
}

impl CandidateRange {
    fn count(self) -> u32 {
        let x = (self.max.x - self.min.x + 1) as u32;
        let y = (self.max.y - self.min.y + 1) as u32;
        let z = (self.max.z - self.min.z + 1) as u32;
        x.saturating_mul(y).saturating_mul(z)
    }

    fn iter(self) -> impl Iterator<Item = VoxelCoord> {
        (self.min.x..=self.max.x).flat_map(move |x| {
            (self.min.y..=self.max.y)
                .flat_map(move |y| (self.min.z..=self.max.z).map(move |z| VoxelCoord::new(x, y, z)))
        })
    }
}

fn validate_capsule(
    capsule: CapsuleQ8,
    bounds: WorldBounds,
    mask: QueryMask,
) -> Result<(), QueryError> {
    if !mask.is_valid() {
        return Err(QueryError::InvalidInput);
    }
    if capsule.radius_q8 < MIN_CAPSULE_RADIUS_Q8 || capsule.radius_q8 > MAX_CAPSULE_RADIUS_Q8 {
        return Err(QueryError::LimitExceeded(QueryLimitKind::CapsuleRadius));
    }
    if capsule.half_segment_q8 > MAX_CAPSULE_HALF_SEGMENT_Q8 {
        return Err(QueryError::LimitExceeded(QueryLimitKind::CapsuleHeight));
    }
    validate_capsule_bounds(capsule, bounds)
}

fn validate_capsule_bounds(capsule: CapsuleQ8, bounds: WorldBounds) -> Result<(), QueryError> {
    let radius = i64::from(capsule.radius_q8);
    let vertical = radius + i64::from(capsule.half_segment_q8);
    let min = bounds.min();
    let max = bounds.max_exclusive();
    let inside = i64::from(capsule.center.x) - radius >= i64::from(min.x)
        && i64::from(capsule.center.x) + radius < i64::from(max.x)
        && i64::from(capsule.center.y) - vertical >= i64::from(min.y)
        && i64::from(capsule.center.y) + vertical < i64::from(max.y)
        && i64::from(capsule.center.z) - radius >= i64::from(min.z)
        && i64::from(capsule.center.z) + radius < i64::from(max.z);
    inside.then_some(()).ok_or(QueryError::OutOfBounds)
}

fn candidate_range(
    start: CapsuleQ8,
    end: CapsuleQ8,
    _bounds: WorldBounds,
) -> Result<CandidateRange, QueryError> {
    let radius = i64::from(start.radius_q8);
    let vertical = radius + i64::from(start.half_segment_q8);
    let min_x = i64::from(start.center.x.min(end.center.x)) - radius;
    let max_x = i64::from(start.center.x.max(end.center.x)) + radius;
    let min_y = i64::from(start.center.y.min(end.center.y)) - vertical;
    let max_y = i64::from(start.center.y.max(end.center.y)) + vertical;
    let min_z = i64::from(start.center.z.min(end.center.z)) - radius;
    let max_z = i64::from(start.center.z.max(end.center.z)) + radius;
    Ok(CandidateRange {
        min: VoxelCoord::new(
            q8_to_voxel(min_x)?,
            q8_to_voxel(min_y)?,
            q8_to_voxel(min_z)?,
        ),
        max: VoxelCoord::new(
            q8_to_voxel(max_x)?,
            q8_to_voxel(max_y)?,
            q8_to_voxel(max_z)?,
        ),
    })
}

fn q8_to_voxel(value: i64) -> Result<i32, QueryError> {
    i32::try_from(value.div_euclid(VOXEL_EDGE_Q8_I64)).map_err(|_| QueryError::InvalidInput)
}

fn squared_length(displacement: Vec3Q8) -> i128 {
    let x = i128::from(displacement.x);
    let y = i128::from(displacement.y);
    let z = i128::from(displacement.z);
    x * x + y * y + z * z
}

fn moved_capsule(
    capsule: CapsuleQ8,
    displacement: Vec3Q8,
    fraction: i64,
) -> Result<CapsuleQ8, QueryError> {
    let move_component = |origin: i32, delta: i32| {
        i32::try_from(i64::from(origin) + i64::from(delta) * fraction / Q16_MAX)
            .map_err(|_| QueryError::InvalidInput)
    };
    Ok(CapsuleQ8::new(
        WorldPointQ8::new(
            move_component(capsule.center.x, displacement.x)?,
            move_component(capsule.center.y, displacement.y)?,
            move_component(capsule.center.z, displacement.z)?,
        ),
        capsule.radius_q8,
        capsule.half_segment_q8,
    ))
}

fn matched_class(sample: WorldSample, mask: QueryMask) -> Option<MatchedQueryMask> {
    if sample.solid_collision && mask.matches(QueryMask::SOLID) {
        Some(MatchedQueryMask::Solid)
    } else if sample.water_volume && mask.matches(QueryMask::WATER) {
        Some(MatchedQueryMask::Water)
    } else {
        None
    }
}

fn capsule_overlaps_voxel(capsule: CapsuleQ8, voxel: VoxelCoord) -> bool {
    let (dx, dy, dz) = capsule_aabb_distance(capsule, voxel);
    let distance_squared = dx * dx + dy * dy + dz * dz;
    distance_squared <= i64::from(capsule.radius_q8).pow(2)
}

fn capsule_aabb_distance(capsule: CapsuleQ8, voxel: VoxelCoord) -> (i64, i64, i64) {
    let min_x = i64::from(voxel.x) * VOXEL_EDGE_Q8_I64;
    let min_y = i64::from(voxel.y) * VOXEL_EDGE_Q8_I64;
    let min_z = i64::from(voxel.z) * VOXEL_EDGE_Q8_I64;
    let max_x = min_x + VOXEL_EDGE_Q8_I64;
    let max_y = min_y + VOXEL_EDGE_Q8_I64;
    let max_z = min_z + VOXEL_EDGE_Q8_I64;
    let x = i64::from(capsule.center.x);
    let z = i64::from(capsule.center.z);
    let segment_min_y = i64::from(capsule.center.y) - i64::from(capsule.half_segment_q8);
    let segment_max_y = i64::from(capsule.center.y) + i64::from(capsule.half_segment_q8);
    (
        interval_distance(x, x, min_x, max_x),
        interval_distance(segment_min_y, segment_max_y, min_y, max_y),
        interval_distance(z, z, min_z, max_z),
    )
}

fn interval_distance(left_min: i64, left_max: i64, right_min: i64, right_max: i64) -> i64 {
    if left_max < right_min {
        right_min - left_max
    } else if right_max < left_min {
        left_min - right_max
    } else {
        0
    }
}

fn first_overlap_fraction(
    capsule: CapsuleQ8,
    displacement: Vec3Q8,
    voxel: VoxelCoord,
) -> Option<i64> {
    let mut fraction = 0;
    loop {
        if capsule_overlaps_voxel(moved_capsule(capsule, displacement, fraction).ok()?, voxel) {
            return Some(fraction);
        }
        let next = next_position_change_fraction(displacement, fraction)?;
        fraction = next;
    }
}

fn next_position_change_fraction(displacement: Vec3Q8, fraction: i64) -> Option<i64> {
    [displacement.x, displacement.y, displacement.z]
        .into_iter()
        .filter_map(|component| next_component_change_fraction(component, fraction))
        .min()
}

fn next_component_change_fraction(component: i32, fraction: i64) -> Option<i64> {
    let magnitude = i64::from(component).abs();
    if magnitude == 0 {
        return None;
    }
    let travelled = magnitude * fraction / Q16_MAX;
    (travelled < magnitude).then(|| {
        let numerator = (travelled + 1) * Q16_MAX;
        (numerator + magnitude - 1) / magnitude
    })
}

fn hit_for(
    capsule: CapsuleQ8,
    voxel: VoxelCoord,
    sample: WorldSample,
    matched: MatchedQueryMask,
) -> WorldHit {
    let (dx, dy, dz) = capsule_aabb_distance(capsule, voxel);
    let normal = normal_for(dx, dy, dz, capsule, voxel);
    let point = closest_point(capsule, voxel);
    WorldHit {
        voxel,
        point,
        normal,
        normal_q16: [
            i32::from(normal.x) * 65_536,
            i32::from(normal.y) * 65_536,
            i32::from(normal.z) * 65_536,
        ],
        material: sample.material,
        matched,
        matched_mask: match matched {
            MatchedQueryMask::Solid => QueryMask::SOLID,
            MatchedQueryMask::Water => QueryMask::WATER,
        },
        distance_q8: (dx * dx + dy * dy + dz * dz).isqrt() as u32,
        revision: sample.revision,
    }
}

fn normal_for(dx: i64, dy: i64, dz: i64, capsule: CapsuleQ8, voxel: VoxelCoord) -> WorldNormal {
    let point = closest_point(capsule, voxel);
    let center = capsule.center;
    let signs = [
        (
            dx,
            WorldNormal {
                x: sign(center.x, point.x),
                y: 0,
                z: 0,
            },
        ),
        (
            dy,
            WorldNormal {
                x: 0,
                y: sign(center.y, point.y),
                z: 0,
            },
        ),
        (
            dz,
            WorldNormal {
                x: 0,
                y: 0,
                z: sign(center.z, point.z),
            },
        ),
    ];
    let normal = signs
        .into_iter()
        .max_by_key(|(distance, normal)| (*distance, *normal))
        .map(|(_, normal)| normal)
        .expect("a capsule-to-voxel normal always has three axes");
    if normal != (WorldNormal { x: 0, y: 0, z: 0 }) {
        return normal;
    }

    let voxel_center = WorldPointQ8::new(voxel.x * 64 + 32, voxel.y * 64 + 32, voxel.z * 64 + 32);
    let fallback = WorldNormal {
        x: sign(center.x, voxel_center.x),
        y: sign(center.y, voxel_center.y),
        z: sign(center.z, voxel_center.z),
    };
    if fallback != (WorldNormal { x: 0, y: 0, z: 0 }) {
        fallback
    } else {
        WorldNormal { x: -1, y: 0, z: 0 }
    }
}

fn sign(value: i32, against: i32) -> i8 {
    match value.cmp(&against) {
        core::cmp::Ordering::Less => -1,
        core::cmp::Ordering::Equal => 0,
        core::cmp::Ordering::Greater => 1,
    }
}

fn closest_point(capsule: CapsuleQ8, voxel: VoxelCoord) -> WorldPointQ8 {
    let clamp = |value: i64, minimum: i64, maximum: i64| value.clamp(minimum, maximum) as i32;
    let min_x = i64::from(voxel.x) * VOXEL_EDGE_Q8_I64;
    let min_y = i64::from(voxel.y) * VOXEL_EDGE_Q8_I64;
    let min_z = i64::from(voxel.z) * VOXEL_EDGE_Q8_I64;
    WorldPointQ8::new(
        clamp(
            i64::from(capsule.center.x),
            min_x,
            min_x + VOXEL_EDGE_Q8_I64,
        ),
        clamp(
            i64::from(capsule.center.y),
            min_y,
            min_y + VOXEL_EDGE_Q8_I64,
        ),
        clamp(
            i64::from(capsule.center.z),
            min_z,
            min_z + VOXEL_EDGE_Q8_I64,
        ),
    )
}

fn push_hit(hits: &mut Vec<WorldHit>, hit: WorldHit) -> Result<(), QueryError> {
    if hits.len() == usize::from(MAX_QUERY_HITS) {
        return Err(QueryError::LimitExceeded(QueryLimitKind::ResultCount));
    }
    hits.push(hit);
    Ok(())
}

fn record_impact_hit(hits: &mut Vec<WorldHit>, exceeded: &mut bool, hit: WorldHit) {
    if hits.len() == usize::from(MAX_QUERY_HITS) {
        *exceeded = true;
    } else {
        hits.push(hit);
    }
}

fn sort_hits(hits: &mut [WorldHit]) {
    hits.sort_by_key(|hit| (hit.voxel, normal_sort_key(hit.normal)));
}

fn normal_sort_key(normal: WorldNormal) -> (u8, i8, WorldNormal) {
    if normal.x != 0 {
        (0, normal.x, normal)
    } else if normal.y != 0 {
        (1, normal.y, normal)
    } else if normal.z != 0 {
        (2, normal.z, normal)
    } else {
        (3, 0, normal)
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::*;

    use crate::{AIR, MaterialRegistry, Voxel, WATER, WaterBodyDef, WorldIdentity};

    use super::*;
    use crate::query::TraversalRoute;
    use crate::query::read::WorldReadState;

    fn identity() -> WorldIdentity {
        WorldIdentity::new(
            7,
            [0; 32],
            WorldBounds::new(
                WorldPointQ8::new(-128_000, -32_768, -128_000),
                WorldPointQ8::new(128_000, 32_768, 128_000),
            )
            .unwrap(),
        )
    }

    fn state(changes: impl IntoIterator<Item = (VoxelCoord, Voxel)>) -> WorldReadState {
        let mut state = WorldReadState::new(
            identity(),
            MaterialRegistry::default(),
            Vec::<WaterBodyDef>::new(),
            TraversalRoute::new(Vec::new()),
        );
        state.commit_test_voxels(changes);
        state
    }

    #[derive(Resource)]
    struct OverlapResult(Result<Vec<WorldHit>, QueryError>);

    #[derive(Resource)]
    struct SweepQueryResult(Result<SweepResult, QueryError>);

    #[test]
    fn overlap_uses_the_selected_authoritative_predicate() {
        let coordinate = VoxelCoord::new(2, 400, 2);
        let center = WorldPointQ8::new(2 * 64 - 32, 400 * 64 + 32, 2 * 64 + 32);
        let capsule = CapsuleQ8::new(center, 32, 0);
        let mut app = App::new();
        app.insert_resource(state([(coordinate, Voxel::new(WATER, 255, 0, 0))]))
            .insert_resource(OverlapResult(Ok(Vec::new())))
            .add_systems(
                Update,
                move |read: WorldRead, mut result: ResMut<OverlapResult>| {
                    result.0 = read.overlap_capsule(capsule, QueryMask::SOLID);
                },
            );
        app.update();
        assert!(
            app.world()
                .resource::<OverlapResult>()
                .0
                .as_ref()
                .unwrap()
                .is_empty()
        );

        app.world_mut().resource_mut::<OverlapResult>().0 = Ok(Vec::new());
        app.world_mut()
            .run_system_once(move |read: WorldRead, mut result: ResMut<OverlapResult>| {
                result.0 = read.overlap_capsule(capsule, QueryMask::WATER);
            })
            .unwrap();
        let hits = app.world().resource::<OverlapResult>().0.as_ref().unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].voxel, coordinate);
        assert_eq!(hits[0].matched, MatchedQueryMask::Water);
    }

    #[test]
    fn sweep_stops_at_the_first_solid_voxel_and_returns_a_safe_fraction() {
        let obstacle = VoxelCoord::new(2, 400, 2);
        let capsule = CapsuleQ8::new(WorldPointQ8::new(32, 400 * 64 + 32, 2 * 64 + 32), 32, 0);
        let mut app = App::new();
        app.insert_resource(state([(obstacle, Voxel::new(crate::GRANITE, 255, 0, 0))]))
            .insert_resource(SweepQueryResult(Err(QueryError::InvalidInput)))
            .add_systems(
                Update,
                move |read: WorldRead, mut result: ResMut<SweepQueryResult>| {
                    result.0 =
                        read.sweep_capsule(capsule, Vec3Q8::new(256, 0, 0), QueryMask::SOLID);
                },
            );
        app.update();

        let result = app
            .world()
            .resource::<SweepQueryResult>()
            .0
            .as_ref()
            .unwrap();
        assert!(result.safe_fraction_q16 < u16::MAX);
        assert_eq!(
            result.hits.iter().map(|hit| hit.voxel).collect::<Vec<_>>(),
            vec![obstacle]
        );
        assert!(!capsule_overlaps_voxel(result.end_capsule, obstacle));
    }

    #[test]
    fn dimensions_displacement_and_combined_candidate_work_fail_before_sampling() {
        let bounds = identity().bounds;
        let center = WorldPointQ8::new(0, 0, 0);
        assert!(matches!(
            validate_capsule(CapsuleQ8::new(center, 31, 0), bounds, QueryMask::SOLID),
            Err(QueryError::LimitExceeded(QueryLimitKind::CapsuleRadius))
        ));
        assert!(matches!(
            validate_capsule(CapsuleQ8::new(center, 32, 193), bounds, QueryMask::SOLID),
            Err(QueryError::LimitExceeded(QueryLimitKind::CapsuleHeight))
        ));
        assert!(
            squared_length(Vec3Q8::new(3_073, 0, 0)) > i128::from(MAX_SWEEP_DISPLACEMENT_Q8).pow(2)
        );

        let max = CapsuleQ8::new(center, MAX_CAPSULE_RADIUS_Q8, MAX_CAPSULE_HALF_SEGMENT_Q8);
        let end = moved_capsule(max, Vec3Q8::new(2_000, 2_000, 2_000), Q16_MAX).unwrap();
        assert!(
            candidate_range(max, end, bounds).unwrap().count()
                > u32::from(MAX_SWEEP_CANDIDATE_TESTS)
        );
    }

    #[test]
    fn extreme_displacement_is_rejected_without_integer_overflow() {
        let capsule = CapsuleQ8::new(WorldPointQ8::new(0, 400 * 64, 0), 32, 0);
        let mut app = App::new();
        app.insert_resource(state([]))
            .insert_resource(SweepQueryResult(Err(QueryError::InvalidInput)))
            .add_systems(
                Update,
                move |read: WorldRead, mut result: ResMut<SweepQueryResult>| {
                    result.0 = read.sweep_capsule(
                        capsule,
                        Vec3Q8::new(i32::MAX, i32::MAX, i32::MAX),
                        QueryMask::SOLID,
                    );
                },
            );

        app.update();

        assert_eq!(
            app.world().resource::<SweepQueryResult>().0,
            Err(QueryError::LimitExceeded(QueryLimitKind::SweepDisplacement))
        );
    }

    #[test]
    fn exact_sweep_candidate_limit_is_accepted_and_one_more_is_rejected() {
        let capsule = CapsuleQ8::new(WorldPointQ8::new(0, 400 * 64, 0), 32, 0);
        let exact_displacement = Vec3Q8::new(415, 1_951, 1_951);
        let excessive_displacement = Vec3Q8::new(416, 1_951, 1_951);
        let bounds = identity().bounds;
        let exact_end = moved_capsule(capsule, exact_displacement, Q16_MAX).unwrap();
        let excessive_end = moved_capsule(capsule, excessive_displacement, Q16_MAX).unwrap();
        assert_eq!(
            candidate_range(capsule, exact_end, bounds).unwrap().count(),
            u32::from(MAX_SWEEP_CANDIDATE_TESTS)
        );
        assert!(
            candidate_range(capsule, excessive_end, bounds)
                .unwrap()
                .count()
                > u32::from(MAX_SWEEP_CANDIDATE_TESTS)
        );

        let mut app = App::new();
        app.insert_resource(state([]))
            .insert_resource(SweepQueryResult(Err(QueryError::InvalidInput)));
        app.world_mut()
            .run_system_once(
                move |read: WorldRead, mut result: ResMut<SweepQueryResult>| {
                    result.0 = read.sweep_capsule(capsule, exact_displacement, QueryMask::SOLID);
                },
            )
            .unwrap();
        assert!(
            app.world()
                .resource::<SweepQueryResult>()
                .0
                .as_ref()
                .is_ok()
        );

        app.world_mut()
            .run_system_once(
                move |read: WorldRead, mut result: ResMut<SweepQueryResult>| {
                    result.0 =
                        read.sweep_capsule(capsule, excessive_displacement, QueryMask::SOLID);
                },
            )
            .unwrap();
        assert_eq!(
            app.world().resource::<SweepQueryResult>().0,
            Err(QueryError::LimitExceeded(
                QueryLimitKind::SweepCandidateWork
            ))
        );
    }

    #[test]
    fn sweep_fractions_match_exhaustive_q16_oracles() {
        let cases = [
            (
                CapsuleQ8::new(WorldPointQ8::new(0, 0, 0), 32, 0),
                Vec3Q8::new(256, 0, 0),
                VoxelCoord::new(2, 0, 0),
            ),
            (
                CapsuleQ8::new(WorldPointQ8::new(256, 0, 0), 48, 96),
                Vec3Q8::new(-320, 96, 64),
                VoxelCoord::new(1, 2, 0),
            ),
            (
                CapsuleQ8::new(WorldPointQ8::new(-96, 96, -96), 64, 32),
                Vec3Q8::new(320, -192, 320),
                VoxelCoord::new(1, -1, 1),
            ),
            (
                CapsuleQ8::new(WorldPointQ8::new(0, 0, 0), 32, 0),
                Vec3Q8::new(-257, -129, 193),
                VoxelCoord::new(-3, -2, 2),
            ),
        ];

        for (capsule, displacement, voxel) in cases {
            let oracle = (0..=Q16_MAX).find(|&fraction| {
                capsule_overlaps_voxel(
                    moved_capsule(capsule, displacement, fraction).unwrap(),
                    voxel,
                )
            });
            assert_eq!(
                first_overlap_fraction(capsule, displacement, voxel),
                oracle,
                "capsule={capsule:?}, displacement={displacement:?}, voxel={voxel:?}"
            );
        }
    }

    #[test]
    fn sweep_finds_the_first_diagonal_grazing_contact() {
        let capsule = CapsuleQ8::new(WorldPointQ8::new(-175, 146, -16), 126, 119);
        let displacement = Vec3Q8::new(120, -272, -854);
        let voxel = VoxelCoord::new(-2, 5, -5);
        let oracle = (0..=Q16_MAX).find(|&fraction| {
            capsule_overlaps_voxel(
                moved_capsule(capsule, displacement, fraction).unwrap(),
                voxel,
            )
        });

        assert_eq!(oracle, Some(15_655));
        assert_eq!(first_overlap_fraction(capsule, displacement, voxel), oracle);
    }

    #[test]
    fn sweep_fractions_match_exhaustive_q16_oracles_across_deterministic_cases() {
        for case_index in 0..32_i32 {
            let capsule = CapsuleQ8::new(
                WorldPointQ8::new(
                    (case_index * 137 + 19).rem_euclid(801) - 400,
                    (case_index * 211 + 43).rem_euclid(801) - 400,
                    (case_index * 317 + 71).rem_euclid(801) - 400,
                ),
                32 + u16::try_from((case_index * 13).rem_euclid(97)).unwrap(),
                u16::try_from((case_index * 29).rem_euclid(193)).unwrap(),
            );
            let displacement = Vec3Q8::new(
                (case_index * 251 + 17).rem_euclid(2_049) - 1_024,
                (case_index * 509 + 31).rem_euclid(2_049) - 1_024,
                (case_index * 761 + 47).rem_euclid(2_049) - 1_024,
            );
            let voxel = VoxelCoord::new(
                (case_index * 7 + 3).rem_euclid(33) - 16,
                (case_index * 11 + 5).rem_euclid(33) - 16,
                (case_index * 13 + 7).rem_euclid(33) - 16,
            );
            let oracle = (0..=Q16_MAX).find(|&fraction| {
                capsule_overlaps_voxel(
                    moved_capsule(capsule, displacement, fraction).unwrap(),
                    voxel,
                )
            });

            assert_eq!(
                first_overlap_fraction(capsule, displacement, voxel),
                oracle,
                "case_index={case_index}, capsule={capsule:?}, displacement={displacement:?}, voxel={voxel:?}"
            );
        }
    }

    #[test]
    fn sweep_contacts_are_sorted_and_use_axis_ordered_normals() {
        let y = 400;
        let obstacles = [VoxelCoord::new(2, y, 3), VoxelCoord::new(2, y, 2)];
        let capsule = CapsuleQ8::new(WorldPointQ8::new(32, y * 64 + 32, 3 * 64), 32, 0);
        let changes = obstacles.map(|voxel| (voxel, Voxel::new(crate::GRANITE, 255, 0, 0)));
        let mut app = App::new();
        app.insert_resource(state(changes))
            .insert_resource(SweepQueryResult(Err(QueryError::InvalidInput)))
            .add_systems(
                Update,
                move |read: WorldRead, mut result: ResMut<SweepQueryResult>| {
                    result.0 =
                        read.sweep_capsule(capsule, Vec3Q8::new(256, 0, 0), QueryMask::SOLID);
                },
            );
        app.update();

        let result = app
            .world()
            .resource::<SweepQueryResult>()
            .0
            .as_ref()
            .unwrap();
        assert_eq!(
            result.hits.iter().map(|hit| hit.voxel).collect::<Vec<_>>(),
            [VoxelCoord::new(2, y, 2), VoxelCoord::new(2, y, 3)]
        );
        assert!(
            result
                .hits
                .iter()
                .all(|hit| hit.normal == WorldNormal { x: -1, y: 0, z: 0 })
        );
    }

    #[test]
    fn hit_sorting_is_coordinate_then_normal() {
        let mut hits = [
            WorldHit {
                voxel: VoxelCoord::new(0, 0, 0),
                point: WorldPointQ8::new(0, 0, 0),
                normal: WorldNormal { x: 1, y: 0, z: 0 },
                normal_q16: [65_536, 0, 0],
                material: AIR,
                matched: MatchedQueryMask::Solid,
                matched_mask: QueryMask::SOLID,
                distance_q8: 0,
                revision: 0,
            },
            WorldHit {
                voxel: VoxelCoord::new(0, 0, 0),
                point: WorldPointQ8::new(0, 0, 0),
                normal: WorldNormal { x: 0, y: 1, z: 0 },
                normal_q16: [0, 65_536, 0],
                material: AIR,
                matched: MatchedQueryMask::Solid,
                matched_mask: QueryMask::SOLID,
                distance_q8: 0,
                revision: 0,
            },
            WorldHit {
                voxel: VoxelCoord::new(0, 0, 0),
                point: WorldPointQ8::new(0, 0, 0),
                normal: WorldNormal { x: 0, y: 0, z: -1 },
                normal_q16: [0, 0, -65_536],
                material: AIR,
                matched: MatchedQueryMask::Solid,
                matched_mask: QueryMask::SOLID,
                distance_q8: 0,
                revision: 0,
            },
        ];
        sort_hits(&mut hits);
        assert_eq!(
            hits.map(|hit| hit.normal),
            [
                WorldNormal { x: 1, y: 0, z: 0 },
                WorldNormal { x: 0, y: 1, z: 0 },
                WorldNormal { x: 0, y: 0, z: -1 },
            ]
        );
    }
}
