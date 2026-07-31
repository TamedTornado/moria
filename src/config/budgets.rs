//! Closed TECH-017 resource-budget records and their normative defaults.

macro_rules! budget_struct {
    ($name:ident { $($field:ident: $ty:ty = $value:expr),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name { $(pub $field: $ty,)+ }
        impl Default for $name {
            fn default() -> Self { Self { $($field: $value,)+ } }
        }
    };
}

budget_struct!(IdentityBudgets {
    worlds: u32 = 1,
    retired_replay_streams_per_client: u32 = 4,
    materials_per_world: u32 = 4_096,
    volumes_per_world: u32 = 65_536,
    participants_per_world: u32 = 64,
    input_sources_per_world: u32 = 4_096,
    base_sources_per_world: u32 = 256,
    base_authorities_per_world: u32 = 65_536,
    content_blob_stores_per_world: u32 = 4,
    checkpoint_stores_per_world: u32 = 4,
    replay_sinks_per_world: u32 = 4,
    rng_streams_per_participant: u32 = 32,
    representation_contracts_per_participant: u32 = 64,
    interests_per_world: u32 = 4_096,
    operation_records_per_world: u32 = 16_384,
    terminal_receipts_per_world: u32 = 8_192,
    terminal_receipt_bytes_per_world: u64 = 64 << 20,
    root_leases_per_world: u32 = 4_096,
    artifact_leases_per_world: u32 = 256,
});
budget_struct!(CanonicalBudgets {
    pending_ticks: u32 = 1,
    inputs_per_tick: u32 = 4_096,
    encoded_bytes_per_tick: u64 = 8 << 20,
    correlation_bytes_per_tick: u64 = 320 << 10,
    bricks_per_command: u32 = 64,
    cells_per_command: u32 = 32_768,
    changed_bricks_per_tick: u32 = 512,
    scratch_bytes: u64 = 256 << 20,
});
budget_struct!(ContentBudgets {
    base_request_queue: u32 = 256,
    base_requests_in_flight: u32 = 32,
    base_completion_bytes_in_flight: u64 = 64 << 10,
    materialization_bricks_per_job: u32 = 4_096,
    resident_dense_bricks: u32 = 65_536,
    resident_uniform_bricks: u32 = 65_536,
    resident_radix_nodes: u32 = 1_048_576,
    resident_directory_buckets: u32 = 1_048_576,
    authoritative_gpu_bytes: u64 = 2 << 30,
});
budget_struct!(QueryBudgets {
    queued_requests: u32 = 256,
    in_flight_requests: u32 = 3,
    bricks_per_request: u32 = 4_096,
    records_per_result: u32 = 65_536,
    bytes_per_result: u64 = 4 << 20,
    volume_revisions_per_request: u32 = 256,
    readback_bytes_in_flight: u64 = 48 << 20,
});
budget_struct!(ObservationBudgets {
    records_per_world: u32 = 8_192,
    payload_bytes_per_world: u64 = 32 << 20,
    bytes_per_record: u32 = 4 << 10,
    subscriptions_per_world: u32 = 64,
    volumes_per_subscription: u32 = 1_024,
    records_per_poll: u32 = 256,
    bytes_per_poll: u64 = 1 << 20,
    resnapshot_volume_summaries: u32 = 1_024,
    resnapshot_region_summaries: u32 = 4_096,
    resnapshot_bytes: u64 = 16 << 20,
});
budget_struct!(PresentationBudgets {
    queued_chunks: u32 = 4_096,
    resident_chunks: u32 = 65_536,
    in_flight_jobs: u32 = 3,
    vertices_per_job: u32 = 1_048_576,
    indices_per_job: u32 = 6_291_456,
    bytes_per_job: u64 = 64 << 20,
    resident_bytes: u64 = 1 << 30,
    dressing_records_per_chunk: u32 = 65_536,
});
budget_struct!(CheckpointBudgets {
    queued_requests: u32 = 4,
    active_requests: u32 = 1,
    staging_slots: u32 = 3,
    mapped_bytes_in_flight: u64 = 16 << 20,
    store_bytes_in_flight: u64 = 64 << 20,
    bytes_per_blob: u64 = 8 << 20,
    bytes_per_checkpoint: u64 = 1 << 30,
    manifest_nodes: u32 = 1_048_576,
    manifest_blobs: u32 = 1_048_576,
    manifest_bytes: u64 = 128 << 20,
});
budget_struct!(RollbackBudgets {
    retained_frontiers: u32 = 32,
    retained_bytes: u64 = 2 << 30,
    genesis_persistent_bytes: u64 = 256 << 20,
    frontier_metadata_bytes: u64 = 2 << 20,
    log_ticks: u32 = 256,
    log_bytes: u64 = 256 << 20,
    replay_sink_records_in_flight: u32 = 64,
    replay_sink_bytes_in_flight: u64 = 64 << 20,
    active_public_replays: u32 = 1,
    ticks_per_public_replay: u32 = 256,
    bytes_per_public_replay: u64 = 1 << 30,
    result_bytes_per_public_replay: u64 = 32 << 20,
    divergence_artifact_bytes: u64 = 32 << 20,
    active_corrections: u32 = 1,
    ticks_per_correction: u32 = 256,
    bytes_per_correction: u64 = 1 << 30,
    result_bytes_per_correction: u64 = 32 << 20,
    recovery_replay_ticks: u32 = 256,
});
budget_struct!(ParticipantBudgets {
    operations_in_flight: u32 = 64,
    input_bytes_per_tick: u64 = 8 << 20,
    effects_per_tick: u32 = 4_096,
    effect_bytes_per_tick: u64 = 8 << 20,
    events_per_tick: u32 = 4_096,
    event_bytes_per_tick: u64 = 4 << 20,
    bytes_per_event: u32 = 1 << 10,
    state_and_snapshot_bytes_per_frontier: u64 = 64 << 20,
    snapshot_bytes_per_checkpoint: u64 = 64 << 20,
    artifact_records_per_tick: u32 = 1_048_576,
    artifact_bytes_per_tick: u64 = 64 << 20,
});
budget_struct!(RuntimeBudgets {
    interest_control_queue: u32 = 4_096,
    callback_completion_slots: u32 = 128,
    callback_completion_bytes: u64 = 128 << 20,
    render_completion_cells: u32 = 32,
});

/// All finite capacities accepted by the v1 configuration contract.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ResourceBudgets {
    pub identity: IdentityBudgets,
    pub canonical: CanonicalBudgets,
    pub content: ContentBudgets,
    pub query: QueryBudgets,
    pub observation: ObservationBudgets,
    pub presentation: PresentationBudgets,
    pub checkpoint: CheckpointBudgets,
    pub rollback: RollbackBudgets,
    pub participant: ParticipantBudgets,
    pub runtime: RuntimeBudgets,
}

impl ResourceBudgets {
    /// Returns the portable v1 compiled maxima for every budget field.
    ///
    /// These are configuration limits, not an allocation request. TECH-036
    /// performs the remaining cross-field and device-limit validation.
    #[must_use]
    pub const fn compiled_maxima() -> Self {
        Self {
            identity: IdentityBudgets {
                worlds: 16,
                retired_replay_streams_per_client: 64,
                materials_per_world: 65_535,
                volumes_per_world: 1_048_576,
                participants_per_world: 1_024,
                input_sources_per_world: 65_535,
                base_sources_per_world: 1_024,
                base_authorities_per_world: 1_048_576,
                content_blob_stores_per_world: 16,
                checkpoint_stores_per_world: 16,
                replay_sinks_per_world: 16,
                rng_streams_per_participant: 256,
                representation_contracts_per_participant: 256,
                interests_per_world: 16_384,
                operation_records_per_world: 65_536,
                terminal_receipts_per_world: 65_536,
                terminal_receipt_bytes_per_world: 512 << 20,
                root_leases_per_world: 16_384,
                artifact_leases_per_world: 1_024,
            },
            canonical: CanonicalBudgets {
                pending_ticks: 1,
                inputs_per_tick: 4_096,
                encoded_bytes_per_tick: 8 << 20,
                correlation_bytes_per_tick: 320 << 10,
                bricks_per_command: 64,
                cells_per_command: 32_768,
                changed_bricks_per_tick: 16_384,
                scratch_bytes: 1 << 30,
            },
            content: ContentBudgets {
                base_request_queue: 1_024,
                base_requests_in_flight: 128,
                base_completion_bytes_in_flight: 256 << 10,
                materialization_bricks_per_job: 16_384,
                resident_dense_bricks: 4_194_304,
                resident_uniform_bricks: 4_194_304,
                resident_radix_nodes: 16_777_216,
                resident_directory_buckets: 16_777_216,
                authoritative_gpu_bytes: 16 << 30,
            },
            query: QueryBudgets {
                queued_requests: 1_024,
                in_flight_requests: 8,
                bricks_per_request: 16_384,
                records_per_result: 262_144,
                bytes_per_result: 16 << 20,
                volume_revisions_per_request: 1_024,
                readback_bytes_in_flight: 128 << 20,
            },
            observation: ObservationBudgets {
                records_per_world: 65_536,
                payload_bytes_per_world: 256 << 20,
                bytes_per_record: 64 << 10,
                subscriptions_per_world: 256,
                volumes_per_subscription: 16_384,
                records_per_poll: 4_096,
                bytes_per_poll: 16 << 20,
                resnapshot_volume_summaries: 16_384,
                resnapshot_region_summaries: 65_536,
                resnapshot_bytes: 256 << 20,
            },
            presentation: PresentationBudgets {
                queued_chunks: 16_384,
                resident_chunks: 1_048_576,
                in_flight_jobs: 8,
                vertices_per_job: 4_194_304,
                indices_per_job: 25_165_824,
                bytes_per_job: 256 << 20,
                resident_bytes: 8 << 30,
                dressing_records_per_chunk: 262_144,
            },
            checkpoint: CheckpointBudgets {
                queued_requests: 16,
                active_requests: 1,
                staging_slots: 3,
                mapped_bytes_in_flight: 64 << 20,
                store_bytes_in_flight: 256 << 20,
                bytes_per_blob: 64 << 20,
                bytes_per_checkpoint: 4 << 30,
                manifest_nodes: 16_777_216,
                manifest_blobs: 16_777_216,
                manifest_bytes: 256 << 20,
            },
            rollback: RollbackBudgets {
                retained_frontiers: 256,
                retained_bytes: 16 << 30,
                genesis_persistent_bytes: 2 << 30,
                frontier_metadata_bytes: 64 << 20,
                log_ticks: 65_536,
                log_bytes: 4 << 30,
                replay_sink_records_in_flight: 1_024,
                replay_sink_bytes_in_flight: 512 << 20,
                active_public_replays: 16,
                ticks_per_public_replay: 4_096,
                bytes_per_public_replay: 4 << 30,
                result_bytes_per_public_replay: 512 << 20,
                divergence_artifact_bytes: 256 << 20,
                active_corrections: 1,
                ticks_per_correction: 4_096,
                bytes_per_correction: 4 << 30,
                result_bytes_per_correction: 512 << 20,
                recovery_replay_ticks: 4_096,
            },
            participant: ParticipantBudgets {
                operations_in_flight: 1_024,
                input_bytes_per_tick: 8 << 20,
                effects_per_tick: 4_096,
                effect_bytes_per_tick: 8 << 20,
                events_per_tick: 16_384,
                event_bytes_per_tick: 16 << 20,
                bytes_per_event: 4 << 10,
                state_and_snapshot_bytes_per_frontier: 64 << 20,
                snapshot_bytes_per_checkpoint: 64 << 20,
                artifact_records_per_tick: 4_194_304,
                artifact_bytes_per_tick: 256 << 20,
            },
            runtime: RuntimeBudgets {
                interest_control_queue: 16_384,
                callback_completion_slots: 256,
                callback_completion_bytes: 512 << 20,
                render_completion_cells: 32,
            },
        }
    }
}
