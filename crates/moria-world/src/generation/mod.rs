//! Immutable identity values for an authoritative generated region.

mod biome;
mod identity;
mod terrain;

pub use biome::{biome_at, BiomeId};
pub use identity::{AabbQ8, BoundsError, WorldBounds, WorldIdentity, WorldSeed};
pub use terrain::{
    classify_brick, evaluate_base_voxel, evaluate_column, ColumnRun, ColumnSample, ProceduralClass,
    RunKind,
};
