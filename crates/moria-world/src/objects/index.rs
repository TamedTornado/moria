//! Compact, immutable spatial tables for registered non-ruin objects.

use std::{collections::BTreeMap, mem::size_of};

use crate::{
    AabbQ8, ManifestError, ObjectGenConfig, ObjectId, ObjectKind, ObjectPlacement,
    Q8_UNITS_PER_METER, VOXEL_EDGE_Q8, VoxelCoord, WorldPointQ8,
};

use super::{OBJECT_EXTRACTION_STENCIL, dependency_contains, raw_shape_bounds, raw_shape_contains};

const ALLOCATOR_BYTES: u64 = 16;
const DEPENDENCY_CELL_METERS: i32 = 32;
const SAMPLE_CELL_METERS: i32 = 4;
const HORIZON_CELL_METERS: i32 = 64;
const SUPPORTED_EDIT_RADIUS_VOXELS: i32 = 3 * Q8_UNITS_PER_METER / VOXEL_EDGE_Q8;
const REGION_XZ_MIN_VOXEL: i32 = -2_000;
const REGION_XZ_MAX_VOXEL_EXCLUSIVE: i32 = 2_000;
const REGION_Y_MIN_VOXEL: i32 = -512;
const REGION_Y_MAX_VOXEL_EXCLUSIVE: i32 = 512;
const REGION_XZ_MIN_Q8: i32 = -500 * Q8_UNITS_PER_METER;
const REGION_XZ_MAX_Q8_EXCLUSIVE: i32 = 500 * Q8_UNITS_PER_METER;
const REGION_Y_MIN_Q8: i32 = -128 * Q8_UNITS_PER_METER;
const REGION_Y_MAX_Q8_EXCLUSIVE: i32 = 128 * Q8_UNITS_PER_METER;

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

/// Inclusive legal edit-center range that can reach an object's dependency box.
#[derive(Clone, Copy, Debug)]
struct EditCenterBounds {
    min: VoxelCoord,
    max: VoxelCoord,
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
    )
    .ok_or(ManifestError::ObjectIndexRetainedBytesExceeded {
        actual: u64::MAX,
        maximum: config.max_retained_bytes,
    })?;
    validate_edit_caps(&dependency_cells, &records, placements, config)?;
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
    placements: &[ObjectPlacement],
    config: &ObjectIndexConfig,
) -> Result<(), ManifestError> {
    let mut starts = BTreeMap::<DependencyGridCellKey, ()>::new();
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
        if actual > config.max_edit_dependency_candidates {
            return Err(ManifestError::ObjectEditCandidatesExceeded {
                actual,
                maximum: config.max_edit_dependency_candidates,
            });
        }
        if actual > u16::from(config.max_affected_objects_per_edit) {
            let affected = max_affected_objects_for_edit(
                &members,
                records,
                placements,
                config.max_affected_objects_per_edit,
            );
            if affected > u16::from(config.max_affected_objects_per_edit) {
                return Err(ManifestError::ObjectEditAffectedExceeded {
                    actual: affected,
                    maximum: config.max_affected_objects_per_edit,
                });
            }
        }
    }
    Ok(())
}

fn max_affected_objects_for_edit(
    members: &[u32],
    records: &[ObjectIndexRecord],
    placements: &[ObjectPlacement],
    maximum: u8,
) -> u16 {
    let dependencies = members
        .iter()
        .filter_map(|&member| {
            let position = usize::try_from(member).ok()?;
            let record = records.get(position)?;
            let centers = edit_center_bounds(record.dependency_bounds)?;
            let dependencies = dependency_voxel_bounds(record.dependency_bounds);
            Some((centers, dependencies))
        })
        .collect::<Vec<_>>();

    if dependencies.is_empty() {
        return 0;
    }
    let Some(broad_overlap) = broad_edit_overlap_exceeding(&dependencies, u16::from(maximum))
    else {
        // An exact dependency hit is necessarily contained in this object's
        // expanded dependency box.  The sweep therefore proves that no legal
        // edit center can exceed the exact cap, without walking the (possibly
        // region-height) union of the boxes.
        return u16::from(maximum);
    };
    let center_bounds = broad_overlap.bounds;
    let mut observed_maximum = 0_u16;
    for x in center_bounds.min.x..=center_bounds.max.x {
        for y in center_bounds.min.y..=center_bounds.max.y {
            for z in center_bounds.min.z..=center_bounds.max.z {
                let center = VoxelCoord::new(x, y, z);
                let candidates = broad_overlap
                    .members
                    .iter()
                    .filter(|&&index| {
                        edit_sphere_can_reach_dependency_bounds(dependencies[index].1, center)
                    })
                    .count();
                if candidates <= usize::from(maximum) {
                    observed_maximum =
                        observed_maximum.max(u16::try_from(candidates).unwrap_or(u16::MAX));
                    continue;
                }
                let affected = u16::try_from(
                    broad_overlap
                        .members
                        .iter()
                        .filter(|&&index| {
                            edit_sphere_can_reach_dependency_bounds(dependencies[index].1, center)
                                && {
                                    let member = members[index];
                                    let position = usize::try_from(member).ok();
                                    position
                                        .and_then(|position| placements.get(position))
                                        .is_some_and(|placement| {
                                            edit_sphere_hits_dependency(placement, center)
                                        })
                                }
                        })
                        .count(),
                )
                .unwrap_or(u16::MAX);
                if affected > u16::from(maximum) {
                    return affected;
                }
                observed_maximum = observed_maximum.max(affected);
            }
        }
    }
    observed_maximum
}

struct BroadEditOverlap {
    bounds: EditCenterBounds,
    members: Vec<usize>,
}

/// Returns one event-local box intersection containing more than `maximum_allowed` candidates.
///
/// The sweep visits only candidate-box boundaries.  It deliberately avoids
/// materializing the potentially region-sized union of all candidate boxes.
fn broad_edit_overlap_exceeding(
    dependencies: &[(EditCenterBounds, EditCenterBounds)],
    maximum_allowed: u16,
) -> Option<BroadEditOverlap> {
    let mut x_witnesses = dependencies
        .iter()
        .map(|(centers, _)| centers.min.x)
        .collect::<Vec<_>>();
    x_witnesses.sort_unstable();
    x_witnesses.dedup();

    for x in x_witnesses {
        let active = dependencies
            .iter()
            .enumerate()
            .filter(|(_, (centers, _))| centers.min.x <= x && x <= centers.max.x)
            .collect::<Vec<_>>();
        let mut y_witnesses = active
            .iter()
            .map(|(_, (centers, _))| centers.min.y)
            .collect::<Vec<_>>();
        y_witnesses.sort_unstable();
        y_witnesses.dedup();
        for y in y_witnesses {
            let active_y = active
                .iter()
                .filter(|(_, (centers, _))| centers.min.y <= y && y <= centers.max.y)
                .copied()
                .collect::<Vec<_>>();
            let mut z_events = active_y
                .iter()
                .flat_map(|(_, (centers, _))| [(centers.min.z, 1_i16), (centers.max.z + 1, -1_i16)])
                .collect::<Vec<_>>();
            z_events.sort_unstable_by_key(|&(z, delta)| (z, -delta));
            let mut overlap = 0_i16;
            for (z, delta) in z_events {
                overlap += delta;
                if overlap <= i16::try_from(maximum_allowed).unwrap_or(i16::MAX) {
                    continue;
                }
                let members = active_y
                    .iter()
                    .filter(|(_, (centers, _))| centers.min.z <= z && z <= centers.max.z)
                    .map(|(index, _)| *index)
                    .collect::<Vec<_>>();
                let bounds = members
                    .iter()
                    .fold(dependencies[members[0]].0, |bounds, &index| {
                        intersect_edit_center_bounds(bounds, dependencies[index].0)
                    });
                return Some(BroadEditOverlap { bounds, members });
            }
        }
    }
    None
}

fn intersect_edit_center_bounds(
    left: EditCenterBounds,
    right: EditCenterBounds,
) -> EditCenterBounds {
    EditCenterBounds {
        min: VoxelCoord::new(
            left.min.x.max(right.min.x),
            left.min.y.max(right.min.y),
            left.min.z.max(right.min.z),
        ),
        max: VoxelCoord::new(
            left.max.x.min(right.max.x),
            left.max.y.min(right.max.y),
            left.max.z.min(right.max.z),
        ),
    }
}

fn edit_center_bounds(bounds: AabbQ8) -> Option<EditCenterBounds> {
    let min = VoxelCoord::new(
        (bounds.min.x.div_euclid(VOXEL_EDGE_Q8) - SUPPORTED_EDIT_RADIUS_VOXELS)
            .clamp(REGION_XZ_MIN_VOXEL, REGION_XZ_MAX_VOXEL_EXCLUSIVE - 1),
        (bounds.min.y.div_euclid(VOXEL_EDGE_Q8) - SUPPORTED_EDIT_RADIUS_VOXELS)
            .clamp(REGION_Y_MIN_VOXEL, REGION_Y_MAX_VOXEL_EXCLUSIVE - 1),
        (bounds.min.z.div_euclid(VOXEL_EDGE_Q8) - SUPPORTED_EDIT_RADIUS_VOXELS)
            .clamp(REGION_XZ_MIN_VOXEL, REGION_XZ_MAX_VOXEL_EXCLUSIVE - 1),
    );
    let max = VoxelCoord::new(
        ((bounds.max_exclusive.x - 1).div_euclid(VOXEL_EDGE_Q8) + SUPPORTED_EDIT_RADIUS_VOXELS)
            .clamp(REGION_XZ_MIN_VOXEL, REGION_XZ_MAX_VOXEL_EXCLUSIVE - 1),
        ((bounds.max_exclusive.y - 1).div_euclid(VOXEL_EDGE_Q8) + SUPPORTED_EDIT_RADIUS_VOXELS)
            .clamp(REGION_Y_MIN_VOXEL, REGION_Y_MAX_VOXEL_EXCLUSIVE - 1),
        ((bounds.max_exclusive.z - 1).div_euclid(VOXEL_EDGE_Q8) + SUPPORTED_EDIT_RADIUS_VOXELS)
            .clamp(REGION_XZ_MIN_VOXEL, REGION_XZ_MAX_VOXEL_EXCLUSIVE - 1),
    );
    (min.x <= max.x && min.y <= max.y && min.z <= max.z).then_some(EditCenterBounds { min, max })
}

fn dependency_voxel_bounds(bounds: AabbQ8) -> EditCenterBounds {
    EditCenterBounds {
        min: VoxelCoord::new(
            bounds.min.x.div_euclid(VOXEL_EDGE_Q8),
            bounds.min.y.div_euclid(VOXEL_EDGE_Q8),
            bounds.min.z.div_euclid(VOXEL_EDGE_Q8),
        ),
        max: VoxelCoord::new(
            (bounds.max_exclusive.x - 1).div_euclid(VOXEL_EDGE_Q8),
            (bounds.max_exclusive.y - 1).div_euclid(VOXEL_EDGE_Q8),
            (bounds.max_exclusive.z - 1).div_euclid(VOXEL_EDGE_Q8),
        ),
    }
}

fn edit_sphere_can_reach_dependency_bounds(
    dependency: EditCenterBounds,
    center: VoxelCoord,
) -> bool {
    let separation = |left_min: i32, left_max: i32, right_min: i32, right_max: i32| {
        if left_max < right_min {
            i64::from(right_min - left_max)
        } else if right_max < left_min {
            i64::from(left_min - right_max)
        } else {
            0
        }
    };
    let x = separation(dependency.min.x, dependency.max.x, center.x, center.x);
    let y = separation(dependency.min.y, dependency.max.y, center.y, center.y);
    let z = separation(dependency.min.z, dependency.max.z, center.z, center.z);
    let radius = i64::from(SUPPORTED_EDIT_RADIUS_VOXELS);
    x * x + y * y + z * z <= radius * radius
}

fn edit_sphere_hits_dependency(placement: &ObjectPlacement, center: VoxelCoord) -> bool {
    if dependency_contains(placement, center) {
        return true;
    }
    let radius = SUPPORTED_EDIT_RADIUS_VOXELS;
    for x_offset in -radius..=radius {
        for y_offset in -radius..=radius {
            for z_offset in -radius..=radius {
                if (x_offset, y_offset, z_offset) == (0, 0, 0)
                    || x_offset * x_offset + y_offset * y_offset + z_offset * z_offset
                        > radius * radius
                {
                    continue;
                }
                let Some(x) = center.x.checked_add(x_offset) else {
                    continue;
                };
                let Some(y) = center.y.checked_add(y_offset) else {
                    continue;
                };
                let Some(z) = center.z.checked_add(z_offset) else {
                    continue;
                };
                if dependency_contains(placement, VoxelCoord::new(x, y, z)) {
                    return true;
                }
            }
        }
    }
    false
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
    ids_from_members(index, members, |_, record| {
        overlaps(record.raw_bounds, bounds)
    })
}

/// Returns the sorted analytic object IDs solid at one voxel coordinate.
#[must_use]
pub fn sample_object_ids_at(
    index: &ObjectSpatialIndex<'_>,
    coordinate: VoxelCoord,
) -> Vec<ObjectId> {
    if !coordinate.is_in_region() {
        return Vec::new();
    }
    let Some(key) = sample_key_for(coordinate) else {
        return Vec::new();
    };
    let Ok(cell_index) = index
        .sample_cells
        .binary_search_by_key(&key, |cell| cell.key)
    else {
        return Vec::new();
    };
    ids_from_members(
        index,
        index.sample_cells[cell_index].members.clone(),
        |position, record| {
            voxel_in_bounds(coordinate, record.raw_bounds)
                && raw_shape_contains(&index.placements[position], coordinate)
        },
    )
}

/// Returns the sorted object IDs whose lazy extraction dependency contains a voxel.
#[must_use]
pub fn dependency_ids_at(index: &ObjectSpatialIndex<'_>, coordinate: VoxelCoord) -> Vec<ObjectId> {
    if !coordinate.is_in_region() {
        return Vec::new();
    }
    let Some(key) = dependency_key_for(coordinate) else {
        return Vec::new();
    };
    let Ok(cell_index) = index
        .dependency_cells
        .binary_search_by_key(&key, |cell| cell.key)
    else {
        return Vec::new();
    };
    let mut ids = index.dependency_cells[cell_index]
        .members
        .iter()
        .filter_map(|&member| {
            let position = usize::try_from(member).ok()?;
            let record = index.records.get(position)?;
            let placement = index.placements.get(position)?;
            (voxel_in_bounds(coordinate, record.dependency_bounds)
                && dependency_contains(placement, coordinate))
            .then_some(placement.id)
        })
        .collect::<Vec<_>>();
    ids.sort_unstable();
    ids.dedup();
    ids
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
    predicate: impl Fn(usize, ObjectIndexRecord) -> bool,
) -> Vec<ObjectId> {
    members.sort_unstable();
    members.dedup();
    let mut ids = members
        .into_iter()
        .filter_map(|member| {
            let position = usize::try_from(member).ok()?;
            predicate(position, index.records[position]).then_some(index.placements[position].id)
        })
        .collect::<Vec<_>>();
    ids.sort_unstable();
    ids.dedup();
    ids
}

fn expand_dependency_bounds(bounds: AabbQ8) -> Option<AabbQ8> {
    let min_offset =
        |axis: fn(&super::VoxelOffset) -> i8| OBJECT_EXTRACTION_STENCIL.iter().map(axis).min();
    let max_offset =
        |axis: fn(&super::VoxelOffset) -> i8| OBJECT_EXTRACTION_STENCIL.iter().map(axis).max();
    let offset_q8 = |offset: i8| i32::from(offset).checked_mul(VOXEL_EDGE_Q8);
    let expanded = AabbQ8::new(
        WorldPointQ8::new(
            bounds
                .min
                .x
                .checked_add(offset_q8(min_offset(|offset| offset.x)?)?)?,
            bounds
                .min
                .y
                .checked_add(offset_q8(min_offset(|offset| offset.y)?)?)?,
            bounds
                .min
                .z
                .checked_add(offset_q8(min_offset(|offset| offset.z)?)?)?,
        ),
        WorldPointQ8::new(
            bounds
                .max_exclusive
                .x
                .checked_add(offset_q8(max_offset(|offset| offset.x)?)?)?,
            bounds
                .max_exclusive
                .y
                .checked_add(offset_q8(max_offset(|offset| offset.y)?)?)?,
            bounds
                .max_exclusive
                .z
                .checked_add(offset_q8(max_offset(|offset| offset.z)?)?)?,
        ),
    )
    .ok()?;
    AabbQ8::new(
        WorldPointQ8::new(
            expanded.min.x.max(REGION_XZ_MIN_Q8),
            expanded.min.y.max(REGION_Y_MIN_Q8),
            expanded.min.z.max(REGION_XZ_MIN_Q8),
        ),
        WorldPointQ8::new(
            expanded.max_exclusive.x.min(REGION_XZ_MAX_Q8_EXCLUSIVE),
            expanded.max_exclusive.y.min(REGION_Y_MAX_Q8_EXCLUSIVE),
            expanded.max_exclusive.z.min(REGION_XZ_MAX_Q8_EXCLUSIVE),
        ),
    )
    .ok()
}

fn dependency_key_for(coordinate: VoxelCoord) -> Option<DependencyGridCellKey> {
    let edge_q8 = DEPENDENCY_CELL_METERS.checked_mul(Q8_UNITS_PER_METER)?;
    Some(DependencyGridCellKey {
        x: i16::try_from(
            coordinate
                .x
                .checked_mul(VOXEL_EDGE_Q8)?
                .checked_sub(REGION_XZ_MIN_Q8)?
                .div_euclid(edge_q8),
        )
        .ok()?,
        z: i16::try_from(
            coordinate
                .z
                .checked_mul(VOXEL_EDGE_Q8)?
                .checked_sub(REGION_XZ_MIN_Q8)?
                .div_euclid(edge_q8),
        )
        .ok()?,
    })
}

fn sample_key_for(coordinate: VoxelCoord) -> Option<SampleGridCellKey> {
    let edge_q8 = SAMPLE_CELL_METERS.checked_mul(Q8_UNITS_PER_METER)?;
    Some(SampleGridCellKey {
        x: i16::try_from(coordinate.x.checked_mul(VOXEL_EDGE_Q8)?.div_euclid(edge_q8)).ok()?,
        z: i16::try_from(coordinate.z.checked_mul(VOXEL_EDGE_Q8)?.div_euclid(edge_q8)).ok()?,
    })
}

fn voxel_in_bounds(coordinate: VoxelCoord, bounds: AabbQ8) -> bool {
    let Some(x) = coordinate.x.checked_mul(VOXEL_EDGE_Q8) else {
        return false;
    };
    let Some(y) = coordinate.y.checked_mul(VOXEL_EDGE_Q8) else {
        return false;
    };
    let Some(z) = coordinate.z.checked_mul(VOXEL_EDGE_Q8) else {
        return false;
    };
    x < bounds.max_exclusive.x
        && x.checked_add(VOXEL_EDGE_Q8)
            .is_some_and(|max| max > bounds.min.x)
        && y < bounds.max_exclusive.y
        && y.checked_add(VOXEL_EDGE_Q8)
            .is_some_and(|max| max > bounds.min.y)
        && z < bounds.max_exclusive.z
        && z.checked_add(VOXEL_EDGE_Q8)
            .is_some_and(|max| max > bounds.min.z)
}

fn dependency_keys_for(bounds: AabbQ8) -> Vec<DependencyGridCellKey> {
    horizontal_range(bounds, DEPENDENCY_CELL_METERS, REGION_XZ_MIN_Q8)
        .into_iter()
        .flat_map(|(x, z)| {
            i16::try_from(x)
                .ok()
                .zip(i16::try_from(z).ok())
                .map(|(x, z)| DependencyGridCellKey { x, z })
        })
        .collect()
}

fn sample_keys_for(bounds: AabbQ8) -> Vec<SampleGridCellKey> {
    horizontal_range(bounds, SAMPLE_CELL_METERS, 0)
        .into_iter()
        .flat_map(|(x, z)| {
            i16::try_from(x)
                .ok()
                .zip(i16::try_from(z).ok())
                .map(|(x, z)| SampleGridCellKey { x, z })
        })
        .collect()
}

fn horizontal_range(bounds: AabbQ8, meters: i32, origin_q8: i32) -> Vec<(i32, i32)> {
    let edge = i64::from(meters) * i64::from(Q8_UNITS_PER_METER);
    let min_x = i64::from(bounds.min.x).max(i64::from(REGION_XZ_MIN_Q8));
    let max_x = i64::from(bounds.max_exclusive.x).min(i64::from(REGION_XZ_MAX_Q8_EXCLUSIVE));
    let min_z = i64::from(bounds.min.z).max(i64::from(REGION_XZ_MIN_Q8));
    let max_z = i64::from(bounds.max_exclusive.z).min(i64::from(REGION_XZ_MAX_Q8_EXCLUSIVE));
    if min_x >= max_x || min_z >= max_z {
        return Vec::new();
    }
    let origin_q8 = i64::from(origin_q8);
    let min_x = (min_x - origin_q8).div_euclid(edge);
    let max_x = (max_x - 1 - origin_q8).div_euclid(edge);
    let min_z = (min_z - origin_q8).div_euclid(edge);
    let max_z = (max_z - 1 - origin_q8).div_euclid(edge);
    (min_x..=max_x)
        .flat_map(move |x| (min_z..=max_z).map(move |z| (x, z)))
        .filter_map(|(x, z)| i32::try_from(x).ok().zip(i32::try_from(z).ok()))
        .collect()
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
) -> Option<u64> {
    let dependency_members = dependencies
        .iter()
        .map(|cell| cell.members.capacity())
        .try_fold(0_usize, usize::checked_add)?;
    let sample_members = samples
        .iter()
        .map(|cell| cell.members.capacity())
        .try_fold(0_usize, usize::checked_add)?;
    let member_capacity = dependency_members.checked_add(sample_members)?;
    let stored_bytes = record_capacity
        .checked_mul(size_of::<ObjectIndexRecord>())?
        .checked_add(dependency_cell_capacity.checked_mul(size_of::<DependencyGridCell>())?)?
        .checked_add(sample_cell_capacity.checked_mul(size_of::<SampleGridCell>())?)?
        .checked_add(member_capacity.checked_mul(size_of::<u32>())?)?;
    let occupied_cells = dependencies.len().checked_add(samples.len())?;
    let allocation_count = u64::try_from(occupied_cells).ok()?.checked_add(2)?;
    u64::try_from(stored_bytes)
        .ok()?
        .checked_add(ALLOCATOR_BYTES.checked_mul(allocation_count)?)
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
    let edge_voxels = HORIZON_CELL_METERS * Q8_UNITS_PER_METER / VOXEL_EDGE_Q8;
    HorizonCellKey {
        x: i16::try_from(
            anchor
                .x
                .saturating_sub(REGION_XZ_MIN_VOXEL)
                .div_euclid(edge_voxels),
        )
        .unwrap_or(if anchor.x < REGION_XZ_MIN_VOXEL {
            i16::MIN
        } else {
            i16::MAX
        }),
        z: i16::try_from(
            anchor
                .z
                .saturating_sub(REGION_XZ_MIN_VOXEL)
                .div_euclid(edge_voxels),
        )
        .unwrap_or(if anchor.z < REGION_XZ_MIN_VOXEL {
            i16::MIN
        } else {
            i16::MAX
        }),
    }
}

fn cell_bounds(key: HorizonCellKey, meters: i32) -> AabbQ8 {
    let edge = meters * Q8_UNITS_PER_METER;
    let min_x = REGION_XZ_MIN_Q8 + i32::from(key.x) * edge;
    let min_z = REGION_XZ_MIN_Q8 + i32::from(key.z) * edge;
    AabbQ8::new(
        WorldPointQ8::new(min_x, i32::MIN / 2, min_z),
        WorldPointQ8::new(min_x + edge, i32::MAX / 2, min_z + edge),
    )
    .expect("cell bounds are valid")
}

fn is_tree(kind: ObjectKind) -> bool {
    matches!(kind, ObjectKind::TreeA | ObjectKind::TreeB)
}
