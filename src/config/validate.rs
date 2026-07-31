//! Pre-genesis validation for the finite TECH-036 resource contract.

use crate::facade::{
    BoundedUtf8, BudgetGroup, ConfigError, ConfigErrorCode, ConfigField, ResourceBudgetField,
};

use super::{ResourceBudgets, RollbackConfig};

const READBACK_PAGE_BYTES: u128 = 16 << 20;
const BRICK_COPY_ON_WRITE_BYTES: u128 = 2_048 + 26 * 1_024;
const CHANGED_VOLUME_RECORD_BYTES: u128 = 256;
const REQUIRED_FRONTIERS: u128 = 20;
const DENSE_BRICK_BYTES: u64 = 2_048;
const UNIFORM_BRICK_BYTES: u64 = 16;
const RADIX_NODE_BYTES: u64 = 1_024;
const DIRECTORY_BUCKET_BYTES: u64 = 32;
const PRESENTATION_VERTEX_BYTES: u64 = 32;
const PRESENTATION_INDEX_BYTES: u64 = 4;
const MANIFEST_ENTRY_BYTES: u64 = 32;
const BASE_CALLBACK_COMPLETION_BYTES: u64 = 2_048;

/// Device allocation limits granted for a Moria page allocator.
///
/// This deliberately mirrors only the portable limits needed by TECH-033 and
/// TECH-036, keeping backend runtime types out of the public configuration
/// contract. Pass the values granted by the selected adapter, before any page
/// allocation.
///
/// ```
/// use moria::config::{DevicePageLimits, ResourceBudgets, RollbackConfig, validate_resource_budgets};
///
/// let limits = DevicePageLimits::portable_baseline();
/// assert!(validate_resource_budgets(
///     &ResourceBudgets::default(),
///     &RollbackConfig::default(),
///     &limits,
/// ).is_ok());
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DevicePageLimits {
    pub max_buffer_size: u64,
    pub max_storage_buffer_binding_size: u64,
    pub min_storage_buffer_offset_alignment: u32,
}

impl DevicePageLimits {
    /// Returns Moria's portable baseline page limits.
    #[must_use]
    pub const fn portable_baseline() -> Self {
        Self {
            max_buffer_size: 256 << 20,
            max_storage_buffer_binding_size: 128 << 20,
            min_storage_buffer_offset_alignment: 256,
        }
    }
}

impl Default for DevicePageLimits {
    fn default() -> Self {
        Self::portable_baseline()
    }
}

/// Validates all TECH-036 budget limits before genesis.
///
/// The returned value is the conservative allocation reservation for the
/// required 20 retained frontiers. Callers must perform this check before
/// invoking a consumer callback or allocating device resources.
///
/// # Errors
///
/// Returns a field-addressed [`ConfigError`] for a zero, fixed-value, compiled
/// maximum, cross-limit, or checked-arithmetic violation.
pub fn validate_resource_budgets(
    budgets: &ResourceBudgets,
    rollback: &RollbackConfig,
    device_limits: &DevicePageLimits,
) -> Result<u128, ConfigError> {
    validate_field_ranges(budgets)?;
    if budgets.rollback.retained_frontiers != rollback.capacity_ticks {
        return Err(error(
            ConfigErrorCode::CrossLimitViolation,
            ConfigField::Rollback,
            "rollback capacity does not match retained frontiers",
        ));
    }
    if budgets.rollback.log_ticks != rollback.log_ticks {
        return Err(error(
            ConfigErrorCode::CrossLimitViolation,
            ConfigField::Rollback,
            "rollback log does not match budget",
        ));
    }
    validate_cross_limits(budgets)?;
    validate_page_limits(budgets, device_limits)?;
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
    let cow = checked_product(
        u128::from(budgets.canonical.changed_bricks_per_tick),
        BRICK_COPY_ON_WRITE_BYTES,
        BudgetGroup::Canonical,
        7,
    )?;
    let records = checked_sum(
        u128::from(budgets.canonical.inputs_per_tick),
        u128::from(budgets.participant.effects_per_tick),
        BudgetGroup::Participant,
        3,
    )?;
    let volumes = checked_product(
        records,
        CHANGED_VOLUME_RECORD_BYTES,
        BudgetGroup::Participant,
        3,
    )?;
    let frontier = checked_sum(cow, volumes, BudgetGroup::Rollback, 4)
        .and_then(|value| {
            checked_sum(
                value,
                u128::from(budgets.rollback.frontier_metadata_bytes),
                BudgetGroup::Rollback,
                4,
            )
        })
        .and_then(|value| {
            checked_sum(
                value,
                u128::from(budgets.participant.state_and_snapshot_bytes_per_frontier),
                BudgetGroup::Rollback,
                4,
            )
        })?;
    let retained = checked_product(REQUIRED_FRONTIERS, frontier, BudgetGroup::Rollback, 2)?;
    checked_sum(
        u128::from(budgets.rollback.genesis_persistent_bytes),
        retained,
        BudgetGroup::Rollback,
        2,
    )
}

fn checked_sum(
    left: u128,
    right: u128,
    group: BudgetGroup,
    field_code: u16,
) -> Result<u128, ConfigError> {
    left.checked_add(right)
        .ok_or_else(|| overflow(group, field_code))
}

fn checked_product(
    left: u128,
    right: u128,
    group: BudgetGroup,
    field_code: u16,
) -> Result<u128, ConfigError> {
    left.checked_mul(right)
        .ok_or_else(|| overflow(group, field_code))
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
        < u128::from(BASE_CALLBACK_COMPLETION_BYTES) * u128::from(b.content.base_requests_in_flight)
    {
        return Err(cross(BudgetGroup::Content, 3, "base completion bytes"));
    }
    let largest_callback = [
        BASE_CALLBACK_COMPLETION_BYTES,
        b.participant.effect_bytes_per_tick,
        b.participant.event_bytes_per_tick,
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
    if b.observation.bytes_per_record as u64 > b.observation.payload_bytes_per_world {
        return Err(cross(BudgetGroup::Observation, 3, "record bytes"));
    }
    if b.observation.bytes_per_poll > b.observation.payload_bytes_per_world {
        return Err(cross(BudgetGroup::Observation, 7, "poll bytes"));
    }
    if b.observation.resnapshot_bytes > b.observation.payload_bytes_per_world {
        return Err(cross(BudgetGroup::Observation, 10, "resnapshot bytes"));
    }
    if b.observation.records_per_poll > b.observation.records_per_world {
        return Err(cross(BudgetGroup::Observation, 6, "poll records"));
    }
    if b.observation.volumes_per_subscription > b.identity.volumes_per_world {
        return Err(cross(BudgetGroup::Observation, 5, "world volume capacity"));
    }
    if b.observation.volumes_per_subscription > b.observation.resnapshot_volume_summaries {
        return Err(cross(
            BudgetGroup::Observation,
            5,
            "resnapshot volume capacity",
        ));
    }
    if b.presentation.resident_bytes < b.presentation.bytes_per_job {
        return Err(cross(BudgetGroup::Presentation, 7, "resident bytes"));
    }
    let presentation_output_bytes = checked_sum(
        checked_product(
            u128::from(b.presentation.vertices_per_job),
            u128::from(PRESENTATION_VERTEX_BYTES),
            BudgetGroup::Presentation,
            6,
        )?,
        checked_product(
            u128::from(b.presentation.indices_per_job),
            u128::from(PRESENTATION_INDEX_BYTES),
            BudgetGroup::Presentation,
            6,
        )?,
        BudgetGroup::Presentation,
        6,
    )?;
    if presentation_output_bytes > u128::from(b.presentation.bytes_per_job) {
        return Err(cross(BudgetGroup::Presentation, 6, "encoded job bytes"));
    }
    let submitted_presentation_bytes = checked_product(
        u128::from(b.presentation.in_flight_jobs),
        u128::from(b.presentation.bytes_per_job),
        BudgetGroup::Presentation,
        7,
    )?;
    if submitted_presentation_bytes > u128::from(b.presentation.resident_bytes) {
        return Err(cross(BudgetGroup::Presentation, 7, "in-flight job bytes"));
    }
    if b.checkpoint.mapped_bytes_in_flight < READBACK_PAGE_BYTES as u64 {
        return Err(cross(BudgetGroup::Checkpoint, 4, "mapped bytes"));
    }
    if b.checkpoint.store_bytes_in_flight < b.checkpoint.bytes_per_blob {
        return Err(cross(BudgetGroup::Checkpoint, 5, "store bytes"));
    }
    if b.checkpoint.bytes_per_blob > b.checkpoint.bytes_per_checkpoint {
        return Err(cross(BudgetGroup::Checkpoint, 6, "blob bytes"));
    }
    if b.checkpoint.manifest_bytes > b.checkpoint.bytes_per_checkpoint {
        return Err(cross(BudgetGroup::Checkpoint, 10, "manifest bytes"));
    }
    let manifest_encoded_bytes = checked_sum(
        checked_product(
            u128::from(b.checkpoint.manifest_nodes),
            u128::from(MANIFEST_ENTRY_BYTES),
            BudgetGroup::Checkpoint,
            10,
        )?,
        checked_product(
            u128::from(b.checkpoint.manifest_blobs),
            u128::from(MANIFEST_ENTRY_BYTES),
            BudgetGroup::Checkpoint,
            10,
        )?,
        BudgetGroup::Checkpoint,
        10,
    )?;
    if manifest_encoded_bytes > u128::from(b.checkpoint.manifest_bytes) {
        return Err(cross(BudgetGroup::Checkpoint, 10, "encoded manifest bytes"));
    }
    if b.rollback.retained_frontiers > b.rollback.log_ticks {
        return Err(cross(BudgetGroup::Rollback, 1, "retained frontiers"));
    }
    if b.rollback.recovery_replay_ticks > b.rollback.log_ticks {
        return Err(cross(BudgetGroup::Rollback, 18, "recovery replay ticks"));
    }
    if b.rollback.ticks_per_correction > b.rollback.log_ticks {
        return Err(cross(BudgetGroup::Rollback, 15, "correction ticks"));
    }
    if b.participant.bytes_per_event as u64 > b.participant.event_bytes_per_tick {
        return Err(cross(BudgetGroup::Participant, 7, "event bytes"));
    }
    if b.participant.snapshot_bytes_per_checkpoint > b.checkpoint.bytes_per_checkpoint {
        return Err(cross(
            BudgetGroup::Participant,
            9,
            "snapshot checkpoint bytes",
        ));
    }
    Ok(())
}

fn validate_page_limits(
    budgets: &ResourceBudgets,
    limits: &DevicePageLimits,
) -> Result<(), ConfigError> {
    let alignment = u64::from(limits.min_storage_buffer_offset_alignment);
    if alignment == 0 || !alignment.is_power_of_two() {
        return Err(cross(BudgetGroup::Content, 9, "storage offset alignment"));
    }

    let content_bytes = checked_sum(
        checked_sum(
            checked_product(
                u128::from(budgets.content.resident_dense_bricks),
                u128::from(DENSE_BRICK_BYTES),
                BudgetGroup::Content,
                9,
            )?,
            checked_product(
                u128::from(budgets.content.resident_uniform_bricks),
                u128::from(UNIFORM_BRICK_BYTES),
                BudgetGroup::Content,
                9,
            )?,
            BudgetGroup::Content,
            9,
        )?,
        checked_sum(
            checked_product(
                u128::from(budgets.content.resident_radix_nodes),
                u128::from(RADIX_NODE_BYTES),
                BudgetGroup::Content,
                9,
            )?,
            checked_product(
                u128::from(budgets.content.resident_directory_buckets),
                u128::from(DIRECTORY_BUCKET_BYTES),
                BudgetGroup::Content,
                9,
            )?,
            BudgetGroup::Content,
            9,
        )?,
        BudgetGroup::Content,
        9,
    )?;
    if content_bytes > u128::from(budgets.content.authoritative_gpu_bytes) {
        return Err(cross(BudgetGroup::Content, 9, "resident page bytes"));
    }

    let pages = [
        (
            content_bytes,
            DENSE_BRICK_BYTES,
            32 << 20,
            BudgetGroup::Content,
            9,
        ),
        (
            u128::from(budgets.canonical.scratch_bytes),
            4,
            32 << 20,
            BudgetGroup::Canonical,
            8,
        ),
        (
            u128::from(budgets.query.readback_bytes_in_flight),
            4,
            16 << 20,
            BudgetGroup::Query,
            7,
        ),
        (
            u128::from(budgets.checkpoint.mapped_bytes_in_flight),
            4,
            16 << 20,
            BudgetGroup::Checkpoint,
            4,
        ),
        (
            u128::from(budgets.checkpoint.bytes_per_checkpoint),
            4,
            16 << 20,
            BudgetGroup::Checkpoint,
            7,
        ),
        (
            u128::from(budgets.rollback.retained_bytes),
            4,
            32 << 20,
            BudgetGroup::Rollback,
            2,
        ),
        (
            u128::from(budgets.presentation.resident_bytes),
            PRESENTATION_VERTEX_BYTES,
            32 << 20,
            BudgetGroup::Presentation,
            7,
        ),
        (
            u128::from(budgets.participant.input_bytes_per_tick),
            4,
            8 << 20,
            BudgetGroup::Participant,
            2,
        ),
        (
            u128::from(budgets.participant.effect_bytes_per_tick),
            4,
            8 << 20,
            BudgetGroup::Participant,
            4,
        ),
        (
            u128::from(budgets.participant.event_bytes_per_tick),
            4,
            8 << 20,
            BudgetGroup::Participant,
            6,
        ),
        (
            u128::from(budgets.participant.state_and_snapshot_bytes_per_frontier),
            4,
            8 << 20,
            BudgetGroup::Participant,
            8,
        ),
    ];
    for (bytes, record_bytes, baseline_page_bytes, group, field) in pages {
        validate_paged_bytes(
            bytes,
            record_bytes,
            baseline_page_bytes,
            alignment,
            limits,
            group,
            field,
        )?;
    }
    Ok(())
}

fn validate_paged_bytes(
    bytes: u128,
    record_bytes: u64,
    baseline_page_bytes: u64,
    alignment: u64,
    limits: &DevicePageLimits,
    group: BudgetGroup,
    field: u16,
) -> Result<(), ConfigError> {
    let limit = baseline_page_bytes
        .min(limits.max_buffer_size)
        .min(limits.max_storage_buffer_binding_size);
    let page_bytes = limit / alignment * alignment;
    if page_bytes < record_bytes {
        return Err(cross(group, field, "page capacity"));
    }
    let page_bytes = u128::from(page_bytes);
    let page_count =
        u64::try_from(bytes / page_bytes + u128::from(!bytes.is_multiple_of(page_bytes)))
            .map_err(|_| overflow(group, field))?;
    let allocated_bytes = checked_product(u128::from(page_count), page_bytes, group, field)?;
    if allocated_bytes < bytes {
        return Err(cross(group, field, "page bytes"));
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
fn error(code: ConfigErrorCode, field: ConfigField, message: &'static str) -> ConfigError {
    ConfigError {
        code,
        field,
        diagnostic: BoundedUtf8::try_from_str(message).expect("short static diagnostic"),
    }
}
fn invalid(group: BudgetGroup, field_code: u16) -> ConfigError {
    error(
        ConfigErrorCode::InvalidValue,
        ConfigField::Budgets(budget_field(group, field_code)),
        "invalid budget value",
    )
}
fn cross(group: BudgetGroup, field_code: u16, message: &'static str) -> ConfigError {
    error(
        ConfigErrorCode::CrossLimitViolation,
        ConfigField::Budgets(budget_field(group, field_code)),
        message,
    )
}
fn overflow(group: BudgetGroup, field_code: u16) -> ConfigError {
    error(
        ConfigErrorCode::ArithmeticOverflow,
        ConfigField::Budgets(budget_field(group, field_code)),
        "budget arithmetic overflow",
    )
}

#[cfg(test)]
mod tests {
    use super::{
        BudgetGroup, checked_product, checked_sum, cross, invalid, validate_field_ranges,
        validate_resource_budgets as validate_resource_budgets_with_limits,
    };
    use crate::{
        config::{DevicePageLimits, ResourceBudgets, RollbackConfig},
        facade::{ConfigError, ConfigErrorCode, ConfigField},
    };

    fn rollback() -> RollbackConfig {
        RollbackConfig::default()
    }

    fn validate_resource_budgets(
        budgets: &ResourceBudgets,
        rollback: &RollbackConfig,
    ) -> Result<u128, ConfigError> {
        validate_resource_budgets_with_limits(
            budgets,
            rollback,
            &DevicePageLimits::portable_baseline(),
        )
    }

    macro_rules! generated_field_boundaries {
        ($group:ident, $member:ident, $minimum:expr, $first_field_code:expr, $($field:ident),+ $(,)?) => {{
            let maxima = ResourceBudgets::compiled_maxima();
            let mut field_code = $first_field_code - 1;
            $(
                field_code += 1;

                let mut zero = ResourceBudgets::default();
                zero.$member.$field = 0;
                assert_eq!(
                    validate_field_ranges(&zero),
                    Err(invalid(BudgetGroup::$group, field_code)),
                    concat!(stringify!($group), ".", stringify!($field), " zero"),
                );

                let mut minimum = ResourceBudgets::default();
                minimum.$member.$field = $minimum;
                assert_eq!(
                    validate_field_ranges(&minimum),
                    Ok(()),
                    concat!(stringify!($group), ".", stringify!($field), " minimum"),
                );

                let defaults = ResourceBudgets::default();
                assert_eq!(
                    validate_field_ranges(&defaults),
                    Ok(()),
                    concat!(stringify!($group), ".", stringify!($field), " default"),
                );

                let mut maximum = ResourceBudgets::default();
                maximum.$member.$field = maxima.$member.$field;
                assert_eq!(
                    validate_field_ranges(&maximum),
                    Ok(()),
                    concat!(stringify!($group), ".", stringify!($field), " maximum"),
                );

                let mut maximum_plus_one = ResourceBudgets::default();
                maximum_plus_one.$member.$field = maxima.$member.$field + 1;
                assert_eq!(
                    validate_field_ranges(&maximum_plus_one),
                    Err(invalid(BudgetGroup::$group, field_code)),
                    concat!(stringify!($group), ".", stringify!($field), " maximum plus one"),
                );
            )+
        }};
    }

    #[test]
    fn generated_resource_budget_field_boundaries_are_exact() {
        generated_field_boundaries!(
            Identity,
            identity,
            1,
            1,
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
            artifact_leases_per_world,
        );
        generated_field_boundaries!(
            Canonical,
            canonical,
            1,
            1,
            pending_ticks,
            inputs_per_tick,
            encoded_bytes_per_tick,
            correlation_bytes_per_tick,
            bricks_per_command,
            cells_per_command,
            changed_bricks_per_tick,
            scratch_bytes,
        );
        generated_field_boundaries!(
            Content,
            content,
            1,
            1,
            base_request_queue,
            base_requests_in_flight,
            base_completion_bytes_in_flight,
            materialization_bricks_per_job,
            resident_dense_bricks,
            resident_uniform_bricks,
            resident_radix_nodes,
            resident_directory_buckets,
            authoritative_gpu_bytes,
        );
        generated_field_boundaries!(
            Query,
            query,
            1,
            1,
            queued_requests,
            in_flight_requests,
            bricks_per_request,
            records_per_result,
            bytes_per_result,
            volume_revisions_per_request,
            readback_bytes_in_flight,
        );
        generated_field_boundaries!(
            Observation,
            observation,
            1,
            1,
            records_per_world,
            payload_bytes_per_world,
            bytes_per_record,
            subscriptions_per_world,
            volumes_per_subscription,
            records_per_poll,
            bytes_per_poll,
            resnapshot_volume_summaries,
            resnapshot_region_summaries,
            resnapshot_bytes,
        );
        generated_field_boundaries!(
            Presentation,
            presentation,
            1,
            1,
            queued_chunks,
            resident_chunks,
            in_flight_jobs,
            vertices_per_job,
            indices_per_job,
            bytes_per_job,
            resident_bytes,
            dressing_records_per_chunk,
        );
        generated_field_boundaries!(
            Checkpoint,
            checkpoint,
            1,
            1,
            queued_requests,
            active_requests,
            staging_slots,
            mapped_bytes_in_flight,
            store_bytes_in_flight,
            bytes_per_blob,
            bytes_per_checkpoint,
            manifest_nodes,
            manifest_blobs,
            manifest_bytes,
        );
        generated_field_boundaries!(
            Rollback,
            rollback,
            1,
            2,
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
            recovery_replay_ticks,
        );
        generated_field_boundaries!(
            Participant,
            participant,
            1,
            1,
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
            artifact_bytes_per_tick,
        );
        generated_field_boundaries!(
            Runtime,
            runtime,
            1,
            1,
            interest_control_queue,
            callback_completion_slots,
            callback_completion_bytes,
        );

        let maxima = ResourceBudgets::compiled_maxima();
        let mut retained_frontiers = ResourceBudgets::default();
        retained_frontiers.rollback.retained_frontiers = 0;
        assert_eq!(
            validate_field_ranges(&retained_frontiers),
            Err(invalid(BudgetGroup::Rollback, 1))
        );
        retained_frontiers.rollback.retained_frontiers = 19;
        assert_eq!(
            validate_field_ranges(&retained_frontiers),
            Err(invalid(BudgetGroup::Rollback, 1))
        );
        retained_frontiers.rollback.retained_frontiers = 20;
        assert_eq!(validate_field_ranges(&retained_frontiers), Ok(()));
        retained_frontiers.rollback.retained_frontiers = 32;
        assert_eq!(validate_field_ranges(&retained_frontiers), Ok(()));
        retained_frontiers.rollback.retained_frontiers = maxima.rollback.retained_frontiers;
        assert_eq!(validate_field_ranges(&retained_frontiers), Ok(()));
        retained_frontiers.rollback.retained_frontiers += 1;
        assert_eq!(
            validate_field_ranges(&retained_frontiers),
            Err(invalid(BudgetGroup::Rollback, 1))
        );

        let mut render_cells = ResourceBudgets::default();
        render_cells.runtime.render_completion_cells = 0;
        assert_eq!(
            validate_field_ranges(&render_cells),
            Err(invalid(BudgetGroup::Runtime, 4))
        );
        for value in [1, 31, 33] {
            render_cells.runtime.render_completion_cells = value;
            assert_eq!(
                validate_field_ranges(&render_cells),
                Err(invalid(BudgetGroup::Runtime, 4))
            );
        }
        render_cells.runtime.render_completion_cells = 32;
        assert_eq!(validate_field_ranges(&render_cells), Ok(()));
    }

    #[test]
    fn defaults_reserve_the_exact_twenty_frontier_bound() {
        let budgets = ResourceBudgets::default();
        assert_eq!(
            validate_resource_budgets(&budgets, &rollback()),
            Ok(1_988_100_096)
        );
    }

    #[test]
    fn maximum_changed_bricks_do_not_fit_default_frontier_bytes() {
        let mut budgets = ResourceBudgets::default();
        budgets.canonical.changed_bricks_per_tick = 16_384;
        assert_eq!(
            validate_resource_budgets(&budgets, &rollback()),
            Err(cross(BudgetGroup::Rollback, 2, "retained bytes"))
        );
    }

    #[test]
    fn encoded_output_counts_must_fit_their_declared_byte_budgets() {
        let mut presentation = ResourceBudgets::default();
        presentation.presentation.bytes_per_job = 1;
        presentation.presentation.resident_bytes = 1;
        assert_eq!(
            validate_resource_budgets(&presentation, &rollback()),
            Err(cross(BudgetGroup::Presentation, 6, "encoded job bytes")),
        );

        let mut manifest = ResourceBudgets::default();
        manifest.checkpoint.manifest_bytes = 1;
        assert_eq!(
            validate_resource_budgets(&manifest, &rollback()),
            Err(cross(BudgetGroup::Checkpoint, 10, "encoded manifest bytes",)),
        );

        let mut resident = ResourceBudgets::default();
        resident.presentation.resident_bytes = resident.presentation.bytes_per_job;
        assert_eq!(
            validate_resource_budgets(&resident, &rollback()),
            Err(cross(BudgetGroup::Presentation, 7, "in-flight job bytes")),
        );
    }

    #[test]
    fn page_limits_reject_unencodable_pages_and_bad_alignment() {
        let budgets = ResourceBudgets::default();
        let rollback = rollback();
        let baseline = DevicePageLimits::portable_baseline();
        assert_eq!(
            validate_resource_budgets_with_limits(&budgets, &rollback, &baseline),
            Ok(1_988_100_096),
        );

        let too_small_buffer = DevicePageLimits {
            max_buffer_size: 2_047,
            ..baseline
        };
        assert_eq!(
            validate_resource_budgets_with_limits(&budgets, &rollback, &too_small_buffer),
            Err(cross(BudgetGroup::Content, 9, "page capacity")),
        );

        let too_small_binding = DevicePageLimits {
            max_storage_buffer_binding_size: 2_047,
            ..baseline
        };
        assert_eq!(
            validate_resource_budgets_with_limits(&budgets, &rollback, &too_small_binding),
            Err(cross(BudgetGroup::Content, 9, "page capacity")),
        );

        let unaligned = DevicePageLimits {
            min_storage_buffer_offset_alignment: 3,
            ..baseline
        };
        assert_eq!(
            validate_resource_budgets_with_limits(&budgets, &rollback, &unaligned),
            Err(cross(BudgetGroup::Content, 9, "storage offset alignment")),
        );
    }

    #[test]
    fn checked_frontier_arithmetic_reports_the_exact_overflow_field() {
        assert_eq!(
            checked_product(u128::MAX, 2, BudgetGroup::Canonical, 7),
            Err(super::overflow(BudgetGroup::Canonical, 7))
        );
        assert_eq!(
            checked_sum(u128::MAX, 1, BudgetGroup::Rollback, 2),
            Err(super::overflow(BudgetGroup::Rollback, 2))
        );
    }

    #[test]
    fn fixed_values_and_rollback_retention_are_exact() {
        let mut pending_ticks = ResourceBudgets::default();
        pending_ticks.canonical.pending_ticks = 2;
        assert_eq!(
            validate_resource_budgets(&pending_ticks, &rollback()),
            Err(invalid(BudgetGroup::Canonical, 1))
        );

        let mut retained_frontiers = ResourceBudgets::default();
        retained_frontiers.rollback.retained_frontiers = 19;
        assert_eq!(
            validate_resource_budgets(&retained_frontiers, &rollback()),
            Err(invalid(BudgetGroup::Rollback, 1))
        );

        let mut rollback_config = rollback();
        rollback_config.capacity_ticks = 20;
        let mut budgets = ResourceBudgets::default();
        budgets.rollback.retained_frontiers = 20;
        assert_eq!(
            validate_resource_budgets(&budgets, &rollback_config),
            Ok(1_988_100_096)
        );

        assert_eq!(
            validate_resource_budgets(&ResourceBudgets::default(), &rollback_config),
            Err(ConfigError {
                code: ConfigErrorCode::CrossLimitViolation,
                field: ConfigField::Rollback,
                diagnostic: crate::facade::BoundedUtf8::try_from_str(
                    "rollback capacity does not match retained frontiers",
                )
                .unwrap(),
            })
        );

        rollback_config = rollback();
        rollback_config.log_ticks = 1;
        assert_eq!(
            validate_resource_budgets(&ResourceBudgets::default(), &rollback_config),
            Err(ConfigError {
                code: ConfigErrorCode::CrossLimitViolation,
                field: ConfigField::Rollback,
                diagnostic: crate::facade::BoundedUtf8::try_from_str(
                    "rollback log does not match budget",
                )
                .unwrap(),
            })
        );
    }

    #[test]
    fn generated_cross_limit_failures_identify_the_constrained_field() {
        macro_rules! assert_cross {
            ($budgets:ident, $group:ident, $field:expr, $message:literal) => {
                assert_eq!(
                    validate_resource_budgets(&$budgets, &rollback()),
                    Err(cross(BudgetGroup::$group, $field, $message)),
                );
            };
        }

        let mut budgets = ResourceBudgets::default();
        budgets.identity.operation_records_per_world = 1;
        assert_cross!(budgets, Identity, 15, "operation records");

        let mut budgets = ResourceBudgets::default();
        budgets.content.base_completion_bytes_in_flight = 1;
        assert_cross!(budgets, Content, 3, "base completion bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.runtime.callback_completion_bytes = 1;
        assert_cross!(budgets, Runtime, 3, "callback completion bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.participant.state_and_snapshot_bytes_per_frontier = 1;
        budgets.participant.snapshot_bytes_per_checkpoint = 1;
        budgets.checkpoint.bytes_per_blob = 1;
        budgets.checkpoint.bytes_per_checkpoint = 1;
        budgets.checkpoint.manifest_bytes = 1;
        budgets.runtime.callback_completion_bytes = budgets.participant.effect_bytes_per_tick - 1;
        assert_cross!(budgets, Runtime, 3, "callback completion bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.participant.effect_bytes_per_tick = 1;
        budgets.participant.state_and_snapshot_bytes_per_frontier = 1;
        budgets.participant.snapshot_bytes_per_checkpoint = 1;
        budgets.checkpoint.bytes_per_blob = 1;
        budgets.checkpoint.bytes_per_checkpoint = 1;
        budgets.checkpoint.manifest_bytes = 1;
        budgets.runtime.callback_completion_bytes = budgets.participant.event_bytes_per_tick - 1;
        assert_cross!(budgets, Runtime, 3, "callback completion bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.query.readback_bytes_in_flight = 1;
        assert_cross!(budgets, Query, 7, "readback bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.identity.terminal_receipt_bytes_per_world = 1;
        assert_cross!(budgets, Identity, 17, "terminal receipt bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.observation.payload_bytes_per_world = 1;
        assert_cross!(budgets, Observation, 3, "record bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.observation.bytes_per_record = 1;
        budgets.observation.payload_bytes_per_world = 1;
        assert_cross!(budgets, Observation, 7, "poll bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.observation.bytes_per_record = 1;
        budgets.observation.bytes_per_poll = 1;
        budgets.observation.payload_bytes_per_world = 1;
        assert_cross!(budgets, Observation, 10, "resnapshot bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.observation.records_per_world = 1;
        assert_cross!(budgets, Observation, 6, "poll records");

        let mut budgets = ResourceBudgets::default();
        budgets.identity.volumes_per_world = 1;
        budgets.observation.resnapshot_volume_summaries = 1;
        assert_cross!(budgets, Observation, 5, "world volume capacity");

        let mut budgets = ResourceBudgets::default();
        budgets.observation.resnapshot_volume_summaries = 1;
        assert_cross!(budgets, Observation, 5, "resnapshot volume capacity");

        let mut budgets = ResourceBudgets::default();
        budgets.presentation.resident_bytes = 1;
        assert_cross!(budgets, Presentation, 7, "resident bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.checkpoint.mapped_bytes_in_flight = 1;
        assert_cross!(budgets, Checkpoint, 4, "mapped bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.checkpoint.store_bytes_in_flight = 1;
        assert_cross!(budgets, Checkpoint, 5, "store bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.checkpoint.bytes_per_checkpoint = 1;
        budgets.checkpoint.manifest_bytes = 1;
        assert_cross!(budgets, Checkpoint, 6, "blob bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.checkpoint.bytes_per_checkpoint = 1;
        budgets.checkpoint.bytes_per_blob = 1;
        assert_cross!(budgets, Checkpoint, 10, "manifest bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.participant.event_bytes_per_tick = 1;
        assert_cross!(budgets, Participant, 7, "event bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.checkpoint.bytes_per_checkpoint = 64;
        budgets.checkpoint.bytes_per_blob = 1;
        budgets.checkpoint.manifest_nodes = 1;
        budgets.checkpoint.manifest_blobs = 1;
        budgets.checkpoint.manifest_bytes = 64;
        assert_cross!(budgets, Participant, 9, "snapshot checkpoint bytes");

        let mut budgets = ResourceBudgets::default();
        budgets.rollback.log_ticks = 1;
        let mut rollback_config = rollback();
        rollback_config.log_ticks = 1;
        assert_eq!(
            validate_resource_budgets(&budgets, &rollback_config),
            Err(cross(BudgetGroup::Rollback, 1, "retained frontiers")),
        );

        let mut budgets = ResourceBudgets::default();
        budgets.rollback.recovery_replay_ticks = 257;
        assert_cross!(budgets, Rollback, 18, "recovery replay ticks");

        let mut budgets = ResourceBudgets::default();
        budgets.rollback.ticks_per_correction = 257;
        assert_cross!(budgets, Rollback, 15, "correction ticks");

        let mut budgets = ResourceBudgets::compiled_maxima();
        budgets.rollback.retained_bytes = u64::MAX;
        budgets.content.authoritative_gpu_bytes = u64::MAX;
        budgets.checkpoint.manifest_nodes = 4_194_304;
        budgets.checkpoint.manifest_blobs = 4_194_304;
        assert_eq!(super::validate_cross_limits(&budgets), Ok(()));
    }

    #[test]
    fn base_callback_pool_is_bounded_per_callback_not_by_the_aggregate_pool() {
        let mut budgets = ResourceBudgets::default();
        budgets.content.base_requests_in_flight = 128;
        budgets.content.base_completion_bytes_in_flight = 128 * 2_048;
        budgets.participant.effect_bytes_per_tick = 2_048;
        budgets.participant.event_bytes_per_tick = 2_048;
        budgets.participant.bytes_per_event = 2_048;
        budgets.participant.state_and_snapshot_bytes_per_frontier = 2_048;
        budgets.participant.snapshot_bytes_per_checkpoint = 2_048;
        budgets.checkpoint.bytes_per_blob = 2_048;
        budgets.runtime.callback_completion_bytes = 2_048;

        assert!(validate_resource_budgets(&budgets, &rollback()).is_ok());
    }
}
