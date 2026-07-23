use std::collections::BTreeSet;

use serde::Serialize;

use crate::{AabbQ8, ObjectKind, ObjectSpatialIndex, VoxelCoord, WorldPointQ8};

const RADIUS_Q8: i32 = 3 * 256;
const RADIUS_VOXELS: i32 = RADIUS_Q8 / 64;
const CELL_Q8: i32 = 32 * 256;
const BRICK_Q8: i32 = 16 * 64;

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
) -> Option<CurationStressTarget> {
    let mut centers = index
        .placements()
        .iter()
        .filter(|placement| matches!(placement.kind, ObjectKind::TreeA | ObjectKind::TreeB))
        .map(|placement| {
            VoxelCoord::new(
                placement.anchor.x - RADIUS_VOXELS,
                placement.anchor.y,
                placement.anchor.z - RADIUS_VOXELS,
            )
        })
        .collect::<Vec<_>>();
    centers.sort_unstable();
    centers.dedup();
    centers
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

fn score(index: &ObjectSpatialIndex<'_>, center: VoxelCoord) -> Option<CurationStressTarget> {
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
    let keys = keys(bounds);
    let broad = index
        .dependency_cells()
        .iter()
        .filter(|cell| keys.contains(&cell.key))
        .flat_map(|cell| cell.members.iter().copied())
        .collect::<BTreeSet<_>>();
    let exact = broad
        .iter()
        .copied()
        .filter(|&member| overlap(index.records()[member as usize].dependency_bounds, bounds))
        .collect::<Vec<_>>();
    Some(CurationStressTarget {
        center,
        broad_dependency_candidates: u16::try_from(broad.len()).unwrap_or(u16::MAX),
        exact_dependency_ids: u16::try_from(exact.len()).unwrap_or(u16::MAX),
        dependency_bricks: exact
            .into_iter()
            .map(|member| bricks(index.records()[member as usize].dependency_bounds))
            .max()
            .unwrap_or(0),
        changed_bricks: bricks(bounds),
    })
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

#[cfg(test)]
mod tests {
    use crate::{
        ObjectId, ObjectIndexConfig, ObjectPlacement, QuantizedTransform, SpeciesId,
        VoxelObjectShape, build_object_index,
    };

    use super::select_radius_three_stress_target;

    #[test]
    fn score_ties_choose_the_lexicographically_smallest_surface_center() {
        let tree = |id, x| ObjectPlacement {
            id: ObjectId(id),
            kind: crate::ObjectKind::TreeA,
            transform_q: QuantizedTransform {
                translation: crate::WorldPointQ8::new(x * 64, 0, 0),
                yaw_quarter_turns: 0,
                uniform_scale_q8: 256,
            },
            species: Some(SpeciesId(1)),
            shape: VoxelObjectShape::Tree {
                trunk_radius_q8: 64,
                trunk_height_q8: 256,
                canopy_radii_q8: [128; 3],
            },
            anchor: crate::VoxelCoord::new(x, 0, 0),
        };
        let placements = [tree(1, -400), tree(2, 400)];
        let index = build_object_index(&placements, &ObjectIndexConfig::default()).unwrap();
        assert_eq!(
            select_radius_three_stress_target(&index).unwrap().center,
            crate::VoxelCoord::new(-412, 0, -12)
        );
    }

    #[test]
    fn considers_surface_centers_away_from_tree_anchors() {
        let tree = |id, x| ObjectPlacement {
            id: ObjectId(id),
            kind: crate::ObjectKind::TreeA,
            transform_q: QuantizedTransform {
                translation: crate::WorldPointQ8::new(x * 64, 0, 0),
                yaw_quarter_turns: 0,
                uniform_scale_q8: 256,
            },
            species: Some(SpeciesId(1)),
            shape: VoxelObjectShape::Tree {
                trunk_radius_q8: 64,
                trunk_height_q8: 256,
                canopy_radii_q8: [128; 3],
            },
            anchor: crate::VoxelCoord::new(x, 0, 0),
        };
        let placements = [tree(1, 0), tree(2, 24)];
        let index = build_object_index(&placements, &ObjectIndexConfig::default()).unwrap();

        let target = select_radius_three_stress_target(&index).unwrap();
        assert_ne!(target.center, placements[0].anchor);
        assert_ne!(target.center, placements[1].anchor);
    }
}
