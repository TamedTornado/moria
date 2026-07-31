//! Bounded, pollable operation lifecycle primitives.

mod operation;
mod receipt;

pub use operation::{
    CancelResult, CancellationCutoff, Operation, OperationPhase, ReceiptFamily, ReceiptPolicy,
    TransitionError,
};
pub use receipt::{
    CancelledOperation, CheckpointReceipt, CorrectionReceipt, GenesisReceipt, InterestReceipt,
    MinimumRevisionGap, ObservationResnapshotReceipt, OperationProgress, ProgressBlocker,
    QueryReadinessReason, QueryReceipt, Receipt, ReceiptState, RecoveryReceipt, ReplayReceipt,
    RestoreReceipt, ResultBackpressure, ShutdownReceipt, TerminalCache, TickReceipt,
};

#[cfg(feature = "bevy")]
pub use receipt::ReceiptNotification;

#[cfg(test)]
mod tests;
