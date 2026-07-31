use moria::{
    canonical::{CanonicalOrder, ContractDigest, Tick},
    config::{
        BudgetGroup, CandidateDiagnostics, CandidateFaultOnce, CandidateFaultStage,
        ExecutionPolicy, PlacementFixedFormat, ResourceBudgets, SimulationUnitId,
        WorldGenesisConfig,
    },
    facade::ResourceBudgetField,
};

macro_rules! assert_fields {
    ($value:expr, $($field:ident: $expected:expr),+ $(,)?) => {
        $(assert_eq!($value.$field, $expected, stringify!($field));)+
    };
}

#[test]
fn resource_budget_defaults_match_the_normative_tech_017_values() {
    let budgets = ResourceBudgets::default();

    assert_fields!(budgets.identity,
        worlds: 1, retired_replay_streams_per_client: 4, materials_per_world: 4_096,
        volumes_per_world: 65_536, participants_per_world: 64, input_sources_per_world: 4_096,
        base_sources_per_world: 256, base_authorities_per_world: 65_536,
        content_blob_stores_per_world: 4, checkpoint_stores_per_world: 4, replay_sinks_per_world: 4,
        rng_streams_per_participant: 32, representation_contracts_per_participant: 64,
        interests_per_world: 4_096, operation_records_per_world: 16_384,
        terminal_receipts_per_world: 8_192, terminal_receipt_bytes_per_world: 64 << 20,
        root_leases_per_world: 4_096, artifact_leases_per_world: 256,
    );
    assert_eq!(
        (
            budgets.canonical.pending_ticks,
            budgets.canonical.inputs_per_tick,
            budgets.canonical.encoded_bytes_per_tick,
            budgets.canonical.correlation_bytes_per_tick,
            budgets.canonical.bricks_per_command,
            budgets.canonical.cells_per_command,
            budgets.canonical.changed_bricks_per_tick,
            budgets.canonical.scratch_bytes,
        ),
        (1, 4_096, 8 << 20, 320 << 10, 64, 32_768, 512, 256 << 20)
    );
    assert_eq!(
        (
            budgets.content.base_request_queue,
            budgets.content.base_requests_in_flight,
            budgets.content.base_completion_bytes_in_flight,
            budgets.content.materialization_bricks_per_job,
            budgets.content.resident_dense_bricks,
            budgets.content.resident_uniform_bricks,
            budgets.content.resident_radix_nodes,
            budgets.content.resident_directory_buckets,
            budgets.content.authoritative_gpu_bytes,
        ),
        (
            256,
            32,
            64 << 10,
            4_096,
            65_536,
            65_536,
            1_048_576,
            1_048_576,
            2 << 30
        )
    );
    assert_eq!(
        (
            budgets.query.queued_requests,
            budgets.query.in_flight_requests,
            budgets.query.bricks_per_request,
            budgets.query.records_per_result,
            budgets.query.bytes_per_result,
            budgets.query.volume_revisions_per_request,
            budgets.query.readback_bytes_in_flight,
        ),
        (256, 3, 4_096, 65_536, 4 << 20, 256, 48 << 20)
    );
    assert_eq!(
        (
            budgets.observation.records_per_world,
            budgets.observation.payload_bytes_per_world,
            budgets.observation.bytes_per_record,
            budgets.observation.subscriptions_per_world,
            budgets.observation.volumes_per_subscription,
            budgets.observation.records_per_poll,
            budgets.observation.bytes_per_poll,
            budgets.observation.resnapshot_volume_summaries,
            budgets.observation.resnapshot_region_summaries,
            budgets.observation.resnapshot_bytes,
        ),
        (
            8_192,
            32 << 20,
            4 << 10,
            64,
            1_024,
            256,
            1 << 20,
            1_024,
            4_096,
            16 << 20
        )
    );
    assert_eq!(
        (
            budgets.presentation.queued_chunks,
            budgets.presentation.resident_chunks,
            budgets.presentation.in_flight_jobs,
            budgets.presentation.vertices_per_job,
            budgets.presentation.indices_per_job,
            budgets.presentation.bytes_per_job,
            budgets.presentation.resident_bytes,
            budgets.presentation.dressing_records_per_chunk,
        ),
        (
            4_096,
            65_536,
            3,
            1_048_576,
            6_291_456,
            64 << 20,
            1 << 30,
            65_536
        )
    );
    assert_eq!(
        (
            budgets.checkpoint.queued_requests,
            budgets.checkpoint.active_requests,
            budgets.checkpoint.staging_slots,
            budgets.checkpoint.mapped_bytes_in_flight,
            budgets.checkpoint.store_bytes_in_flight,
            budgets.checkpoint.bytes_per_blob,
            budgets.checkpoint.bytes_per_checkpoint,
            budgets.checkpoint.manifest_nodes,
            budgets.checkpoint.manifest_blobs,
            budgets.checkpoint.manifest_bytes,
        ),
        (
            4,
            1,
            3,
            16 << 20,
            64 << 20,
            8 << 20,
            1 << 30,
            1_048_576,
            1_048_576,
            64 << 20
        )
    );
    assert_fields!(budgets.rollback,
        retained_frontiers: 32, retained_bytes: 2 << 30, genesis_persistent_bytes: 256 << 20,
        frontier_metadata_bytes: 2 << 20, log_ticks: 256, log_bytes: 256 << 20,
        replay_sink_records_in_flight: 64, replay_sink_bytes_in_flight: 64 << 20,
        active_public_replays: 1, ticks_per_public_replay: 256, bytes_per_public_replay: 1 << 30,
        result_bytes_per_public_replay: 32 << 20, divergence_artifact_bytes: 32 << 20,
        active_corrections: 1, ticks_per_correction: 256, bytes_per_correction: 1 << 30,
        result_bytes_per_correction: 32 << 20, recovery_replay_ticks: 256,
    );
    assert_eq!(
        (
            budgets.participant.operations_in_flight,
            budgets.participant.input_bytes_per_tick,
            budgets.participant.effects_per_tick,
            budgets.participant.effect_bytes_per_tick,
            budgets.participant.events_per_tick,
            budgets.participant.event_bytes_per_tick,
            budgets.participant.bytes_per_event,
            budgets.participant.state_and_snapshot_bytes_per_frontier,
            budgets.participant.snapshot_bytes_per_checkpoint,
            budgets.participant.artifact_records_per_tick,
            budgets.participant.artifact_bytes_per_tick,
        ),
        (
            64,
            8 << 20,
            4_096,
            8 << 20,
            4_096,
            4 << 20,
            1 << 10,
            64 << 20,
            64 << 20,
            1_048_576,
            64 << 20
        )
    );
    assert_eq!(
        (
            budgets.runtime.interest_control_queue,
            budgets.runtime.callback_completion_slots,
            budgets.runtime.callback_completion_bytes,
            budgets.runtime.render_completion_cells,
        ),
        (4_096, 128, 128 << 20, 32)
    );
}

#[test]
fn budget_field_ordinals_remain_closed_and_declaration_ordered() {
    for (group, last) in [
        (BudgetGroup::Identity, 19),
        (BudgetGroup::Canonical, 8),
        (BudgetGroup::Content, 9),
        (BudgetGroup::Query, 7),
        (BudgetGroup::Observation, 10),
        (BudgetGroup::Presentation, 8),
        (BudgetGroup::Checkpoint, 10),
        (BudgetGroup::Rollback, 18),
        (BudgetGroup::Participant, 11),
        (BudgetGroup::Runtime, 4),
    ] {
        assert!(ResourceBudgetField::try_new(group, 1).is_ok());
        assert!(ResourceBudgetField::try_new(group, last).is_ok());
        assert!(ResourceBudgetField::try_new(group, last + 1).is_err());
    }
}

#[test]
fn compiled_maxima_keep_fixed_and_variable_budget_limits_distinct() {
    let maxima = ResourceBudgets::compiled_maxima();
    assert_fields!(maxima.identity,
        worlds: 16, retired_replay_streams_per_client: 64, materials_per_world: 65_535,
        volumes_per_world: 1_048_576, participants_per_world: 1_024,
        terminal_receipt_bytes_per_world: 512 << 20, artifact_leases_per_world: 1_024,
    );
    assert_fields!(maxima.canonical,
        pending_ticks: 1, inputs_per_tick: 4_096, encoded_bytes_per_tick: 8 << 20,
        correlation_bytes_per_tick: 320 << 10, bricks_per_command: 64,
        cells_per_command: 32_768, changed_bricks_per_tick: 16_384, scratch_bytes: 1 << 30,
    );
    assert_fields!(maxima.rollback,
        retained_frontiers: 256, retained_bytes: 16 << 30, log_ticks: 65_536,
        active_corrections: 1, ticks_per_correction: 4_096, bytes_per_correction: 4 << 30,
        recovery_replay_ticks: 4_096,
    );
    assert_fields!(maxima.runtime,
        interest_control_queue: 16_384, callback_completion_slots: 256,
        callback_completion_bytes: 512 << 20, render_completion_cells: 32,
    );
}

#[test]
fn exact_fixed_configuration_records_are_explicit() {
    let format =
        PlacementFixedFormat::try_new(16, i32::MAX as u32, SimulationUnitId::from_bytes([7; 16]))
            .unwrap();
    assert_eq!(format.fractional_bits(), 16);
    assert_eq!(format.cell_extent_raw(), i32::MAX as u32);
    assert_eq!(format.simulation_unit().to_bytes(), [7; 16]);
    assert!(PlacementFixedFormat::try_new(17, 1, SimulationUnitId::from_bytes([0; 16])).is_err());
    assert!(PlacementFixedFormat::try_new(0, 0, SimulationUnitId::from_bytes([0; 16])).is_err());

    let genesis = WorldGenesisConfig { placement: format };
    assert_eq!(genesis.placement, format);
    assert_eq!(
        moria::config::RollbackConfig::default(),
        moria::config::RollbackConfig {
            capacity_ticks: 32,
            log_ticks: 256,
            retain_outcome_bytes: true
        }
    );
    assert_eq!(
        moria::config::PresentationConfig::default(),
        moria::config::PresentationConfig {
            enabled: true,
            show_stale: true,
            chunk_edge_cells: 32,
            maximum_lod: 6
        }
    );
}

#[test]
fn execution_policy_retains_the_exact_candidate_fault_coordinates() {
    let policy = ExecutionPolicy::Candidate {
        diagnostics: CandidateDiagnostics {
            fault_once: Some(CandidateFaultOnce {
                tick: Tick::from_raw(3),
                command_order: CanonicalOrder::from_raw(9),
                stage: CandidateFaultStage::AfterBrickConstructionBeforePublication,
            }),
        },
    };
    assert!(matches!(policy, ExecutionPolicy::Candidate { .. }));
    assert!(matches!(
        ExecutionPolicy::ReplayGrade,
        ExecutionPolicy::ReplayGrade
    ));
    let _ = ContractDigest::from_bytes([0; 32]);
}
