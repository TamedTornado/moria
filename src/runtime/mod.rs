//! Bounded, pollable operation lifecycle primitives.

mod operation;
mod receipt;

pub use operation::{
    CancelResult, CancellationCutoff, Operation, OperationPhase, ReceiptFamily, ReceiptPolicy,
    TransitionError,
};
pub use receipt::{
    CancelledOperation, OperationProgress, ProgressBlocker, Receipt, ReceiptState,
    ResultBackpressure, TerminalCache,
};

#[cfg(test)]
mod tests;
