use std::sync::Arc;

use crate::canonical::{DeviceGeneration, ReceiptId};

use super::{
    CancelResult, OperationPhase, ProgressBlocker, ReceiptFamily, ReceiptPolicy, ReceiptState,
    TerminalCache,
};

#[test]
fn cloned_receipts_share_idempotent_terminal_result_until_cache_eviction() {
    let cache = TerminalCache::<u32, &'static str>::try_new(1, 16).unwrap();
    let receipt = cache
        .admit(
            ReceiptId::from_raw(7),
            DeviceGeneration::from_raw(2),
            ReceiptPolicy::for_family(ReceiptFamily::Query),
            16,
        )
        .unwrap();
    let clone = receipt.clone();

    receipt
        .operation()
        .advance(OperationPhase::Encoded)
        .unwrap();
    receipt
        .operation()
        .advance(OperationPhase::Submitted)
        .unwrap();
    assert_eq!(receipt.cancel(), CancelResult::DeliverySuppressed);
    receipt.operation().complete_ready(41).unwrap();

    let ReceiptState::Cancelled(cancelled) = receipt.poll() else {
        panic!("suppressed delivery must be terminally cancelled");
    };
    assert_eq!(cancelled.receipt, ReceiptId::from_raw(7));
    assert!(cancelled.submitted_work_drained);
    assert!(matches!(clone.poll(), ReceiptState::Cancelled(_)));
    drop(receipt);
    assert!(matches!(clone.poll(), ReceiptState::Cancelled(_)));
}

#[test]
fn query_blockers_are_visible_only_during_waiting_for_readiness() {
    let cache = TerminalCache::<u32, &'static str>::try_new(1, 8).unwrap();
    let query = cache
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Query),
            8,
        )
        .unwrap();
    assert!(query.operation().set_query_blocker().is_err());
    query
        .operation()
        .advance(OperationPhase::WaitingForReadiness)
        .unwrap();
    query.operation().set_query_blocker().unwrap();
    let ReceiptState::Pending(progress) = query.poll() else {
        panic!("waiting query stays pending");
    };
    assert_eq!(progress.blocker, Some(ProgressBlocker::Query));
    query.operation().advance(OperationPhase::Encoded).unwrap();
    let ReceiptState::Pending(progress) = query.poll() else {
        panic!("encoded query stays pending");
    };
    assert_eq!(progress.blocker, None);
}

#[test]
fn lifecycle_phase_matrix_rejects_cross_family_phases() {
    assert!(
        ReceiptPolicy::for_family(ReceiptFamily::Genesis)
            .allows_phase(OperationPhase::ExportingReplayHeader)
    );
    assert!(
        ReceiptPolicy::for_family(ReceiptFamily::Tick).allows_phase(OperationPhase::GpuComplete)
    );
    assert!(
        ReceiptPolicy::for_family(ReceiptFamily::Interest).allows_phase(OperationPhase::Applying)
    );
    assert!(
        ReceiptPolicy::for_family(ReceiptFamily::Query)
            .allows_phase(OperationPhase::WaitingForReadiness)
    );
    assert!(
        ReceiptPolicy::for_family(ReceiptFamily::ObservationResnapshot)
            .allows_phase(OperationPhase::Pinning)
    );
    assert!(
        ReceiptPolicy::for_family(ReceiptFamily::Checkpoint)
            .allows_phase(OperationPhase::CommittingManifest)
    );
    assert!(
        ReceiptPolicy::for_family(ReceiptFamily::Correction)
            .allows_phase(OperationPhase::ExportingCorrectionBranch)
    );
    assert!(
        ReceiptPolicy::for_family(ReceiptFamily::Restore)
            .allows_phase(OperationPhase::RestoringParticipants)
    );
    assert!(
        ReceiptPolicy::for_family(ReceiptFamily::Replay)
            .allows_phase(OperationPhase::ComparingExpected)
    );
    assert!(
        ReceiptPolicy::for_family(ReceiptFamily::Recovery)
            .allows_phase(OperationPhase::CreatingGeneration)
    );
    assert!(
        ReceiptPolicy::for_family(ReceiptFamily::Shutdown)
            .allows_phase(OperationPhase::ClosingAdmission)
    );
    assert!(
        !ReceiptPolicy::for_family(ReceiptFamily::Tick)
            .allows_phase(OperationPhase::FinalCheckpoint)
    );
    assert!(
        !ReceiptPolicy::for_family(ReceiptFamily::Shutdown)
            .allows_phase(OperationPhase::Publishing)
    );
}

#[test]
fn result_capacity_is_reserved_at_admission_and_reclaimed_only_after_eviction() {
    let cache = TerminalCache::<u32, &'static str>::try_new(1, 8).unwrap();
    let receipt = cache
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Interest),
            8,
        )
        .unwrap();

    assert!(
        cache
            .admit(
                ReceiptId::from_raw(2),
                DeviceGeneration::from_raw(1),
                ReceiptPolicy::for_family(ReceiptFamily::Interest),
                1,
            )
            .is_err()
    );
    receipt.operation().complete_ready(9).unwrap();
    drop(receipt);

    assert!(
        cache
            .admit(
                ReceiptId::from_raw(2),
                DeviceGeneration::from_raw(1),
                ReceiptPolicy::for_family(ReceiptFamily::Interest),
                8,
            )
            .is_ok()
    );
}

#[test]
fn pre_submission_cancellation_is_immediate_but_tick_and_shutdown_are_not_cancellable() {
    let cache = TerminalCache::<u32, &'static str>::try_new(3, 24).unwrap();
    let query = cache
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Query),
            8,
        )
        .unwrap();
    assert_eq!(query.cancel(), CancelResult::CancelledBeforeSubmit);
    assert!(matches!(query.poll(), ReceiptState::Cancelled(_)));

    let tick = cache
        .admit(
            ReceiptId::from_raw(2),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Tick),
            8,
        )
        .unwrap();
    assert_eq!(tick.cancel(), CancelResult::NotCancellable);

    let shutdown = cache
        .admit(
            ReceiptId::from_raw(3),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Shutdown),
            8,
        )
        .unwrap();
    assert_eq!(shutdown.cancel(), CancelResult::NotCancellable);
}

#[test]
fn cancellation_cutoffs_distinguish_applied_and_encoded_work() {
    let cache = TerminalCache::<u32, &'static str>::try_new(3, 24).unwrap();
    let interest = cache
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Interest),
            8,
        )
        .unwrap();
    interest
        .operation()
        .advance(OperationPhase::Applying)
        .unwrap();
    assert_eq!(interest.cancel(), CancelResult::NotCancellable);

    let query = cache
        .admit(
            ReceiptId::from_raw(2),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Query),
            8,
        )
        .unwrap();
    query.operation().advance(OperationPhase::Encoded).unwrap();
    assert_eq!(query.cancel(), CancelResult::DeliverySuppressed);
    query.operation().complete_failed("ignored").unwrap();
    assert!(matches!(query.poll(), ReceiptState::Cancelled(_)));

    let correction = cache
        .admit(
            ReceiptId::from_raw(3),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Correction),
            8,
        )
        .unwrap();
    correction
        .operation()
        .advance(OperationPhase::Submitted)
        .unwrap();
    assert_eq!(correction.cancel(), CancelResult::AbortRequested);
    correction
        .operation()
        .advance(OperationPhase::ExportingCorrectionBranch)
        .unwrap();
    assert_eq!(correction.cancel(), CancelResult::NotCancellable);
}

#[test]
fn stale_generation_cannot_publish_and_invalid_phase_transition_is_rejected() {
    let cache = TerminalCache::<u32, &'static str>::try_new(1, 8).unwrap();
    let receipt = cache
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(4),
            ReceiptPolicy::for_family(ReceiptFamily::Query),
            8,
        )
        .unwrap();

    assert!(
        receipt
            .operation()
            .advance(OperationPhase::Publishing)
            .is_err()
    );
    assert!(
        receipt
            .operation()
            .complete_ready_for_generation(DeviceGeneration::from_raw(3), 1)
            .is_err()
    );
    receipt
        .operation()
        .complete_ready_for_generation(DeviceGeneration::from_raw(4), 1)
        .unwrap();
    let ReceiptState::Ready(value) = receipt.poll() else {
        panic!("matching generation publishes the result");
    };
    assert_eq!(*value, 1);
    assert_eq!(Arc::strong_count(&value), 2);
}
