use bevy::prelude::*;
use moria_world::{
    EditExecution, EditOperation, MoriaWorldPlugin, QueryError, SubmitError, VoxelCoord,
    WorldEditCommand, WorldEditWrite, WorldLifecycle, WorldOpenError, WorldPointQ8, WorldRead,
};

#[test]
fn lifecycle_accepts_only_opening_and_terminal_transitions() {
    let mut lifecycle = WorldLifecycle::default();

    assert!(lifecycle.mark_ready().is_err());
    assert!(matches!(lifecycle, WorldLifecycle::Uninitialized));
    assert!(lifecycle.start_loading().is_ok());
    assert!(matches!(lifecycle, WorldLifecycle::Loading));
    assert!(lifecycle.mark_ready().is_ok());
    assert!(matches!(lifecycle, WorldLifecycle::Ready));
    assert!(lifecycle.start_loading().is_err());
    assert!(lifecycle.mark_ready().is_err());
    assert!(
        lifecycle
            .fail(WorldOpenError::InitialActivation(
                "corrupt activation".into()
            ))
            .is_ok()
    );
    assert!(matches!(
        lifecycle,
        WorldLifecycle::Failed(WorldOpenError::InitialActivation(_))
    ));
    assert!(lifecycle.start_loading().is_err());
}

#[test]
fn all_open_failures_are_terminal_and_preserve_their_kind() {
    let failures = [
        WorldOpenError::Asset("asset".into()),
        WorldOpenError::ManifestIdentity("identity".into()),
        WorldOpenError::InvalidConfig("config".into()),
        WorldOpenError::GenerationContract("generation".into()),
        WorldOpenError::Save("save".into()),
        WorldOpenError::InitialActivation("activation".into()),
    ];

    for failure in failures {
        let mut lifecycle = WorldLifecycle::default();
        lifecycle.start_loading().unwrap();
        lifecycle.fail(failure.clone()).unwrap();
        assert_eq!(lifecycle, WorldLifecycle::Failed(failure));
        assert!(lifecycle.mark_ready().is_err());
    }
}

#[test]
fn pre_ready_and_failed_worlds_reject_reads_and_edits() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, MoriaWorldPlugin))
        .add_systems(Update, assert_access_is_guarded);

    app.update();
    {
        let mut lifecycle = app.world_mut().resource_mut::<WorldLifecycle>();
        lifecycle.start_loading().unwrap();
        lifecycle
            .fail(WorldOpenError::Save("unavailable".into()))
            .unwrap();
    }
    app.update();
}

fn assert_access_is_guarded(mut edits: WorldEditWrite, reads: WorldRead) {
    assert_eq!(
        reads.sample_voxel(VoxelCoord::new(0, 0, 0)),
        Err(QueryError::NotReady)
    );
    assert_eq!(
        edits.submit(WorldEditCommand {
            request_id: 1,
            operation: EditOperation::DigSphere {
                center: WorldPointQ8::new(0, 0, 0),
                radius_q8: 64,
                strength: 255,
            },
            execution: EditExecution::Atomic,
        }),
        Err(SubmitError::NotReady)
    );
}
