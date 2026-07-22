use bevy::prelude::*;
use moria_world::{
    DiagnosticPageRequest, FocusPurpose, FocusSource, MoriaWorldPlugin, QueryError, SetFocusSource,
    WorldPointQ8, WorldRead, WorldTelemetryRead,
};

#[test]
fn external_consumers_publish_focuses_and_read_constant_time_telemetry_without_private_handles() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(MoriaWorldPlugin);
    app.world_mut().write_message(SetFocusSource(FocusSource {
        id: 7,
        position: WorldPointQ8::new(0, 0, 0),
        purpose: FocusPurpose::Inspection,
    }));
    app.add_systems(Update, |telemetry: WorldTelemetryRead| {
        assert_eq!(telemetry.active_counts().bricks, 0);
        assert_eq!(telemetry.queue_depths().dropped_edit_observations, 0);
        assert!(telemetry.edit_observations().is_empty());
    });
    app.add_systems(Update, |read: WorldRead| {
        assert_eq!(
            read.diagnostic_snapshot(DiagnosticPageRequest {
                snapshot: None,
                after_brick: None,
                max_bricks: 1,
                include_cells: false,
            }),
            Err(QueryError::NotReady)
        );
    });

    app.update();
}
