//! Private sparse current-truth composition of regenerated base and edit deltas.

use std::collections::{BTreeMap, HashMap};

use super::brick::BrickRecord;
use crate::{BrickCoord, Voxel, VoxelCoord, WorldIdentity, evaluate_base_voxel};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct VoxelDelta {
    local_index: u16,
    value: Voxel,
}

#[derive(Debug)]
struct BrickDelta {
    brick: BrickCoord,
    voxels: Vec<VoxelDelta>,
}

impl BrickDelta {
    fn current(&self, local_index: u16) -> Option<Voxel> {
        self.voxels
            .binary_search_by_key(&local_index, |delta| delta.local_index)
            .ok()
            .map(|index| self.voxels[index].value)
    }

    fn set(&mut self, local_index: u16, value: Option<Voxel>) {
        match self
            .voxels
            .binary_search_by_key(&local_index, |delta| delta.local_index)
        {
            Ok(index) => match value {
                Some(value) => self.voxels[index].value = value,
                None => {
                    self.voxels.remove(index);
                }
            },
            Err(index) => {
                if let Some(value) = value {
                    self.voxels.insert(index, VoxelDelta { local_index, value });
                }
            }
        }
    }
}

/// Authoritative sparse state. This type is crate-private and is never an ECS entity.
pub(crate) struct WorldStore {
    identity: WorldIdentity,
    active: HashMap<BrickCoord, BrickRecord>,
    deltas: BTreeMap<BrickCoord, BrickDelta>,
    revision: u64,
}

impl WorldStore {
    pub(crate) fn new(identity: WorldIdentity) -> Self {
        Self {
            identity,
            active: HashMap::new(),
            deltas: BTreeMap::new(),
            revision: 0,
        }
    }

    #[must_use]
    pub(crate) const fn identity(&self) -> &WorldIdentity {
        &self.identity
    }

    #[must_use]
    pub(crate) const fn revision(&self) -> u64 {
        self.revision
    }

    #[must_use]
    pub(crate) fn current_voxel(&self, coordinate: VoxelCoord) -> Voxel {
        let (brick, local_index) = coordinate
            .to_brick_and_local_index()
            .expect("WorldStore receives validated in-region coordinates");
        self.deltas
            .get(&brick)
            .and_then(|delta| delta.current(local_index))
            .unwrap_or_else(|| self.base_voxel(brick, coordinate, local_index))
    }

    /// Expands regenerated base truth for an active purpose without materializing current truth.
    pub(crate) fn materialize_detail(&mut self, brick: BrickCoord) {
        let record = self
            .active
            .entry(brick)
            .or_insert_with(|| BrickRecord::regenerate(&self.identity, brick));
        record.materialize(&self.identity, brick);
    }

    /// Applies the final values of one atomic batch and returns its revision.
    ///
    /// Empty and net-no-op batches preserve the current revision.
    pub(crate) fn commit_current<I>(&mut self, changes: I) -> u64
    where
        I: IntoIterator<Item = (VoxelCoord, Voxel)>,
    {
        let changes: BTreeMap<_, _> = changes.into_iter().collect();
        if changes
            .iter()
            .all(|(coordinate, value)| self.current_voxel(*coordinate) == *value)
        {
            return self.revision;
        }

        self.revision = self
            .revision
            .checked_add(1)
            .expect("world revision cannot wrap");
        let revision = self.revision;

        for (coordinate, value) in changes {
            let (brick, local_index) = coordinate
                .to_brick_and_local_index()
                .expect("WorldStore receives validated in-region coordinates");
            let base = self.base_voxel(brick, coordinate, local_index);
            let value = (value != base).then_some(value);

            if let Some(delta) = self.deltas.get_mut(&brick) {
                delta.set(local_index, value);
            } else if let Some(value) = value {
                let mut delta = BrickDelta {
                    brick,
                    voxels: Vec::new(),
                };
                delta.set(local_index, Some(value));
                self.deltas.insert(brick, delta);
            }

            if self
                .deltas
                .get(&brick)
                .is_some_and(|delta| delta.voxels.is_empty())
            {
                self.deltas.remove(&brick);
            }
            if let Some(record) = self.active.get_mut(&brick) {
                record.set_revision(revision);
            }
        }

        revision
    }

    fn base_voxel(&self, brick: BrickCoord, coordinate: VoxelCoord, local_index: u16) -> Voxel {
        self.active.get(&brick).map_or_else(
            || evaluate_base_voxel(&self.identity, coordinate),
            |record| record.base_voxel(&self.identity, coordinate, local_index),
        )
    }

    #[cfg(test)]
    pub(super) fn active_brick_count(&self) -> usize {
        self.active.len()
    }

    #[cfg(test)]
    pub(super) fn delta_count(&self) -> usize {
        self.deltas.values().map(|delta| delta.voxels.len()).sum()
    }

    #[cfg(test)]
    pub(super) fn delta_entries_are_sorted(&self) -> bool {
        self.deltas.iter().all(|(brick, delta)| {
            *brick == delta.brick
                && delta
                    .voxels
                    .windows(2)
                    .all(|pair| pair[0].local_index < pair[1].local_index)
        })
    }
}
