//! Deterministic selection of the radius-three-metre curation stress target.

use std::collections::BTreeSet;

use moria_world::{AabbQ8, ObjectSpatialIndex, VoxelCoord, WorldPointQ8};

const Q8_PER_METER: i32 = 256;
const RADIUS_Q8: i32 = 3 * Q8_PER_METER;
const DEPENDENCY_CELL_Q8: i32 = 32 * Q8_PER_METER;
const BRICK_Q8: i32 = 16 * 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct StressTarget {
    pub center: VoxelCoord,
    pub broad_dependency_candidates: u16,
    pub exact_dependency_ids: u16,
    pub dependency_bricks: u16,
    pub changed_bricks: u16,
}

/// Selects the maximum-candidate target, breaking score ties by voxel center.
pub(crate) fn select_radius_three_stress_target(
    index: &ObjectSpatialIndex<'_>,
) -> Option<StressTarget> {
    let mut candidates = index
        .placements()
        .iter()
        .map(|placement| placement.anchor)
        .collect::<Vec<_>>();
    candidates.sort_unstable();
    candidates.dedup();
    candidates
        .into_iter()
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

fn score(index: &ObjectSpatialIndex<'_>, center: VoxelCoord) -> Option<StressTarget> {
    let point = WorldPointQ8::new(center.x * 64, center.y * 64, center.z * 64);
    let bounds = AabbQ8::new(
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
    .ok()?;
    let keys = dependency_keys(bounds);
    let broad_members = index
        .dependency_cells()
        .iter()
        .filter(|cell| keys.contains(&cell.key))
        .flat_map(|cell| cell.members.iter().copied())
        .collect::<BTreeSet<_>>();
    let exact = broad_members
        .iter()
        .filter(|&&member| overlaps(index.records()[member as usize].dependency_bounds, bounds))
        .count();
    let dependency_bricks = broad_members
        .iter()
        .filter(|&&member| overlaps(index.records()[member as usize].dependency_bounds, bounds))
        .map(|&member| brick_count(index.records()[member as usize].dependency_bounds))
        .fold(0_u16, u16::saturating_add);
    Some(StressTarget {
        center,
        broad_dependency_candidates: u16::try_from(broad_members.len()).unwrap_or(u16::MAX),
        exact_dependency_ids: u16::try_from(exact).unwrap_or(u16::MAX),
        dependency_bricks,
        changed_bricks: brick_count(bounds),
    })
}

fn dependency_keys(bounds: AabbQ8) -> BTreeSet<moria_world::DependencyGridCellKey> {
    let min_x = bounds.min.x.div_euclid(DEPENDENCY_CELL_Q8);
    let max_x = (bounds.max_exclusive.x - 1).div_euclid(DEPENDENCY_CELL_Q8);
    let min_z = bounds.min.z.div_euclid(DEPENDENCY_CELL_Q8);
    let max_z = (bounds.max_exclusive.z - 1).div_euclid(DEPENDENCY_CELL_Q8);
    (min_x..=max_x)
        .flat_map(|x| (min_z..=max_z).map(move |z| (x, z)))
        .filter_map(|(x, z)| {
            i16::try_from(x)
                .ok()
                .zip(i16::try_from(z).ok())
                .map(|(x, z)| moria_world::DependencyGridCellKey { x, z })
        })
        .collect()
}

fn overlaps(left: AabbQ8, right: AabbQ8) -> bool {
    left.min.x < right.max_exclusive.x
        && left.max_exclusive.x > right.min.x
        && left.min.y < right.max_exclusive.y
        && left.max_exclusive.y > right.min.y
        && left.min.z < right.max_exclusive.z
        && left.max_exclusive.z > right.min.z
}

fn brick_count(bounds: AabbQ8) -> u16 {
    let extent = |min: i32, max: i32| (max - 1).div_euclid(BRICK_Q8) - min.div_euclid(BRICK_Q8) + 1;
    let count = i64::from(extent(bounds.min.x, bounds.max_exclusive.x))
        * i64::from(extent(bounds.min.y, bounds.max_exclusive.y))
        * i64::from(extent(bounds.min.z, bounds.max_exclusive.z));
    u16::try_from(count).unwrap_or(u16::MAX)
}

#[cfg(test)]
mod tests {
    use moria_world::{ObjectIndexConfig, build_object_index, generate_manifest};

    use super::select_radius_three_stress_target;

    #[test]
    fn selects_the_same_lexicographic_maximum_on_every_run() {
        let manifest = generate_manifest(
            include_bytes!("../../../assets/config/product_one_region.ron"),
            include_bytes!("../../../assets/stamps/ruin_p1.ron"),
        )
        .unwrap();
        let index = build_object_index(&manifest.objects, &ObjectIndexConfig::default()).unwrap();

        let first = select_radius_three_stress_target(&index).unwrap();
        let second = select_radius_three_stress_target(&index).unwrap();

        assert_eq!(first, second);
        assert!(first.broad_dependency_candidates >= first.exact_dependency_ids);
    }
}
