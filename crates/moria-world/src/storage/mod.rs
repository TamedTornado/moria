//! Authoritative voxel storage value types.

#[allow(dead_code, reason = "storage activation is wired by later feature plugins")]
mod brick;
mod coordinates;
#[allow(dead_code, reason = "public read and mutation facades are added by later feature plugins")]
mod store;
mod voxel;

#[cfg(test)]
mod store_tests;

pub use coordinates::{
    BRICK_EDGE_VOXELS, BrickCoord, ColumnCoord, CoordinateError, Q8_UNITS_PER_METER, VOXEL_EDGE_Q8,
    VoxelCoord, WorldPointQ8,
};
pub use voxel::{
    AIR, CUT_STONE, GRANITE, GRAVEL, IRON_ORE, LEAF, LIMESTONE, SAND, SANDSTONE, SHALE, SUBSOIL,
    TOPSOIL, Voxel, WATER, WOOD, material_present, solid_collision, water_volume,
};
