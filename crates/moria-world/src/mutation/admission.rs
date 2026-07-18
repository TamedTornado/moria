//! Constant-work admission and bounded request reservations.

use std::collections::{BTreeSet, VecDeque};

use bevy::prelude::*;

use crate::{
    AIR, MaterialRegistry, MutationConfig, VOXEL_EDGE_Q8, VoxelCoord, WATER, WorldLifecycle,
    WorldLifecyclePhase, WorldPointQ8,
};

use super::api::{EditAccepted, EditExecution, EditOperation, SubmitError, WorldEditCommand};

const REGION_MIN_XZ_VOXEL: i32 = -2_000;
const REGION_MAX_XZ_VOXEL_EXCLUSIVE: i32 = 2_000;
const REGION_MIN_Y_VOXEL: i32 = -512;
const REGION_MAX_Y_VOXEL_EXCLUSIVE: i32 = 512;

/// Private fixed-capacity admission and lifecycle reservation state.
#[derive(Resource)]
pub(crate) struct AdmissionState {
    config: MutationConfig,
    materials: MaterialRegistry,
    pending: VecDeque<Reservation>,
    request_ids: BTreeSet<u64>,
}

#[allow(
    dead_code,
    reason = "the scheduler consumes these reservations in the following mutation slice"
)]
struct Reservation {
    command: WorldEditCommand,
    estimated_bricks: u32,
    submitted_frame: u64,
}

impl Default for AdmissionState {
    fn default() -> Self {
        Self::new(MutationConfig::default(), MaterialRegistry::default())
    }
}

impl AdmissionState {
    pub(crate) fn new(config: MutationConfig, materials: MaterialRegistry) -> Self {
        let capacity = usize::from(config.max_queued_edits);
        Self {
            config,
            materials,
            pending: VecDeque::with_capacity(capacity),
            request_ids: BTreeSet::new(),
        }
    }

    pub(crate) fn admit(
        &mut self,
        command: WorldEditCommand,
        lifecycle: &WorldLifecycle,
        submitted_frame: u64,
    ) -> Result<EditAccepted, SubmitError> {
        match lifecycle.phase() {
            WorldLifecyclePhase::Ready => {}
            WorldLifecyclePhase::Loading => return Err(SubmitError::LoadInProgress),
            WorldLifecyclePhase::Uninitialized | WorldLifecyclePhase::Failed => {
                return Err(SubmitError::NotReady);
            }
        }
        if self.request_ids.contains(&command.request_id) {
            return Err(SubmitError::DuplicateRequestId);
        }

        let estimated_bricks = self.validate(&command)?;
        if self.pending.len() == usize::from(self.config.max_queued_edits) {
            return Err(SubmitError::QueueFull);
        }

        // Both collections are bounded by `max_queued_edits`; reserve their
        // accounting before publishing acceptance.
        let inserted = self.request_ids.insert(command.request_id);
        debug_assert!(
            inserted,
            "duplicate request IDs are rejected before reservation"
        );
        self.pending.push_back(Reservation {
            command: command.clone(),
            estimated_bricks,
            submitted_frame,
        });
        Ok(EditAccepted {
            request_id: command.request_id,
            submitted_frame,
            estimated_bricks,
        })
    }

    fn validate(&self, command: &WorldEditCommand) -> Result<u32, SubmitError> {
        let estimated_bricks = match &command.operation {
            EditOperation::DigSphere {
                center,
                radius_q8,
                strength,
            } => {
                self.validate_strength(*strength)?;
                self.estimate_sphere_bricks(*center, *radius_q8)?
            }
            EditOperation::PlaceSphere {
                center,
                radius_q8,
                strength,
                material,
            } => {
                self.validate_strength(*strength)?;
                self.validate_material(*material)?;
                self.estimate_sphere_bricks(*center, *radius_q8)?
            }
            EditOperation::DigBox {
                min,
                max_exclusive,
                strength,
            } => {
                self.validate_strength(*strength)?;
                self.estimate_box_bricks(*min, *max_exclusive)?
            }
            EditOperation::PlaceBox {
                min,
                max_exclusive,
                strength,
                material,
            } => {
                self.validate_strength(*strength)?;
                self.validate_material(*material)?;
                self.estimate_box_bricks(*min, *max_exclusive)?
            }
        };
        match command.execution {
            EditExecution::Atomic
                if estimated_bricks > u32::from(self.config.max_atomic_bricks) =>
            {
                Err(SubmitError::AtomicWorkLimitExceeded)
            }
            EditExecution::Progressive if estimated_bricks > self.config.max_progressive_bricks => {
                Err(SubmitError::ProgressiveWorkLimitExceeded)
            }
            EditExecution::Atomic | EditExecution::Progressive => Ok(estimated_bricks),
        }
    }

    fn validate_strength(&self, strength: u8) -> Result<(), SubmitError> {
        (strength > 0)
            .then_some(())
            .ok_or(SubmitError::InvalidInput)
    }

    fn validate_material(&self, material: crate::MaterialId) -> Result<(), SubmitError> {
        (material != AIR
            && material != WATER
            && self
                .materials
                .materials
                .iter()
                .any(|entry| entry.id == material))
        .then_some(())
        .ok_or(SubmitError::InvalidInput)
    }

    fn estimate_sphere_bricks(
        &self,
        center: WorldPointQ8,
        radius_q8: u16,
    ) -> Result<u32, SubmitError> {
        if radius_q8 < self.config.min_radius_q8 || radius_q8 > self.config.max_radius_q8 {
            return Err(SubmitError::InvalidBounds);
        }
        center
            .to_voxel_coord()
            .map_err(|_| SubmitError::InvalidBounds)?;
        let radius = i32::from(radius_q8);
        let min = WorldPointQ8::new(
            center
                .x
                .checked_sub(radius)
                .ok_or(SubmitError::InvalidBounds)?,
            center
                .y
                .checked_sub(radius)
                .ok_or(SubmitError::InvalidBounds)?,
            center
                .z
                .checked_sub(radius)
                .ok_or(SubmitError::InvalidBounds)?,
        );
        let max = WorldPointQ8::new(
            center
                .x
                .checked_add(radius)
                .ok_or(SubmitError::InvalidBounds)?,
            center
                .y
                .checked_add(radius)
                .ok_or(SubmitError::InvalidBounds)?,
            center
                .z
                .checked_add(radius)
                .ok_or(SubmitError::InvalidBounds)?,
        );
        let min = VoxelCoord::new(
            (min.x.div_euclid(VOXEL_EDGE_Q8))
                .clamp(REGION_MIN_XZ_VOXEL, REGION_MAX_XZ_VOXEL_EXCLUSIVE - 1),
            (min.y.div_euclid(VOXEL_EDGE_Q8))
                .clamp(REGION_MIN_Y_VOXEL, REGION_MAX_Y_VOXEL_EXCLUSIVE - 1),
            (min.z.div_euclid(VOXEL_EDGE_Q8))
                .clamp(REGION_MIN_XZ_VOXEL, REGION_MAX_XZ_VOXEL_EXCLUSIVE - 1),
        );
        let max = VoxelCoord::new(
            (max.x.div_euclid(VOXEL_EDGE_Q8))
                .clamp(REGION_MIN_XZ_VOXEL, REGION_MAX_XZ_VOXEL_EXCLUSIVE - 1),
            (max.y.div_euclid(VOXEL_EDGE_Q8))
                .clamp(REGION_MIN_Y_VOXEL, REGION_MAX_Y_VOXEL_EXCLUSIVE - 1),
            (max.z.div_euclid(VOXEL_EDGE_Q8))
                .clamp(REGION_MIN_XZ_VOXEL, REGION_MAX_XZ_VOXEL_EXCLUSIVE - 1),
        );
        Self::brick_count(min, max)
    }

    fn estimate_box_bricks(
        &self,
        min: VoxelCoord,
        max_exclusive: VoxelCoord,
    ) -> Result<u32, SubmitError> {
        if min.x >= max_exclusive.x || min.y >= max_exclusive.y || min.z >= max_exclusive.z {
            return Err(SubmitError::InvalidBounds);
        }
        let min = VoxelCoord::new(
            min.x.max(REGION_MIN_XZ_VOXEL),
            min.y.max(REGION_MIN_Y_VOXEL),
            min.z.max(REGION_MIN_XZ_VOXEL),
        );
        let max = VoxelCoord::new(
            max_exclusive.x.min(REGION_MAX_XZ_VOXEL_EXCLUSIVE) - 1,
            max_exclusive.y.min(REGION_MAX_Y_VOXEL_EXCLUSIVE) - 1,
            max_exclusive.z.min(REGION_MAX_XZ_VOXEL_EXCLUSIVE) - 1,
        );
        if !min.is_in_region() || !max.is_in_region() {
            return Err(SubmitError::InvalidBounds);
        }
        Self::brick_count(min, max)
    }

    fn brick_count(min: VoxelCoord, max: VoxelCoord) -> Result<u32, SubmitError> {
        let min = min
            .to_brick_coord()
            .map_err(|_| SubmitError::InvalidBounds)?;
        let max = max
            .to_brick_coord()
            .map_err(|_| SubmitError::InvalidBounds)?;
        let x = u32::try_from(i32::from(max.x() - min.x()) + 1).expect("validated brick span");
        let y = u32::try_from(i32::from(max.y() - min.y()) + 1).expect("validated brick span");
        let z = u32::try_from(i32::from(max.z() - min.z()) + 1).expect("validated brick span");
        x.checked_mul(y)
            .and_then(|count| count.checked_mul(z))
            .ok_or(SubmitError::ProgressiveWorkLimitExceeded)
    }
}

#[cfg(test)]
mod tests {
    use super::AdmissionState;
    use crate::{
        EditExecution, EditOperation, MutationConfig, SubmitError, VoxelCoord, WorldEditCommand,
        WorldLifecycle,
    };

    #[test]
    fn progressive_limit_is_checked_without_enumerating_bricks() {
        let config = MutationConfig {
            max_progressive_bricks: 1,
            ..MutationConfig::default()
        };
        let mut state = AdmissionState::new(config, crate::MaterialRegistry::default());
        let mut lifecycle = WorldLifecycle::default();
        lifecycle.start_loading().unwrap();
        lifecycle.mark_ready().unwrap();

        assert_eq!(
            state.admit(
                WorldEditCommand {
                    request_id: 1,
                    operation: EditOperation::DigBox {
                        min: VoxelCoord::new(-16, -16, -16),
                        max_exclusive: VoxelCoord::new(16, 16, 16),
                        strength: 255,
                    },
                    execution: EditExecution::Progressive,
                },
                &lifecycle,
                0,
            ),
            Err(SubmitError::ProgressiveWorkLimitExceeded)
        );
    }
}
