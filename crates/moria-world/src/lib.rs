//! Public facade for the reusable sparse voxel world.
//!
//! Consumer packages depend on this crate only through its explicit public API.

use bevy::prelude::*;

pub mod testing;

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
