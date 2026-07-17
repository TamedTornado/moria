//! Public facade for the reusable sparse voxel world.
//!
//! Consumer packages depend on this crate only through its explicit public API.

use bevy::prelude::*;

pub mod config;
pub mod config_validation;
pub mod curation;
pub mod generation;
pub mod presentation;
mod storage;
pub mod testing;

pub use config::{
    ActionBinding, BandConfig, BenchmarkConfig, BiomeConfig, BoundsConfig, CameraConfig,
    CapsuleConfig, CaveConfig, CaveLightConfig, CollisionClass, GeologyConfig, InputAction,
    InputConfig, MaterialDef, MaterialId, MaterialRegistry, MutationConfig, ObjectGenConfig,
    PresentationConfig, RangeQ8, RegionConfig, RenderingConfig, StreamingConfig, SurfaceClass,
    TerrainGenConfig, WaterGenConfig, WindowConfig, parameters_digest_from_bytes,
};
pub use config_validation::{
    ConfigValidationError, validate_input_config, validate_material_registry,
    validate_presentation_config, validate_region_config,
};
pub use curation::{
    CuratedManifest, FeatureInstance, FeatureKind, ManifestError, ObjectId, ObjectKind,
    ObjectPlacement, QuantizedTransform, RouteTag, RouteWaypoint, RuinPoi, SparseVoxelStamp,
    SpeciesId, StampRun, VoxelObjectShape, WaterBodyDef, WaterKind,
};
pub use generation::{
    AabbQ8, BiomeId, ColumnRun, ColumnSample, ProceduralClass, RunKind, WorldBounds,
    WorldIdentity, WorldSeed, biome_at, classify_brick, evaluate_base_voxel, evaluate_column,
};
pub use storage::{
    AIR, BRICK_EDGE_VOXELS, BrickCoord, CUT_STONE, ColumnCoord, CoordinateError, GRANITE, GRAVEL,
    IRON_ORE, LEAF, LIMESTONE, Q8_UNITS_PER_METER, SAND, SANDSTONE, SHALE, SUBSOIL, TOPSOIL,
    VOXEL_EDGE_Q8, Voxel, VoxelCoord, WATER, WOOD, WorldPointQ8, material_present, solid_collision,
    water_volume,
};

/// Installs the reusable world feature set.
///
/// Feature plugins are added here as their public contracts are implemented.
pub struct MoriaWorldPlugin;

impl Plugin for MoriaWorldPlugin {
    fn build(&self, _app: &mut App) {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::MoriaWorldPlugin;

    #[test]
    fn public_world_plugin_installs_in_a_headless_app() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins)
            .add_plugins(MoriaWorldPlugin);
        app.update();
    }
}
