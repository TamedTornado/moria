//! Compact and detailed regenerated base representations for active bricks.

use crate::{
    BRICK_EDGE_VOXELS, BrickCoord, ProceduralClass, Voxel, VoxelCoord, WorldIdentity,
    classify_brick, evaluate_base_voxel,
};

pub(super) const BRICK_VOXEL_COUNT: usize = 4_096;

/// The regenerated base truth retained by one active brick.
pub(super) enum BrickBase {
    Procedural,
    Uniform(Voxel),
    Detailed(Box<[Voxel; BRICK_VOXEL_COUNT]>),
}

impl BrickBase {
    pub(super) fn regenerate(identity: &WorldIdentity, brick: BrickCoord) -> Self {
        match classify_brick(identity, brick) {
            ProceduralClass::Uniform(voxel) => Self::Uniform(voxel),
            ProceduralClass::Procedural => Self::Procedural,
        }
    }

    pub(super) fn voxel_at(
        &self,
        identity: &WorldIdentity,
        coordinate: VoxelCoord,
        local_index: u16,
    ) -> Voxel {
        match self {
            Self::Procedural => evaluate_base_voxel(identity, coordinate),
            Self::Uniform(voxel) => *voxel,
            Self::Detailed(voxels) => voxels[usize::from(local_index)],
        }
    }

    pub(super) fn materialize(&mut self, identity: &WorldIdentity, brick: BrickCoord) {
        if matches!(self, Self::Detailed(_)) {
            return;
        }

        let mut voxels = Box::new([Voxel::new(crate::AIR, 0, 0, 0); BRICK_VOXEL_COUNT]);
        let origin_x = -2_000 + i32::from(brick.x()) * BRICK_EDGE_VOXELS;
        let origin_y = -512 + i32::from(brick.y()) * BRICK_EDGE_VOXELS;
        let origin_z = -2_000 + i32::from(brick.z()) * BRICK_EDGE_VOXELS;

        for local_y in 0..BRICK_EDGE_VOXELS {
            for local_z in 0..BRICK_EDGE_VOXELS {
                for local_x in 0..BRICK_EDGE_VOXELS {
                    let local_index = (local_x
                        + BRICK_EDGE_VOXELS * (local_z + BRICK_EDGE_VOXELS * local_y))
                        as usize;
                    let coordinate =
                        VoxelCoord::new(origin_x + local_x, origin_y + local_y, origin_z + local_z);
                    voxels[local_index] = evaluate_base_voxel(identity, coordinate);
                }
            }
        }

        *self = Self::Detailed(voxels);
    }
}

/// Active regenerated base metadata. Current truth remains the store delta overlay.
pub(super) struct BrickRecord {
    base: BrickBase,
    revision: u64,
}

impl BrickRecord {
    pub(super) fn regenerate(identity: &WorldIdentity, brick: BrickCoord) -> Self {
        Self {
            base: BrickBase::regenerate(identity, brick),
            revision: 0,
        }
    }

    pub(super) fn base_voxel(
        &self,
        identity: &WorldIdentity,
        coordinate: VoxelCoord,
        local_index: u16,
    ) -> Voxel {
        self.base.voxel_at(identity, coordinate, local_index)
    }

    pub(super) fn materialize(&mut self, identity: &WorldIdentity, brick: BrickCoord) {
        self.base.materialize(identity, brick);
    }

    pub(super) fn set_revision(&mut self, revision: u64) {
        self.revision = revision;
    }
}
