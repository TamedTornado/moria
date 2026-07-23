//! Compact, immutable spatial tables for registered non-ruin objects.

use std::{collections::BTreeMap, mem::size_of};

use crate::{
    AabbQ8, ManifestError, ObjectGenConfig, ObjectId, ObjectKind, ObjectPlacement,
    Q8_UNITS_PER_METER, VOXEL_EDGE_Q8, VoxelCoord, WorldPointQ8,
};

use super::{OBJECT_EXTRACTION_STENCIL, raw_shape_bounds};

const ALLOCATOR_BYTES: u64 = 16;
const DEPENDENCY_CELL_METERS: i32 = 32;
const SAMPLE_CELL_METERS: i32 = 4;
const HORIZON_CELL_METERS: i32 = 64;
const EDIT_RADIUS_Q8: i32 = 3 * Q8_UNITS_PER_METER;

/// One horizontal 32 m dependency-table key.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DependencyGridCellKey {
    pub x: i16,
    pub z: i16,
}

/// One horizontal 4 m sample-table key.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SampleGridCellKey {
    pub x: i16,
    pub z: i16,
}

/// One aligned 64 m Horizon tree-cell key.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct HorizonCellKey {
    pub x: i16,
    pub z: i16,
}

impl HorizonCellKey {
    #[must_use]
    pub const fn new(x: i16, z: i16) -> Self {
        Self { x, z }
    }
}

/// Fixed derived metadata retained for one manifest placement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectIndexRecord {
    pub raw_bounds: AabbQ8,
    pub dependency_bounds: AabbQ8,
}

/// A sorted dependency-grid member list containing manifest indices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyGridCell {
    pub key: DependencyGridCellKey,
    pub members: Vec<u32>,
}

/// A sorted fine sample-grid member list containing manifest indices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SampleGridCell {
    pub key: SampleGridCellKey,
    pub members: Vec<u32>,
}

/// Limits used to build the two object spatial tables.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectIndexConfig {
    pub max_dependency_cells_per_object: u8,
    pub max_dependency_members_per_cell: u16,
    pub max_sample_cells_per_object: u8,
    pub max_sample_members_per_cell: u8,
    pub max_edit_dependency_candidates: u16,
    pub max_affected_objects_per_edit: u8,
    pub max_dependency_bricks_per_object: u16,
    pub max_retained_bytes: u32,
    pub max_horizon_tree_members_per_cell: u16,
}

impl Default for ObjectIndexConfig {
    fn default() -> Self {
        Self {
            max_dependency_cells_per_object: 16,
            max_dependency_members_per_cell: 1024,
            max_sample_cells_per_object: 16,
            max_sample_members_per_cell: 64,
            max_edit_dependency_candidates: 256,
            max_affected_objects_per_edit: 64,
            max_dependency_bricks_per_object: 128,
            max_retained_bytes: 16 * 1024 * 1024,
            max_horizon_tree_members_per_cell: 1024,
        }
    }
}

impl ObjectIndexConfig {
    /// Combines object-generation limits with the rendering Horizon membership cap.
    #[must_use]
    pub fn from_configs(objects: &ObjectGenConfig, max_horizon_tree_members_per_cell: u16) -> Self {
        Self {
            max_dependency_cells_per_object: objects.max_index_cells_per_object,
            max_dependency_members_per_cell: objects.max_index_entries_per_cell,
            max_sample_cells_per_object: objects.max_sample_cells_per_object,
            max_sample_members_per_cell: objects.max_sample_entries_per_cell,
            max_edit_dependency_candidates: objects.max_edit_dependency_candidates,
            max_affected_objects_per_edit: objects.max_affected_objects_per_edit,
            max_dependency_bricks_per_object: objects.max_dependency_bricks_per_object,
            max_retained_bytes: objects.max_retained_index_bytes,
            max_horizon_tree_members_per_cell,
        }
    }
}

/// Private immutable index data retained after manifest validation succeeds.
#[derive(Clone, Debug)]
pub struct ObjectSpatialIndex<'a> {
    placements: &'a [ObjectPlacement],
    records: Vec<ObjectIndexRecord>,
    dependency_cells: Vec<DependencyGridCell>,
    sample_cells: Vec<SampleGridCell>,
    retained_bytes: u64,
}

impl ObjectSpatialIndex<'_> {
    #[must_use]
    pub fn records(&self) -> &[ObjectIndexRecord] {
        &self.records
    }

    #[must_use]
    pub fn placements(&self) -> &[ObjectPlacement] {
        self.placements
    }

    #[must_use]
    pub fn dependency_cells(&self) -> &[DependencyGridCell] {
        &self.dependency_cells
    }

    #[must_use]
    pub fn sample_cells(&self) -> &[SampleGridCell] {
        &self.sample_cells
    }

    #[must_use]
    pub const fn retained_bytes(&self) -> u64 {
        self.retained_bytes
    }

    /// This index deliberately retains only boxes and table members.
    #[must_use]
    pub const fn dependency_coordinate_allocation_bytes(&self) -> u64 {
        0
    }
}

/// Builds the 32 m dependency and 4 m sample tables without expanding shapes.
pub fn build_object_index<'a>(
    placements: &'a [ObjectPlacement],
    config: &ObjectIndexConfig,
) -> Result<ObjectSpatialIndex<'a>, ManifestError> {
    let mut records = Vec::with_capacity(placements.len());
    let mut dependency_members = BTreeMap::<DependencyGridCellKey, Vec<u32>>::new();
    let mut sample_members = BTreeMap::<SampleGridCellKey, Vec<u32>>::new();
    let mut horizon_counts = BTreeMap::<HorizonCellKey, u16>::new();

    for (index, placement) in placements.iter().enumerate() {
        let raw_bounds =
            raw_shape_bounds(placement).ok_or(ManifestError::ObjectRawBoundsUnavailable {
                object_id: placement.id,
            })?;
        let dependency_bounds = expand_dependency_bounds(raw_bounds).ok_or(
            ManifestError::ObjectDependencyBoundsOverflow {
                object_id: placement.id,
            },
        )?;
        let dependency_bricks = brick_count(dependency_bounds);
        if dependency_bricks > config.max_dependency_bricks_per_object {
            return Err(ManifestError::ObjectDependencyBricksExceeded {
                object_id: placement.id,
                actual: dependency_bricks,
                maximum: config.max_dependency_bricks_per_object,
            });
        }
        let dependency_keys = dependency_keys_for(dependency_bounds);
        enforce_cell_count(
            placement.id,
            dependency_keys.len(),
            config.max_dependency_cells_per_object,
        )?;
        let sample_keys = sample_keys_for(raw_bounds);
        enforce_cell_count(
            placement.id,
            sample_keys.len(),
            config.max_sample_cells_per_object,
        )?;
        let member = u32::try_from(index).expect("manifest index fits u32");
        for key in dependency_keys {
            dependency_members.entry(key).or_default().push(member);
        }
        for key in sample_keys {
            sample_members.entry(key).or_default().push(member);
        }
        if is_tree(placement.kind) {
            let count = horizon_counts
                .entry(horizon_key(placement.anchor))
                .or_default();
            *count = count.saturating_add(1);
            if *count > config.max_horizon_tree_members_per_cell {
                return Err(ManifestError::HorizonTreeCellCapacityExceeded {
                    actual: *count,
                    maximum: config.max_horizon_tree_members_per_cell,
                });
            }
        }
        records.push(ObjectIndexRecord {
            raw_bounds,
            dependency_bounds,
        });
    }

    let dependency_cells = dependency_members
        .into_iter()
        .map(|(key, mut members)| {
            if members.len() > usize::from(config.max_dependency_members_per_cell) {
                return Err(ManifestError::ObjectIndexCellCapacityExceeded {
                    actual: u16::try_from(members.len()).unwrap_or(u16::MAX),
                    maximum: config.max_dependency_members_per_cell,
                });
            }
            members.sort_unstable_by_key(|&member| placements[member as usize].id);
            Ok(DependencyGridCell { key, members })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let sample_cells = sample_members
        .into_iter()
        .map(|(key, mut members)| {
            if members.len() > usize::from(config.max_sample_members_per_cell) {
                return Err(ManifestError::ObjectSampleCellCapacityExceeded {
                    actual: u16::try_from(members.len()).unwrap_or(u16::MAX),
                    maximum: config.max_sample_members_per_cell,
                });
            }
            members.sort_unstable_by_key(|&member| placements[member as usize].id);
            Ok(SampleGridCell { key, members })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let retained_bytes = retained_bytes(
        records.capacity(),
        dependency_cells.capacity(),
        sample_cells.capacity(),
        &dependency_cells,
        &sample_cells,
    );
    validate_edit_caps(&dependency_cells, &records, config)?;
    if retained_bytes > u64::from(config.max_retained_bytes) {
        return Err(ManifestError::ObjectIndexRetainedBytesExceeded {
            actual: retained_bytes,
            maximum: config.max_retained_bytes,
        });
    }
    Ok(ObjectSpatialIndex {
        placements,
        records,
        dependency_cells,
        sample_cells,
        retained_bytes,
    })
}

fn validate_edit_caps(
    cells: &[DependencyGridCell],
    records: &[ObjectIndexRecord],
    config: &ObjectIndexConfig,
) -> Result<(), ManifestError> {
    let mut starts = BTreeMap::<DependencyGridCellKey, ()>::new();
    let mut exact_overlap_cache = BTreeMap::<Vec<u32>, Option<u16>>::new();
    for cell in cells {
        for x_offset in [-1_i16, 0] {
            for z_offset in [-1_i16, 0] {
                starts.insert(
                    DependencyGridCellKey {
                        x: cell.key.x.saturating_add(x_offset),
                        z: cell.key.z.saturating_add(z_offset),
                    },
                    (),
                );
            }
        }
    }
    for (start, ()) in starts {
        let mut members = Vec::new();
        for x_offset in [0_i16, 1] {
            for z_offset in [0_i16, 1] {
                let key = DependencyGridCellKey {
                    x: start.x.saturating_add(x_offset),
                    z: start.z.saturating_add(z_offset),
                };
                if let Ok(index) = cells.binary_search_by_key(&key, |cell| cell.key) {
                    members.extend_from_slice(&cells[index].members);
                }
            }
        }
        members.sort_unstable();
        members.dedup();
        let actual = u16::try_from(members.len()).unwrap_or(u16::MAX);
        if actual > u16::from(config.max_affected_objects_per_edit) {
            let affected = *exact_overlap_cache
                .entry(members.clone())
                .or_insert_with(|| {
                    exact_edit_overlap_exceeding(
                        &members,
                        records,
                        u16::from(config.max_affected_objects_per_edit),
                    )
                });
            if let Some(affected) = affected {
                return Err(ManifestError::ObjectEditAffectedExceeded {
                    actual: affected,
                    maximum: config.max_affected_objects_per_edit,
                });
            }
        }
    }
    Ok(())
}

/// Returns an exact dependency overlap above `maximum` for a legal radius-3 m edit center.
///
/// A dependency bound overlaps an edit precisely while the integer voxel center
/// lies inside its transformed center box.  Sweeping box boundaries therefore
/// checks every distinct overlap region without conflating broad grid candidates
/// with exact affected objects.
fn exact_edit_overlap_exceeding(
    members: &[u32],
    records: &[ObjectIndexRecord],
    maximum: u16,
) -> Option<u16> {
    let bounds = members
        .iter()
        .filter_map(|&member| records.get(usize::try_from(member).ok()?))
        .map(|record| edit_center_bounds(record.dependency_bounds))
        .collect::<Vec<_>>();
    for x_members in sweep_axis(&bounds, &(0..bounds.len()).collect::<Vec<_>>(), |bound| {
        (bound.min.x, bound.max.x)
    }) {
        if x_members.len() <= usize::from(maximum) {
            continue;
        }
        for y_members in sweep_axis(&bounds, &x_members, |bound| (bound.min.y, bound.max.y)) {
            if y_members.len() <= usize::from(maximum) {
                continue;
            }
            for z_members in sweep_axis(&bounds, &y_members, |bound| (bound.min.z, bound.max.z)) {
                if z_members.len() > usize::from(maximum) {
                    return Some(u16::try_from(z_members.len()).unwrap_or(u16::MAX));
                }
            }
        }
    }
    None
}

#[derive(Clone, Copy)]
struct EditCenterBounds {
    min: VoxelCoord,
    max: VoxelCoord,
}

fn edit_center_bounds(bounds: AabbQ8) -> EditCenterBounds {
    let minimum = |coordinate: i32| (coordinate - EDIT_RADIUS_Q8).div_euclid(VOXEL_EDGE_Q8);
    let maximum = |coordinate: i32| (coordinate + EDIT_RADIUS_Q8 - 1).div_euclid(VOXEL_EDGE_Q8);
    EditCenterBounds {
        min: VoxelCoord::new(
            minimum(bounds.min.x),
            minimum(bounds.min.y),
            minimum(bounds.min.z),
        ),
        max: VoxelCoord::new(
            maximum(bounds.max_exclusive.x),
            maximum(bounds.max_exclusive.y),
            maximum(bounds.max_exclusive.z),
        ),
    }
}

/// Returns active member sets for all nonempty integer-coordinate slabs on one axis.
fn sweep_axis(
    bounds: &[EditCenterBounds],
    members: &[usize],
    axis: impl Fn(EditCenterBounds) -> (i32, i32),
) -> Vec<Vec<usize>> {
    let mut events = members
        .iter()
        .flat_map(|&member| {
            let (minimum, maximum) = axis(bounds[member]);
            [
                (minimum, member, true),
                (maximum.saturating_add(1), member, false),
            ]
        })
        .collect::<Vec<_>>();
    events.sort_unstable_by_key(|&(coordinate, member, starts)| (coordinate, member, starts));

    let mut active = vec![false; bounds.len()];
    let mut slabs = Vec::new();
    for group in events.chunk_by(|left, right| left.0 == right.0) {
        for &(_, member, starts) in group {
            active[member] = starts;
        }
        let members = active
            .iter()
            .enumerate()
            .filter_map(|(member, &is_active)| is_active.then_some(member))
            .collect::<Vec<_>>();
        if !members.is_empty() {
            slabs.push(members);
        }
    }
    slabs
}

/// Returns active placements whose raw bounds overlap `bounds`, sorted by ID.
#[must_use]
pub fn placement_ids_in(index: &ObjectSpatialIndex<'_>, bounds: AabbQ8) -> Vec<ObjectId> {
    let mut members = Vec::new();
    for key in dependency_keys_for(bounds) {
        if let Ok(cell_index) = index
            .dependency_cells
            .binary_search_by_key(&key, |cell| cell.key)
        {
            members.extend_from_slice(&index.dependency_cells[cell_index].members);
        }
    }
    ids_from_members(index, members, |record| overlaps(record.raw_bounds, bounds))
}

/// Returns the exact sorted tree-anchor membership of an aligned Horizon cell.
#[must_use]
pub fn horizon_tree_ids(index: &ObjectSpatialIndex<'_>, key: HorizonCellKey) -> Vec<ObjectId> {
    let bounds = cell_bounds(key, HORIZON_CELL_METERS);
    let mut members = Vec::new();
    for dependency_key in dependency_keys_for(bounds) {
        if let Ok(cell_index) = index
            .dependency_cells
            .binary_search_by_key(&dependency_key, |cell| cell.key)
        {
            members.extend_from_slice(&index.dependency_cells[cell_index].members);
        }
    }
    members.sort_unstable();
    members.dedup();
    let mut ids = members
        .into_iter()
        .filter_map(|member| {
            let member = usize::try_from(member).ok()?;
            let placement = index.placements.get(member)?;
            (is_tree(placement.kind) && horizon_key(placement.anchor) == key)
                .then_some(placement.id)
        })
        .collect::<Vec<_>>();
    ids.sort_unstable();
    ids
}

fn ids_from_members(
    index: &ObjectSpatialIndex<'_>,
    mut members: Vec<u32>,
    predicate: impl Fn(ObjectIndexRecord) -> bool,
) -> Vec<ObjectId> {
    members.sort_unstable();
    members.dedup();
    let mut ids = members
        .into_iter()
        .filter_map(|member| {
            let position = usize::try_from(member).ok()?;
            predicate(index.records[position]).then_some(index.placements[position].id)
        })
        .collect::<Vec<_>>();
    ids.sort_unstable();
    ids
}

fn expand_dependency_bounds(bounds: AabbQ8) -> Option<AabbQ8> {
    let halo = OBJECT_EXTRACTION_STENCIL
        .iter()
        .map(|offset| i32::from(offset.x).unsigned_abs())
        .max()?;
    let halo_q8 = i32::try_from(halo).ok()?.checked_mul(VOXEL_EDGE_Q8)?;
    AabbQ8::new(
        WorldPointQ8::new(
            bounds.min.x.checked_sub(halo_q8)?,
            bounds.min.y.checked_sub(halo_q8)?,
            bounds.min.z.checked_sub(halo_q8)?,
        ),
        WorldPointQ8::new(
            bounds.max_exclusive.x.checked_add(halo_q8)?,
            bounds.max_exclusive.y.checked_add(halo_q8)?,
            bounds.max_exclusive.z.checked_add(halo_q8)?,
        ),
    )
    .ok()
}

fn dependency_keys_for(bounds: AabbQ8) -> Vec<DependencyGridCellKey> {
    horizontal_range(bounds, DEPENDENCY_CELL_METERS)
        .flat_map(|(x, z)| {
            i16::try_from(x)
                .ok()
                .zip(i16::try_from(z).ok())
                .map(|(x, z)| DependencyGridCellKey { x, z })
        })
        .collect()
}

fn sample_keys_for(bounds: AabbQ8) -> Vec<SampleGridCellKey> {
    horizontal_range(bounds, SAMPLE_CELL_METERS)
        .flat_map(|(x, z)| {
            i16::try_from(x)
                .ok()
                .zip(i16::try_from(z).ok())
                .map(|(x, z)| SampleGridCellKey { x, z })
        })
        .collect()
}

fn horizontal_range(bounds: AabbQ8, meters: i32) -> impl Iterator<Item = (i32, i32)> {
    let edge = meters * Q8_UNITS_PER_METER;
    let min_x = bounds.min.x.div_euclid(edge);
    let max_x = (bounds.max_exclusive.x - 1).div_euclid(edge);
    let min_z = bounds.min.z.div_euclid(edge);
    let max_z = (bounds.max_exclusive.z - 1).div_euclid(edge);
    (min_x..=max_x).flat_map(move |x| (min_z..=max_z).map(move |z| (x, z)))
}

fn brick_count(bounds: AabbQ8) -> u16 {
    let edge = VOXEL_EDGE_Q8 * 16;
    let extent = |min: i32, max: i32| (max - 1).div_euclid(edge) - min.div_euclid(edge) + 1;
    let count = i64::from(extent(bounds.min.x, bounds.max_exclusive.x))
        * i64::from(extent(bounds.min.y, bounds.max_exclusive.y))
        * i64::from(extent(bounds.min.z, bounds.max_exclusive.z));
    u16::try_from(count).unwrap_or(u16::MAX)
}

fn enforce_cell_count(
    object_id: ObjectId,
    actual: usize,
    maximum: u8,
) -> Result<(), ManifestError> {
    if actual > usize::from(maximum) {
        return Err(ManifestError::ObjectIndexCellsExceeded {
            object_id,
            actual: u16::try_from(actual).unwrap_or(u16::MAX),
            maximum,
        });
    }
    Ok(())
}

fn retained_bytes(
    record_capacity: usize,
    dependency_cell_capacity: usize,
    sample_cell_capacity: usize,
    dependencies: &[DependencyGridCell],
    samples: &[SampleGridCell],
) -> u64 {
    let members = dependencies
        .iter()
        .map(|cell| cell.members.capacity())
        .sum::<usize>()
        + samples
            .iter()
            .map(|cell| cell.members.capacity())
            .sum::<usize>();
    u64::try_from(
        record_capacity * size_of::<ObjectIndexRecord>()
            + dependency_cell_capacity * size_of::<DependencyGridCell>()
            + sample_cell_capacity * size_of::<SampleGridCell>()
            + members * size_of::<u32>(),
    )
    .expect("index byte count fits u64")
        + ALLOCATOR_BYTES
            * (2 + u64::try_from(dependencies.len() + samples.len()).expect("cell count fits u64"))
}

fn overlaps(left: AabbQ8, right: AabbQ8) -> bool {
    left.min.x < right.max_exclusive.x
        && right.min.x < left.max_exclusive.x
        && left.min.y < right.max_exclusive.y
        && right.min.y < left.max_exclusive.y
        && left.min.z < right.max_exclusive.z
        && right.min.z < left.max_exclusive.z
}

fn horizon_key(anchor: VoxelCoord) -> HorizonCellKey {
    HorizonCellKey {
        x: i16::try_from(anchor.x.div_euclid(256)).unwrap_or(if anchor.x < 0 {
            i16::MIN
        } else {
            i16::MAX
        }),
        z: i16::try_from(anchor.z.div_euclid(256)).unwrap_or(if anchor.z < 0 {
            i16::MIN
        } else {
            i16::MAX
        }),
    }
}

fn cell_bounds(key: HorizonCellKey, meters: i32) -> AabbQ8 {
    let edge = meters * Q8_UNITS_PER_METER;
    let min_x = i32::from(key.x) * edge;
    let min_z = i32::from(key.z) * edge;
    AabbQ8::new(
        WorldPointQ8::new(min_x, i32::MIN / 2, min_z),
        WorldPointQ8::new(min_x + edge, i32::MAX / 2, min_z + edge),
    )
    .expect("cell bounds are valid")
}

fn is_tree(kind: ObjectKind) -> bool {
    matches!(kind, ObjectKind::TreeA | ObjectKind::TreeB)
}
