//! Private sparse current-truth composition of regenerated base and edit deltas.

use std::collections::{BTreeMap, HashMap};

use super::brick::BrickRecord;
use crate::{
    AIR, BrickCoord, FeatureInstance, FeatureKind, IRON_ORE, ObjectIndexConfig, ObjectIndexRecord,
    ObjectPlacement, SampleGridCell, SampleGridCellKey, Voxel, VoxelCoord, WATER, WorldIdentity,
    WorldPointQ8, build_object_index, evaluate_base_voxel, sample_object_shape,
};

/// Immutable generated base layers retained by the authoritative store.
///
/// The object sample grid is copied from the validated manifest index.  It keeps
/// scalar reads bounded to one 4 m cell rather than scanning all placements.
#[derive(Default)]
pub(crate) struct CuratedBaseTruth {
    features: Vec<FeatureInstance>,
    objects: Vec<ObjectPlacement>,
    object_records: Vec<ObjectIndexRecord>,
    sample_cells: Vec<SampleGridCell>,
}

impl CuratedBaseTruth {
    fn install(&mut self, features: Vec<FeatureInstance>, objects: Vec<ObjectPlacement>) {
        let index = build_object_index(&objects, &ObjectIndexConfig::default())
            .expect("curated truth is installed only after manifest validation");
        let object_records = index.records().to_vec();
        let sample_cells = index.sample_cells().to_vec();
        self.features = features;
        self.objects = objects;
        self.object_records = object_records;
        self.sample_cells = sample_cells;
    }

    fn voxel(&self, identity: &WorldIdentity, coordinate: VoxelCoord) -> Voxel {
        if let Some(voxel) = self.object_voxel(coordinate) {
            return voxel;
        }
        if let Some(voxel) = self.feature_voxel(coordinate) {
            return voxel;
        }
        evaluate_base_voxel(identity, coordinate)
    }

    fn feature_voxel(&self, coordinate: VoxelCoord) -> Option<Voxel> {
        let point = WorldPointQ8::new(
            coordinate.x * 64 + 32,
            coordinate.y * 64 + 32,
            coordinate.z * 64 + 32,
        );
        // The manifest is sorted by ID.  Explicit precedence remains independent
        // of insertion order and never scans more than the sixteen feature cap.
        self.features
            .iter()
            .filter(|feature| feature.bounds.contains(point))
            .max_by_key(|feature| {
                (
                    feature_precedence(feature.kind),
                    std::cmp::Reverse(feature.id),
                )
            })
            .map(|feature| match feature.kind {
                FeatureKind::KarstCave => Voxel::new(AIR, 0, 0, 0),
                FeatureKind::Aquifer => Voxel::new(WATER, u8::MAX, 0, 0),
                FeatureKind::IronVein => Voxel::new(IRON_ORE, u8::MAX, 0, 0),
                FeatureKind::Topsoil | FeatureKind::Subsoil | FeatureKind::Stratum => {
                    Voxel::new(feature.host_material, u8::MAX, 0, 0)
                }
            })
    }

    fn object_voxel(&self, coordinate: VoxelCoord) -> Option<Voxel> {
        let key = SampleGridCellKey {
            x: i16::try_from(coordinate.x.div_euclid(16)).ok()?,
            z: i16::try_from(coordinate.z.div_euclid(16)).ok()?,
        };
        let cell = self
            .sample_cells
            .binary_search_by_key(&key, |cell| cell.key)
            .ok()
            .map(|index| &self.sample_cells[index])?;
        cell.members.iter().find_map(|&member| {
            let index = member as usize;
            self.object_records[index]
                .raw_bounds
                .contains(WorldPointQ8::new(
                    coordinate.x * 64 + 32,
                    coordinate.y * 64 + 32,
                    coordinate.z * 64 + 32,
                ))
                .then(|| sample_object_shape(&self.objects[index], coordinate))
                .flatten()
        })
    }
}

const fn feature_precedence(kind: FeatureKind) -> u8 {
    match kind {
        FeatureKind::Topsoil => 1,
        FeatureKind::Subsoil => 2,
        FeatureKind::Stratum => 3,
        FeatureKind::Aquifer => 4,
        FeatureKind::IronVein => 5,
        FeatureKind::KarstCave => 6,
    }
}

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
    curated_base: CuratedBaseTruth,
    active: HashMap<BrickCoord, BrickRecord>,
    deltas: BTreeMap<BrickCoord, BrickDelta>,
    revision: u64,
}

impl WorldStore {
    pub(crate) fn new(identity: WorldIdentity) -> Self {
        Self {
            identity,
            curated_base: CuratedBaseTruth::default(),
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

    /// Installs immutable validated manifest truth before the world becomes readable.
    pub(crate) fn install_curated_truth(
        &mut self,
        features: Vec<FeatureInstance>,
        objects: Vec<ObjectPlacement>,
    ) {
        self.curated_base.install(features, objects);
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

    fn base_voxel(&self, _brick: BrickCoord, coordinate: VoxelCoord, _local_index: u16) -> Voxel {
        // Active brick records are terrain materialization caches. They cannot
        // replace manifest composition, which remains authoritative for both
        // inactive and active public reads.
        self.curated_base.voxel(&self.identity, coordinate)
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
