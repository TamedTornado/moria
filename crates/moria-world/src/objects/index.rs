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
    validate_edit_caps(&dependency_cells, placements, &records, config)?;
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
    placements: &[ObjectPlacement],
    records: &[ObjectIndexRecord],
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
            let affected = max_affected_objects_for_edit(&members, placements, records);
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
    placements: &[ObjectPlacement],
    records: &[ObjectIndexRecord],
) -> u16 {
    let mut centers = BTreeMap::<VoxelCoord, ()>::new();
    let dependencies = members
        .iter()
        .filter_map(|&member| {
            let position = usize::try_from(member).ok()?;
            let placement = placements.get(position)?;
            let record = records.get(position)?;
            let coordinates = dependency_coordinates(placement, record.dependency_bounds);
            centers.extend(
                coordinates
                    .iter()
                    .copied()
                    .map(|coordinate| (coordinate, ())),
            );
            Some((record.dependency_bounds, coordinates))
        })
        .collect::<Vec<_>>();

    centers
        .into_keys()
        .map(|center| {
            u16::try_from(
                dependencies
                    .iter()
                    .filter(|(bounds, coordinates)| {
                        dependency_bounds_near_edit_center(*bounds, center)
                            && coordinates
                                .iter()
                                .any(|&coordinate| voxel_in_edit_sphere(coordinate, center))
                    })
                    .count(),
            )
            .unwrap_or(u16::MAX)
        })
        .max()
        .unwrap_or(0)
}

fn dependency_coordinates(placement: &ObjectPlacement, bounds: AabbQ8) -> Vec<VoxelCoord> {
    let min = VoxelCoord::new(
        bounds.min.x.div_euclid(VOXEL_EDGE_Q8),
        bounds.min.y.div_euclid(VOXEL_EDGE_Q8),
        bounds.min.z.div_euclid(VOXEL_EDGE_Q8),
    );
    let max = VoxelCoord::new(
        (bounds.max_exclusive.x - 1).div_euclid(VOXEL_EDGE_Q8),
        (bounds.max_exclusive.y - 1).div_euclid(VOXEL_EDGE_Q8),
        (bounds.max_exclusive.z - 1).div_euclid(VOXEL_EDGE_Q8),
    );
    let mut coordinates = Vec::new();
    for x in min.x..=max.x {
        for y in min.y..=max.y {
            for z in min.z..=max.z {
                let coordinate = VoxelCoord::new(x, y, z);
                if dependency_contains(placement, coordinate) {
                    coordinates.push(coordinate);
                }
            }
        }
    }
    coordinates
}

fn voxel_in_edit_sphere(coordinate: VoxelCoord, center: VoxelCoord) -> bool {
    let x = i64::from(coordinate.x) - i64::from(center.x);
    let y = i64::from(coordinate.y) - i64::from(center.y);
    let z = i64::from(coordinate.z) - i64::from(center.z);
    x * x + y * y + z * z <= i64::from(SUPPORTED_EDIT_RADIUS_VOXELS).pow(2)
}

fn dependency_bounds_near_edit_center(bounds: AabbQ8, center: VoxelCoord) -> bool {
    let radius = SUPPORTED_EDIT_RADIUS_VOXELS * VOXEL_EDGE_Q8;
    let x = center.x * VOXEL_EDGE_Q8;
    let y = center.y * VOXEL_EDGE_Q8;
    let z = center.z * VOXEL_EDGE_Q8;
    bounds.min.x < x + radius + VOXEL_EDGE_Q8
        && x - radius < bounds.max_exclusive.x
        && bounds.min.y < y + radius + VOXEL_EDGE_Q8
        && y - radius < bounds.max_exclusive.y
        && bounds.min.z < z + radius + VOXEL_EDGE_Q8
        && z - radius < bounds.max_exclusive.z
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
        x: i16::try_from(coordinate.x.checked_mul(VOXEL_EDGE_Q8)?.div_euclid(edge_q8)).ok()?,
        z: i16::try_from(coordinate.z.checked_mul(VOXEL_EDGE_Q8)?.div_euclid(edge_q8)).ok()?,
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
