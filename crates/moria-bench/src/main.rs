use bevy::prelude::*;
use moria_world::MoriaWorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MoriaWorldPlugin)
        .run();
}
