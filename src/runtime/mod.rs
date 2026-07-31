//! Bounded, pollable operation lifecycle primitives.

#[allow(dead_code)] // Runtime-owned drivers are wired by product features, not this scaffold.
mod operation;
#[allow(dead_code)] // Concrete admission is intentionally unavailable to public consumers.
mod receipt;

#[cfg(test)]
pub(crate) use operation::{CancelResult, OperationPhase, ReceiptFamily};
#[cfg(test)]
pub(crate) use receipt::{ProgressBlocker, QueryReadinessReason, ReceiptState};

#[cfg(all(test, feature = "bevy"))]
pub(crate) use receipt::{
    ReceiptNotification, ReceiptNotificationBridge, emit_terminal_notifications,
};

#[cfg(test)]
mod tests;
