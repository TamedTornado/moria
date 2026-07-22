//! Immutable base provenance and normal solid-presentation routing.

use crate::{MaterialRegistry, ObjectId, Voxel, solid_collision};

/// One regenerated base voxel together with its immutable provenance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BaseVoxel {
    pub voxel: Voxel,
    pub source: VoxelSource,
}

/// The regenerated base source of a voxel; deltas never replace this value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VoxelSource {
    Terrain,
    Object(ObjectId),
    Ruin(ObjectId),
}

/// The sole normal-world presentation owner for a current solid voxel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SolidPresentationOwner {
    TerrainChunk,
    NonRuinObject(ObjectId),
}

/// Routes a current solid voxel using its regenerated base provenance.
#[must_use]
pub fn solid_presentation_owner(
    current: Voxel,
    base_source: VoxelSource,
    materials: &MaterialRegistry,
) -> Option<SolidPresentationOwner> {
    solid_collision(current, materials).then_some(match base_source {
        VoxelSource::Terrain | VoxelSource::Ruin(_) => SolidPresentationOwner::TerrainChunk,
        VoxelSource::Object(id) => SolidPresentationOwner::NonRuinObject(id),
    })
}
