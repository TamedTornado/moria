//! Shared receipt handles and bounded terminal-result retention.

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::{
    canonical::{DeviceGeneration, ReceiptId, VolumeId, VolumeRevision},
    facade::{BoundedVec, MissingRange, ResourceBudgetField},
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

struct CacheState<T, E> {
    entries: VecDeque<Arc<Operation<T, E>>>,
    operations: VecDeque<std::sync::Weak<Operation<T, E>>>,
    used_records: u32,
    used_bytes: u64,
}

/// A count- and byte-bounded cache of terminal operation records.
///
/// Each admission reserves one record and its declared worst-case terminal
/// bytes. A terminal cache entry is evicted only when no receipt clone retains
/// it, so consumers holding old receipts exert explicit backpressure.
pub struct TerminalCache<T, E> {
    record_capacity: u32,
    byte_capacity: u64,
    current_generation: Arc<Mutex<Option<DeviceGeneration>>>,
    state: Mutex<CacheState<T, E>>,
}

impl<T, E> TerminalCache<T, E> {
    /// Creates a finite cache. Its index capacity is reserved before use.
    pub fn try_new(
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
            state: Mutex::new(CacheState {
                entries,
                operations,
                used_records: 0,
                used_bytes: 0,
            }),
        }))
    }

    /// Admits an operation after reserving one terminal record and all result bytes.
    pub fn admit(
        self: &Arc<Self>,
        receipt: ReceiptId,
        generation: DeviceGeneration,
        policy: ReceiptPolicy,
        result_bytes: u64,
    ) -> Result<Receipt<T, E>, ResultBackpressure> {
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
            let operation = Arc::new(Operation::new(
                receipt,
                generation,
                policy,
                Arc::clone(&self.current_generation),
                Arc::downgrade(self),
                result_bytes,
            ));
            let mut state = self.state.lock().expect("terminal cache mutex poisoned");
            state
                .operations
                .retain(|operation| operation.strong_count() != 0);
            state.operations.push_back(Arc::downgrade(&operation));
            drop(current_generation);
            return Ok(Receipt { operation });
        }
    }

    /// Changes the publishing generation and fails every pending earlier receipt.
    ///
    /// `old_generation_failure` must be the caller's typed device-loss or
    /// no-advance failure. Already terminal receipts remain unchanged, and late
    /// completion attempts from an old generation are still rejected.
    pub fn set_current_generation(&self, generation: DeviceGeneration, old_generation_failure: E)
    where
        E: Clone,
    {
        let mut current_generation = self
            .current_generation
            .lock()
            .expect("device generation mutex poisoned");
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
            operation.terminalize_old_generation(old_generation_failure.clone());
        }
    }

    pub(super) fn retain_terminal(&self, operation: Arc<Operation<T, E>>) {
        let mut state = self.state.lock().expect("terminal cache mutex poisoned");
        state.entries.push_back(operation);
    }

    pub(super) fn release(&self, result_bytes: u64) {
        let mut state = self.state.lock().expect("terminal cache mutex poisoned");
        state.used_records -= 1;
        state.used_bytes -= result_bytes;
    }
}

fn has_capacity<T, E>(
    state: &CacheState<T, E>,
    record_capacity: u32,
    byte_capacity: u64,
    result_bytes: u64,
) -> bool {
    state.used_records < record_capacity
        && result_bytes <= byte_capacity.saturating_sub(state.used_bytes)
}

/// A cloneable, pollable receipt sharing one operation record.
pub struct Receipt<T, E> {
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
    /// Returns an operation view for runtime-owned progress and completion driving.
    #[must_use]
    pub fn operation(&self) -> Arc<Operation<T, E>> {
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

    /// Returns the terminal Bevy notification payload, if this receipt has completed.
    #[cfg(feature = "bevy")]
    #[must_use]
    pub fn terminal_notification(&self) -> Option<ReceiptNotification> {
        match self.poll() {
            ReceiptState::Pending(_) => None,
            ReceiptState::Ready(_) | ReceiptState::Failed(_) | ReceiptState::Cancelled(_) => {
                Some(ReceiptNotification {
                    receipt: self.operation.receipt_id(),
                    family: self.operation.family(),
                })
            }
        }
    }
}

/// A Bevy notification that a receipt family has reached a terminal state.
///
/// The notification carries identity only; consumers poll their retained
/// concrete receipt to obtain the shared terminal value.
#[cfg(feature = "bevy")]
#[derive(Clone, Copy, Debug, bevy::ecs::message::Message)]
pub struct ReceiptNotification {
    /// The terminal receipt's stable identity.
    pub receipt: ReceiptId,
    /// The terminal receipt's operation family.
    pub family: super::operation::ReceiptFamily,
}

macro_rules! concrete_receipt {
    ($name:ident, cancellable) => {
        #[doc = concat!("The public ", stringify!($name), " facade handle.")]
        pub struct $name<T, E>(Receipt<T, E>);

        impl<T, E> Clone for $name<T, E> {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl<T, E> From<Receipt<T, E>> for $name<T, E> {
            fn from(receipt: Receipt<T, E>) -> Self {
                Self(receipt)
            }
        }

        impl<T, E> $name<T, E> {
            /// Returns the family-specialized nonblocking receipt state.
            #[must_use]
            pub fn poll(&self) -> ReceiptState<T, E> {
                self.0.poll()
            }

            /// Requests cancellation according to this family's policy.
            pub fn cancel(&self) -> super::operation::CancelResult {
                self.0.cancel()
            }

            /// Returns the terminal Bevy notification payload, if complete.
            #[cfg(feature = "bevy")]
            #[must_use]
            pub fn terminal_notification(&self) -> Option<ReceiptNotification> {
                self.0.terminal_notification()
            }
        }
    };
    ($name:ident, terminal_only) => {
        #[doc = concat!("The public ", stringify!($name), " facade handle.")]
        pub struct $name<T, E>(Receipt<T, E>);

        impl<T, E> Clone for $name<T, E> {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl<T, E> From<Receipt<T, E>> for $name<T, E> {
            fn from(receipt: Receipt<T, E>) -> Self {
                Self(receipt)
            }
        }

        impl<T, E> $name<T, E> {
            /// Returns the family-specialized nonblocking receipt state.
            #[must_use]
            pub fn poll(&self) -> ReceiptState<T, E> {
                self.0.poll()
            }

            /// Returns the terminal Bevy notification payload, if complete.
            #[cfg(feature = "bevy")]
            #[must_use]
            pub fn terminal_notification(&self) -> Option<ReceiptNotification> {
                self.0.terminal_notification()
            }
        }
    };
}

concrete_receipt!(GenesisReceipt, terminal_only);
concrete_receipt!(TickReceipt, cancellable);
concrete_receipt!(InterestReceipt, cancellable);
concrete_receipt!(QueryReceipt, cancellable);
concrete_receipt!(ObservationResnapshotReceipt, cancellable);
concrete_receipt!(CheckpointReceipt, cancellable);
concrete_receipt!(CorrectionReceipt, cancellable);
concrete_receipt!(RestoreReceipt, cancellable);
concrete_receipt!(ReplayReceipt, cancellable);
concrete_receipt!(RecoveryReceipt, cancellable);
concrete_receipt!(ShutdownReceipt, terminal_only);
