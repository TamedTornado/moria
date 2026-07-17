//! Headless test support for consumers of the public world facade.

pub mod conformance;

use bevy::{
    app::FixedUpdate,
    prelude::{App, Fixed, ResMut, Resource, Time},
    time::TimeUpdateStrategy,
};

#[derive(Resource, Default)]
struct FixedTickObserver(u64);

fn observe_fixed_tick(mut observer: ResMut<FixedTickObserver>) {
    observer.0 += 1;
}

/// Advances an app by exactly `count` [`FixedUpdate`] executions.
///
/// The app must include [`bevy::prelude::MinimalPlugins`] (or another plugin set
/// that provides Bevy's time and fixed schedules). The helper uses Bevy's virtual
/// fixed-time strategy, calls [`App::update`], and verifies the observed count.
pub fn run_fixed_ticks(app: &mut App, count: u32) {
    assert!(
        app.world().contains_resource::<Time<Fixed>>(),
        "run_fixed_ticks requires Bevy's TimePlugin; add MinimalPlugins to the test app"
    );

    let needs_initial_frame = !app.world().contains_resource::<FixedTickObserver>();
    if needs_initial_frame {
        app.init_resource::<FixedTickObserver>()
            .add_systems(FixedUpdate, observe_fixed_tick);
    }

    let previous_strategy = app
        .world_mut()
        .remove_resource::<TimeUpdateStrategy>()
        .expect("run_fixed_ticks requires Bevy's TimePlugin");

    if needs_initial_frame {
        app.insert_resource(TimeUpdateStrategy::FixedTimesteps(0));
        app.update();
    }

    let observed_before = app.world().resource::<FixedTickObserver>().0;
    app.insert_resource(TimeUpdateStrategy::FixedTimesteps(count));
    app.update();

    app.insert_resource(previous_strategy);

    let observed = app.world().resource::<FixedTickObserver>().0 - observed_before;
    assert_eq!(
        observed,
        u64::from(count),
        "run_fixed_ticks must execute exactly the requested number of fixed ticks"
    );
}
