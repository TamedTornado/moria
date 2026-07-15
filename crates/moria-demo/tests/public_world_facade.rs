use bevy::prelude::*;
use moria_world::{MoriaWorldPlugin, testing::run_fixed_ticks};

#[derive(Resource, Default)]
struct FixedTickCount(u32);

fn count_fixed_ticks(mut fixed_tick_count: ResMut<FixedTickCount>) {
    fixed_tick_count.0 += 1;
}

#[test]
fn external_consumer_uses_the_public_world_testing_facade() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(MoriaWorldPlugin)
        .init_resource::<FixedTickCount>()
        .add_systems(FixedUpdate, count_fixed_ticks);

    run_fixed_ticks(&mut app, 2);

    assert_eq!(app.world().resource::<FixedTickCount>().0, 2);
}
