//! Public facade for the reusable sparse voxel world.
//!
//! Consumer packages depend on this crate only through its explicit public API.

use bevy::prelude::*;

pub mod config;
pub mod config_validation;
pub mod presentation;
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
