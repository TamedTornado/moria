//! Bounded, pollable operation lifecycle primitives.

#[allow(dead_code)] // Runtime-owned drivers are wired by product features, not this scaffold.
mod operation;
#[allow(dead_code)] // Concrete admission is intentionally unavailable to public consumers.
mod receipt;

pub use operation::{CancelResult, OperationPhase, ReceiptFamily};
pub use receipt::{
    CancelledOperation, CheckpointReceipt, CorrectionReceipt, GenesisReceipt, InterestReceipt,
    MinimumRevisionGap, ObservationResnapshotReceipt, OperationProgress, ProgressBlocker,
    QueryReadinessReason, QueryReceipt, ReceiptState, RecoveryReceipt, ReplayReceipt,
    RestoreReceipt, ShutdownReceipt, TickReceipt,
};

#[cfg(feature = "bevy")]
pub use receipt::{
    ReceiptNotification, ReceiptNotificationBridge, ReceiptNotificationRegistrationError,
    emit_terminal_notifications,
};

#[cfg(test)]
mod tests;
