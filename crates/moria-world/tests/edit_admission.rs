use bevy::prelude::*;
use moria_world::{
    EditAccepted, EditExecution, EditOperation, GRANITE, MoriaWorldPlugin, SubmitError, VoxelCoord,
    WorldEditCommand, WorldEditWrite, WorldLifecycle, WorldPointQ8,
};

#[derive(Resource, Default)]
struct SubmissionResults(Vec<Result<(), SubmitError>>);

#[derive(Resource, Default)]
struct CommandsToSubmit(Vec<WorldEditCommand>);

fn submit_queued_commands(
    mut commands: ResMut<CommandsToSubmit>,
    mut results: ResMut<SubmissionResults>,
    mut edits: WorldEditWrite,
) {
    for command in core::mem::take(&mut commands.0) {
        results.0.push(edits.submit(command));
    }
}

fn ready_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, MoriaWorldPlugin))
        .init_resource::<CommandsToSubmit>()
        .init_resource::<SubmissionResults>()
        .add_systems(Update, submit_queued_commands);
    let mut lifecycle = app.world_mut().resource_mut::<WorldLifecycle>();
    lifecycle.start_loading().unwrap();
    lifecycle.mark_ready().unwrap();
    app
}

fn dig_sphere(request_id: u64, radius_q8: u16, execution: EditExecution) -> WorldEditCommand {
    WorldEditCommand {
        request_id,
        operation: EditOperation::DigSphere {
            center: WorldPointQ8::new(0, 0, 0),
            radius_q8,
            strength: 255,
        },
        execution,
    }
}

#[test]
fn admission_accepts_bounded_operations_once_and_requires_progressive_for_large_work() {
    let mut app = ready_app();
    app.world_mut().resource_mut::<CommandsToSubmit>().0 = vec![
        dig_sphere(1, 64, EditExecution::Atomic),
        dig_sphere(2, 4096, EditExecution::Atomic),
        dig_sphere(3, 4096, EditExecution::Progressive),
        WorldEditCommand {
            request_id: 4,
            operation: EditOperation::PlaceBox {
                min: VoxelCoord::new(-16, -16, -16),
                max_exclusive: VoxelCoord::new(16, 16, 16),
                strength: 255,
                material: GRANITE,
            },
            execution: EditExecution::Atomic,
        },
    ];

    app.update();

    assert_eq!(
        app.world().resource::<SubmissionResults>().0,
        vec![
            Ok(()),
            Err(SubmitError::AtomicWorkLimitExceeded),
            Ok(()),
            Ok(())
        ]
    );
    assert_eq!(app.world().resource::<Messages<EditAccepted>>().len(), 3);
}

#[test]
fn admission_rejects_invalid_duplicate_and_full_requests_without_lifecycle_messages() {
    let mut app = ready_app();
    let mut commands = vec![
        dig_sphere(1, 64, EditExecution::Atomic),
        dig_sphere(1, 64, EditExecution::Atomic),
        dig_sphere(2, 63, EditExecution::Atomic),
        WorldEditCommand {
            request_id: 3,
            operation: EditOperation::DigBox {
                min: VoxelCoord::new(2_000, 0, 0),
                max_exclusive: VoxelCoord::new(2_001, 1, 1),
                strength: 255,
            },
            execution: EditExecution::Atomic,
        },
    ];
    commands.extend((4..=35).map(|request_id| dig_sphere(request_id, 64, EditExecution::Atomic)));
    app.world_mut().resource_mut::<CommandsToSubmit>().0 = commands;

    app.update();

    let results = &app.world().resource::<SubmissionResults>().0;
    assert_eq!(results[0], Ok(()));
    assert_eq!(results[1], Err(SubmitError::DuplicateRequestId));
    assert_eq!(results[2], Err(SubmitError::InvalidBounds));
    assert_eq!(results[3], Err(SubmitError::InvalidBounds));
    assert_eq!(results.last(), Some(&Err(SubmitError::QueueFull)));
    assert_eq!(app.world().resource::<Messages<EditAccepted>>().len(), 32);
}

#[test]
fn admission_rejects_unready_and_loading_worlds_before_reserving_or_publishing() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, MoriaWorldPlugin))
        .init_resource::<CommandsToSubmit>()
        .init_resource::<SubmissionResults>()
        .add_systems(Update, submit_queued_commands);

    app.world_mut().resource_mut::<CommandsToSubmit>().0 =
        vec![dig_sphere(1, 64, EditExecution::Atomic)];
    app.update();
    assert_eq!(
        app.world().resource::<SubmissionResults>().0,
        vec![Err(SubmitError::NotReady)]
    );

    app.world_mut()
        .resource_mut::<WorldLifecycle>()
        .start_loading()
        .unwrap();
    app.world_mut().resource_mut::<CommandsToSubmit>().0 =
        vec![dig_sphere(2, 64, EditExecution::Atomic)];
    app.update();
    assert_eq!(
        app.world().resource::<SubmissionResults>().0.last(),
        Some(&Err(SubmitError::LoadInProgress))
    );
    assert!(app.world().resource::<Messages<EditAccepted>>().is_empty());
}

#[test]
fn admission_enforces_the_operation_limit_before_execution_mode_limits() {
    let mut app = ready_app();
    let operation = EditOperation::DigBox {
        min: VoxelCoord::new(-2_000, -512, -2_000),
        max_exclusive: VoxelCoord::new(-1_472, -256, -1_744),
        strength: 255,
    };
    app.world_mut().resource_mut::<CommandsToSubmit>().0 = vec![
        WorldEditCommand {
            request_id: 1,
            operation: operation.clone(),
            execution: EditExecution::Atomic,
        },
        WorldEditCommand {
            request_id: 2,
            operation,
            execution: EditExecution::Progressive,
        },
    ];

    app.update();

    assert_eq!(
        app.world().resource::<SubmissionResults>().0,
        vec![
            Err(SubmitError::ProgressiveWorkLimitExceeded),
            Err(SubmitError::ProgressiveWorkLimitExceeded),
        ]
    );
    assert!(app.world().resource::<Messages<EditAccepted>>().is_empty());
}
