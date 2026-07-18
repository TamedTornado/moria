//! Library-owned world opening lifecycle and public access guards.

use std::{error::Error, fmt};

use bevy::{ecs::system::SystemParam, prelude::*};

/// The opening state of the library-owned world.
///
/// This is deliberately a resource rather than a Bevy [`State`]: consumers may
/// integrate it with their own application state machine.
#[derive(Clone, Debug, Default, Eq, PartialEq, Resource)]
pub enum WorldLifecycle {
    #[default]
    Uninitialized,
    Loading,
    Ready,
    Failed(WorldOpenError),
}

impl WorldLifecycle {
    /// Requests the one legal opening transition.
    pub fn start_loading(&mut self) -> Result<(), WorldLifecycleInvariantError> {
        self.apply(WorldLifecycleTransition::StartLoading)
    }

    /// Marks a successfully opened world ready for ordinary public access.
    pub fn mark_ready(&mut self) -> Result<(), WorldLifecycleInvariantError> {
        self.apply(WorldLifecycleTransition::MarkReady)
    }

    /// Makes an opening or fatal runtime failure terminal.
    pub fn fail(&mut self, error: WorldOpenError) -> Result<(), WorldLifecycleInvariantError> {
        self.apply(WorldLifecycleTransition::Fail(error))
    }

    /// Applies a lifecycle request, preserving the state when it is illegal.
    pub fn apply(
        &mut self,
        transition: WorldLifecycleTransition,
    ) -> Result<(), WorldLifecycleInvariantError> {
        let next = match (&*self, &transition) {
            (Self::Uninitialized, WorldLifecycleTransition::StartLoading) => Self::Loading,
            (Self::Loading, WorldLifecycleTransition::MarkReady) => Self::Ready,
            (Self::Loading | Self::Ready, WorldLifecycleTransition::Fail(error)) => {
                Self::Failed(error.clone())
            }
            _ => {
                return Err(WorldLifecycleInvariantError {
                    current: self.phase(),
                    requested: transition,
                });
            }
        };
        *self = next;
        Ok(())
    }

    /// Returns whether ordinary query and edit access is permitted.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns the state without exposing a terminal failure's details.
    #[must_use]
    pub const fn phase(&self) -> WorldLifecyclePhase {
        match self {
            Self::Uninitialized => WorldLifecyclePhase::Uninitialized,
            Self::Loading => WorldLifecyclePhase::Loading,
            Self::Ready => WorldLifecyclePhase::Ready,
            Self::Failed(_) => WorldLifecyclePhase::Failed,
        }
    }
}

/// A lifecycle request accepted only in its documented source state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorldLifecycleTransition {
    StartLoading,
    MarkReady,
    Fail(WorldOpenError),
}

/// A failure kind that prevents a world from becoming ready.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorldOpenError {
    Asset(String),
    ManifestIdentity(String),
    InvalidConfig(String),
    GenerationContract(String),
    Save(String),
    InitialActivation(String),
}

impl fmt::Display for WorldOpenError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Asset(reason) => write!(formatter, "asset failure: {reason}"),
            Self::ManifestIdentity(reason) => {
                write!(formatter, "manifest identity failure: {reason}")
            }
            Self::InvalidConfig(reason) => write!(formatter, "invalid configuration: {reason}"),
            Self::GenerationContract(reason) => {
                write!(formatter, "generation contract failure: {reason}")
            }
            Self::Save(reason) => write!(formatter, "save failure: {reason}"),
            Self::InitialActivation(reason) => {
                write!(formatter, "initial activation failure: {reason}")
            }
        }
    }
}

impl Error for WorldOpenError {}

/// The lifecycle stage recorded with an ignored illegal request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorldLifecyclePhase {
    Uninitialized,
    Loading,
    Ready,
    Failed,
}

/// An ignored lifecycle request that would violate the state invariant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorldLifecycleInvariantError {
    pub current: WorldLifecyclePhase,
    pub requested: WorldLifecycleTransition,
}

impl fmt::Display for WorldLifecycleInvariantError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid world lifecycle transition from {:?}: {:?}",
            self.current, self.requested
        )
    }
}

impl Error for WorldLifecycleInvariantError {}

/// A public edit request placeholder guarded by [`WorldEditWrite`].
///
/// The mutation pipeline supplies command fields and admission semantics; this
/// lifecycle slice owns only the readiness guard.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorldEditCommand;

/// A synchronous rejection from the public edit entry point.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubmitError {
    NotReady,
}

/// The public mutation entry point.
#[derive(SystemParam)]
pub struct WorldEditWrite<'w, 's> {
    lifecycle: Res<'w, WorldLifecycle>,
    _system_state: Local<'s, ()>,
}

impl WorldEditWrite<'_, '_> {
    /// Rejects edits until the opening lifecycle has reached [`WorldLifecycle::Ready`].
    pub fn submit(&mut self, _command: WorldEditCommand) -> Result<(), SubmitError> {
        if self.lifecycle.is_ready() {
            Ok(())
        } else {
            Err(SubmitError::NotReady)
        }
    }
}
