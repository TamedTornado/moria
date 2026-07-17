//! Immutable telemetry evidence and validation for feasibility gates.

mod reports;

pub use reports::{
    BuildProfile, Distribution, ForestFeasibilityReport, MachineProfile, MutationFeasibilityReport,
    MutationWorkloadEvidence, MutationWorkloadRole, ObjectIndexEvidence, QueryCostEvidence,
    ReportValidationError, WorstEditTargetEvidence,
};
