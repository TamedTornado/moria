//! The minimal public admission path for world genesis.

use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use crate::{
    canonical::{CanonicalHash, DeviceGeneration, ReceiptId, Tick, WorldId},
    runtime::{GenesisReceipt, ResultBackpressure},
};

use super::{
    AuthorityStatus, FrontierPosition, FrontierSummary, GenesisReady, ReplayStreamKey,
    ReplayStreamPosition,
};

use crate::runtime::TerminalCache;

const GENESIS_RESULT_BYTES: u64 = 96;

/// A client that admits bounded public world operations.
///
/// `terminal_records` and `terminal_bytes` bound retained receipt results.
/// Admission returns [`ResultBackpressure::Full`] once outstanding receipts
/// exhaust either bound.
pub struct MoriaClient {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    next_receipt: AtomicU64,
    generation: DeviceGeneration,
    genesis: Arc<TerminalCache<GenesisReady, crate::facade::GenesisError>>,
}

impl MoriaClient {
    /// Creates a client with an explicit bounded terminal-result cache.
    ///
    /// # Errors
    ///
    /// Returns [`ResultBackpressure::AllocationFailed`] if the finite cache
    /// index cannot be reserved.
    pub fn try_new(terminal_records: u32, terminal_bytes: u64) -> Result<Self, ResultBackpressure> {
        Ok(Self {
            inner: Arc::new(ClientInner {
                next_receipt: AtomicU64::new(1),
                generation: DeviceGeneration::from_raw(1),
                genesis: TerminalCache::try_new(terminal_records, terminal_bytes)?,
            }),
        })
    }

    /// Begins the one-shot genesis admission workflow for `world`.
    #[must_use]
    pub fn begin_world(&self, world: WorldId, replay_stream: ReplayStreamKey) -> WorldBuilder {
        WorldBuilder {
            client: Arc::clone(&self.inner),
            world,
            replay_stream,
        }
    }
}

/// An un-published world whose genesis can be admitted exactly once.
pub struct WorldBuilder {
    client: Arc<ClientInner>,
    world: WorldId,
    replay_stream: ReplayStreamKey,
}

impl WorldBuilder {
    /// Admits genesis and returns its concrete pollable receipt.
    ///
    /// The current bounded scaffold completes genesis synchronously after the
    /// accepted lifecycle reaches replay-header durability. Consumers still
    /// use the same receipt and notification path as asynchronous drivers.
    ///
    /// # Errors
    ///
    /// Returns [`ResultBackpressure::Full`] when retained terminal receipts
    /// consume the configured record or byte capacity.
    pub fn publish_genesis(self) -> Result<GenesisReceipt, ResultBackpressure> {
        let receipt_id =
            ReceiptId::from_raw(self.client.next_receipt.fetch_add(1, Ordering::Relaxed));
        let receipt = self.client.genesis.admit_genesis(
            receipt_id,
            self.client.generation,
            GENESIS_RESULT_BYTES,
        )?;
        let operation = receipt.operation();
        operation
            .advance(crate::runtime::OperationPhase::Materializing)
            .expect("genesis lifecycle is fixed by the public facade");
        operation
            .advance(crate::runtime::OperationPhase::Submitting)
            .expect("genesis lifecycle is fixed by the public facade");
        operation
            .advance(crate::runtime::OperationPhase::ExportingReplayHeader)
            .expect("genesis lifecycle is fixed by the public facade");
        operation
            .complete_ready(GenesisReady {
                frontier: FrontierSummary {
                    world: self.world,
                    position: FrontierPosition::Genesis,
                    root_hash: CanonicalHash::from_bytes([0; 32]),
                    status: AuthorityStatus::DiagnosticCandidate,
                },
                next_tick: Tick::from_raw(0),
                replay: ReplayStreamPosition {
                    stream: self.replay_stream,
                    sequence: 0,
                },
            })
            .expect("genesis lifecycle reaches its terminal phase");
        Ok(receipt)
    }
}
