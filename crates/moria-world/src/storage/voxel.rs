//! Four-byte voxel truth and its distinct material, water, and collision predicates.

use crate::{CollisionClass, MaterialId, MaterialRegistry};

pub const AIR: MaterialId = MaterialId(0);
pub const WATER: MaterialId = MaterialId(1);
pub const TOPSOIL: MaterialId = MaterialId(2);
pub const SUBSOIL: MaterialId = MaterialId(3);
pub const SAND: MaterialId = MaterialId(4);
pub const GRAVEL: MaterialId = MaterialId(5);
pub const LIMESTONE: MaterialId = MaterialId(6);
pub const SANDSTONE: MaterialId = MaterialId(7);
pub const SHALE: MaterialId = MaterialId(8);
pub const GRANITE: MaterialId = MaterialId(9);
pub const IRON_ORE: MaterialId = MaterialId(10);
pub const WOOD: MaterialId = MaterialId(11);
pub const LEAF: MaterialId = MaterialId(12);
pub const CUT_STONE: MaterialId = MaterialId(13);

/// Authoritative per-cell voxel state, stored without padding.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Voxel {
    pub material: MaterialId,
    pub density: u8,
    pub state: u8,
    pub flags: u8,
}

impl Voxel {
    #[must_use]
    pub const fn new(material: MaterialId, density: u8, state: u8, flags: u8) -> Self {
        Self {
            material,
            density,
            state,
            flags,
        }
    }
}

const _: [(); 4] = [(); core::mem::size_of::<Voxel>()];

/// Returns whether a cell contains any non-air material volume.
#[must_use]
pub const fn material_present(voxel: Voxel) -> bool {
    voxel.material.0 != AIR.0 && voxel.density > 0
}

/// Returns whether a cell contains static water volume.
#[must_use]
pub const fn water_volume(voxel: Voxel) -> bool {
    voxel.material.0 == WATER.0 && voxel.density > 0
}

/// Returns whether a cell is solid at the authoritative density isovalue.
#[must_use]
pub fn solid_collision(voxel: Voxel, materials: &MaterialRegistry) -> bool {
    materials
        .materials
        .iter()
        .find(|material| material.id == voxel.material)
        .is_some_and(|material| material.collision_class == CollisionClass::Solid)
        && voxel.density >= 128
}
