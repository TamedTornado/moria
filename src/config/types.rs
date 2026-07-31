//! Explicit consumer-authorized configuration and genesis inputs.

use crate::{
    canonical::{CanonicalOrder, ContractDigest, NewtypeValueError, Tick},
    facade::{CheckpointStoreId, ReplaySinkId},
};

use super::ResourceBudgets;

/// All canonical-contract digests bound into one world configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalContract {
    pub canonical_encoding: ContractDigest,
    pub arithmetic: ContractDigest,
    pub transition: ContractDigest,
    pub collision: ContractDigest,
    pub hash: ContractDigest,
    pub persistence: ContractDigest,
}

/// Rollback retention selected explicitly by the consumer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RollbackConfig {
    pub capacity_ticks: u32,
    pub log_ticks: u32,
    pub retain_outcome_bytes: bool,
}

impl Default for RollbackConfig {
    fn default() -> Self {
        Self {
            capacity_ticks: 32,
            log_ticks: 256,
            retain_outcome_bytes: true,
        }
    }
}

/// The default durable providers frozen for a world.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistenceConfig {
    pub default_checkpoint_store: CheckpointStoreId,
    pub replay_sink: ReplaySinkId,
}

/// Presentation policy frozen at genesis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PresentationConfig {
    pub enabled: bool,
    pub show_stale: bool,
    pub chunk_edge_cells: u16,
    pub maximum_lod: u8,
}

impl Default for PresentationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            show_stale: true,
            chunk_edge_cells: 32,
            maximum_lod: 6,
        }
    }
}

/// A consumer-defined semantic simulation-unit identity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SimulationUnitId([u8; 16]);

impl SimulationUnitId {
    /// Creates an exact simulation-unit identity.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Borrows the exact simulation-unit bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    /// Returns the exact simulation-unit bytes.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; 16] {
        self.0
    }
}

/// The fixed integer representation frozen into a world genesis.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlacementFixedFormat {
    fractional_bits: u8,
    cell_extent_raw: u32,
    simulation_unit: SimulationUnitId,
}

impl PlacementFixedFormat {
    /// Validates a placement representation without converting any value.
    ///
    /// # Errors
    ///
    /// Returns [`NewtypeValueError::OutOfRange`] if `fractional_bits` exceeds
    /// 16 or `cell_extent_raw` is zero or cannot fit a signed `i32`.
    pub const fn try_new(
        fractional_bits: u8,
        cell_extent_raw: u32,
        simulation_unit: SimulationUnitId,
    ) -> Result<Self, NewtypeValueError> {
        if fractional_bits > 16 || cell_extent_raw == 0 || cell_extent_raw > i32::MAX as u32 {
            return Err(NewtypeValueError::OutOfRange);
        }
        Ok(Self {
            fractional_bits,
            cell_extent_raw,
            simulation_unit,
        })
    }

    #[must_use]
    pub const fn fractional_bits(self) -> u8 {
        self.fractional_bits
    }

    #[must_use]
    pub const fn cell_extent_raw(self) -> u32 {
        self.cell_extent_raw
    }

    #[must_use]
    pub const fn simulation_unit(self) -> SimulationUnitId {
        self.simulation_unit
    }
}

/// Required per-world canonical placement input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorldGenesisConfig {
    pub placement: PlacementFixedFormat,
}

/// The closed execution mode selected for a Moria client.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionPolicy {
    ReplayGrade,
    Candidate { diagnostics: CandidateDiagnostics },
}

/// Candidate-only diagnostics which do not alter the public API shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CandidateDiagnostics {
    pub fault_once: Option<CandidateFaultOnce>,
}

/// One deterministic candidate fault injection coordinate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CandidateFaultOnce {
    pub tick: Tick,
    pub command_order: CanonicalOrder,
    pub stage: CandidateFaultStage,
}

/// The sole v1 candidate fault injection stage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CandidateFaultStage {
    AfterBrickConstructionBeforePublication,
}

/// The complete explicit configuration required to create a client.
///
/// This type deliberately has no `Default`: provider IDs, contract digests,
/// execution policy, and frozen placement all require consumer authority.
///
/// ```compile_fail
/// use moria::config::MoriaConfig;
///
/// let _ = MoriaConfig::default();
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MoriaConfig {
    pub canonical: CanonicalContract,
    pub budgets: ResourceBudgets,
    pub rollback: RollbackConfig,
    pub persistence: PersistenceConfig,
    pub presentation: PresentationConfig,
    pub execution: ExecutionPolicy,
}
