//! Bounded, pollable operation lifecycle primitives.

mod operation;
mod receipt;

pub use operation::{
    CancelResult, CancellationCutoff, Operation, OperationPhase, ReceiptFamily, ReceiptPolicy,
    TransitionError,
};
pub use receipt::{
    CancelledOperation, MinimumRevisionGap, OperationProgress, ProgressBlocker,
    QueryReadinessReason, Receipt, ReceiptState, ResultBackpressure, TerminalCache,
};

#[cfg(test)]
mod tests;
