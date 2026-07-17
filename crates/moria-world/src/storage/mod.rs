//! Authoritative voxel storage value types.

mod coordinates;
mod voxel;

pub use coordinates::{
    BRICK_EDGE_VOXELS, BrickCoord, ColumnCoord, CoordinateError, Q8_UNITS_PER_METER, VOXEL_EDGE_Q8,
    VoxelCoord, WorldPointQ8,
};
pub use voxel::{
    AIR, CUT_STONE, GRANITE, GRAVEL, IRON_ORE, LEAF, LIMESTONE, SAND, SANDSTONE, SHALE, SUBSOIL,
    TOPSOIL, Voxel, WATER, WOOD, material_present, solid_collision, water_volume,
};
