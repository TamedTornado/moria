//! Immutable telemetry evidence and validation for feasibility gates.

mod reports;
mod runtime;

pub use reports::{
    BuildProfile, Distribution, ForestFeasibilityReport, MachineProfile, MutationFeasibilityReport,
    MutationWorkloadEvidence, MutationWorkloadRole, ObjectIndexEvidence, QueryCostEvidence,
    ReportValidationError, TrustedGateIdentity, WorstEditTargetEvidence,
};
pub use runtime::{
    ActiveCounts, EditObservation, GraphicsMemoryEstimate, QueueDepths, WorldTelemetryRead,
};

pub(crate) use runtime::{WorldTelemetryState, advance_frame_index};
