//! Bounded upright-capsule observations against authoritative voxel truth.

use crate::{VoxelCoord, WorldBounds, WorldPointQ8};

use super::{QueryError, QueryLimitKind, WorldRead, WorldSample};

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

/// Authoritative collision classes to include in a query.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct QueryMask(u8);

impl QueryMask {
    pub const SOLID: Self = Self(1);
    pub const WATER: Self = Self(2);
    pub const SOLID_AND_WATER: Self = Self(Self::SOLID.0 | Self::WATER.0);

    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != 0 && self.0 & !Self::SOLID_AND_WATER.0 == 0
    }

    const fn includes(self, class: MatchedQueryMask) -> bool {
        match class {
            MatchedQueryMask::Solid => self.0 & Self::SOLID.0 != 0,
            MatchedQueryMask::Water => self.0 & Self::WATER.0 != 0,
        }
    }
}

impl core::ops::BitOr for QueryMask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

/// A quantized collision normal, sorted by axis then sign.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorldNormal {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

/// One authoritative capsule contact.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorldHit {
    pub voxel: VoxelCoord,
    pub point: WorldPointQ8,
    pub normal: WorldNormal,
    pub material: crate::MaterialId,
    pub matched: MatchedQueryMask,
    pub distance_q8: u32,
    pub revision: u64,
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
        hits.sort_by_key(|hit| (hit.voxel, hit.normal));
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
        if squared_length(displacement) > i64::from(MAX_SWEEP_DISPLACEMENT_Q8).pow(2) {
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
        let mut impact_voxels = Vec::with_capacity(usize::from(MAX_QUERY_HITS));
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
                    push_impact(&mut impact_voxels, voxel, sample, class)?;
                }
                Some(current) if fraction < current => {
                    impact = Some(fraction);
                    impact_voxels.clear();
                    push_impact(&mut impact_voxels, voxel, sample, class)?;
                }
                Some(current) if fraction == current => {
                    push_impact(&mut impact_voxels, voxel, sample, class)?;
                }
                Some(_) => {}
            }
        }

        let impact_fraction = impact.unwrap_or(Q16_MAX);
        let safe_fraction = impact.map_or(Q16_MAX, |value| value.saturating_sub(1));
        let end_capsule = moved_capsule(capsule, displacement, safe_fraction)?;
        let mut hits = Vec::with_capacity(usize::from(MAX_QUERY_HITS));
        let contact_capsule = moved_capsule(capsule, displacement, impact_fraction)?;
        for (voxel, sample, class) in impact_voxels {
            push_hit(&mut hits, hit_for(contact_capsule, voxel, sample, class))?;
        }
        hits.sort_by_key(|hit| (hit.voxel, hit.normal));
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

fn squared_length(displacement: Vec3Q8) -> i64 {
    let x = i64::from(displacement.x);
    let y = i64::from(displacement.y);
    let z = i64::from(displacement.z);
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
    if sample.solid_collision && mask.includes(MatchedQueryMask::Solid) {
        Some(MatchedQueryMask::Solid)
    } else if sample.water_volume && mask.includes(MatchedQueryMask::Water) {
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
    if capsule_overlaps_voxel(capsule, voxel) {
        return Some(0);
    }
    let mut low = 0;
    let mut high = Q16_MAX;
    for _ in 0..20 {
        let third = (high - low) / 3;
        let left = low + third;
        let right = high - third;
        if distance_squared_at(capsule, displacement, voxel, left)
            <= distance_squared_at(capsule, displacement, voxel, right)
        {
            high = right;
        } else {
            low = left;
        }
    }
    let mut minimum = low;
    for fraction in low..=high {
        if distance_squared_at(capsule, displacement, voxel, fraction)
            < distance_squared_at(capsule, displacement, voxel, minimum)
        {
            minimum = fraction;
        }
    }
    if !capsule_overlaps_voxel(moved_capsule(capsule, displacement, minimum).ok()?, voxel) {
        return None;
    }
    let mut first = 1;
    let mut last = minimum;
    while first < last {
        let middle = first + (last - first) / 2;
        if capsule_overlaps_voxel(moved_capsule(capsule, displacement, middle).ok()?, voxel) {
            last = middle;
        } else {
            first = middle + 1;
        }
    }
    Some(first)
}

fn distance_squared_at(
    capsule: CapsuleQ8,
    displacement: Vec3Q8,
    voxel: VoxelCoord,
    fraction: i64,
) -> i64 {
    let moved =
        moved_capsule(capsule, displacement, fraction).expect("validated sweep cannot overflow");
    let (x, y, z) = capsule_aabb_distance(moved, voxel);
    x * x + y * y + z * z
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
        material: sample.material,
        matched,
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

fn push_impact(
    impacts: &mut Vec<(VoxelCoord, WorldSample, MatchedQueryMask)>,
    voxel: VoxelCoord,
    sample: WorldSample,
    matched: MatchedQueryMask,
) -> Result<(), QueryError> {
    if impacts.len() == usize::from(MAX_QUERY_HITS) {
        return Err(QueryError::LimitExceeded(QueryLimitKind::ResultCount));
    }
    impacts.push((voxel, sample, matched));
    Ok(())
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
            squared_length(Vec3Q8::new(3_073, 0, 0)) > i64::from(MAX_SWEEP_DISPLACEMENT_Q8).pow(2)
        );

        let max = CapsuleQ8::new(center, MAX_CAPSULE_RADIUS_Q8, MAX_CAPSULE_HALF_SEGMENT_Q8);
        let end = moved_capsule(max, Vec3Q8::new(2_000, 2_000, 2_000), Q16_MAX).unwrap();
        assert!(
            candidate_range(max, end, bounds).unwrap().count()
                > u32::from(MAX_SWEEP_CANDIDATE_TESTS)
        );
    }

    #[test]
    fn hit_sorting_is_coordinate_then_normal() {
        let mut hits = [
            WorldHit {
                voxel: VoxelCoord::new(1, 0, 0),
                point: WorldPointQ8::new(0, 0, 0),
                normal: WorldNormal { x: 1, y: 0, z: 0 },
                material: AIR,
                matched: MatchedQueryMask::Solid,
                distance_q8: 0,
                revision: 0,
            },
            WorldHit {
                voxel: VoxelCoord::new(0, 0, 0),
                point: WorldPointQ8::new(0, 0, 0),
                normal: WorldNormal { x: 0, y: 1, z: 0 },
                material: AIR,
                matched: MatchedQueryMask::Solid,
                distance_q8: 0,
                revision: 0,
            },
        ];
        hits.sort_by_key(|hit| (hit.voxel, hit.normal));
        assert_eq!(hits[0].voxel, VoxelCoord::new(0, 0, 0));
    }
}
