//! Immutable identity values for an authoritative generated region.

mod biome;
mod identity;
mod terrain;

pub use biome::{BiomeId, biome_at};
pub use identity::{AabbQ8, BoundsError, WorldBounds, WorldIdentity, WorldSeed};
pub use terrain::{
    ColumnRun, ColumnSample, ProceduralClass, RunKind, classify_brick, evaluate_base_voxel,
    evaluate_column,
};
