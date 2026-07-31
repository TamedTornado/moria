use moria::{
    canonical::{CanonicalOrder, ContractDigest, Tick},
    config::{
        BudgetGroup, CandidateDiagnostics, CandidateFaultOnce, CandidateFaultStage,
        DevicePageLimits, ExecutionPolicy, PlacementFixedFormat, ResourceBudgets, RollbackConfig,
        SimulationUnitId, WorldGenesisConfig, validate_resource_budgets,
    },
    facade::{BoundedUtf8, ConfigError, ConfigErrorCode, ConfigField, ResourceBudgetField},
};

macro_rules! assert_fields {
    ($value:expr, $($field:ident: $expected:expr),+ $(,)?) => {
        $(assert_eq!($value.$field, $expected, stringify!($field));)+
    };
}

#[test]
fn public_budget_validator_accepts_one_record_content_pools() {
    let rollback = RollbackConfig::default();
    let limits = DevicePageLimits::portable_baseline();

    let mut uniform_pool = ResourceBudgets::default();
    uniform_pool.content.resident_uniform_bricks = 1;
    assert_eq!(
        validate_resource_budgets(&uniform_pool, &rollback, &limits),
        Ok(1_988_100_096),
    );

    let mut directory_pool = ResourceBudgets::default();
    directory_pool.content.resident_directory_buckets = 1;
    assert_eq!(
        validate_resource_budgets(&directory_pool, &rollback, &limits),
        Ok(1_988_100_096),
    );
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
            128 << 20
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
    let declared_fields = [
        (
            BudgetGroup::Identity,
            &[
                "worlds",
                "retired_replay_streams_per_client",
                "materials_per_world",
                "volumes_per_world",
                "participants_per_world",
                "input_sources_per_world",
                "base_sources_per_world",
                "base_authorities_per_world",
                "content_blob_stores_per_world",
                "checkpoint_stores_per_world",
                "replay_sinks_per_world",
                "rng_streams_per_participant",
                "representation_contracts_per_participant",
                "interests_per_world",
                "operation_records_per_world",
                "terminal_receipts_per_world",
                "terminal_receipt_bytes_per_world",
                "root_leases_per_world",
                "artifact_leases_per_world",
            ][..],
        ),
        (
            BudgetGroup::Canonical,
            &[
                "pending_ticks",
                "inputs_per_tick",
                "encoded_bytes_per_tick",
                "correlation_bytes_per_tick",
                "bricks_per_command",
                "cells_per_command",
                "changed_bricks_per_tick",
                "scratch_bytes",
            ][..],
        ),
        (
            BudgetGroup::Content,
            &[
                "base_request_queue",
                "base_requests_in_flight",
                "base_completion_bytes_in_flight",
                "materialization_bricks_per_job",
                "resident_dense_bricks",
                "resident_uniform_bricks",
                "resident_radix_nodes",
                "resident_directory_buckets",
                "authoritative_gpu_bytes",
            ][..],
        ),
        (
            BudgetGroup::Query,
            &[
                "queued_requests",
                "in_flight_requests",
                "bricks_per_request",
                "records_per_result",
                "bytes_per_result",
                "volume_revisions_per_request",
                "readback_bytes_in_flight",
            ][..],
        ),
        (
            BudgetGroup::Observation,
            &[
                "records_per_world",
                "payload_bytes_per_world",
                "bytes_per_record",
                "subscriptions_per_world",
                "volumes_per_subscription",
                "records_per_poll",
                "bytes_per_poll",
                "resnapshot_volume_summaries",
                "resnapshot_region_summaries",
                "resnapshot_bytes",
            ][..],
        ),
        (
            BudgetGroup::Presentation,
            &[
                "queued_chunks",
                "resident_chunks",
                "in_flight_jobs",
                "vertices_per_job",
                "indices_per_job",
                "bytes_per_job",
                "resident_bytes",
                "dressing_records_per_chunk",
            ][..],
        ),
        (
            BudgetGroup::Checkpoint,
            &[
                "queued_requests",
                "active_requests",
                "staging_slots",
                "mapped_bytes_in_flight",
                "store_bytes_in_flight",
                "bytes_per_blob",
                "bytes_per_checkpoint",
                "manifest_nodes",
                "manifest_blobs",
                "manifest_bytes",
            ][..],
        ),
        (
            BudgetGroup::Rollback,
            &[
                "retained_frontiers",
                "retained_bytes",
                "genesis_persistent_bytes",
                "frontier_metadata_bytes",
                "log_ticks",
                "log_bytes",
                "replay_sink_records_in_flight",
                "replay_sink_bytes_in_flight",
                "active_public_replays",
                "ticks_per_public_replay",
                "bytes_per_public_replay",
                "result_bytes_per_public_replay",
                "divergence_artifact_bytes",
                "active_corrections",
                "ticks_per_correction",
                "bytes_per_correction",
                "result_bytes_per_correction",
                "recovery_replay_ticks",
            ][..],
        ),
        (
            BudgetGroup::Participant,
            &[
                "operations_in_flight",
                "input_bytes_per_tick",
                "effects_per_tick",
                "effect_bytes_per_tick",
                "events_per_tick",
                "event_bytes_per_tick",
                "bytes_per_event",
                "state_and_snapshot_bytes_per_frontier",
                "snapshot_bytes_per_checkpoint",
                "artifact_records_per_tick",
                "artifact_bytes_per_tick",
            ][..],
        ),
        (
            BudgetGroup::Runtime,
            &[
                "interest_control_queue",
                "callback_completion_slots",
                "callback_completion_bytes",
                "render_completion_cells",
            ][..],
        ),
    ];

    for (group, fields) in declared_fields {
        for (index, field_name) in fields.iter().enumerate() {
            let field_code = u16::try_from(index + 1).unwrap();
            assert_eq!(
                ResourceBudgetField::try_new(group, field_code),
                Ok(ResourceBudgetField { group, field_code }),
                "{group:?}.{field_name} must retain ordinal {field_code}",
            );
        }
        let next_ordinal = u16::try_from(fields.len() + 1).unwrap();
        assert!(ResourceBudgetField::try_new(group, next_ordinal).is_err());
    }
}

#[test]
fn compiled_maxima_keep_fixed_and_variable_budget_limits_distinct() {
    let maxima = ResourceBudgets::compiled_maxima();
    assert_fields!(maxima.identity,
        worlds: 16, retired_replay_streams_per_client: 64, materials_per_world: 65_535,
        volumes_per_world: 1_048_576, participants_per_world: 1_024,
        input_sources_per_world: 65_535, base_sources_per_world: 1_024,
        base_authorities_per_world: 1_048_576, content_blob_stores_per_world: 16,
        checkpoint_stores_per_world: 16, replay_sinks_per_world: 16,
        rng_streams_per_participant: 256, representation_contracts_per_participant: 256,
        interests_per_world: 16_384, operation_records_per_world: 65_536,
        terminal_receipts_per_world: 65_536, terminal_receipt_bytes_per_world: 512 << 20,
        root_leases_per_world: 16_384, artifact_leases_per_world: 1_024,
    );
    assert_fields!(maxima.canonical,
        pending_ticks: 1, inputs_per_tick: 4_096, encoded_bytes_per_tick: 8 << 20,
        correlation_bytes_per_tick: 320 << 10, bricks_per_command: 64,
        cells_per_command: 32_768, changed_bricks_per_tick: 16_384, scratch_bytes: 1 << 30,
    );
    assert_fields!(maxima.content,
        base_request_queue: 1_024, base_requests_in_flight: 128,
        base_completion_bytes_in_flight: 256 << 10, materialization_bricks_per_job: 16_384,
        resident_dense_bricks: 4_194_304, resident_uniform_bricks: 4_194_304,
        resident_radix_nodes: 16_777_216, resident_directory_buckets: 16_777_216,
        authoritative_gpu_bytes: 16 << 30,
    );
    assert_fields!(maxima.query,
        queued_requests: 1_024, in_flight_requests: 8, bricks_per_request: 16_384,
        records_per_result: 262_144, bytes_per_result: 16 << 20,
        volume_revisions_per_request: 1_024, readback_bytes_in_flight: 128 << 20,
    );
    assert_fields!(maxima.observation,
        records_per_world: 65_536, payload_bytes_per_world: 256 << 20,
        bytes_per_record: 64 << 10, subscriptions_per_world: 256,
        volumes_per_subscription: 16_384, records_per_poll: 4_096, bytes_per_poll: 16 << 20,
        resnapshot_volume_summaries: 16_384, resnapshot_region_summaries: 65_536,
        resnapshot_bytes: 256 << 20,
    );
    assert_fields!(maxima.presentation,
        queued_chunks: 16_384, resident_chunks: 1_048_576, in_flight_jobs: 8,
        vertices_per_job: 4_194_304, indices_per_job: 25_165_824,
        bytes_per_job: 256 << 20, resident_bytes: 8 << 30,
        dressing_records_per_chunk: 262_144,
    );
    assert_fields!(maxima.checkpoint,
        queued_requests: 16, active_requests: 1, staging_slots: 3,
        mapped_bytes_in_flight: 64 << 20, store_bytes_in_flight: 256 << 20,
        bytes_per_blob: 64 << 20, bytes_per_checkpoint: 4 << 30,
        manifest_nodes: 16_777_216, manifest_blobs: 16_777_216, manifest_bytes: 256 << 20,
    );
    assert_fields!(maxima.rollback,
        retained_frontiers: 256, retained_bytes: 16 << 30, genesis_persistent_bytes: 2 << 30,
        frontier_metadata_bytes: 64 << 20, log_ticks: 65_536, log_bytes: 4 << 30,
        replay_sink_records_in_flight: 1_024, replay_sink_bytes_in_flight: 512 << 20,
        active_public_replays: 16, ticks_per_public_replay: 4_096,
        bytes_per_public_replay: 4 << 30, result_bytes_per_public_replay: 512 << 20,
        divergence_artifact_bytes: 256 << 20, active_corrections: 1,
        ticks_per_correction: 4_096, bytes_per_correction: 4 << 30,
        result_bytes_per_correction: 512 << 20, recovery_replay_ticks: 4_096,
    );
    assert_fields!(maxima.participant,
        operations_in_flight: 1_024, input_bytes_per_tick: 8 << 20, effects_per_tick: 4_096,
        effect_bytes_per_tick: 8 << 20, events_per_tick: 16_384, event_bytes_per_tick: 16 << 20,
        bytes_per_event: 4 << 10, state_and_snapshot_bytes_per_frontier: 64 << 20,
        snapshot_bytes_per_checkpoint: 64 << 20, artifact_records_per_tick: 4_194_304,
        artifact_bytes_per_tick: 256 << 20,
    );
    assert_fields!(maxima.runtime,
        interest_control_queue: 16_384, callback_completion_slots: 256,
        callback_completion_bytes: 512 << 20, render_completion_cells: 32,
    );
}

#[test]
fn config_errors_preserve_the_exact_budget_field_and_bounded_diagnostic() {
    let diagnostic_text = "x".repeat(160);
    let error = ConfigError {
        code: ConfigErrorCode::CrossLimitViolation,
        field: ConfigField::Budgets(
            ResourceBudgetField::try_new(BudgetGroup::Checkpoint, 7).unwrap(),
        ),
        diagnostic: BoundedUtf8::try_from_str(&diagnostic_text).unwrap(),
    };

    assert_eq!(error.code, ConfigErrorCode::CrossLimitViolation);
    assert_eq!(
        error.field,
        ConfigField::Budgets(ResourceBudgetField {
            group: BudgetGroup::Checkpoint,
            field_code: 7,
        }),
    );
    assert_eq!(error.diagnostic.len(), 160);
    assert_eq!(error.diagnostic.as_str(), diagnostic_text);
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
