//! Constant-time runtime telemetry observations.

use bevy::{ecs::system::SystemParam, prelude::*};

/// Active world representation counts.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ActiveCounts {
    pub bricks: u32,
    pub meshes: u32,
    pub objects: u32,
}

/// Bounded pipeline queue depths and telemetry loss count.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QueueDepths {
    pub extraction: u32,
    pub installation: u32,
    pub render: u32,
    pub dropped_edit_observations: u64,
}

/// Application-owned graphics allocation ledger.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraphicsMemoryEstimate {
    pub peak_bytes: u64,
    pub end_bytes: u64,
}

/// One bounded edit lifecycle observation, without edit values or store access.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EditObservation {
    pub request_id: u64,
    pub revision: u64,
    pub frame: u64,
}

/// Private runtime telemetry state.
#[derive(Resource)]
pub(crate) struct WorldTelemetryState {
    active_counts: ActiveCounts,
    queue_depths: QueueDepths,
    graphics_allocations: GraphicsMemoryEstimate,
    frame_index: u64,
    observations: Vec<EditObservation>,
}

impl Default for WorldTelemetryState {
    fn default() -> Self {
        Self {
            active_counts: ActiveCounts::default(),
            queue_depths: QueueDepths::default(),
            graphics_allocations: GraphicsMemoryEstimate::default(),
            frame_index: 0,
            observations: Vec::with_capacity(256),
        }
    }
}

impl WorldTelemetryState {
    #[allow(
        dead_code,
        reason = "edit lifecycle integration records these observations after mutation is installed"
    )]
    pub(crate) fn record_edit_observation(&mut self, observation: EditObservation) {
        if self.observations.len() == 256 {
            self.observations.remove(0);
            self.queue_depths.dropped_edit_observations = self
                .queue_depths
                .dropped_edit_observations
                .checked_add(1)
                .expect("dropped telemetry count cannot wrap");
        }
        self.observations.push(observation);
    }
}

/// Read-only constant-time telemetry access for consumers.
#[derive(SystemParam)]
pub struct WorldTelemetryRead<'w, 's> {
    state: Res<'w, WorldTelemetryState>,
    _system_state: Local<'s, ()>,
}

impl WorldTelemetryRead<'_, '_> {
    #[must_use]
    pub fn active_counts(&self) -> ActiveCounts {
        self.state.active_counts
    }
    #[must_use]
    pub fn queue_depths(&self) -> QueueDepths {
        self.state.queue_depths
    }
    #[must_use]
    pub fn graphics_allocations(&self) -> GraphicsMemoryEstimate {
        self.state.graphics_allocations
    }
    #[must_use]
    pub fn frame_index(&self) -> u64 {
        self.state.frame_index
    }
    #[must_use]
    pub fn edit_observations(&self) -> &[EditObservation] {
        &self.state.observations
    }
}

pub(crate) fn advance_frame_index(mut state: ResMut<WorldTelemetryState>) {
    state.frame_index = state
        .frame_index
        .checked_add(1)
        .expect("frame index cannot wrap");
}

#[cfg(test)]
mod tests {
    use super::{EditObservation, WorldTelemetryState};

    #[test]
    fn ring_keeps_chronological_latest_256_observations_and_counts_drops() {
        let mut state = WorldTelemetryState::default();
        for request_id in 0..257 {
            state.record_edit_observation(EditObservation {
                request_id,
                ..Default::default()
            });
        }
        assert_eq!(state.observations.len(), 256);
        assert_eq!(state.observations.first().unwrap().request_id, 1);
        assert_eq!(state.observations.last().unwrap().request_id, 256);
        assert_eq!(state.queue_depths.dropped_edit_observations, 1);
    }
}
