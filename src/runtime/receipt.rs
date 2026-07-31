//! Shared receipt handles and bounded terminal-result retention.

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

#[cfg(feature = "bevy")]
use std::sync::atomic::{AtomicBool, Ordering};

use crate::{
    canonical::{DeviceGeneration, ReceiptId, VolumeId, VolumeRevision},
    facade::{
        BoundedVec, CheckpointCommitted, CheckpointError, CorrectionCommitted, CorrectionError,
        FailedNoAdvance, GenesisError, GenesisReady, InterestApplied, InterestError, MissingRange,
        ObservationResnapshot, ObservationSnapshotError, QueryResult, QueryUnavailable, Recovered,
        RecoveryError, ReplayCompleted, ReplayFailure, ResourceBudgetField, RestoreError,
        RestoreReady, ShutdownError, ShutdownReport, TickConfirmed,
    },
};

use super::operation::{Operation, OperationPhase, ReceiptPolicy};

/// An unmet minimum revision required by a pending query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MinimumRevisionGap {
    /// The queried volume whose revision is too old or unavailable.
    pub volume: VolumeId,
    /// The minimum revision accepted by the query.
    pub required: VolumeRevision,
    /// The currently available revision, if the volume is ready.
    pub current: Option<VolumeRevision>,
}

/// The exact reason a complete query is waiting for readiness.
#[derive(Debug)]
pub enum QueryReadinessReason {
    /// One or more queried ranges are not currently available.
    Availability {
        /// The bounded, exact unavailable ranges.
        missing: BoundedVec<MissingRange>,
    },
    /// One or more queried volumes have not reached their required revision.
    MinimumRevision {
        /// The bounded, exact revision gaps.
        unmet: BoundedVec<MinimumRevisionGap>,
    },
    /// Completing the query would exceed an admitted resource budget.
    ResourcePressure {
        /// The budget field that cannot satisfy the request.
        field: ResourceBudgetField,
        /// The exact resource amount required.
        required: u64,
        /// The exact resource amount supported.
        supported: u64,
    },
}

/// The query-specific blocker carried by a pending operation.
#[derive(Clone, Debug)]
pub enum ProgressBlocker {
    /// A query is waiting for a feature-owned readiness fact.
    Query(Arc<QueryReadinessReason>),
}

/// The observable pending phase and optional query blocker.
#[derive(Clone, Debug)]
pub struct OperationProgress {
    /// The operation's current lifecycle phase.
    pub phase: OperationPhase,
    /// Present only for a query waiting on readiness.
    pub blocker: Option<ProgressBlocker>,
}

impl OperationProgress {
    pub(super) fn new(phase: OperationPhase, blocker: Option<ProgressBlocker>) -> Self {
        Self { phase, blocker }
    }
}

/// A terminal cancellation record shared by every receipt clone.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CancelledOperation {
    /// The accepted receipt identity.
    pub receipt: ReceiptId,
    /// The last observed pending phase.
    pub last_phase: OperationPhase,
    /// Whether already-submitted work was retained until safe drain.
    pub submitted_work_drained: bool,
}

/// A nonblocking snapshot of one receipt.
#[derive(Debug)]
pub enum ReceiptState<T, E> {
    Pending(OperationProgress),
    Ready(Arc<T>),
    Failed(Arc<E>),
    Cancelled(CancelledOperation),
}

/// Admission rejection caused by bounded terminal result retention.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResultBackpressure {
    /// The requested record or byte reservation exceeds cache capacity.
    Full,
    /// The cache could not reserve its finite terminal-entry index.
    AllocationFailed,
    /// The admission named a device generation that is no longer current.
    StaleGeneration {
        /// The generation currently accepting new operations.
        expected: DeviceGeneration,
        /// The generation attached to the rejected operation.
        actual: DeviceGeneration,
    },
}

/// A rejected attempt to move a device generation backwards or repeat it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GenerationTransitionError {
    pub(crate) current: DeviceGeneration,
    pub(crate) requested: DeviceGeneration,
}

pub(super) trait TerminalOperation: Send + Sync {
    fn generation(&self) -> DeviceGeneration;
    fn terminalize_old_generation(self: Arc<Self>);
    fn activate_reservation(&self);
}

struct ReservationState {
    entries: VecDeque<Arc<dyn TerminalOperation>>,
    operations: VecDeque<std::sync::Weak<dyn TerminalOperation>>,
    used_records: u32,
    used_bytes: u64,
}

/// Per-world count, byte, and device-generation authority for all receipt families.
///
/// Each admission reserves one record and its declared worst-case terminal
/// bytes. A terminal cache entry is evicted only when no receipt clone retains
/// it, so consumers holding old receipts exert explicit backpressure.
pub(crate) struct TerminalReservation {
    record_capacity: u32,
    byte_capacity: u64,
    current_generation: Arc<Mutex<Option<DeviceGeneration>>>,
    state: Mutex<ReservationState>,
}

impl TerminalReservation {
    /// Creates a finite shared reservation. Its index capacity is reserved before use.
    pub(crate) fn try_new(
        record_capacity: u32,
        byte_capacity: u64,
    ) -> Result<Arc<Self>, ResultBackpressure> {
        let capacity =
            usize::try_from(record_capacity).map_err(|_| ResultBackpressure::AllocationFailed)?;
        let mut entries = VecDeque::new();
        entries
            .try_reserve_exact(capacity)
            .map_err(|_| ResultBackpressure::AllocationFailed)?;
        let mut operations = VecDeque::new();
        operations
            .try_reserve_exact(capacity)
            .map_err(|_| ResultBackpressure::AllocationFailed)?;
        Ok(Arc::new(Self {
            record_capacity,
            byte_capacity,
            current_generation: Arc::new(Mutex::new(None)),
            state: Mutex::new(ReservationState {
                entries,
                operations,
                used_records: 0,
                used_bytes: 0,
            }),
        }))
    }

    fn reserve_and_register(
        &self,
        generation: DeviceGeneration,
        result_bytes: u64,
        operation: Arc<dyn TerminalOperation>,
    ) -> Result<(), ResultBackpressure> {
        // Keep the generation guard until the operation is registered. A
        // rollover therefore sees every accepted old-generation operation.
        let mut current_generation = self
            .current_generation
            .lock()
            .expect("device generation mutex poisoned");
        match *current_generation {
            Some(expected) if expected != generation => {
                return Err(ResultBackpressure::StaleGeneration {
                    expected,
                    actual: generation,
                });
            }
            Some(_) => {}
            None => *current_generation = Some(generation),
        }
        loop {
            let victim = {
                let mut state = self.state.lock().expect("terminal cache mutex poisoned");
                if has_capacity(
                    &state,
                    self.record_capacity,
                    self.byte_capacity,
                    result_bytes,
                ) {
                    state.used_records += 1;
                    state.used_bytes += result_bytes;
                    None
                } else if let Some(index) = state
                    .entries
                    .iter()
                    .position(|operation| Arc::strong_count(operation) == 1)
                {
                    state.entries.remove(index)
                } else {
                    return Err(ResultBackpressure::Full);
                }
            };
            if let Some(victim) = victim {
                drop(victim);
                continue;
            }
            let mut state = self.state.lock().expect("terminal cache mutex poisoned");
            state
                .operations
                .retain(|operation| operation.strong_count() != 0);
            state.operations.push_back(Arc::downgrade(&operation));
            operation.activate_reservation();
            drop(current_generation);
            return Ok(());
        }
    }

    /// Changes the publishing generation and fails every pending earlier receipt.
    ///
    /// Each typed receipt adapter provides its own device-loss failure. Already
    /// terminal receipts remain unchanged, and late completion attempts from an
    /// old generation are still rejected.
    pub(crate) fn set_current_generation(
        &self,
        generation: DeviceGeneration,
    ) -> Result<(), GenerationTransitionError> {
        let mut current_generation = self
            .current_generation
            .lock()
            .expect("device generation mutex poisoned");
        if let Some(current) = *current_generation {
            // A generation is a monotonic device epoch. Accepting an older or
            // equal epoch would revive operations terminalized during a prior
            // rollover and make their late callbacks publishable again.
            if generation.get() <= current.get() {
                return Err(GenerationTransitionError {
                    current,
                    requested: generation,
                });
            }
        }
        *current_generation = Some(generation);

        // Snapshot while registration is excluded by `current_generation`.
        // Do not hold cache locks while terminalization retains a cache entry.
        let operations = {
            let mut state = self.state.lock().expect("terminal cache mutex poisoned");
            state
                .operations
                .retain(|operation| operation.strong_count() != 0);
            state
                .operations
                .iter()
                .filter_map(std::sync::Weak::upgrade)
                .filter(|operation| operation.generation().get() < generation.get())
                .collect::<Vec<_>>()
        };
        drop(current_generation);
        for operation in operations {
            operation.terminalize_old_generation();
        }
        Ok(())
    }

    pub(super) fn retain_terminal(&self, operation: Arc<dyn TerminalOperation>) {
        let mut state = self.state.lock().expect("terminal cache mutex poisoned");
        state.entries.push_back(operation);
    }

    pub(super) fn release(&self, result_bytes: u64) {
        let mut state = self.state.lock().expect("terminal cache mutex poisoned");
        state.used_records -= 1;
        state.used_bytes -= result_bytes;
    }

    pub(super) fn with_current_generation<R>(
        &self,
        action: impl FnOnce(DeviceGeneration) -> R,
    ) -> R {
        let generation = self
            .current_generation
            .lock()
            .expect("device generation mutex poisoned")
            .expect("admitted operation has a device generation");
        action(generation)
    }
}

fn has_capacity(
    state: &ReservationState,
    record_capacity: u32,
    byte_capacity: u64,
    result_bytes: u64,
) -> bool {
    state.used_records < record_capacity
        && result_bytes <= byte_capacity.saturating_sub(state.used_bytes)
}

/// A typed receipt adapter backed by a shared per-world terminal reservation.
pub(crate) struct TerminalCache<T, E> {
    reservation: Arc<TerminalReservation>,
    old_generation_failure: Arc<
        dyn Fn(ReceiptId, super::operation::ReceiptFamily, DeviceGeneration) -> E + Send + Sync,
    >,
    marker: std::marker::PhantomData<T>,
}

impl<T, E> TerminalCache<T, E>
where
    T: Send + Sync + 'static,
    E: Send + Sync + 'static,
{
    pub(crate) fn new(
        reservation: Arc<TerminalReservation>,
        old_generation_failure: impl Fn(
            ReceiptId,
            super::operation::ReceiptFamily,
            DeviceGeneration,
        ) -> E
        + Send
        + Sync
        + 'static,
    ) -> Arc<Self> {
        Arc::new(Self {
            reservation,
            old_generation_failure: Arc::new(old_generation_failure),
            marker: std::marker::PhantomData,
        })
    }

    /// Admits an operation after reserving one shared terminal record and all result bytes.
    pub(crate) fn admit(
        self: &Arc<Self>,
        receipt: ReceiptId,
        generation: DeviceGeneration,
        policy: ReceiptPolicy,
        result_bytes: u64,
    ) -> Result<Receipt<T, E>, ResultBackpressure> {
        let reservation_for_retain = Arc::clone(&self.reservation);
        let retainer = Arc::new(move |operation: Arc<Operation<T, E>>| {
            let tracked: Arc<dyn TerminalOperation> = operation;
            reservation_for_retain.retain_terminal(tracked);
        });
        let operation = Arc::new(Operation::new(
            receipt,
            generation,
            policy,
            Arc::clone(&self.reservation),
            Arc::clone(&self.old_generation_failure),
            retainer,
            result_bytes,
        ));
        let tracked: Arc<dyn TerminalOperation> = operation.clone();
        self.reservation
            .reserve_and_register(generation, result_bytes, tracked)?;
        Ok(Receipt { operation })
    }

    pub(crate) fn set_current_generation(
        &self,
        generation: DeviceGeneration,
    ) -> Result<(), GenerationTransitionError> {
        self.reservation.set_current_generation(generation)
    }
}

/// A cloneable, pollable receipt sharing one operation record.
pub(crate) struct Receipt<T, E> {
    operation: Arc<Operation<T, E>>,
}

impl<T, E> Clone for Receipt<T, E> {
    fn clone(&self) -> Self {
        Self {
            operation: Arc::clone(&self.operation),
        }
    }
}

impl<T, E> Receipt<T, E> {
    pub(super) fn operation(&self) -> Arc<Operation<T, E>> {
        Arc::clone(&self.operation)
    }

    /// Returns an idempotent nonblocking state snapshot.
    #[must_use]
    pub fn poll(&self) -> ReceiptState<T, E> {
        self.operation.poll()
    }

    /// Requests cancellation according to the operation's family policy.
    pub fn cancel(&self) -> super::operation::CancelResult {
        self.operation.cancel()
    }
}

/// A Bevy notification that a receipt family has reached a terminal state.
///
/// The notification carries identity only; consumers poll their retained
/// concrete receipt to obtain the shared terminal value.
#[cfg(feature = "bevy")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, bevy::ecs::message::Message)]
pub struct ReceiptNotification {
    /// The terminal receipt's stable identity.
    pub receipt: ReceiptId,
    /// The terminal receipt's operation family.
    pub family: super::operation::ReceiptFamily,
}

#[cfg(feature = "bevy")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptNotificationRegistrationError {
    /// The bridge has reached its declared finite watch capacity.
    Full,
    /// The same receipt is already watched by this bridge.
    AlreadyRegistered,
}

#[cfg(feature = "bevy")]
struct WatchedReceipt {
    receipt: ReceiptId,
    family: super::operation::ReceiptFamily,
    terminal_signal: Arc<AtomicBool>,
}

/// A bounded Bevy bridge that emits one message for each watched receipt's
/// terminal transition.
///
/// Register concrete receipts with their `watch_terminal_notification` method,
/// add this resource and [`emit_terminal_notifications`] to an app, then read
/// [`ReceiptNotification`] with a Bevy `MessageReader`. A registration remains
/// until its terminal message has been emitted.
#[cfg(feature = "bevy")]
#[derive(bevy::ecs::prelude::Resource)]
pub struct ReceiptNotificationBridge {
    capacity: usize,
    watched: Vec<WatchedReceipt>,
}

#[cfg(feature = "bevy")]
impl ReceiptNotificationBridge {
    /// Creates a bridge with a finite number of receipt watches.
    pub fn try_new(capacity: usize) -> Result<Self, ReceiptNotificationRegistrationError> {
        let mut watched = Vec::new();
        watched
            .try_reserve_exact(capacity)
            .map_err(|_| ReceiptNotificationRegistrationError::Full)?;
        Ok(Self { capacity, watched })
    }

    fn watch<T, E>(
        &mut self,
        receipt: &Receipt<T, E>,
    ) -> Result<(), ReceiptNotificationRegistrationError> {
        let operation = receipt.operation();
        let receipt_id = operation.receipt_id();
        if self
            .watched
            .iter()
            .any(|watched| watched.receipt == receipt_id)
        {
            return Err(ReceiptNotificationRegistrationError::AlreadyRegistered);
        }
        if self.watched.len() == self.capacity {
            return Err(ReceiptNotificationRegistrationError::Full);
        }
        self.watched.push(WatchedReceipt {
            receipt: receipt_id,
            family: operation.family(),
            terminal_signal: operation.terminal_signal(),
        });
        Ok(())
    }
}

/// Emits one [`ReceiptNotification`] for each watched receipt that becomes
/// terminal. Add this system to a Bevy schedule after the runtime driver.
#[cfg(feature = "bevy")]
pub fn emit_terminal_notifications(
    mut bridge: bevy::ecs::prelude::ResMut<ReceiptNotificationBridge>,
    mut notifications: bevy::ecs::message::MessageWriter<ReceiptNotification>,
) {
    bridge.watched.retain(|watched| {
        if watched.terminal_signal.load(Ordering::Acquire) {
            notifications.write(ReceiptNotification {
                receipt: watched.receipt,
                family: watched.family,
            });
            false
        } else {
            true
        }
    });
}

macro_rules! concrete_receipt {
    ($name:ident, $ready:ty, $error:ty, cancellable) => {
        #[doc = concat!("The public ", stringify!($name), " facade handle.")]
        pub struct $name(Receipt<$ready, $error>);

        impl Clone for $name {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl $name {
            pub(crate) fn operation(&self) -> Arc<Operation<$ready, $error>> {
                self.0.operation()
            }

            /// Returns the family-specialized nonblocking receipt state.
            #[must_use]
            pub fn poll(&self) -> ReceiptState<$ready, $error> {
                self.0.poll()
            }

            /// Requests cancellation according to this family's policy.
            pub fn cancel(&self) -> super::operation::CancelResult {
                self.0.cancel()
            }

            /// Registers this concrete receipt for a once-only terminal Bevy message.
            #[cfg(feature = "bevy")]
            pub fn watch_terminal_notification(
                &self,
                bridge: &mut ReceiptNotificationBridge,
            ) -> Result<(), ReceiptNotificationRegistrationError> {
                bridge.watch(&self.0)
            }
        }
    };
    ($name:ident, $ready:ty, $error:ty, terminal_only) => {
        #[doc = concat!("The public ", stringify!($name), " facade handle.")]
        pub struct $name(Receipt<$ready, $error>);

        impl Clone for $name {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl $name {
            pub(crate) fn operation(&self) -> Arc<Operation<$ready, $error>> {
                self.0.operation()
            }

            /// Returns the family-specialized nonblocking receipt state.
            #[must_use]
            pub fn poll(&self) -> ReceiptState<$ready, $error> {
                self.0.poll()
            }

            /// Registers this concrete receipt for a once-only terminal Bevy message.
            #[cfg(feature = "bevy")]
            pub fn watch_terminal_notification(
                &self,
                bridge: &mut ReceiptNotificationBridge,
            ) -> Result<(), ReceiptNotificationRegistrationError> {
                bridge.watch(&self.0)
            }
        }
    };
}

concrete_receipt!(GenesisReceipt, GenesisReady, GenesisError, terminal_only);
concrete_receipt!(TickReceipt, TickConfirmed, FailedNoAdvance, cancellable);
concrete_receipt!(InterestReceipt, InterestApplied, InterestError, cancellable);
concrete_receipt!(QueryReceipt, QueryResult, QueryUnavailable, cancellable);
concrete_receipt!(
    ObservationResnapshotReceipt,
    ObservationResnapshot,
    ObservationSnapshotError,
    cancellable
);
concrete_receipt!(
    CheckpointReceipt,
    CheckpointCommitted,
    CheckpointError,
    cancellable
);
concrete_receipt!(
    CorrectionReceipt,
    CorrectionCommitted,
    CorrectionError,
    cancellable
);
concrete_receipt!(RestoreReceipt, RestoreReady, RestoreError, cancellable);
concrete_receipt!(ReplayReceipt, ReplayCompleted, ReplayFailure, cancellable);
concrete_receipt!(RecoveryReceipt, Recovered, RecoveryError, cancellable);
concrete_receipt!(
    ShutdownReceipt,
    ShutdownReport,
    ShutdownError,
    terminal_only
);

macro_rules! admit_concrete_receipt {
    ($method:ident, $name:ident, $ready:ty, $error:ty, $family:ident) => {
        impl TerminalCache<$ready, $error> {
            pub(crate) fn $method(
                self: &Arc<Self>,
                receipt: ReceiptId,
                generation: DeviceGeneration,
                result_bytes: u64,
            ) -> Result<$name, ResultBackpressure> {
                self.admit(
                    receipt,
                    generation,
                    ReceiptPolicy::for_family(super::operation::ReceiptFamily::$family),
                    result_bytes,
                )
                .map($name)
            }
        }
    };
}

admit_concrete_receipt!(
    admit_genesis,
    GenesisReceipt,
    GenesisReady,
    GenesisError,
    Genesis
);
admit_concrete_receipt!(
    admit_tick,
    TickReceipt,
    TickConfirmed,
    FailedNoAdvance,
    Tick
);
admit_concrete_receipt!(
    admit_interest,
    InterestReceipt,
    InterestApplied,
    InterestError,
    Interest
);
admit_concrete_receipt!(
    admit_query,
    QueryReceipt,
    QueryResult,
    QueryUnavailable,
    Query
);
admit_concrete_receipt!(
    admit_observation_resnapshot,
    ObservationResnapshotReceipt,
    ObservationResnapshot,
    ObservationSnapshotError,
    ObservationResnapshot
);
admit_concrete_receipt!(
    admit_checkpoint,
    CheckpointReceipt,
    CheckpointCommitted,
    CheckpointError,
    Checkpoint
);
admit_concrete_receipt!(
    admit_correction,
    CorrectionReceipt,
    CorrectionCommitted,
    CorrectionError,
    Correction
);
admit_concrete_receipt!(
    admit_restore,
    RestoreReceipt,
    RestoreReady,
    RestoreError,
    Restore
);
admit_concrete_receipt!(
    admit_replay,
    ReplayReceipt,
    ReplayCompleted,
    ReplayFailure,
    Replay
);
admit_concrete_receipt!(
    admit_recovery,
    RecoveryReceipt,
    Recovered,
    RecoveryError,
    Recovery
);
admit_concrete_receipt!(
    admit_shutdown,
    ShutdownReceipt,
    ShutdownReport,
    ShutdownError,
    Shutdown
);
