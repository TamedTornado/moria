//! Public, noncanonical request and result owners.

mod bounded;
mod error;
mod receipt_result;

pub use bounded::{
    BoundedBytes, BoundedBytes64, BoundedOwnerError, BoundedPushRejected, BoundedUtf8, BoundedVec,
    BytesConstructionRejected, OwnedBytes, VecConstructionRejected,
};
pub use error::*;
pub(crate) use receipt_result::{
    CheckpointCommitted, CorrectionCommitted, GenesisReady, InterestApplied, ObservationResnapshot,
    QueryResult, Recovered, ReplayCompleted, RestoreReady, ShutdownReport, TickConfirmed,
};
