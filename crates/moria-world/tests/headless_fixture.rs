use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use moria_world::testing::run_fixed_ticks;

#[derive(Component)]
struct FixtureEntity;

#[derive(Resource, Default)]
struct FixedTickCount(u32);

#[derive(Resource, Default)]
struct FrameUpdateCount(u32);

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
enum FixtureState {
    #[default]
    Ready,
}

fn fixture_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin))
        .init_state::<FixtureState>()
        .init_resource::<FixedTickCount>()
        .add_systems(FixedUpdate, observe_fixed_tick);
    app.world_mut().spawn(FixtureEntity);
    app
}

fn observe_fixed_tick(mut fixed_tick_count: ResMut<FixedTickCount>) {
    fixed_tick_count.0 += 1;
}

fn observe_frame_update(mut frame_update_count: ResMut<FrameUpdateCount>) {
    frame_update_count.0 += 1;
}

#[test]
fn headless_fixture_runs_the_requested_number_of_fixed_ticks() {
    let mut app = fixture_app();

    run_fixed_ticks(&mut app, 0);
    assert_eq!(app.world().resource::<FixedTickCount>().0, 0);

    run_fixed_ticks(&mut app, 3);
    assert_eq!(app.world().resource::<FixedTickCount>().0, 3);
    let fixture_entity_count = {
        let world = app.world_mut();
        world
            .query_filtered::<Entity, With<FixtureEntity>>()
            .iter(world)
            .count()
    };
    assert_eq!(fixture_entity_count, 1);
    assert_eq!(
        *app.world().resource::<State<FixtureState>>().get(),
        FixtureState::Ready
    );
}

#[test]
fn headless_fixture_is_not_limited_by_the_virtual_time_frame_cap() {
    let mut app = fixture_app();
    run_fixed_ticks(&mut app, 0);
    app.init_resource::<FrameUpdateCount>()
        .add_systems(Update, observe_frame_update);
    let previous_max_delta = app.world().resource::<Time<Virtual>>().max_delta();

    run_fixed_ticks(&mut app, 20);

    assert_eq!(app.world().resource::<FixedTickCount>().0, 20);
    assert_eq!(app.world().resource::<FrameUpdateCount>().0, 1);
    assert_eq!(
        app.world().resource::<Time<Virtual>>().max_delta(),
        previous_max_delta
    );
}
