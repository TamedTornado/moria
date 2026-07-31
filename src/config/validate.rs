//! Pre-genesis validation for the finite TECH-036 resource contract.

use crate::facade::{
    BoundedUtf8, BudgetGroup, ConfigError, ConfigErrorCode, ConfigField, ResourceBudgetField,
};

use super::ResourceBudgets;

const READBACK_PAGE_BYTES: u128 = 16 << 20;
const BRICK_COPY_ON_WRITE_BYTES: u128 = 2_048 + 26 * 1_024;
const CHANGED_VOLUME_RECORD_BYTES: u128 = 256;
const REQUIRED_FRONTIERS: u128 = 20;

/// Validates all standalone TECH-036 budget limits before genesis.
///
/// The returned value is the conservative allocation reservation for the
/// required 20 retained frontiers. Callers must perform this check before
/// invoking a consumer callback or allocating device resources.
///
/// # Errors
///
/// Returns a field-addressed [`ConfigError`] for a zero, fixed-value, compiled
/// maximum, cross-limit, or checked-arithmetic violation.
pub fn validate_resource_budgets(budgets: &ResourceBudgets) -> Result<u128, ConfigError> {
    validate_field_ranges(budgets)?;
    validate_cross_limits(budgets)?;
    let required = required_twenty_frontier_bytes(budgets)?;
    if required > u128::from(budgets.rollback.retained_bytes) {
        return Err(cross(BudgetGroup::Rollback, 2, "retained bytes"));
    }
    if required > u128::from(budgets.content.authoritative_gpu_bytes) {
        return Err(cross(BudgetGroup::Content, 9, "GPU bytes"));
    }
    Ok(required)
}

/// Calculates TECH-036's conservative 20-frontier allocation reservation.
///
/// This is intentionally independent of field-range validation so arithmetic
/// failure remains observable and cannot become a wrapped allocation size.
///
/// # Errors
///
/// Returns [`ConfigErrorCode::ArithmeticOverflow`] if any required product or
/// sum cannot be represented in `u128`.
pub fn required_twenty_frontier_bytes(budgets: &ResourceBudgets) -> Result<u128, ConfigError> {
    let cow = u128::from(budgets.canonical.changed_bricks_per_tick)
        .checked_mul(BRICK_COPY_ON_WRITE_BYTES)
        .ok_or_else(|| overflow(BudgetGroup::Canonical, 7))?;
    let records = u128::from(budgets.canonical.inputs_per_tick)
        .checked_add(u128::from(budgets.participant.effects_per_tick))
        .ok_or_else(|| overflow(BudgetGroup::Participant, 3))?;
    let volumes = records
        .checked_mul(CHANGED_VOLUME_RECORD_BYTES)
        .ok_or_else(|| overflow(BudgetGroup::Participant, 3))?;
    let frontier = cow
        .checked_add(volumes)
        .and_then(|v| v.checked_add(u128::from(budgets.rollback.frontier_metadata_bytes)))
        .and_then(|v| {
            v.checked_add(u128::from(
                budgets.participant.state_and_snapshot_bytes_per_frontier,
            ))
        })
        .ok_or_else(|| overflow(BudgetGroup::Rollback, 4))?;
    u128::from(budgets.rollback.genesis_persistent_bytes)
        .checked_add(
            REQUIRED_FRONTIERS
                .checked_mul(frontier)
                .ok_or_else(|| overflow(BudgetGroup::Rollback, 2))?,
        )
        .ok_or_else(|| overflow(BudgetGroup::Rollback, 2))
}

fn validate_cross_limits(b: &ResourceBudgets) -> Result<(), ConfigError> {
    let operation_minimum = [
        b.canonical.pending_ticks,
        b.content.base_request_queue,
        b.runtime.interest_control_queue,
        b.query.queued_requests,
        b.checkpoint.queued_requests,
        b.rollback.active_public_replays,
        b.rollback.active_corrections,
        1,
        b.rollback.replay_sink_records_in_flight,
        b.presentation.queued_chunks,
        b.participant.operations_in_flight,
        b.observation.subscriptions_per_world,
    ]
    .into_iter()
    .try_fold(0_u128, |sum, count| sum.checked_add(u128::from(count)))
    .ok_or_else(|| overflow(BudgetGroup::Identity, 15))?;
    if u128::from(b.identity.operation_records_per_world) < operation_minimum {
        return Err(cross(BudgetGroup::Identity, 15, "operation records"));
    }
    if u128::from(b.content.base_completion_bytes_in_flight)
        < 2_048_u128 * u128::from(b.content.base_requests_in_flight)
    {
        return Err(cross(BudgetGroup::Content, 3, "base completion bytes"));
    }
    let largest_callback = [
        b.content.base_completion_bytes_in_flight,
        b.participant.state_and_snapshot_bytes_per_frontier,
        b.participant.snapshot_bytes_per_checkpoint,
        b.checkpoint.bytes_per_blob,
    ]
    .into_iter()
    .max()
    .expect("nonempty callback limits");
    if b.runtime.callback_completion_bytes < largest_callback {
        return Err(cross(BudgetGroup::Runtime, 3, "callback completion bytes"));
    }
    let render_cells = 1_u128
        .checked_add(u128::from(b.query.in_flight_requests))
        .and_then(|v| v.checked_add(u128::from(b.checkpoint.staging_slots)))
        .and_then(|v| v.checked_add(u128::from(b.presentation.in_flight_jobs)))
        .and_then(|v| v.checked_add(2))
        .ok_or_else(|| overflow(BudgetGroup::Runtime, 4))?;
    if u128::from(b.runtime.render_completion_cells) < render_cells {
        return Err(cross(BudgetGroup::Runtime, 4, "render cells"));
    }
    if b.query.readback_bytes_in_flight < b.query.bytes_per_result {
        return Err(cross(BudgetGroup::Query, 7, "readback bytes"));
    }
    let largest_terminal_result = [
        b.query.bytes_per_result,
        b.observation.resnapshot_bytes,
        b.rollback.result_bytes_per_public_replay,
        b.rollback.result_bytes_per_correction,
        b.rollback.divergence_artifact_bytes,
    ]
    .into_iter()
    .max()
    .expect("nonempty terminal result limits");
    if b.identity.terminal_receipt_bytes_per_world < largest_terminal_result {
        return Err(cross(BudgetGroup::Identity, 17, "terminal receipt bytes"));
    }
    if b.observation.bytes_per_record as u64 > b.observation.payload_bytes_per_world
        || b.observation.bytes_per_poll > b.observation.payload_bytes_per_world
        || b.observation.resnapshot_bytes > b.observation.payload_bytes_per_world
        || b.observation.records_per_poll > b.observation.records_per_world
        || b.observation.resnapshot_volume_summaries > b.identity.volumes_per_world
        || b.observation.resnapshot_volume_summaries > b.observation.volumes_per_subscription
    {
        return Err(cross(BudgetGroup::Observation, 2, "observation capacity"));
    }
    if b.presentation.resident_bytes < b.presentation.bytes_per_job {
        return Err(cross(BudgetGroup::Presentation, 7, "resident bytes"));
    }
    if b.checkpoint.mapped_bytes_in_flight < READBACK_PAGE_BYTES as u64
        || b.checkpoint.store_bytes_in_flight < b.checkpoint.bytes_per_blob
        || b.checkpoint.bytes_per_blob > b.checkpoint.bytes_per_checkpoint
        || b.checkpoint.manifest_bytes > b.checkpoint.bytes_per_checkpoint
    {
        return Err(cross(BudgetGroup::Checkpoint, 4, "checkpoint capacity"));
    }
    if b.rollback.retained_frontiers > b.rollback.log_ticks {
        return Err(cross(BudgetGroup::Rollback, 1, "retained frontiers"));
    }
    if b.rollback.recovery_replay_ticks > b.rollback.log_ticks
        || b.rollback.ticks_per_correction > b.rollback.log_ticks
    {
        return Err(cross(BudgetGroup::Rollback, 18, "rollback log"));
    }
    if b.participant.bytes_per_event as u64 > b.participant.event_bytes_per_tick
        || b.participant.snapshot_bytes_per_checkpoint > b.checkpoint.bytes_per_checkpoint
    {
        return Err(cross(BudgetGroup::Participant, 7, "participant capacity"));
    }
    Ok(())
}

fn validate_field_ranges(b: &ResourceBudgets) -> Result<(), ConfigError> {
    let m = ResourceBudgets::compiled_maxima();
    macro_rules! fields { ($group:ident, $value:expr, $max:expr, $($field:ident),+ $(,)?) => {{ let mut code = 0; $(code += 1; check($value.$field, $max.$field, BudgetGroup::$group, code)?;)+ }}; }
    fields!(
        Identity,
        b.identity,
        m.identity,
        worlds,
        retired_replay_streams_per_client,
        materials_per_world,
        volumes_per_world,
        participants_per_world,
        input_sources_per_world,
        base_sources_per_world,
        base_authorities_per_world,
        content_blob_stores_per_world,
        checkpoint_stores_per_world,
        replay_sinks_per_world,
        rng_streams_per_participant,
        representation_contracts_per_participant,
        interests_per_world,
        operation_records_per_world,
        terminal_receipts_per_world,
        terminal_receipt_bytes_per_world,
        root_leases_per_world,
        artifact_leases_per_world
    );
    fields!(
        Canonical,
        b.canonical,
        m.canonical,
        pending_ticks,
        inputs_per_tick,
        encoded_bytes_per_tick,
        correlation_bytes_per_tick,
        bricks_per_command,
        cells_per_command,
        changed_bricks_per_tick,
        scratch_bytes
    );
    fields!(
        Content,
        b.content,
        m.content,
        base_request_queue,
        base_requests_in_flight,
        base_completion_bytes_in_flight,
        materialization_bricks_per_job,
        resident_dense_bricks,
        resident_uniform_bricks,
        resident_radix_nodes,
        resident_directory_buckets,
        authoritative_gpu_bytes
    );
    fields!(
        Query,
        b.query,
        m.query,
        queued_requests,
        in_flight_requests,
        bricks_per_request,
        records_per_result,
        bytes_per_result,
        volume_revisions_per_request,
        readback_bytes_in_flight
    );
    fields!(
        Observation,
        b.observation,
        m.observation,
        records_per_world,
        payload_bytes_per_world,
        bytes_per_record,
        subscriptions_per_world,
        volumes_per_subscription,
        records_per_poll,
        bytes_per_poll,
        resnapshot_volume_summaries,
        resnapshot_region_summaries,
        resnapshot_bytes
    );
    fields!(
        Presentation,
        b.presentation,
        m.presentation,
        queued_chunks,
        resident_chunks,
        in_flight_jobs,
        vertices_per_job,
        indices_per_job,
        bytes_per_job,
        resident_bytes,
        dressing_records_per_chunk
    );
    fields!(
        Checkpoint,
        b.checkpoint,
        m.checkpoint,
        queued_requests,
        active_requests,
        staging_slots,
        mapped_bytes_in_flight,
        store_bytes_in_flight,
        bytes_per_blob,
        bytes_per_checkpoint,
        manifest_nodes,
        manifest_blobs,
        manifest_bytes
    );
    fields!(
        Rollback,
        b.rollback,
        m.rollback,
        retained_frontiers,
        retained_bytes,
        genesis_persistent_bytes,
        frontier_metadata_bytes,
        log_ticks,
        log_bytes,
        replay_sink_records_in_flight,
        replay_sink_bytes_in_flight,
        active_public_replays,
        ticks_per_public_replay,
        bytes_per_public_replay,
        result_bytes_per_public_replay,
        divergence_artifact_bytes,
        active_corrections,
        ticks_per_correction,
        bytes_per_correction,
        result_bytes_per_correction,
        recovery_replay_ticks
    );
    fields!(
        Participant,
        b.participant,
        m.participant,
        operations_in_flight,
        input_bytes_per_tick,
        effects_per_tick,
        effect_bytes_per_tick,
        events_per_tick,
        event_bytes_per_tick,
        bytes_per_event,
        state_and_snapshot_bytes_per_frontier,
        snapshot_bytes_per_checkpoint,
        artifact_records_per_tick,
        artifact_bytes_per_tick
    );
    fields!(
        Runtime,
        b.runtime,
        m.runtime,
        interest_control_queue,
        callback_completion_slots,
        callback_completion_bytes,
        render_completion_cells
    );
    if b.canonical.pending_ticks != 1 {
        return Err(invalid(BudgetGroup::Canonical, 1));
    }
    if b.checkpoint.active_requests != 1 {
        return Err(invalid(BudgetGroup::Checkpoint, 2));
    }
    if b.checkpoint.staging_slots > 3 {
        return Err(invalid(BudgetGroup::Checkpoint, 3));
    }
    if b.rollback.active_corrections != 1 {
        return Err(invalid(BudgetGroup::Rollback, 15));
    }
    if b.rollback.retained_frontiers < REQUIRED_FRONTIERS as u32 {
        return Err(invalid(BudgetGroup::Rollback, 1));
    }
    if b.runtime.render_completion_cells != 32 {
        return Err(invalid(BudgetGroup::Runtime, 4));
    }
    Ok(())
}

fn check<T: Into<u128> + Copy>(
    value: T,
    maximum: T,
    group: BudgetGroup,
    field_code: u16,
) -> Result<(), ConfigError> {
    if value.into() == 0 || value.into() > maximum.into() {
        Err(invalid(group, field_code))
    } else {
        Ok(())
    }
}

fn budget_field(group: BudgetGroup, field_code: u16) -> ResourceBudgetField {
    ResourceBudgetField::try_new(group, field_code).expect("declared budget field")
}
fn error(
    code: ConfigErrorCode,
    group: BudgetGroup,
    field_code: u16,
    message: &'static str,
) -> ConfigError {
    ConfigError {
        code,
        field: ConfigField::Budgets(budget_field(group, field_code)),
        diagnostic: BoundedUtf8::try_from_str(message).expect("short static diagnostic"),
    }
}
fn invalid(group: BudgetGroup, field_code: u16) -> ConfigError {
    error(
        ConfigErrorCode::InvalidValue,
        group,
        field_code,
        "invalid budget value",
    )
}
fn cross(group: BudgetGroup, field_code: u16, message: &'static str) -> ConfigError {
    error(
        ConfigErrorCode::CrossLimitViolation,
        group,
        field_code,
        message,
    )
}
fn overflow(group: BudgetGroup, field_code: u16) -> ConfigError {
    error(
        ConfigErrorCode::ArithmeticOverflow,
        group,
        field_code,
        "budget arithmetic overflow",
    )
}

#[cfg(test)]
mod tests {
    use super::validate_resource_budgets;
    use crate::config::ResourceBudgets;

    #[test]
    fn defaults_reserve_the_exact_twenty_frontier_bound() {
        let budgets = ResourceBudgets::default();
        assert_eq!(validate_resource_budgets(&budgets), Ok(1_988_100_096));
    }

    #[test]
    fn maximum_changed_bricks_do_not_fit_default_frontier_bytes() {
        let mut budgets = ResourceBudgets::default();
        budgets.canonical.changed_bricks_per_tick = 16_384;
        assert!(validate_resource_budgets(&budgets).is_err());
    }

    #[test]
    fn boundary_values_are_checked_before_cross_limit_accounting() {
        let maximum = ResourceBudgets::compiled_maxima();
        let mut zero = ResourceBudgets::default();
        zero.identity.worlds = 0;
        assert_eq!(
            validate_resource_budgets(&zero).unwrap_err().code,
            crate::facade::ConfigErrorCode::InvalidValue
        );

        let mut plus_one = ResourceBudgets::default();
        plus_one.content.base_request_queue = maximum.content.base_request_queue + 1;
        assert_eq!(
            validate_resource_budgets(&plus_one).unwrap_err().code,
            crate::facade::ConfigErrorCode::InvalidValue
        );

        let mut minimum = ResourceBudgets::default();
        minimum.rollback.retained_frontiers = 20;
        assert!(validate_resource_budgets(&minimum).is_ok());
    }
}
