//! Shared receipt handles and bounded terminal-result retention.

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::canonical::{DeviceGeneration, ReceiptId};

use super::operation::{Operation, OperationPhase, ReceiptPolicy};

/// The query-specific blocker carried by a pending operation.
///
/// Query readiness facts are owned by the query feature; this shared lifecycle
/// layer records only whether the currently pending operation has one.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgressBlocker {
    /// A query is waiting for a feature-owned readiness fact.
    Query,
}

/// The observable pending phase and optional query blocker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OperationProgress {
    /// The operation's current lifecycle phase.
    pub phase: OperationPhase,
    /// Present only for a query waiting on readiness.
    pub blocker: Option<ProgressBlocker>,
}

impl OperationProgress {
    pub(super) const fn new(phase: OperationPhase, blocker: Option<ProgressBlocker>) -> Self {
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
}

struct CacheState<T, E> {
    entries: VecDeque<Arc<Operation<T, E>>>,
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
        Ok(Arc::new(Self {
            record_capacity,
            byte_capacity,
            current_generation: Arc::new(Mutex::new(None)),
            state: Mutex::new(CacheState {
                entries,
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
        {
            let mut current_generation = self
                .current_generation
                .lock()
                .expect("device generation mutex poisoned");
            if current_generation.is_none() {
                *current_generation = Some(generation);
            }
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
            return Ok(Receipt { operation });
        }
    }

    /// Atomically changes the generation that may complete this cache's operations.
    ///
    /// Existing operations from an earlier generation remain drainable but cannot
    /// publish a ready or failed terminal result.
    pub fn set_current_generation(&self, generation: DeviceGeneration) {
        *self
            .current_generation
            .lock()
            .expect("device generation mutex poisoned") = Some(generation);
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
}
