use std::{
    sync::{Arc, Barrier},
    thread,
};

use crate::canonical::{DeviceGeneration, ReceiptId};

use super::{
    CancelResult, OperationPhase, ProgressBlocker, QueryReadinessReason, ReceiptFamily,
    ReceiptPolicy, ReceiptState, TerminalCache, TransitionError,
};
use crate::facade::{BoundedVec, BudgetGroup, ResourceBudgetField};

const PHASES: &[OperationPhase] = &[
    OperationPhase::Verifying,
    OperationPhase::Queued,
    OperationPhase::Applying,
    OperationPhase::WaitingForReadiness,
    OperationPhase::Pinning,
    OperationPhase::Loading,
    OperationPhase::LoadingOwnedRecords,
    OperationPhase::VerifyingHeader,
    OperationPhase::ExportingReplayHeader,
    OperationPhase::ExportingReplayPrefix,
    OperationPhase::ExportingCorrectionBranch,
    OperationPhase::Reading,
    OperationPhase::Materializing,
    OperationPhase::Preparing,
    OperationPhase::Encoding,
    OperationPhase::Encoded,
    OperationPhase::Submitting,
    OperationPhase::Submitted,
    OperationPhase::GpuComplete,
    OperationPhase::Mapping,
    OperationPhase::Decoding,
    OperationPhase::StoringBlobs,
    OperationPhase::CommittingManifest,
    OperationPhase::RestoringPrivate,
    OperationPhase::ReplayingPrivate,
    OperationPhase::ComparingExpected,
    OperationPhase::Comparing,
    OperationPhase::Querying,
    OperationPhase::Rebuilding,
    OperationPhase::RestoringParticipants,
    OperationPhase::ValidatingFinal,
    OperationPhase::Publishing,
    OperationPhase::CreatingGeneration,
    OperationPhase::LoadingAnchor,
    OperationPhase::Replaying,
    OperationPhase::ClosingAdmission,
    OperationPhase::Draining,
    OperationPhase::FinalCheckpoint,
    OperationPhase::Releasing,
];

const LIFECYCLES: &[(ReceiptFamily, &[OperationPhase])] = &[
    (
        ReceiptFamily::Genesis,
        &[
            OperationPhase::Verifying,
            OperationPhase::Materializing,
            OperationPhase::Submitting,
            OperationPhase::ExportingReplayHeader,
        ],
    ),
    (
        ReceiptFamily::Tick,
        &[
            OperationPhase::Queued,
            OperationPhase::Preparing,
            OperationPhase::Encoded,
            OperationPhase::Submitted,
            OperationPhase::GpuComplete,
            OperationPhase::Decoding,
        ],
    ),
    (
        ReceiptFamily::Interest,
        &[OperationPhase::Queued, OperationPhase::Applying],
    ),
    (
        ReceiptFamily::Query,
        &[
            OperationPhase::Queued,
            OperationPhase::WaitingForReadiness,
            OperationPhase::Encoded,
            OperationPhase::Submitted,
            OperationPhase::Mapping,
            OperationPhase::Decoding,
        ],
    ),
    (
        ReceiptFamily::ObservationResnapshot,
        &[
            OperationPhase::Queued,
            OperationPhase::Pinning,
            OperationPhase::Querying,
            OperationPhase::Encoding,
        ],
    ),
    (
        ReceiptFamily::Checkpoint,
        &[
            OperationPhase::Queued,
            OperationPhase::Pinning,
            OperationPhase::Reading,
            OperationPhase::StoringBlobs,
            OperationPhase::CommittingManifest,
        ],
    ),
    (
        ReceiptFamily::Correction,
        &[
            OperationPhase::Queued,
            OperationPhase::RestoringPrivate,
            OperationPhase::ReplayingPrivate,
            OperationPhase::ValidatingFinal,
            OperationPhase::ExportingCorrectionBranch,
            OperationPhase::Publishing,
        ],
    ),
    (
        ReceiptFamily::Restore,
        &[
            OperationPhase::Loading,
            OperationPhase::Verifying,
            OperationPhase::Rebuilding,
            OperationPhase::RestoringParticipants,
            OperationPhase::ExportingReplayHeader,
            OperationPhase::Publishing,
        ],
    ),
    (
        ReceiptFamily::Replay,
        &[
            OperationPhase::LoadingOwnedRecords,
            OperationPhase::VerifyingHeader,
            OperationPhase::ReplayingPrivate,
            OperationPhase::ComparingExpected,
            OperationPhase::ExportingReplayHeader,
            OperationPhase::ExportingReplayPrefix,
            OperationPhase::Publishing,
        ],
    ),
    (
        ReceiptFamily::Recovery,
        &[
            OperationPhase::Queued,
            OperationPhase::CreatingGeneration,
            OperationPhase::LoadingAnchor,
            OperationPhase::Replaying,
            OperationPhase::Comparing,
        ],
    ),
    (
        ReceiptFamily::Shutdown,
        &[
            OperationPhase::ClosingAdmission,
            OperationPhase::Draining,
            OperationPhase::FinalCheckpoint,
            OperationPhase::Releasing,
        ],
    ),
];

fn receipt(family: ReceiptFamily) -> super::Receipt<u32, &'static str> {
    TerminalCache::try_new(1, 8)
        .unwrap()
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(family),
            8,
        )
        .unwrap()
}

fn advance_to(
    receipt: &super::Receipt<u32, &'static str>,
    phases: &[OperationPhase],
    index: usize,
) {
    for phase in &phases[1..=index] {
        receipt.operation().advance(*phase).unwrap();
    }
}

#[test]
fn lifecycle_matrices_admit_exactly_the_documented_phases() {
    for (family, lifecycle) in LIFECYCLES {
        for phase in PHASES {
            assert_eq!(
                ReceiptPolicy::for_family(*family).allows_phase(*phase),
                lifecycle.contains(phase),
                "{family:?} phase {phase:?}",
            );
        }
    }
}

#[test]
fn lifecycle_matrices_allow_every_edge_and_reject_every_non_edge() {
    for (family, lifecycle) in LIFECYCLES {
        for (index, from) in lifecycle.iter().enumerate() {
            for to in PHASES {
                let receipt = receipt(*family);
                advance_to(&receipt, lifecycle, index);
                let result = receipt.operation().advance(*to);
                if lifecycle.get(index + 1) == Some(to) {
                    assert!(result.is_ok(), "{family:?}: {from:?} -> {to:?}");
                } else if lifecycle.contains(to) {
                    assert_eq!(
                        result,
                        Err(TransitionError::InvalidTransition {
                            family: *family,
                            from: *from,
                            to: *to,
                        }),
                        "{family:?}: {from:?} -> {to:?}",
                    );
                } else {
                    assert_eq!(
                        result,
                        Err(TransitionError::InvalidPhase {
                            family: *family,
                            phase: *to,
                        }),
                        "{family:?}: {from:?} -> {to:?}",
                    );
                }
            }
        }
    }
}

#[test]
fn query_blockers_carry_each_typed_reason_only_during_waiting_for_readiness() {
    let query = receipt(ReceiptFamily::Query);
    let availability = || QueryReadinessReason::Availability {
        missing: BoundedVec::try_with_capacity(0).unwrap(),
    };
    assert!(query.operation().set_query_blocker(availability()).is_err());
    query
        .operation()
        .advance(OperationPhase::WaitingForReadiness)
        .unwrap();
    query.operation().set_query_blocker(availability()).unwrap();
    let ReceiptState::Pending(progress) = query.poll() else {
        panic!("waiting query stays pending")
    };
    assert!(matches!(
        progress.blocker,
        Some(ProgressBlocker::Query(reason))
            if matches!(reason.as_ref(), QueryReadinessReason::Availability { missing }
                if missing.is_empty())
    ));
    query
        .operation()
        .set_query_blocker(QueryReadinessReason::MinimumRevision {
            unmet: BoundedVec::try_with_capacity(0).unwrap(),
        })
        .unwrap();
    let ReceiptState::Pending(progress) = query.poll() else {
        panic!("waiting query stays pending")
    };
    assert!(matches!(
        progress.blocker,
        Some(ProgressBlocker::Query(reason))
            if matches!(reason.as_ref(), QueryReadinessReason::MinimumRevision { unmet }
                if unmet.is_empty())
    ));
    query
        .operation()
        .set_query_blocker(QueryReadinessReason::ResourcePressure {
            field: ResourceBudgetField::try_new(BudgetGroup::Query, 1).unwrap(),
            required: 9,
            supported: 8,
        })
        .unwrap();
    let ReceiptState::Pending(progress) = query.poll() else {
        panic!("waiting query stays pending")
    };
    assert!(matches!(
        progress.blocker,
        Some(ProgressBlocker::Query(reason))
            if matches!(reason.as_ref(), QueryReadinessReason::ResourcePressure {
                required: 9,
                supported: 8,
                ..
            })
    ));
    query.operation().advance(OperationPhase::Encoded).unwrap();
    let ReceiptState::Pending(progress) = query.poll() else {
        panic!("encoded query stays pending")
    };
    assert!(progress.blocker.is_none());
}

#[test]
fn abort_requests_allow_drain_but_block_every_later_visible_effect() {
    let checkpoint = receipt(ReceiptFamily::Checkpoint);
    advance_to(&checkpoint, LIFECYCLES[5].1, 2);
    assert_eq!(checkpoint.cancel(), CancelResult::AbortRequested);
    assert!(checkpoint.operation().abort_requested());
    checkpoint
        .operation()
        .advance(OperationPhase::StoringBlobs)
        .unwrap();
    assert_eq!(
        checkpoint
            .operation()
            .advance(OperationPhase::CommittingManifest),
        Err(TransitionError::AbortBoundary {
            family: ReceiptFamily::Checkpoint,
            phase: OperationPhase::CommittingManifest,
        })
    );

    let correction = receipt(ReceiptFamily::Correction);
    advance_to(&correction, LIFECYCLES[6].1, 3);
    assert_eq!(correction.cancel(), CancelResult::AbortRequested);
    assert_eq!(
        correction
            .operation()
            .advance(OperationPhase::ExportingCorrectionBranch),
        Err(TransitionError::AbortBoundary {
            family: ReceiptFamily::Correction,
            phase: OperationPhase::ExportingCorrectionBranch,
        })
    );

    let restore = receipt(ReceiptFamily::Restore);
    advance_to(&restore, LIFECYCLES[7].1, 2);
    assert_eq!(restore.cancel(), CancelResult::AbortRequested);
    restore
        .operation()
        .advance(OperationPhase::RestoringParticipants)
        .unwrap();
    restore
        .operation()
        .advance(OperationPhase::ExportingReplayHeader)
        .unwrap();
    assert!(matches!(
        restore.operation().advance(OperationPhase::Publishing),
        Err(TransitionError::AbortBoundary { .. })
    ));

    let replay = receipt(ReceiptFamily::Replay);
    advance_to(&replay, LIFECYCLES[8].1, 2);
    assert_eq!(replay.cancel(), CancelResult::AbortRequested);
    replay
        .operation()
        .advance(OperationPhase::ComparingExpected)
        .unwrap();
    assert!(matches!(
        replay
            .operation()
            .advance(OperationPhase::ExportingReplayHeader),
        Err(TransitionError::AbortBoundary { .. })
    ));

    let recovery = receipt(ReceiptFamily::Recovery);
    advance_to(&recovery, LIFECYCLES[9].1, 3);
    assert_eq!(recovery.cancel(), CancelResult::AbortRequested);
    recovery
        .operation()
        .advance(OperationPhase::Comparing)
        .unwrap();
    recovery.operation().complete_ready(1).unwrap();
    assert!(matches!(
        recovery.poll(),
        ReceiptState::Cancelled(cancelled) if cancelled.submitted_work_drained
    ));
}

#[test]
fn abort_boundary_matrix_covers_every_prohibited_effect_phase() {
    use super::operation::abort_prohibits_phase;

    for (family, phase) in [
        (
            ReceiptFamily::Checkpoint,
            OperationPhase::CommittingManifest,
        ),
        (
            ReceiptFamily::Correction,
            OperationPhase::ExportingCorrectionBranch,
        ),
        (ReceiptFamily::Correction, OperationPhase::Publishing),
        (ReceiptFamily::Restore, OperationPhase::Publishing),
        (ReceiptFamily::Replay, OperationPhase::ExportingReplayHeader),
        (ReceiptFamily::Replay, OperationPhase::ExportingReplayPrefix),
        (ReceiptFamily::Replay, OperationPhase::Publishing),
    ] {
        assert!(abort_prohibits_phase(family, phase), "{family:?} {phase:?}");
    }
    assert!(!abort_prohibits_phase(
        ReceiptFamily::Checkpoint,
        OperationPhase::StoringBlobs
    ));
    assert!(!abort_prohibits_phase(
        ReceiptFamily::Recovery,
        OperationPhase::Comparing
    ));
}

#[test]
fn checkpoint_and_correction_cancellation_cutoffs_latch_irreversibly() {
    for phase in [
        OperationPhase::Reading,
        OperationPhase::StoringBlobs,
        OperationPhase::CommittingManifest,
    ] {
        let checkpoint = receipt(ReceiptFamily::Checkpoint);
        let lifecycle = LIFECYCLES[5].1;
        advance_to(
            &checkpoint,
            lifecycle,
            lifecycle
                .iter()
                .position(|candidate| *candidate == phase)
                .unwrap(),
        );
        assert_eq!(
            checkpoint.cancel(),
            CancelResult::AbortRequested,
            "checkpoint at {phase:?}"
        );
    }

    let correction_lifecycle = LIFECYCLES[6].1;
    for phase in [
        OperationPhase::RestoringPrivate,
        OperationPhase::ReplayingPrivate,
        OperationPhase::ValidatingFinal,
    ] {
        let correction = receipt(ReceiptFamily::Correction);
        advance_to(
            &correction,
            correction_lifecycle,
            correction_lifecycle
                .iter()
                .position(|candidate| *candidate == phase)
                .unwrap(),
        );
        assert_eq!(
            correction.cancel(),
            CancelResult::AbortRequested,
            "correction at {phase:?}"
        );
    }
    for phase in [
        OperationPhase::ExportingCorrectionBranch,
        OperationPhase::Publishing,
    ] {
        let correction = receipt(ReceiptFamily::Correction);
        advance_to(
            &correction,
            correction_lifecycle,
            correction_lifecycle
                .iter()
                .position(|candidate| *candidate == phase)
                .unwrap(),
        );
        assert_eq!(
            correction.cancel(),
            CancelResult::NotCancellable,
            "correction at {phase:?}"
        );
    }
}

#[test]
fn cancellation_and_completion_race_to_one_retained_terminal_state() {
    let cache = TerminalCache::<u32, &'static str>::try_new(1, 8).unwrap();
    let receipt = cache
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Query),
            8,
        )
        .unwrap();
    receipt
        .operation()
        .advance(OperationPhase::WaitingForReadiness)
        .unwrap();
    receipt
        .operation()
        .advance(OperationPhase::Encoded)
        .unwrap();
    receipt
        .operation()
        .advance(OperationPhase::Submitted)
        .unwrap();
    receipt
        .operation()
        .advance(OperationPhase::Mapping)
        .unwrap();
    receipt
        .operation()
        .advance(OperationPhase::Decoding)
        .unwrap();
    let operation = receipt.operation();
    let canceller = receipt.clone();
    let complete = thread::spawn(move || operation.complete_ready(9));
    let cancelled = thread::spawn(move || canceller.cancel());
    let completion = complete.join().unwrap();
    let cancellation = cancelled.join().unwrap();
    assert!(matches!(
        completion,
        Ok(()) | Err(TransitionError::AlreadyTerminal)
    ));
    assert!(matches!(
        cancellation,
        CancelResult::DeliverySuppressed | CancelResult::AlreadyTerminal
    ));
    assert!(matches!(
        receipt.poll(),
        ReceiptState::Ready(_) | ReceiptState::Cancelled(_)
    ));
}

#[test]
fn interest_cancellation_and_application_are_atomic() {
    for receipt_number in 1..=64 {
        let cache = TerminalCache::<u32, &'static str>::try_new(1, 8).unwrap();
        let receipt = cache
            .admit(
                ReceiptId::from_raw(receipt_number),
                DeviceGeneration::from_raw(1),
                ReceiptPolicy::for_family(ReceiptFamily::Interest),
                8,
            )
            .unwrap();
        let start = Arc::new(Barrier::new(3));
        let advance_operation = receipt.operation();
        let advance_start = Arc::clone(&start);
        let advance = thread::spawn(move || {
            advance_start.wait();
            advance_operation.advance(OperationPhase::Applying)
        });
        let cancel_receipt = receipt.clone();
        let cancel_start = Arc::clone(&start);
        let cancel = thread::spawn(move || {
            cancel_start.wait();
            cancel_receipt.cancel()
        });
        start.wait();

        let advanced = advance.join().unwrap();
        let cancelled = cancel.join().unwrap();
        match (advanced, cancelled, receipt.poll()) {
            (Ok(()), CancelResult::NotCancellable, ReceiptState::Pending(progress)) => {
                assert_eq!(progress.phase, OperationPhase::Applying);
            }
            (
                Err(TransitionError::AlreadyTerminal),
                CancelResult::CancelledBeforeSubmit,
                ReceiptState::Cancelled(cancelled),
            ) => assert_eq!(cancelled.last_phase, OperationPhase::Queued),
            outcome => panic!("interest cancel/apply race produced {outcome:?}"),
        }
    }
}

#[test]
fn successful_completion_requires_each_family_final_phase() {
    for (family, lifecycle) in LIFECYCLES {
        let receipt = receipt(*family);
        let initial = receipt.operation().complete_ready(1);
        assert_eq!(
            initial,
            Err(TransitionError::InvalidTransition {
                family: *family,
                from: lifecycle[0],
                to: *lifecycle.last().unwrap(),
            })
        );
        advance_to(&receipt, lifecycle, lifecycle.len() - 1);
        receipt.operation().complete_ready(1).unwrap();
        assert!(matches!(receipt.poll(), ReceiptState::Ready(value) if *value == 1));
    }
}

#[test]
fn submitted_cancellation_only_reports_drain_after_its_milestone() {
    let checkpoint = receipt(ReceiptFamily::Checkpoint);
    advance_to(&checkpoint, LIFECYCLES[5].1, 2);
    assert_eq!(checkpoint.cancel(), CancelResult::AbortRequested);
    assert_eq!(
        checkpoint.operation().complete_ready(1),
        Err(TransitionError::DrainIncomplete {
            family: ReceiptFamily::Checkpoint,
            phase: OperationPhase::Reading,
        })
    );
    checkpoint
        .operation()
        .advance(OperationPhase::StoringBlobs)
        .unwrap();
    checkpoint.operation().complete_ready(1).unwrap();
    assert!(matches!(
        checkpoint.poll(),
        ReceiptState::Cancelled(cancelled)
            if cancelled.last_phase == OperationPhase::StoringBlobs
                && cancelled.submitted_work_drained
    ));
}

#[test]
fn generation_change_terminalizes_old_receipts_and_rejects_late_publication() {
    let cache = TerminalCache::<u32, &'static str>::try_new(3, 24).unwrap();
    let stale_ready = cache
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(4),
            ReceiptPolicy::for_family(ReceiptFamily::Query),
            8,
        )
        .unwrap();
    let stale_failed = cache
        .admit(
            ReceiptId::from_raw(2),
            DeviceGeneration::from_raw(4),
            ReceiptPolicy::for_family(ReceiptFamily::Interest),
            8,
        )
        .unwrap();
    cache.set_current_generation(DeviceGeneration::from_raw(5), "device lost");
    assert!(matches!(
        stale_ready.poll(),
        ReceiptState::Failed(error) if *error == "device lost"
    ));
    assert!(matches!(
        stale_failed.poll(),
        ReceiptState::Failed(error) if *error == "device lost"
    ));
    assert_eq!(
        stale_ready
            .operation()
            .complete_ready_for_generation(DeviceGeneration::from_raw(4), 1),
        Err(TransitionError::StaleGeneration {
            expected: DeviceGeneration::from_raw(5),
            actual: DeviceGeneration::from_raw(4)
        })
    );
    assert_eq!(
        stale_ready.operation().complete_ready(1),
        Err(TransitionError::StaleGeneration {
            expected: DeviceGeneration::from_raw(5),
            actual: DeviceGeneration::from_raw(4)
        })
    );
    assert_eq!(
        stale_failed.operation().complete_failed("late failure"),
        Err(TransitionError::StaleGeneration {
            expected: DeviceGeneration::from_raw(5),
            actual: DeviceGeneration::from_raw(4)
        })
    );

    let current = cache
        .admit(
            ReceiptId::from_raw(3),
            DeviceGeneration::from_raw(5),
            ReceiptPolicy::for_family(ReceiptFamily::Interest),
            8,
        )
        .unwrap();
    current
        .operation()
        .advance(OperationPhase::Applying)
        .unwrap();
    current
        .operation()
        .complete_ready_for_generation(DeviceGeneration::from_raw(5), 2)
        .unwrap();
    assert!(matches!(current.poll(), ReceiptState::Ready(value) if *value == 2));
}

#[test]
fn generation_mismatched_admission_is_rejected_after_rollover() {
    let cache = TerminalCache::<u32, &'static str>::try_new(1, 8).unwrap();
    cache.set_current_generation(DeviceGeneration::from_raw(5), "device lost");
    let admission = cache.admit(
        ReceiptId::from_raw(1),
        DeviceGeneration::from_raw(4),
        ReceiptPolicy::for_family(ReceiptFamily::Interest),
        8,
    );
    let Err(error) = admission else {
        panic!("stale generation must not be admitted")
    };
    assert_eq!(
        error,
        super::ResultBackpressure::StaleGeneration {
            expected: DeviceGeneration::from_raw(5),
            actual: DeviceGeneration::from_raw(4),
        }
    );
}

#[test]
fn cloned_after_terminal_receipts_retain_the_shared_terminal_result() {
    let receipt = receipt(ReceiptFamily::Interest);
    receipt
        .operation()
        .advance(OperationPhase::Applying)
        .unwrap();
    receipt.operation().complete_ready(9).unwrap();
    let clone = receipt.clone();
    drop(receipt);
    let ReceiptState::Ready(value) = clone.poll() else {
        panic!("terminal result must be shared")
    };
    assert_eq!(*value, 9);
    assert_eq!(Arc::strong_count(&value), 2);
}

#[test]
fn dropped_pending_and_submitted_receipts_remain_owned_by_the_runtime_driver() {
    let cache = TerminalCache::<u32, &'static str>::try_new(1, 8).unwrap();
    let pending = cache
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Interest),
            8,
        )
        .unwrap();
    let pending_driver = pending.operation();
    drop(pending);
    assert!(
        cache
            .admit(
                ReceiptId::from_raw(2),
                DeviceGeneration::from_raw(1),
                ReceiptPolicy::for_family(ReceiptFamily::Interest),
                8
            )
            .is_err()
    );
    drop(pending_driver);
    assert!(
        cache
            .admit(
                ReceiptId::from_raw(2),
                DeviceGeneration::from_raw(1),
                ReceiptPolicy::for_family(ReceiptFamily::Interest),
                8
            )
            .is_ok()
    );

    let cache = TerminalCache::<u32, &'static str>::try_new(1, 8).unwrap();
    let submitted = cache
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Query),
            8,
        )
        .unwrap();
    submitted
        .operation()
        .advance(OperationPhase::WaitingForReadiness)
        .unwrap();
    submitted
        .operation()
        .advance(OperationPhase::Encoded)
        .unwrap();
    submitted
        .operation()
        .advance(OperationPhase::Submitted)
        .unwrap();
    submitted
        .operation()
        .advance(OperationPhase::Mapping)
        .unwrap();
    submitted
        .operation()
        .advance(OperationPhase::Decoding)
        .unwrap();
    let driver = submitted.operation();
    drop(submitted);
    assert!(matches!(driver.poll(), ReceiptState::Pending(_)));
    driver.complete_ready(1).unwrap();
    assert!(
        cache
            .admit(
                ReceiptId::from_raw(2),
                DeviceGeneration::from_raw(1),
                ReceiptPolicy::for_family(ReceiptFamily::Interest),
                8
            )
            .is_err()
    );
    drop(driver);
    assert!(
        cache
            .admit(
                ReceiptId::from_raw(2),
                DeviceGeneration::from_raw(1),
                ReceiptPolicy::for_family(ReceiptFamily::Interest),
                8
            )
            .is_ok()
    );
}

#[test]
fn result_capacity_rejects_every_count_and_byte_saturation_boundary() {
    let zero = TerminalCache::<u32, &'static str>::try_new(0, 0).unwrap();
    assert!(
        zero.admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Interest),
            0
        )
        .is_err()
    );

    let count = TerminalCache::<u32, &'static str>::try_new(1, 8).unwrap();
    let first = count
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Interest),
            8,
        )
        .unwrap();
    assert!(
        count
            .admit(
                ReceiptId::from_raw(2),
                DeviceGeneration::from_raw(1),
                ReceiptPolicy::for_family(ReceiptFamily::Interest),
                0
            )
            .is_err()
    );
    drop(first);

    let bytes = TerminalCache::<u32, &'static str>::try_new(2, 8).unwrap();
    let exact = bytes
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Interest),
            8,
        )
        .unwrap();
    assert!(
        bytes
            .admit(
                ReceiptId::from_raw(2),
                DeviceGeneration::from_raw(1),
                ReceiptPolicy::for_family(ReceiptFamily::Interest),
                1
            )
            .is_err()
    );
    drop(exact);

    let maximum = TerminalCache::<u32, &'static str>::try_new(2, u64::MAX).unwrap();
    let exact = maximum
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Interest),
            u64::MAX,
        )
        .unwrap();
    assert!(
        maximum
            .admit(
                ReceiptId::from_raw(2),
                DeviceGeneration::from_raw(1),
                ReceiptPolicy::for_family(ReceiptFamily::Interest),
                1
            )
            .is_err()
    );
    drop(exact);
}
