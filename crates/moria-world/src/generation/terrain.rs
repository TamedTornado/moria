//! Fixed-point broad terrain, soil bands, and compact brick classification.

use super::biome::keyed_hash;
use super::{BiomeId, biome_at};
use crate::{
    AIR, BRICK_EDGE_VOXELS, BrickCoord, ColumnCoord, GRANITE, MaterialId, Q8_UNITS_PER_METER,
    SUBSOIL, TOPSOIL, VOXEL_EDGE_Q8, Voxel, VoxelCoord, WorldIdentity,
};

/// A contiguous vertical material interval in a generated column.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColumnRun {
    pub y_min_voxel: i16,
    pub y_max_voxel_exclusive: i16,
    pub material: MaterialId,
    pub kind: RunKind,
}

/// The occupancy interpretation of a [`ColumnRun`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunKind {
    Matter,
    Air,
    Water,
    CaveGap,
}

/// An on-demand, bounded representation of one complete vertical column.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnSample {
    pub coord: ColumnCoord,
    pub surface_y_q8: i32,
    pub runs: Vec<ColumnRun>,
    pub biome: BiomeId,
    pub feature_mask: u32,
}

/// Compact base representation selected before any 4,096-cell expansion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProceduralClass {
    Uniform(Voxel),
    Procedural,
}

/// Product One's configured 64 m typical surface level in Q8 units.
const TYPICAL_SURFACE_Q8: i32 = 64 * Q8_UNITS_PER_METER;
/// Product One's configured 220 m lattice period in Q8 units.
const BROAD_SCALE_Q8: i32 = 220 * Q8_UNITS_PER_METER;
/// Product One's configured 72 m detail lattice period in Q8 units.
const MEANDER_SCALE_Q8: i32 = 72 * Q8_UNITS_PER_METER;
/// Product One's configured 34 m maximum broad relief in Q8 units.
const RELIEF_Q8: i32 = 34 * Q8_UNITS_PER_METER;
/// Product One's configured one metre topsoil depth in Q8 units.
const TOPSOIL_DEPTH_Q8: i32 = Q8_UNITS_PER_METER;
/// Product One's configured three metre subsoil depth in Q8 units.
const SUBSOIL_DEPTH_Q8: i32 = 3 * Q8_UNITS_PER_METER;
/// The complete soil profile: topsoil followed by subsoil.
const SOIL_DEPTH_Q8: i32 = TOPSOIL_DEPTH_Q8 + SUBSOIL_DEPTH_Q8;
const Q16_ONE: i64 = 65_536;

/// Evaluates one full vertical column without retaining world state.
#[must_use]
pub fn evaluate_column(identity: &WorldIdentity, coord: ColumnCoord) -> ColumnSample {
    let surface_y_q8 = surface_y_q8(identity, coord);
    let mut runs = Vec::with_capacity(4);
    let min_y = identity.bounds.min().y.div_euclid(VOXEL_EDGE_Q8);
    let max_y = identity.bounds.max_exclusive().y.div_euclid(VOXEL_EDGE_Q8);

    for y in min_y..max_y {
        let voxel = terrain_voxel(surface_y_q8, y);
        let kind = run_kind(voxel);
        let extends_previous = runs
            .last()
            .is_some_and(|last: &ColumnRun| last.material == voxel.material && last.kind == kind);
        if extends_previous {
            runs.last_mut()
                .expect("a checked last run exists")
                .y_max_voxel_exclusive = (y + 1) as i16;
        } else {
            runs.push(ColumnRun {
                y_min_voxel: y as i16,
                y_max_voxel_exclusive: (y + 1) as i16,
                material: voxel.material,
                kind,
            });
        }
    }

    ColumnSample {
        coord,
        surface_y_q8,
        runs,
        biome: biome_at(identity, coord),
        feature_mask: 0,
    }
}

/// Evaluates immutable terrain truth for one in-bounds voxel coordinate.
#[must_use]
pub fn evaluate_base_voxel(identity: &WorldIdentity, coord: VoxelCoord) -> Voxel {
    let surface = surface_y_q8(
        identity,
        ColumnCoord {
            x: coord.x,
            z: coord.z,
        },
    );
    terrain_voxel(surface, coord.y)
}

/// Conservatively classifies a brick without allocating an expanded voxel array.
#[must_use]
pub fn classify_brick(identity: &WorldIdentity, brick: BrickCoord) -> ProceduralClass {
    let min_y = -512 + i32::from(brick.y()) * BRICK_EDGE_VOXELS;
    let min_y_q8 = min_y * VOXEL_EDGE_Q8;

    // The terrain equation is a sum of two Q16 values whose absolute weights
    // total one, so RELIEF_Q8 is a strict upper bound.  Any brick above it is
    // uniformly air without inspecting individual voxels.
    if min_y_q8 >= TYPICAL_SURFACE_Q8 + RELIEF_Q8 {
        return ProceduralClass::Uniform(Voxel::new(AIR, 0, 0, 0));
    }
    let max_y_q8 = (min_y + BRICK_EDGE_VOXELS) * VOXEL_EDGE_Q8;
    if max_y_q8 <= TYPICAL_SURFACE_Q8 - RELIEF_Q8 - SOIL_DEPTH_Q8 {
        return ProceduralClass::Uniform(Voxel::new(GRANITE, u8::MAX, 0, 0));
    }
    let _ = identity;
    ProceduralClass::Procedural
}

fn surface_y_q8(identity: &WorldIdentity, coord: ColumnCoord) -> i32 {
    let x_q8 = coord.x * VOXEL_EDGE_Q8;
    let z_q8 = coord.z * VOXEL_EDGE_Q8;
    let broad = value_noise_q16(identity.seed, x_q8, z_q8, BROAD_SCALE_Q8);
    let meander = value_noise_q16(
        identity.seed ^ 0xA5A5_5A5A_3C3C_C3C3,
        x_q8,
        z_q8,
        MEANDER_SCALE_Q8,
    );
    let combined = (i64::from(broad) * 3 + i64::from(meander)) / 4;
    TYPICAL_SURFACE_Q8 + ((combined * i64::from(RELIEF_Q8)) / Q16_ONE) as i32
}

fn terrain_voxel(surface_y_q8: i32, y: i32) -> Voxel {
    let voxel_top_q8 = (y + 1) * VOXEL_EDGE_Q8;
    if voxel_top_q8 > surface_y_q8 {
        return Voxel::new(AIR, 0, 0, 0);
    }
    let depth_q8 = surface_y_q8 - voxel_top_q8;
    let material = if depth_q8 < TOPSOIL_DEPTH_Q8 {
        TOPSOIL
    } else if depth_q8 < SOIL_DEPTH_Q8 {
        SUBSOIL
    } else {
        GRANITE
    };
    Voxel::new(material, u8::MAX, 0, 0)
}

const fn run_kind(voxel: Voxel) -> RunKind {
    if voxel.material.0 == AIR.0 {
        RunKind::Air
    } else {
        RunKind::Matter
    }
}

fn value_noise_q16(seed: u64, x_q8: i32, z_q8: i32, scale_q8: i32) -> i32 {
    let grid_x = x_q8.div_euclid(scale_q8);
    let grid_z = z_q8.div_euclid(scale_q8);
    let frac_x = i64::from(x_q8.rem_euclid(scale_q8)) * Q16_ONE / i64::from(scale_q8);
    let frac_z = i64::from(z_q8.rem_euclid(scale_q8)) * Q16_ONE / i64::from(scale_q8);
    let tx = smoothstep_q16(frac_x);
    let tz = smoothstep_q16(frac_z);
    let a = interpolate_q16(
        hash_q16(seed, grid_x, grid_z),
        hash_q16(seed, grid_x + 1, grid_z),
        tx,
    );
    let b = interpolate_q16(
        hash_q16(seed, grid_x, grid_z + 1),
        hash_q16(seed, grid_x + 1, grid_z + 1),
        tx,
    );
    interpolate_q16(a, b, tz) as i32
}

const fn hash_q16(seed: u64, x: i32, z: i32) -> i64 {
    (keyed_hash(seed, x, z) >> 48) as i64 - 32_768
}

const fn smoothstep_q16(value: i64) -> i64 {
    value * value * (3 * Q16_ONE - 2 * value) / (Q16_ONE * Q16_ONE)
}

const fn interpolate_q16(a: i64, b: i64, t: i64) -> i64 {
    a + ((b - a) * t) / Q16_ONE
}
