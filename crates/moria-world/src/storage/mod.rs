//! Authoritative voxel storage value types.

mod coordinates;
mod voxel;

pub use coordinates::{
    BrickCoord, ColumnCoord, CoordinateError, VoxelCoord, WorldPointQ8, BRICK_EDGE_VOXELS,
    Q8_UNITS_PER_METER, VOXEL_EDGE_Q8,
};
pub use voxel::{
    material_present, solid_collision, water_volume, Voxel, AIR, CUT_STONE, GRANITE, GRAVEL,
    IRON_ORE, LEAF, LIMESTONE, SAND, SANDSTONE, SHALE, SUBSOIL, TOPSOIL, WATER, WOOD,
};
