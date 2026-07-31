//! Operation state transitions and cancellation policy.

use std::sync::{
    Arc, Mutex, Weak,
    atomic::{AtomicBool, Ordering},
};

use crate::canonical::{DeviceGeneration, ReceiptId};

use super::receipt::{
    CancelledOperation, OperationProgress, ProgressBlocker, QueryReadinessReason, ReceiptState,
    TerminalReservation,
};

/// The finite receipt families defined by TECH-021.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptFamily {
    /// World construction before publication.
    Genesis,
    /// A sealed canonical tick.
    Tick,
    /// Interest installation or withdrawal.
    Interest,
    /// A bounded truth query.
    Query,
    /// An observation-ring resnapshot.
    ObservationResnapshot,
    /// A durable checkpoint request.
    Checkpoint,
    /// A private correction and durable branch export.
    Correction,
    /// A private restore before world publication.
    Restore,
    /// A private replay before world publication.
    Replay,
    /// Participant/device recovery.
    Recovery,
    /// Ordered world closure.
    Shutdown,
}

/// The phase exposed by a pending receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum OperationPhase {
    Verifying,
    Queued,
    Applying,
    WaitingForReadiness,
    Pinning,
    Loading,
    LoadingOwnedRecords,
    VerifyingHeader,
    ExportingReplayHeader,
    ExportingReplayPrefix,
    ExportingCorrectionBranch,
    Reading,
    Materializing,
    Preparing,
    Encoding,
    Encoded,
    Submitting,
    Submitted,
    GpuComplete,
    Mapping,
    Decoding,
    StoringBlobs,
    CommittingManifest,
    RestoringPrivate,
    ReplayingPrivate,
    ComparingExpected,
    Comparing,
    Querying,
    Rebuilding,
    RestoringParticipants,
    ValidatingFinal,
    Publishing,
    CreatingGeneration,
    LoadingAnchor,
    Replaying,
    ClosingAdmission,
    Draining,
    FinalCheckpoint,
    Releasing,
}

/// The last point at which a family accepts consumer cancellation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationCutoff {
    /// The family is never consumer-cancellable.
    Never,
    /// Cancellation directly completes before any submission.
    BeforeSubmission,
    /// Later cancellation only suppresses consumer delivery.
    SuppressAfterSubmission,
    /// Later cancellation drains private work and prevents publication.
    AbortAfterSubmission,
    /// A correction becomes non-cancellable once its branch export starts.
    BeforeCorrectionExport,
}

/// A family-specific lifecycle policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReceiptPolicy {
    family: ReceiptFamily,
    cutoff: CancellationCutoff,
}

impl ReceiptPolicy {
    /// Returns the exact cancellation policy for a TECH-021 receipt family.
    #[must_use]
    pub const fn for_family(family: ReceiptFamily) -> Self {
        let cutoff = match family {
            ReceiptFamily::Genesis | ReceiptFamily::Tick | ReceiptFamily::Shutdown => {
                CancellationCutoff::Never
            }
            ReceiptFamily::Interest => CancellationCutoff::BeforeSubmission,
            ReceiptFamily::Query | ReceiptFamily::ObservationResnapshot => {
                CancellationCutoff::SuppressAfterSubmission
            }
            ReceiptFamily::Checkpoint => CancellationCutoff::AbortAfterSubmission,
            ReceiptFamily::Correction => CancellationCutoff::BeforeCorrectionExport,
            ReceiptFamily::Restore | ReceiptFamily::Replay | ReceiptFamily::Recovery => {
                CancellationCutoff::AbortAfterSubmission
            }
        };
        Self { family, cutoff }
    }

    /// Returns the policy's receipt family.
    #[must_use]
    pub const fn family(self) -> ReceiptFamily {
        self.family
    }

    /// Reports whether `phase` is legal for this receipt family.
    #[must_use]
    pub const fn allows_phase(self, phase: OperationPhase) -> bool {
        phase_allowed(self.family, phase)
    }
}

/// The observable result of requesting cancellation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancelResult {
    CancelledBeforeSubmit,
    DeliverySuppressed,
    AbortRequested,
    NotCancellable,
    AlreadyTerminal,
}

/// A rejected lifecycle transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransitionError {
    /// The requested phase is not legal for the receipt family.
    InvalidPhase {
        family: ReceiptFamily,
        phase: OperationPhase,
    },
    /// The requested phase is legal for the family but is not its next phase.
    InvalidTransition {
        /// The receipt family whose lifecycle rejected the transition.
        family: ReceiptFamily,
        /// The operation's current pending phase.
        from: OperationPhase,
        /// The requested next phase.
        to: OperationPhase,
    },
    /// A terminal operation cannot be advanced or completed again.
    AlreadyTerminal,
    /// A stale device generation attempted to publish a result.
    StaleGeneration {
        expected: DeviceGeneration,
        actual: DeviceGeneration,
    },
    /// Cancellation permits drain work but forbids this externally visible phase.
    AbortBoundary {
        /// The cancelled receipt family.
        family: ReceiptFamily,
        /// The prohibited phase.
        phase: OperationPhase,
    },
    /// Submitted work has not reached the family's required drain milestone.
    DrainIncomplete {
        /// The receipt family whose drain is incomplete.
        family: ReceiptFamily,
        /// The current pending phase.
        phase: OperationPhase,
    },
}

enum Terminal<T, E> {
    Pending,
    Ready(Arc<T>),
    Failed(Arc<E>),
    Cancelled(CancelledOperation),
}

type TerminalRetainer<T, E> = Arc<dyn Fn(Arc<Operation<T, E>>) + Send + Sync>;

struct OperationState<T, E> {
    phase: OperationPhase,
    blocker: Option<ProgressBlocker>,
    submitted: bool,
    cancellation_cutoff_reached: bool,
    delivery_suppressed: bool,
    abort_requested: bool,
    terminal: Terminal<T, E>,
}

/// Shared mutable state owned by all clones of one receipt.
pub(crate) struct Operation<T, E> {
    receipt: ReceiptId,
    generation: DeviceGeneration,
    policy: ReceiptPolicy,
    // The terminal cache retains completed operations. This back-reference
    // must therefore be weak so the cache can be dropped with its world.
    reservation: Weak<TerminalReservation>,
    old_generation_failure:
        Arc<dyn Fn(ReceiptId, ReceiptFamily, DeviceGeneration) -> E + Send + Sync>,
    state: Mutex<OperationState<T, E>>,
    // Store/device callbacks can be outstanding before their corresponding
    // lifecycle phase is entered. Keep that invocation as a separate atomic
    // latch so cancellation cannot mistake it for work that was never begun.
    submission_or_invocation: AtomicBool,
    terminal_signal: Arc<AtomicBool>,
    retainer: TerminalRetainer<T, E>,
    reservation_active: AtomicBool,
    result_bytes: u64,
}

impl<T, E> Operation<T, E> {
    pub(super) fn new(
        receipt: ReceiptId,
        generation: DeviceGeneration,
        policy: ReceiptPolicy,
        reservation: Arc<TerminalReservation>,
        old_generation_failure: Arc<
            dyn Fn(ReceiptId, ReceiptFamily, DeviceGeneration) -> E + Send + Sync,
        >,
        retainer: TerminalRetainer<T, E>,
        result_bytes: u64,
    ) -> Self {
        Self {
            receipt,
            generation,
            policy,
            reservation: Arc::downgrade(&reservation),
            old_generation_failure,
            state: Mutex::new(OperationState {
                phase: initial_phase(policy.family),
                blocker: None,
                submitted: false,
                cancellation_cutoff_reached: false,
                delivery_suppressed: false,
                abort_requested: false,
                terminal: Terminal::Pending,
            }),
            submission_or_invocation: AtomicBool::new(false),
            terminal_signal: Arc::new(AtomicBool::new(false)),
            retainer,
            reservation_active: AtomicBool::new(false),
            result_bytes,
        }
    }

    /// Returns this operation's immutable accepted receipt identity.
    #[must_use]
    pub(crate) const fn receipt_id(&self) -> ReceiptId {
        self.receipt
    }

    /// Returns the device generation allowed to publish this operation.
    #[must_use]
    pub(crate) const fn generation(&self) -> DeviceGeneration {
        self.generation
    }

    /// Returns the immutable lifecycle family selected at admission.
    #[must_use]
    pub(crate) const fn family(&self) -> ReceiptFamily {
        self.policy.family
    }

    /// Advances a pending operation to a legal family phase.
    pub(crate) fn advance(&self, phase: OperationPhase) -> Result<(), TransitionError> {
        if !self.policy.allows_phase(phase) {
            return Err(TransitionError::InvalidPhase {
                family: self.policy.family,
                phase,
            });
        }
        let mut state = self.state.lock().expect("operation state mutex poisoned");
        if !matches!(state.terminal, Terminal::Pending) {
            return Err(TransitionError::AlreadyTerminal);
        }
        if state.abort_requested && abort_prohibits_phase(self.policy.family, phase) {
            return Err(TransitionError::AbortBoundary {
                family: self.policy.family,
                phase,
            });
        }
        if !phase_successor(self.policy.family, state.phase, phase) {
            return Err(TransitionError::InvalidTransition {
                family: self.policy.family,
                from: state.phase,
                to: phase,
            });
        }
        state.phase = phase;
        state.blocker = None;
        if work_started(self.policy.family, phase) {
            state.submitted = true;
            self.submission_or_invocation.store(true, Ordering::Release);
        }
        state.cancellation_cutoff_reached |= cancellation_cutoff_reached(self.policy.family, phase);
        Ok(())
    }

    /// Latches that the driver invoked checkpoint-store or device work.
    ///
    /// Drivers must call this before invoking an asynchronous provider callback.
    /// In particular, restore loads can invoke a provider while their receipt is
    /// still in `Loading` or `Verifying`; a subsequent cancellation must drain
    /// that private work rather than terminalizing immediately.
    pub(crate) fn mark_submission_or_invocation(&self) -> Result<(), TransitionError> {
        let mut state = self.state.lock().expect("operation state mutex poisoned");
        if !matches!(state.terminal, Terminal::Pending) {
            return Err(TransitionError::AlreadyTerminal);
        }
        self.submission_or_invocation.store(true, Ordering::Release);
        state.submitted = true;
        Ok(())
    }

    /// Records a query readiness blocker while the query waits for readiness.
    pub(crate) fn set_query_blocker(
        &self,
        reason: QueryReadinessReason,
    ) -> Result<(), TransitionError> {
        let mut state = self.state.lock().expect("operation state mutex poisoned");
        if self.policy.family != ReceiptFamily::Query
            || state.phase != OperationPhase::WaitingForReadiness
        {
            return Err(TransitionError::InvalidPhase {
                family: self.policy.family,
                phase: state.phase,
            });
        }
        if !matches!(state.terminal, Terminal::Pending) {
            return Err(TransitionError::AlreadyTerminal);
        }
        state.blocker = Some(ProgressBlocker::Query(Arc::new(reason)));
        Ok(())
    }

    /// Reports whether cancellation requires the owner to stop publication work.
    ///
    /// A true result does not cancel submitted GPU work; the driver must continue
    /// the family-specific drain phases and then complete the receipt.
    #[must_use]
    pub(crate) fn abort_requested(&self) -> bool {
        self.state
            .lock()
            .expect("operation state mutex poisoned")
            .abort_requested
    }

    /// Takes a nonblocking, idempotent snapshot of this operation.
    #[must_use]
    pub fn poll(&self) -> ReceiptState<T, E> {
        let state = self.state.lock().expect("operation state mutex poisoned");
        match &state.terminal {
            Terminal::Pending => {
                ReceiptState::Pending(OperationProgress::new(state.phase, state.blocker.clone()))
            }
            Terminal::Ready(value) => ReceiptState::Ready(Arc::clone(value)),
            Terminal::Failed(error) => ReceiptState::Failed(Arc::clone(error)),
            Terminal::Cancelled(cancelled) => ReceiptState::Cancelled(*cancelled),
        }
    }

    #[cfg(feature = "bevy")]
    pub(super) fn terminal_signal(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.terminal_signal)
    }

    /// Requests cancellation according to the receipt family's cutoff.
    pub fn cancel(self: &Arc<Self>) -> CancelResult {
        let result = {
            let mut state = self.state.lock().expect("operation state mutex poisoned");
            if !matches!(state.terminal, Terminal::Pending) {
                return CancelResult::AlreadyTerminal;
            }
            match self.policy.cutoff {
                CancellationCutoff::Never => return CancelResult::NotCancellable,
                CancellationCutoff::BeforeSubmission
                    if before_cancellation_cutoff(self.policy.family, state.phase) =>
                {
                    state.terminal =
                        Terminal::Cancelled(cancelled(self.receipt, state.phase, false));
                    self.terminal_signal.store(true, Ordering::Release);
                    CancelResult::CancelledBeforeSubmit
                }
                CancellationCutoff::BeforeSubmission => return CancelResult::NotCancellable,
                CancellationCutoff::SuppressAfterSubmission
                    if before_cancellation_cutoff(self.policy.family, state.phase) =>
                {
                    state.terminal =
                        Terminal::Cancelled(cancelled(self.receipt, state.phase, false));
                    self.terminal_signal.store(true, Ordering::Release);
                    CancelResult::CancelledBeforeSubmit
                }
                CancellationCutoff::SuppressAfterSubmission => {
                    state.delivery_suppressed = true;
                    return CancelResult::DeliverySuppressed;
                }
                CancellationCutoff::AbortAfterSubmission
                    if !self.submission_or_invocation.load(Ordering::Acquire)
                        && !state.submitted =>
                {
                    state.terminal =
                        Terminal::Cancelled(cancelled(self.receipt, state.phase, false));
                    self.terminal_signal.store(true, Ordering::Release);
                    CancelResult::CancelledBeforeSubmit
                }
                CancellationCutoff::AbortAfterSubmission => {
                    state.submitted |= self.submission_or_invocation.load(Ordering::Acquire);
                    state.abort_requested = true;
                    return CancelResult::AbortRequested;
                }
                CancellationCutoff::BeforeCorrectionExport if state.cancellation_cutoff_reached => {
                    return CancelResult::NotCancellable;
                }
                CancellationCutoff::BeforeCorrectionExport
                    if !self.submission_or_invocation.load(Ordering::Acquire)
                        && !state.submitted =>
                {
                    state.terminal =
                        Terminal::Cancelled(cancelled(self.receipt, state.phase, false));
                    self.terminal_signal.store(true, Ordering::Release);
                    CancelResult::CancelledBeforeSubmit
                }
                CancellationCutoff::BeforeCorrectionExport => {
                    state.submitted |= self.submission_or_invocation.load(Ordering::Acquire);
                    state.abort_requested = true;
                    return CancelResult::AbortRequested;
                }
            }
        };
        if result == CancelResult::CancelledBeforeSubmit {
            self.retain_terminal();
        }
        result
    }

    /// Completes successfully, retaining the shared result while handles or cache retain it.
    pub fn complete_ready(self: &Arc<Self>, value: T) -> Result<(), TransitionError> {
        self.finish(self.generation, Terminal::Ready(Arc::new(value)))
    }

    /// Completes successfully only when the producing generation is current.
    pub fn complete_ready_for_generation(
        self: &Arc<Self>,
        generation: DeviceGeneration,
        value: T,
    ) -> Result<(), TransitionError> {
        self.finish(generation, Terminal::Ready(Arc::new(value)))
    }

    /// Completes with a typed failure.
    pub fn complete_failed(self: &Arc<Self>, error: E) -> Result<(), TransitionError> {
        self.finish(self.generation, Terminal::Failed(Arc::new(error)))
    }

    /// Completes with a typed failure only when the producing generation is current.
    pub fn complete_failed_for_generation(
        self: &Arc<Self>,
        generation: DeviceGeneration,
        error: E,
    ) -> Result<(), TransitionError> {
        self.finish(generation, Terminal::Failed(Arc::new(error)))
    }

    fn finish(
        self: &Arc<Self>,
        producing_generation: DeviceGeneration,
        terminal: Terminal<T, E>,
    ) -> Result<(), TransitionError> {
        {
            self.reservation
                .upgrade()
                .expect("live operation completion requires its terminal reservation")
                .with_current_generation(|current_generation| {
                    if self.generation != current_generation
                        || producing_generation != current_generation
                    {
                        return Err(TransitionError::StaleGeneration {
                            expected: current_generation,
                            actual: producing_generation,
                        });
                    }
                    let mut state = self.state.lock().expect("operation state mutex poisoned");
                    if !matches!(state.terminal, Terminal::Pending) {
                        return Err(TransitionError::AlreadyTerminal);
                    }
                    if state.delivery_suppressed || state.abort_requested {
                        let submitted = state.submitted
                            || self.submission_or_invocation.load(Ordering::Acquire);
                        if submitted && !drain_complete(self.policy.family, state.phase) {
                            return Err(TransitionError::DrainIncomplete {
                                family: self.policy.family,
                                phase: state.phase,
                            });
                        }
                        state.terminal =
                            Terminal::Cancelled(cancelled(self.receipt, state.phase, submitted));
                    } else {
                        if matches!(terminal, Terminal::Ready(_))
                            && state.phase != final_phase(self.policy.family)
                        {
                            return Err(TransitionError::InvalidTransition {
                                family: self.policy.family,
                                from: state.phase,
                                to: final_phase(self.policy.family),
                            });
                        }
                        state.terminal = terminal;
                    }
                    self.terminal_signal.store(true, Ordering::Release);
                    Ok(())
                })?;
        }
        self.retain_terminal();
        Ok(())
    }

    pub(super) fn terminalize_old_generation(self: &Arc<Self>, error: E) {
        self.set_terminal(Terminal::Failed(Arc::new(error)));
    }

    fn set_terminal(self: &Arc<Self>, terminal: Terminal<T, E>) -> bool {
        {
            let mut state = self.state.lock().expect("operation state mutex poisoned");
            if !matches!(state.terminal, Terminal::Pending) {
                return false;
            }
            state.terminal = terminal;
            self.terminal_signal.store(true, Ordering::Release);
        }
        self.retain_terminal();
        true
    }

    fn retain_terminal(self: &Arc<Self>) {
        (self.retainer)(Arc::clone(self));
    }
}

impl<T, E> Drop for Operation<T, E> {
    fn drop(&mut self) {
        if self.reservation_active.load(Ordering::Acquire)
            && let Some(reservation) = self.reservation.upgrade()
        {
            reservation.release(self.result_bytes);
        }
    }
}

impl<T, E> super::receipt::TerminalOperation for Operation<T, E>
where
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
{
    fn generation(&self) -> DeviceGeneration {
        self.generation
    }

    fn terminalize_old_generation(self: Arc<Self>) {
        let error =
            (self.old_generation_failure)(self.receipt, self.policy.family, self.generation);
        self.set_terminal(Terminal::Failed(Arc::new(error)));
    }

    fn activate_reservation(&self) {
        self.reservation_active.store(true, Ordering::Release);
    }
}

fn cancelled(receipt: ReceiptId, phase: OperationPhase, submitted: bool) -> CancelledOperation {
    CancelledOperation {
        receipt,
        last_phase: phase,
        submitted_work_drained: submitted,
    }
}

const fn initial_phase(family: ReceiptFamily) -> OperationPhase {
    match family {
        ReceiptFamily::Genesis => OperationPhase::Verifying,
        ReceiptFamily::Tick | ReceiptFamily::Interest | ReceiptFamily::Query => {
            OperationPhase::Queued
        }
        ReceiptFamily::ObservationResnapshot
        | ReceiptFamily::Checkpoint
        | ReceiptFamily::Correction => OperationPhase::Queued,
        ReceiptFamily::Restore => OperationPhase::Loading,
        ReceiptFamily::Replay => OperationPhase::LoadingOwnedRecords,
        ReceiptFamily::Recovery => OperationPhase::Queued,
        ReceiptFamily::Shutdown => OperationPhase::ClosingAdmission,
    }
}

const fn final_phase(family: ReceiptFamily) -> OperationPhase {
    use OperationPhase::*;
    match family {
        ReceiptFamily::Genesis => ExportingReplayHeader,
        ReceiptFamily::Tick | ReceiptFamily::Query => Decoding,
        ReceiptFamily::Interest => Applying,
        ReceiptFamily::ObservationResnapshot => Encoding,
        ReceiptFamily::Checkpoint => CommittingManifest,
        ReceiptFamily::Correction | ReceiptFamily::Restore | ReceiptFamily::Replay => Publishing,
        ReceiptFamily::Recovery => Comparing,
        ReceiptFamily::Shutdown => Releasing,
    }
}

fn drain_complete(family: ReceiptFamily, phase: OperationPhase) -> bool {
    use OperationPhase::*;
    match family {
        ReceiptFamily::Checkpoint => matches!(phase, StoringBlobs | CommittingManifest),
        ReceiptFamily::Correction => matches!(
            phase,
            ValidatingFinal | ExportingCorrectionBranch | Publishing
        ),
        ReceiptFamily::Restore => matches!(
            phase,
            RestoringParticipants | ExportingReplayHeader | Publishing
        ),
        ReceiptFamily::Replay => matches!(
            phase,
            ComparingExpected | ExportingReplayHeader | ExportingReplayPrefix | Publishing
        ),
        _ => phase == final_phase(family),
    }
}

const fn work_started(family: ReceiptFamily, phase: OperationPhase) -> bool {
    use OperationPhase::*;
    match family {
        ReceiptFamily::Genesis => matches!(phase, Submitting),
        ReceiptFamily::Tick | ReceiptFamily::Query => matches!(phase, Submitted),
        ReceiptFamily::Interest => false,
        ReceiptFamily::ObservationResnapshot => matches!(phase, Encoding),
        ReceiptFamily::Checkpoint => matches!(phase, Reading),
        ReceiptFamily::Correction => matches!(phase, RestoringPrivate),
        ReceiptFamily::Restore => matches!(phase, Rebuilding),
        ReceiptFamily::Replay => matches!(phase, ReplayingPrivate),
        ReceiptFamily::Recovery => matches!(phase, Replaying),
        ReceiptFamily::Shutdown => false,
    }
}

const fn cancellation_cutoff_reached(family: ReceiptFamily, phase: OperationPhase) -> bool {
    matches!(
        (family, phase),
        (ReceiptFamily::Checkpoint, OperationPhase::Reading)
            | (
                ReceiptFamily::Correction,
                OperationPhase::ExportingCorrectionBranch
            )
    )
}

const fn before_cancellation_cutoff(family: ReceiptFamily, phase: OperationPhase) -> bool {
    match family {
        ReceiptFamily::Interest => matches!(phase, OperationPhase::Queued),
        ReceiptFamily::Query => matches!(
            phase,
            OperationPhase::Queued | OperationPhase::WaitingForReadiness
        ),
        ReceiptFamily::ObservationResnapshot => matches!(
            phase,
            OperationPhase::Queued | OperationPhase::Pinning | OperationPhase::Querying
        ),
        ReceiptFamily::Checkpoint => {
            matches!(phase, OperationPhase::Queued | OperationPhase::Pinning)
        }
        _ => false,
    }
}

pub(super) const fn abort_prohibits_phase(family: ReceiptFamily, phase: OperationPhase) -> bool {
    matches!(
        (family, phase),
        (
            ReceiptFamily::Checkpoint,
            OperationPhase::CommittingManifest
        ) | (
            ReceiptFamily::Correction,
            OperationPhase::ExportingCorrectionBranch
        ) | (ReceiptFamily::Correction, OperationPhase::Publishing)
            | (
                ReceiptFamily::Restore,
                OperationPhase::ExportingReplayHeader
            )
            | (ReceiptFamily::Restore, OperationPhase::Publishing)
            | (ReceiptFamily::Replay, OperationPhase::ExportingReplayHeader)
            | (ReceiptFamily::Replay, OperationPhase::ExportingReplayPrefix)
            | (ReceiptFamily::Replay, OperationPhase::Publishing)
    )
}

const fn phase_allowed(family: ReceiptFamily, phase: OperationPhase) -> bool {
    use OperationPhase::*;
    match family {
        ReceiptFamily::Genesis => matches!(
            phase,
            Verifying | Materializing | Submitting | ExportingReplayHeader
        ),
        ReceiptFamily::Tick => matches!(
            phase,
            Queued | Preparing | Encoded | Submitted | GpuComplete | Decoding
        ),
        ReceiptFamily::Interest => matches!(phase, Queued | Applying),
        ReceiptFamily::Query => matches!(
            phase,
            Queued | WaitingForReadiness | Encoded | Submitted | Mapping | Decoding
        ),
        ReceiptFamily::ObservationResnapshot => {
            matches!(phase, Queued | Pinning | Querying | Encoding)
        }
        ReceiptFamily::Checkpoint => matches!(
            phase,
            Queued | Pinning | Reading | StoringBlobs | CommittingManifest
        ),
        ReceiptFamily::Correction => matches!(
            phase,
            Queued
                | RestoringPrivate
                | ReplayingPrivate
                | ValidatingFinal
                | ExportingCorrectionBranch
                | Publishing
        ),
        ReceiptFamily::Restore => matches!(
            phase,
            Loading
                | Verifying
                | Rebuilding
                | RestoringParticipants
                | ExportingReplayHeader
                | Publishing
        ),
        ReceiptFamily::Replay => matches!(
            phase,
            LoadingOwnedRecords
                | VerifyingHeader
                | ReplayingPrivate
                | ComparingExpected
                | ExportingReplayHeader
                | ExportingReplayPrefix
                | Publishing
        ),
        ReceiptFamily::Recovery => matches!(
            phase,
            Queued | CreatingGeneration | LoadingAnchor | Replaying | Comparing
        ),
        ReceiptFamily::Shutdown => matches!(
            phase,
            ClosingAdmission | Draining | FinalCheckpoint | Releasing
        ),
    }
}

const fn phase_successor(family: ReceiptFamily, from: OperationPhase, to: OperationPhase) -> bool {
    use OperationPhase::*;
    matches!(
        (family, from, to),
        (ReceiptFamily::Genesis, Verifying, Materializing)
            | (ReceiptFamily::Genesis, Materializing, Submitting)
            | (ReceiptFamily::Genesis, Submitting, ExportingReplayHeader)
            | (ReceiptFamily::Tick, Queued, Preparing)
            | (ReceiptFamily::Tick, Preparing, Encoded)
            | (ReceiptFamily::Tick, Encoded, Submitted)
            | (ReceiptFamily::Tick, Submitted, GpuComplete)
            | (ReceiptFamily::Tick, GpuComplete, Decoding)
            | (ReceiptFamily::Interest, Queued, Applying)
            | (ReceiptFamily::Query, Queued, WaitingForReadiness)
            | (ReceiptFamily::Query, WaitingForReadiness, Encoded)
            | (ReceiptFamily::Query, Encoded, Submitted)
            | (ReceiptFamily::Query, Submitted, Mapping)
            | (ReceiptFamily::Query, Mapping, Decoding)
            | (ReceiptFamily::ObservationResnapshot, Queued, Pinning)
            | (ReceiptFamily::ObservationResnapshot, Pinning, Querying)
            | (ReceiptFamily::ObservationResnapshot, Querying, Encoding)
            | (ReceiptFamily::Checkpoint, Queued, Pinning)
            | (ReceiptFamily::Checkpoint, Pinning, Reading)
            | (ReceiptFamily::Checkpoint, Reading, StoringBlobs)
            | (ReceiptFamily::Checkpoint, StoringBlobs, CommittingManifest)
            | (ReceiptFamily::Correction, Queued, RestoringPrivate)
            | (
                ReceiptFamily::Correction,
                RestoringPrivate,
                ReplayingPrivate
            )
            | (ReceiptFamily::Correction, ReplayingPrivate, ValidatingFinal)
            | (
                ReceiptFamily::Correction,
                ValidatingFinal,
                ExportingCorrectionBranch
            )
            | (
                ReceiptFamily::Correction,
                ExportingCorrectionBranch,
                Publishing
            )
            | (ReceiptFamily::Restore, Loading, Verifying)
            | (ReceiptFamily::Restore, Verifying, Rebuilding)
            | (ReceiptFamily::Restore, Rebuilding, RestoringParticipants)
            | (
                ReceiptFamily::Restore,
                RestoringParticipants,
                ExportingReplayHeader
            )
            | (ReceiptFamily::Restore, ExportingReplayHeader, Publishing)
            | (ReceiptFamily::Replay, LoadingOwnedRecords, VerifyingHeader)
            | (ReceiptFamily::Replay, VerifyingHeader, ReplayingPrivate)
            | (ReceiptFamily::Replay, ReplayingPrivate, ComparingExpected)
            | (
                ReceiptFamily::Replay,
                ComparingExpected,
                ExportingReplayHeader
            )
            | (
                ReceiptFamily::Replay,
                ExportingReplayHeader,
                ExportingReplayPrefix
            )
            | (ReceiptFamily::Replay, ExportingReplayPrefix, Publishing)
            | (ReceiptFamily::Recovery, Queued, CreatingGeneration)
            | (ReceiptFamily::Recovery, CreatingGeneration, LoadingAnchor)
            | (ReceiptFamily::Recovery, LoadingAnchor, Replaying)
            | (ReceiptFamily::Recovery, Replaying, Comparing)
            | (ReceiptFamily::Shutdown, ClosingAdmission, Draining)
            | (ReceiptFamily::Shutdown, Draining, FinalCheckpoint)
            | (ReceiptFamily::Shutdown, FinalCheckpoint, Releasing)
    )
}
