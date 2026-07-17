//! Exact, bounded object-shape conflict validation.

use std::collections::BTreeSet;

use crate::{
    ManifestError, RuinPoi, SparseVoxelStamp, VOXEL_EDGE_Q8, VoxelCoord, raw_shape_contains,
};

use super::ObjectSpatialIndex;

/// Validates non-ruin shape disjointness and conflicts with authored ruin runs.
///
/// The sparse stamp is supplied by the caller because `RuinPoi` deliberately
/// stores only its immutable asset key. No dependency coordinates are retained.
pub fn validate_object_shape_disjointness(
    index: &ObjectSpatialIndex<'_>,
    ruin: &RuinPoi,
    stamp: &SparseVoxelStamp,
) -> Result<(), ManifestError> {
    let mut pairs = BTreeSet::new();
    for cell in index.dependency_cells() {
        for (offset, &left) in cell.members.iter().enumerate() {
            for &right in &cell.members[offset + 1..] {
                pairs.insert((left.min(right), left.max(right)));
            }
        }
    }
    let mut ordered = pairs.into_iter().collect::<Vec<_>>();
    ordered.sort_unstable_by_key(|&(left, right)| {
        let placements = index.placements();
        (placements[left as usize].id, placements[right as usize].id)
    });
    for (left, right) in ordered {
        let left_index = left as usize;
        let right_index = right as usize;
        let left_record = index.records()[left_index];
        let right_record = index.records()[right_index];
        if let Some(first_voxel) = first_shared_solid(
            &index.placements()[left_index],
            &index.placements()[right_index],
            left_record.raw_bounds,
            right_record.raw_bounds,
        ) {
            let left_id = index.placements()[left_index].id;
            let right_id = index.placements()[right_index].id;
            return Err(ManifestError::ObjectShapeOverlap {
                lower_id: left_id.min(right_id),
                higher_id: left_id.max(right_id),
                first_voxel,
            });
        }
    }

    let mut object_indices = (0..index.placements().len()).collect::<Vec<_>>();
    object_indices.sort_unstable_by_key(|&object_index| index.placements()[object_index].id);
    let ruin_coordinates = transformed_authored_coordinates(ruin, stamp);
    for object_index in object_indices {
        let placement = &index.placements()[object_index];
        for &coordinate in &ruin_coordinates {
            if raw_shape_contains(placement, coordinate) {
                return Err(ManifestError::ObjectRuinOverlap {
                    object_id: index.placements()[object_index].id,
                    first_voxel: coordinate,
                });
            }
        }
    }
    Ok(())
}

fn first_shared_solid(
    left: &crate::ObjectPlacement,
    right: &crate::ObjectPlacement,
    left_bounds: crate::AabbQ8,
    right_bounds: crate::AabbQ8,
) -> Option<VoxelCoord> {
    let min = VoxelCoord::new(
        left_bounds
            .min
            .x
            .max(right_bounds.min.x)
            .div_euclid(VOXEL_EDGE_Q8),
        left_bounds
            .min
            .y
            .max(right_bounds.min.y)
            .div_euclid(VOXEL_EDGE_Q8),
        left_bounds
            .min
            .z
            .max(right_bounds.min.z)
            .div_euclid(VOXEL_EDGE_Q8),
    );
    let max = VoxelCoord::new(
        (left_bounds
            .max_exclusive
            .x
            .min(right_bounds.max_exclusive.x)
            - 1)
        .div_euclid(VOXEL_EDGE_Q8),
        (left_bounds
            .max_exclusive
            .y
            .min(right_bounds.max_exclusive.y)
            - 1)
        .div_euclid(VOXEL_EDGE_Q8),
        (left_bounds
            .max_exclusive
            .z
            .min(right_bounds.max_exclusive.z)
            - 1)
        .div_euclid(VOXEL_EDGE_Q8),
    );
    if min.x > max.x || min.y > max.y || min.z > max.z {
        return None;
    }
    for x in min.x..=max.x {
        for y in min.y..=max.y {
            for z in min.z..=max.z {
                let coordinate = VoxelCoord::new(x, y, z);
                if raw_shape_contains(left, coordinate) && raw_shape_contains(right, coordinate) {
                    return Some(coordinate);
                }
            }
        }
    }
    None
}

fn transformed_authored_coordinates(ruin: &RuinPoi, stamp: &SparseVoxelStamp) -> Vec<VoxelCoord> {
    let mut coordinates = Vec::new();
    let width = u32::from(stamp.size_voxels[0]);
    let depth = u32::from(stamp.size_voxels[2]);
    let origin = ruin.placement.transform_q.translation;
    if origin.x.rem_euclid(VOXEL_EDGE_Q8) != 0
        || origin.y.rem_euclid(VOXEL_EDGE_Q8) != 0
        || origin.z.rem_euclid(VOXEL_EDGE_Q8) != 0
    {
        return coordinates;
    }
    for run in &stamp.runs {
        for linear in run.start_linear..run.start_linear + u32::from(run.len) {
            let local = VoxelCoord::new(
                (linear % width) as i32,
                (linear / width / depth) as i32,
                ((linear / width) % depth) as i32,
            );
            let rotated = rotate_about_pivot(
                local,
                stamp.pivot_voxel,
                ruin.placement.transform_q.yaw_quarter_turns,
            );
            coordinates.push(VoxelCoord::new(
                origin.x.div_euclid(VOXEL_EDGE_Q8) + rotated.x,
                origin.y.div_euclid(VOXEL_EDGE_Q8) + rotated.y,
                origin.z.div_euclid(VOXEL_EDGE_Q8) + rotated.z,
            ));
        }
    }
    coordinates.sort_unstable();
    coordinates.dedup();
    coordinates
}

fn rotate_about_pivot(coord: VoxelCoord, pivot: [i16; 3], quarter_turns: u8) -> VoxelCoord {
    let x = coord.x - i32::from(pivot[0]);
    let z = coord.z - i32::from(pivot[2]);
    let (x, z) = match quarter_turns % 4 {
        0 => (x, z),
        1 => (-z, x),
        2 => (-x, -z),
        _ => (z, -x),
    };
    VoxelCoord::new(x + i32::from(pivot[0]), coord.y, z + i32::from(pivot[2]))
}
