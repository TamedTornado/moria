//! Fixed-point registered-object sampling.

use crate::{
    AabbQ8, GRANITE, LEAF, ObjectPlacement, SparseVoxelStamp, VOXEL_EDGE_Q8, Voxel, VoxelCoord,
    VoxelObjectShape, WOOD, WorldPointQ8,
};

/// One relative voxel read declared by the object extraction contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VoxelOffset {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl VoxelOffset {
    const fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }
}

const fn extraction_stencil() -> [VoxelOffset; 125] {
    let mut offsets = [VoxelOffset::new(0, 0, 0); 125];
    let mut index = 0;
    let mut y = -2;
    while y <= 2 {
        let mut z = -2;
        while z <= 2 {
            let mut x = -2;
            while x <= 2 {
                offsets[index] = VoxelOffset::new(x, y, z);
                index += 1;
                x += 1;
            }
            z += 1;
        }
        y += 1;
    }
    offsets
}

/// The union of all currently supported object extractor reads.
///
/// A two-voxel halo covers gradients, material weighting, provenance, and the
/// coarsest supported downsampling inputs while remaining below the 512-read
/// contract cap.
pub const OBJECT_EXTRACTION_STENCIL: [VoxelOffset; 125] = extraction_stencil();

/// Returns the base voxel contributed by a non-ruin analytic object shape.
#[must_use]
pub fn sample_object_shape(placement: &ObjectPlacement, coord: VoxelCoord) -> Option<Voxel> {
    let local = local_center_q8(placement, coord);
    match &placement.shape {
        VoxelObjectShape::Tree {
            trunk_radius_q8,
            trunk_height_q8,
            canopy_radii_q8,
        } => {
            let scale = i64::from(placement.transform_q.uniform_scale_q8);
            let trunk_radius = scaled(*trunk_radius_q8, scale);
            let trunk_height = scaled(*trunk_height_q8, scale);
            if cylinder_contains(local, trunk_radius, trunk_height) {
                Some(solid(WOOD))
            } else {
                let canopy = [
                    scaled(canopy_radii_q8[0], scale),
                    scaled(canopy_radii_q8[1], scale),
                    scaled(canopy_radii_q8[2], scale),
                ];
                let canopy_center = [0, trunk_height - canopy[1], 0];
                ellipsoid_contains(local, canopy_center, canopy).then_some(solid(LEAF))
            }
        }
        VoxelObjectShape::Bush { radii_q8 } => ellipsoid_contains(
            local,
            [0, 0, 0],
            scaled_radii(*radii_q8, placement.transform_q.uniform_scale_q8),
        )
        .then_some(solid(LEAF)),
        VoxelObjectShape::Boulder {
            radii_q8,
            perturbation_key,
        }
        | VoxelObjectShape::Rock {
            radii_q8,
            perturbation_key,
        } => perturbed_ellipsoid_contains(
            local,
            scaled_radii(*radii_q8, placement.transform_q.uniform_scale_q8),
            *perturbation_key,
        )
        .then_some(solid(GRANITE)),
        VoxelObjectShape::Stump {
            radius_q8,
            height_q8,
        } => cylinder_contains(
            local,
            scaled(
                *radius_q8,
                i64::from(placement.transform_q.uniform_scale_q8),
            ),
            scaled(
                *height_q8,
                i64::from(placement.transform_q.uniform_scale_q8),
            ),
        )
        .then_some(solid(WOOD)),
        VoxelObjectShape::SparseStamp { .. } => None,
    }
}

/// Returns whether an analytic object has a solid raw-shape cell at `coord`.
#[must_use]
pub fn raw_shape_contains(placement: &ObjectPlacement, coord: VoxelCoord) -> bool {
    sample_object_shape(placement, coord).is_some()
}

/// Samples one transformed sparse-stamp coordinate, including authored air carves.
#[must_use]
pub fn sample_sparse_stamp(
    placement: &ObjectPlacement,
    stamp: &SparseVoxelStamp,
    coord: VoxelCoord,
) -> Option<Voxel> {
    let translation = placement.transform_q.translation;
    if translation.x.rem_euclid(VOXEL_EDGE_Q8) != 0
        || translation.y.rem_euclid(VOXEL_EDGE_Q8) != 0
        || translation.z.rem_euclid(VOXEL_EDGE_Q8) != 0
    {
        return None;
    }
    let relative = VoxelCoord::new(
        coord.x - translation.x.div_euclid(VOXEL_EDGE_Q8),
        coord.y - translation.y.div_euclid(VOXEL_EDGE_Q8),
        coord.z - translation.z.div_euclid(VOXEL_EDGE_Q8),
    );
    let local = inverse_rotate(
        relative,
        stamp.pivot_voxel,
        placement.transform_q.yaw_quarter_turns,
    );
    if local.x < 0
        || local.y < 0
        || local.z < 0
        || local.x >= i32::from(stamp.size_voxels[0])
        || local.y >= i32::from(stamp.size_voxels[1])
        || local.z >= i32::from(stamp.size_voxels[2])
    {
        return None;
    }
    let linear = u32::try_from(
        local.x
            + i32::from(stamp.size_voxels[0])
                * (local.z + i32::from(stamp.size_voxels[2]) * local.y),
    )
    .expect("validated local stamp coordinate is non-negative");
    stamp.runs.iter().find_map(|run| {
        let end = run.start_linear + u32::from(run.len);
        (linear >= run.start_linear && linear < end).then(|| {
            Voxel::new(
                stamp.palette[usize::from(run.palette_index)],
                run.density,
                0,
                0,
            )
        })
    })
}

/// Conservative Q8 bounds for the raw analytic shape.
#[must_use]
pub fn raw_shape_bounds(placement: &ObjectPlacement) -> Option<AabbQ8> {
    let scale = i64::from(placement.transform_q.uniform_scale_q8);
    let (below, above) = match &placement.shape {
        VoxelObjectShape::Tree {
            trunk_radius_q8,
            trunk_height_q8,
            canopy_radii_q8,
        } => {
            let height = scaled(*trunk_height_q8, scale);
            let radii = scaled_radii(*canopy_radii_q8, placement.transform_q.uniform_scale_q8);
            let (canopy_x, canopy_z) =
                horizontal_extents(radii, placement.transform_q.yaw_quarter_turns);
            let trunk_radius = scaled(*trunk_radius_q8, scale);
            let x_radius = canopy_x.max(trunk_radius);
            let z_radius = canopy_z.max(trunk_radius);
            ([x_radius, 0, z_radius], [x_radius, height, z_radius])
        }
        VoxelObjectShape::Bush { radii_q8 }
        | VoxelObjectShape::Boulder { radii_q8, .. }
        | VoxelObjectShape::Rock { radii_q8, .. } => {
            let radii = scaled_radii(*radii_q8, placement.transform_q.uniform_scale_q8);
            let (x_radius, z_radius) =
                horizontal_extents(radii, placement.transform_q.yaw_quarter_turns);
            (
                [x_radius, radii[1], z_radius],
                [x_radius, radii[1], z_radius],
            )
        }
        VoxelObjectShape::Stump {
            radius_q8,
            height_q8,
        } => {
            let radius = scaled(*radius_q8, scale);
            (
                [radius, 0, radius],
                [radius, scaled(*height_q8, scale), radius],
            )
        }
        VoxelObjectShape::SparseStamp { .. } => return None,
    };
    let translation = placement.transform_q.translation;
    let min = WorldPointQ8::new(
        i32::try_from(i64::from(translation.x) - below[0]).ok()?,
        i32::try_from(i64::from(translation.y) - below[1]).ok()?,
        i32::try_from(i64::from(translation.z) - below[2]).ok()?,
    );
    let max_exclusive = WorldPointQ8::new(
        i32::try_from(i64::from(translation.x) + above[0] + 1).ok()?,
        i32::try_from(i64::from(translation.y) + above[1] + 1).ok()?,
        i32::try_from(i64::from(translation.z) + above[2] + 1).ok()?,
    );
    AabbQ8::new(min, max_exclusive).ok()
}

/// Lazily tests the mathematical object-surface dependency set.
#[must_use]
pub fn dependency_contains(placement: &ObjectPlacement, coord: VoxelCoord) -> bool {
    OBJECT_EXTRACTION_STENCIL.iter().any(|offset| {
        let Some(x) = coord.x.checked_sub(i32::from(offset.x)) else {
            return false;
        };
        let Some(y) = coord.y.checked_sub(i32::from(offset.y)) else {
            return false;
        };
        let Some(z) = coord.z.checked_sub(i32::from(offset.z)) else {
            return false;
        };
        let owner = VoxelCoord::new(x, y, z);
        owner.is_in_region() && raw_shape_contains(placement, owner)
    })
}

fn solid(material: crate::MaterialId) -> Voxel {
    Voxel::new(material, u8::MAX, 0, 0)
}

fn local_center_q8(placement: &ObjectPlacement, coord: VoxelCoord) -> [i64; 3] {
    let point = [
        i64::from(coord.x) * i64::from(VOXEL_EDGE_Q8) + i64::from(VOXEL_EDGE_Q8 / 2),
        i64::from(coord.y) * i64::from(VOXEL_EDGE_Q8) + i64::from(VOXEL_EDGE_Q8 / 2),
        i64::from(coord.z) * i64::from(VOXEL_EDGE_Q8) + i64::from(VOXEL_EDGE_Q8 / 2),
    ];
    let translation = placement.transform_q.translation;
    let x = point[0] - i64::from(translation.x);
    let z = point[2] - i64::from(translation.z);
    let (x, z) = match placement.transform_q.yaw_quarter_turns % 4 {
        0 => (x, z),
        1 => (z, -x),
        2 => (-x, -z),
        _ => (-z, x),
    };
    [x, point[1] - i64::from(translation.y), z]
}

fn scaled(value: u16, scale_q8: i64) -> i64 {
    i64::from(value) * scale_q8 / 256
}

fn scaled_radii(radii: [u16; 3], scale_q8: u16) -> [i64; 3] {
    let scale = i64::from(scale_q8);
    [
        scaled(radii[0], scale),
        scaled(radii[1], scale),
        scaled(radii[2], scale),
    ]
}

fn horizontal_extents(radii: [i64; 3], quarter_turns: u8) -> (i64, i64) {
    if quarter_turns.is_multiple_of(2) {
        (radii[0], radii[2])
    } else {
        (radii[2], radii[0])
    }
}

fn cylinder_contains(point: [i64; 3], radius: i64, height: i64) -> bool {
    point[1] >= 0
        && point[1] < height
        && point[0] * point[0] + point[2] * point[2] <= radius * radius
}

fn ellipsoid_contains(point: [i64; 3], center: [i64; 3], radii: [i64; 3]) -> bool {
    radii.iter().all(|radius| *radius > 0)
        && (point[0] - center[0])
            * (point[0] - center[0])
            * radii[1]
            * radii[1]
            * radii[2]
            * radii[2]
            + (point[1] - center[1])
                * (point[1] - center[1])
                * radii[0]
                * radii[0]
                * radii[2]
                * radii[2]
            + (point[2] - center[2])
                * (point[2] - center[2])
                * radii[0]
                * radii[0]
                * radii[1]
                * radii[1]
            <= radii[0] * radii[0] * radii[1] * radii[1] * radii[2] * radii[2]
}

fn perturbed_ellipsoid_contains(point: [i64; 3], radii: [i64; 3], key: u64) -> bool {
    if !ellipsoid_contains(point, [0, 0, 0], radii) {
        return false;
    }
    let hash = keyed_hash(key, point[0], point[1], point[2]);
    !hash.is_multiple_of(11)
        || ellipsoid_contains(point, [0, 0, 0], [radii[0] / 2, radii[1] / 2, radii[2] / 2])
}

fn keyed_hash(key: u64, x: i64, y: i64, z: i64) -> u64 {
    let mut value = key ^ (x as u64).wrapping_mul(0x9E37_79B1_85EB_CA87);
    value ^= (y as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    value ^= (z as u64).wrapping_mul(0x1656_67B1_9E37_79F9);
    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^ (value >> 27)
}

fn inverse_rotate(coord: VoxelCoord, pivot: [i16; 3], quarter_turns: u8) -> VoxelCoord {
    let x = coord.x - i32::from(pivot[0]);
    let z = coord.z - i32::from(pivot[2]);
    let (x, z) = match quarter_turns % 4 {
        0 => (x, z),
        1 => (z, -x),
        2 => (-x, -z),
        _ => (-z, x),
    };
    VoxelCoord::new(x + i32::from(pivot[0]), coord.y, z + i32::from(pivot[2]))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{OBJECT_EXTRACTION_STENCIL, dependency_contains, raw_shape_contains};
    use crate::{
        ObjectId, ObjectKind, ObjectPlacement, QuantizedTransform, VoxelCoord, VoxelObjectShape,
        WorldPointQ8,
    };

    fn boulder() -> ObjectPlacement {
        ObjectPlacement {
            id: ObjectId(1),
            kind: ObjectKind::Boulder,
            transform_q: QuantizedTransform {
                translation: WorldPointQ8::new(0, 0, 0),
                yaw_quarter_turns: 0,
                uniform_scale_q8: 256,
            },
            species: None,
            shape: VoxelObjectShape::Boulder {
                radii_q8: [128, 128, 128],
                perturbation_key: 7,
            },
            anchor: VoxelCoord::new(0, 0, 0),
        }
    }

    #[test]
    fn lazy_dependency_membership_equals_explicit_small_shape_oracle() {
        let placement = boulder();
        let mut explicit = BTreeSet::new();
        for owner_y in -4..=4 {
            for owner_z in -4..=4 {
                for owner_x in -4..=4 {
                    let owner = VoxelCoord::new(owner_x, owner_y, owner_z);
                    if !raw_shape_contains(&placement, owner) {
                        continue;
                    }
                    for offset in OBJECT_EXTRACTION_STENCIL {
                        explicit.insert(VoxelCoord::new(
                            owner.x + i32::from(offset.x),
                            owner.y + i32::from(offset.y),
                            owner.z + i32::from(offset.z),
                        ));
                    }
                }
            }
        }

        for y in -7..=7 {
            for z in -7..=7 {
                for x in -7..=7 {
                    let coordinate = VoxelCoord::new(x, y, z);
                    assert_eq!(
                        dependency_contains(&placement, coordinate),
                        explicit.contains(&coordinate),
                        "dependency mismatch at {coordinate:?}"
                    );
                }
            }
        }
    }
}
