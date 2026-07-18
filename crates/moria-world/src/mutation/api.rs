//! Consumer-facing edit protocol.

use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    MaterialId, VoxelCoord, WorldLifecycle, WorldPointQ8, mutation::admission::AdmissionState,
    telemetry::WorldTelemetryState,
};

/// One bounded world mutation request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorldEditCommand {
    pub request_id: u64,
    pub operation: EditOperation,
    pub execution: EditExecution,
}

/// Matter-only operations supported by the world substrate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EditOperation {
    DigSphere {
        center: WorldPointQ8,
        radius_q8: u16,
        strength: u8,
    },
    PlaceSphere {
        center: WorldPointQ8,
        radius_q8: u16,
        strength: u8,
        material: MaterialId,
    },
    DigBox {
        min: VoxelCoord,
        max_exclusive: VoxelCoord,
        strength: u8,
    },
    PlaceBox {
        min: VoxelCoord,
        max_exclusive: VoxelCoord,
        strength: u8,
        material: MaterialId,
    },
}

/// The visibility contract selected by the caller for an edit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditExecution {
    Atomic,
    Progressive,
}

/// A synchronous admission failure. Rejected requests emit no lifecycle messages.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubmitError {
    NotReady,
    LoadInProgress,
    DuplicateRequestId,
    InvalidBounds,
    InvalidInput,
    AtomicWorkLimitExceeded,
    ProgressiveWorkLimitExceeded,
    QueueFull,
}

/// Immutable acknowledgement emitted exactly once for every accepted request.
#[derive(Clone, Copy, Debug, Eq, Message, PartialEq)]
pub struct EditAccepted {
    pub request_id: u64,
    pub submitted_frame: u64,
    pub estimated_bricks: u32,
}

/// The typed reason carried by a rejected lifecycle record when a future
/// asynchronous protocol needs to report one.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditRejectReason {
    NotReady,
    LoadInProgress,
    DuplicateRequestId,
    InvalidBounds,
    InvalidInput,
    AtomicWorkLimitExceeded,
    ProgressiveWorkLimitExceeded,
    QueueFull,
}

impl From<SubmitError> for EditRejectReason {
    fn from(error: SubmitError) -> Self {
        match error {
            SubmitError::NotReady => Self::NotReady,
            SubmitError::LoadInProgress => Self::LoadInProgress,
            SubmitError::DuplicateRequestId => Self::DuplicateRequestId,
            SubmitError::InvalidBounds => Self::InvalidBounds,
            SubmitError::InvalidInput => Self::InvalidInput,
            SubmitError::AtomicWorkLimitExceeded => Self::AtomicWorkLimitExceeded,
            SubmitError::ProgressiveWorkLimitExceeded => Self::ProgressiveWorkLimitExceeded,
            SubmitError::QueueFull => Self::QueueFull,
        }
    }
}

/// A typed rejection record reserved for asynchronous protocol stages.
#[derive(Clone, Copy, Debug, Eq, Message, PartialEq)]
pub struct EditRejected {
    pub request_id: u64,
    pub reason: EditRejectReason,
}

/// The only public mutation entry point.
#[derive(SystemParam)]
pub struct WorldEditWrite<'w, 's> {
    lifecycle: Res<'w, WorldLifecycle>,
    admission: ResMut<'w, AdmissionState>,
    telemetry: Res<'w, WorldTelemetryState>,
    accepted: MessageWriter<'w, EditAccepted>,
    _system_state: Local<'s, ()>,
}

impl WorldEditWrite<'_, '_> {
    /// Validates and reserves one bounded edit request synchronously.
    pub fn submit(&mut self, command: WorldEditCommand) -> Result<(), SubmitError> {
        let accepted =
            self.admission
                .admit(command, &self.lifecycle, self.telemetry.frame_index())?;
        self.accepted.write(accepted);
        Ok(())
    }
}
