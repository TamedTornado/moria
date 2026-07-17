//! Deterministic broad biome selection.

use crate::{ColumnCoord, Q8_UNITS_PER_METER, VOXEL_EDGE_Q8, WorldIdentity};

/// The broad biome assigned to a generated column.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BiomeId {
    Meadow,
    Forest,
}

/// The documented 220 m broad-terrain scale also bounds biome patches.
const BROAD_BIOME_SCALE_METERS: i32 = 220;
const VOXELS_PER_METER: i32 = Q8_UNITS_PER_METER / VOXEL_EDGE_Q8;

/// Returns the deterministic biome for a column.
#[must_use]
pub fn biome_at(identity: &WorldIdentity, coord: ColumnCoord) -> BiomeId {
    let cell_x = coord
        .x
        .div_euclid(BROAD_BIOME_SCALE_METERS * VOXELS_PER_METER);
    let cell_z = coord
        .z
        .div_euclid(BROAD_BIOME_SCALE_METERS * VOXELS_PER_METER);
    let value = keyed_hash(identity.seed, cell_x, cell_z);

    // The 55/45 species split is not a biome-density knob.  A half-space split
    // keeps biome selection independent of object placement and iteration order.
    if value & 1 == 0 {
        BiomeId::Meadow
    } else {
        BiomeId::Forest
    }
}

pub(super) const fn keyed_hash(seed: u64, x: i32, z: i32) -> u64 {
    let mut value = seed
        ^ (x as i64 as u64).wrapping_mul(0x9E37_79B1_85EB_CA87)
        ^ (z as i64 as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}
