//! Fixed-point, bounded ray-query values and deterministic voxel traversal.

use crate::{MaterialId, VOXEL_EDGE_Q8, VoxelCoord, WorldPointQ8};

use super::{QueryError, QueryLimitKind, WorldSample};

/// The Q8 distance limit for one synchronous world ray (64 m).
pub const MAX_RAY_DISTANCE_Q8: u32 = 16_384;
/// The maximum number of voxels a synchronous world ray may inspect.
pub const MAX_RAY_VOXEL_VISITS: u16 = 448;

const Q16_ONE: i64 = 65_536;
const NORMALIZED_Q16_SQUARED: i64 = Q16_ONE * Q16_ONE;
// Rounding each of the three components to Q16 can move the squared length by
// roughly three Q16 units.  Keep the acceptance window integral and symmetric.
const NORMALIZED_Q16_TOLERANCE: i64 = 3 * Q16_ONE;

/// A normalized Q16 direction ray beginning at a Q8 world position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorldRayQ8 {
    origin: WorldPointQ8,
    direction_q16: [i32; 3],
}

impl WorldRayQ8 {
    /// Creates a ray only when `direction_q16` has unit length in Q16.
    pub fn new(origin: WorldPointQ8, direction_q16: [i32; 3]) -> Result<Self, QueryError> {
        if !is_normalized_q16(direction_q16) {
            return Err(QueryError::InvalidInput);
        }
        Ok(Self {
            origin,
            direction_q16,
        })
    }

    #[must_use]
    pub const fn origin(self) -> WorldPointQ8 {
        self.origin
    }

    #[must_use]
    pub const fn direction_q16(self) -> [i32; 3] {
        self.direction_q16
    }
}

/// The voxel-volume classes a query may observe.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct QueryMask(u8);

impl QueryMask {
    pub const EMPTY: Self = Self(0);
    pub const SOLID: Self = Self(1);
    pub const WATER: Self = Self(2);
    pub const ALL: Self = Self(Self::SOLID.0 | Self::WATER.0);

    #[must_use]
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn matches(self, class: Self) -> bool {
        self.0 & class.0 != 0
    }
}

impl core::ops::BitOr for QueryMask {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        Self(self.0 | other.0)
    }
}

/// The first voxel volume matched by a ray query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorldHit {
    pub voxel: VoxelCoord,
    pub point: WorldPointQ8,
    /// A deterministic Q16 face normal. Origin-cell hits use the zero vector.
    pub normal_q16: [i32; 3],
    pub material: MaterialId,
    pub matched_mask: QueryMask,
    pub distance_q8: u32,
    pub revision: u64,
}

pub(super) fn cast(
    ray: WorldRayQ8,
    max_distance_q8: u32,
    mask: QueryMask,
    mut sample: impl FnMut(VoxelCoord) -> WorldSample,
) -> Result<Option<WorldHit>, QueryError> {
    if max_distance_q8 > MAX_RAY_DISTANCE_Q8 {
        return Err(QueryError::LimitExceeded(QueryLimitKind::RayDistance));
    }
    if mask.is_empty() || !is_normalized_q16(ray.direction_q16) {
        return Err(QueryError::InvalidInput);
    }

    let mut voxel = ray
        .origin
        .to_voxel_coord()
        .map_err(|_| QueryError::OutOfBounds)?;
    preflight_voxel_visits(ray, voxel, max_distance_q8)?;

    let direction = ray.direction_q16;
    let mut next_numerator = [i64::MAX; 3];
    let mut numerator_step = [i64::MAX; 3];
    let mut denominator = [1_i64; 3];
    let mut step = [0_i32; 3];
    initialize_axis(
        ray.origin.x,
        voxel.x,
        direction[0],
        &mut step[0],
        &mut next_numerator[0],
        &mut numerator_step[0],
        &mut denominator[0],
    );
    initialize_axis(
        ray.origin.y,
        voxel.y,
        direction[1],
        &mut step[1],
        &mut next_numerator[1],
        &mut numerator_step[1],
        &mut denominator[1],
    );
    initialize_axis(
        ray.origin.z,
        voxel.z,
        direction[2],
        &mut step[2],
        &mut next_numerator[2],
        &mut numerator_step[2],
        &mut denominator[2],
    );

    let mut visits = 0_u16;
    let mut distance = RayDistance::ZERO;
    let mut normal_q16 = [0; 3];
    loop {
        if visits == MAX_RAY_VOXEL_VISITS {
            return Err(QueryError::LimitExceeded(QueryLimitKind::RayVoxelVisits));
        }
        visits += 1;

        let world_sample = sample(voxel);
        if let Some(matched_mask) = matched_mask(world_sample, mask) {
            return Ok(Some(WorldHit {
                voxel,
                point: point_at_distance(ray, distance),
                normal_q16,
                material: world_sample.material,
                matched_mask,
                distance_q8: distance.q8_floor() as u32,
                revision: world_sample.revision,
            }));
        }

        let next_axis = earliest_axis(next_numerator, denominator);
        let next = RayDistance {
            numerator: next_numerator[next_axis],
            denominator: denominator[next_axis],
        };
        if next.exceeds(max_distance_q8) {
            return Ok(None);
        }
        distance = next;
        normal_q16 = [0; 3];
        for axis in 0..3 {
            if (RayDistance {
                numerator: next_numerator[axis],
                denominator: denominator[axis],
            }) == next
            {
                let axis_step = step[axis];
                let coordinate = match axis {
                    0 => &mut voxel.x,
                    1 => &mut voxel.y,
                    2 => &mut voxel.z,
                    _ => unreachable!("ray axes are fixed"),
                };
                *coordinate += axis_step;
                next_numerator[axis] = next_numerator[axis].saturating_add(numerator_step[axis]);
                if normal_q16 == [0; 3] {
                    normal_q16[axis] = -axis_step * Q16_ONE as i32;
                }
            }
        }
        if !voxel.is_in_region() {
            return Ok(None);
        }
    }
}

fn preflight_voxel_visits(
    ray: WorldRayQ8,
    mut voxel: VoxelCoord,
    max_distance_q8: u32,
) -> Result<(), QueryError> {
    let direction = ray.direction_q16;
    let mut next_numerator = [i64::MAX; 3];
    let mut numerator_step = [i64::MAX; 3];
    let mut denominator = [1_i64; 3];
    let mut step = [0_i32; 3];
    initialize_axis(
        ray.origin.x,
        voxel.x,
        direction[0],
        &mut step[0],
        &mut next_numerator[0],
        &mut numerator_step[0],
        &mut denominator[0],
    );
    initialize_axis(
        ray.origin.y,
        voxel.y,
        direction[1],
        &mut step[1],
        &mut next_numerator[1],
        &mut numerator_step[1],
        &mut denominator[1],
    );
    initialize_axis(
        ray.origin.z,
        voxel.z,
        direction[2],
        &mut step[2],
        &mut next_numerator[2],
        &mut numerator_step[2],
        &mut denominator[2],
    );

    let mut visits = 1_u16;
    loop {
        let axis = earliest_axis(next_numerator, denominator);
        let next = RayDistance {
            numerator: next_numerator[axis],
            denominator: denominator[axis],
        };
        if next.exceeds(max_distance_q8) {
            return Ok(());
        }
        for axis in 0..3 {
            if (RayDistance {
                numerator: next_numerator[axis],
                denominator: denominator[axis],
            }) == next
            {
                match axis {
                    0 => voxel.x += step[axis],
                    1 => voxel.y += step[axis],
                    2 => voxel.z += step[axis],
                    _ => unreachable!("ray axes are fixed"),
                }
                next_numerator[axis] = next_numerator[axis].saturating_add(numerator_step[axis]);
            }
        }
        if !voxel.is_in_region() {
            return Ok(());
        }
        if visits == MAX_RAY_VOXEL_VISITS {
            return Err(QueryError::LimitExceeded(QueryLimitKind::RayVoxelVisits));
        }
        visits += 1;
    }
}

fn is_normalized_q16(direction: [i32; 3]) -> bool {
    let length_squared = direction.into_iter().fold(0_i64, |sum, component| {
        sum + i64::from(component) * i64::from(component)
    });
    (length_squared - NORMALIZED_Q16_SQUARED).abs() <= NORMALIZED_Q16_TOLERANCE
}

fn initialize_axis(
    origin_q8: i32,
    voxel: i32,
    direction_q16: i32,
    step: &mut i32,
    next_numerator: &mut i64,
    numerator_step: &mut i64,
    denominator: &mut i64,
) {
    if direction_q16 == 0 {
        return;
    }

    let direction = i64::from(direction_q16);
    *step = direction_q16.signum();
    let boundary_q8 = if direction_q16 > 0 {
        i64::from(voxel + 1) * i64::from(VOXEL_EDGE_Q8)
    } else {
        i64::from(voxel) * i64::from(VOXEL_EDGE_Q8)
    };
    let delta_q8 = (boundary_q8 - i64::from(origin_q8)).abs();
    *next_numerator = delta_q8 * Q16_ONE;
    *numerator_step = i64::from(VOXEL_EDGE_Q8) * Q16_ONE;
    *denominator = direction.abs();
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct RayDistance {
    numerator: i64,
    denominator: i64,
}

impl RayDistance {
    const ZERO: Self = Self {
        numerator: 0,
        denominator: 1,
    };

    fn exceeds(self, distance_q8: u32) -> bool {
        self.numerator > i64::from(distance_q8) * self.denominator
    }

    fn q8_floor(self) -> i64 {
        self.numerator / self.denominator
    }
}

impl Ord for RayDistance {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.numerator == i64::MAX || other.numerator == i64::MAX {
            return self.numerator.cmp(&other.numerator);
        }
        (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
    }
}

impl PartialOrd for RayDistance {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn earliest_axis(numerators: [i64; 3], denominators: [i64; 3]) -> usize {
    let mut earliest = 0;
    for axis in 1..3 {
        if (RayDistance {
            numerator: numerators[axis],
            denominator: denominators[axis],
        }) < (RayDistance {
            numerator: numerators[earliest],
            denominator: denominators[earliest],
        }) {
            earliest = axis;
        }
    }
    earliest
}

fn matched_mask(sample: WorldSample, requested: QueryMask) -> Option<QueryMask> {
    if requested.matches(QueryMask::SOLID) && sample.solid_collision {
        Some(QueryMask::SOLID)
    } else if requested.matches(QueryMask::WATER) && sample.water_volume {
        Some(QueryMask::WATER)
    } else {
        None
    }
}

fn point_at_distance(ray: WorldRayQ8, distance: RayDistance) -> WorldPointQ8 {
    let direction = ray.direction_q16;
    WorldPointQ8::new(
        ray.origin.x
            + ((i64::from(direction[0]) * distance.numerator) / (Q16_ONE * distance.denominator))
                as i32,
        ray.origin.y
            + ((i64::from(direction[1]) * distance.numerator) / (Q16_ONE * distance.denominator))
                as i32,
        ray.origin.z
            + ((i64::from(direction[2]) * distance.numerator) / (Q16_ONE * distance.denominator))
                as i32,
    )
}

#[cfg(test)]
mod tests {
    use crate::{AIR, GRANITE};

    use super::*;

    fn sample(coordinate: VoxelCoord, solid: VoxelCoord) -> WorldSample {
        let voxel = coordinate == solid;
        WorldSample {
            coordinate,
            material: if voxel { GRANITE } else { AIR },
            density: u8::from(voxel) * u8::MAX,
            state: 0,
            material_present: voxel,
            solid_collision: voxel,
            water_volume: false,
            revision: 9,
        }
    }

    #[test]
    fn axis_ray_reports_the_first_solid_voxel_and_face_normal() {
        let ray = WorldRayQ8::new(WorldPointQ8::new(0, 0, 0), [65_536, 0, 0]).unwrap();
        let hit = cast(ray, 1_000, QueryMask::SOLID, |coordinate| {
            sample(coordinate, VoxelCoord::new(3, 0, 0))
        })
        .unwrap()
        .unwrap();

        assert_eq!(hit.voxel, VoxelCoord::new(3, 0, 0));
        assert_eq!(hit.distance_q8, 192);
        assert_eq!(hit.normal_q16, [-65_536, 0, 0]);
        assert_eq!(hit.matched_mask, QueryMask::SOLID);
    }

    #[test]
    fn diagonal_ties_step_all_axes_with_a_stable_first_axis_normal() {
        let ray = WorldRayQ8::new(WorldPointQ8::new(0, 0, 0), [46_341, 46_341, 0]).unwrap();
        let hit = cast(ray, 1_000, QueryMask::SOLID, |coordinate| {
            sample(coordinate, VoxelCoord::new(1, 1, 0))
        })
        .unwrap()
        .unwrap();

        assert_eq!(hit.voxel, VoxelCoord::new(1, 1, 0));
        assert_eq!(hit.normal_q16, [-65_536, 0, 0]);
    }

    #[test]
    fn negative_direction_and_masks_select_only_the_requested_volume() {
        let ray = WorldRayQ8::new(WorldPointQ8::new(-1, 0, 0), [-65_536, 0, 0]).unwrap();
        let water = VoxelCoord::new(-2, 0, 0);
        assert!(
            cast(ray, 128, QueryMask::SOLID, |coordinate| WorldSample {
                coordinate,
                material: if coordinate == water {
                    crate::WATER
                } else {
                    AIR
                },
                density: if coordinate == water { u8::MAX } else { 0 },
                state: 0,
                material_present: coordinate == water,
                solid_collision: false,
                water_volume: coordinate == water,
                revision: 1,
            })
            .unwrap()
            .is_none()
        );

        let hit = cast(ray, 128, QueryMask::WATER, |coordinate| WorldSample {
            coordinate,
            material: if coordinate == water {
                crate::WATER
            } else {
                AIR
            },
            density: if coordinate == water { u8::MAX } else { 0 },
            state: 0,
            material_present: coordinate == water,
            solid_collision: false,
            water_volume: coordinate == water,
            revision: 1,
        })
        .unwrap()
        .unwrap();
        assert_eq!(hit.voxel, water);
        assert_eq!(hit.normal_q16, [65_536, 0, 0]);
    }

    #[test]
    fn exact_distance_is_accepted_and_one_q8_over_is_rejected_before_sampling() {
        let ray = WorldRayQ8::new(WorldPointQ8::new(0, 0, 0), [65_536, 0, 0]).unwrap();
        let mut samples = 0;
        assert!(
            cast(ray, MAX_RAY_DISTANCE_Q8, QueryMask::SOLID, |_| {
                samples += 1;
                sample(VoxelCoord::new(0, 0, 0), VoxelCoord::new(10_000, 0, 0))
            })
            .unwrap()
            .is_none()
        );
        assert!(samples > 0);

        samples = 0;
        assert_eq!(
            cast(ray, MAX_RAY_DISTANCE_Q8 + 1, QueryMask::SOLID, |_| {
                samples += 1;
                sample(VoxelCoord::new(0, 0, 0), VoxelCoord::new(10_000, 0, 0))
            }),
            Err(QueryError::LimitExceeded(QueryLimitKind::RayDistance))
        );
        assert_eq!(samples, 0);
    }

    #[test]
    fn empty_mask_rejects_before_sampling() {
        let ray = WorldRayQ8::new(WorldPointQ8::new(0, 0, 0), [65_536, 0, 0]).unwrap();
        assert_eq!(
            cast(ray, 0, QueryMask::EMPTY, |_| unreachable!(
                "must not sample"
            )),
            Err(QueryError::InvalidInput)
        );
    }
}
