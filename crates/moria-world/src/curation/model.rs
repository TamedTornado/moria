//! Serializable, deterministic generated-world metadata.

use std::{collections::BTreeMap, error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::{AIR, AabbQ8, CUT_STONE, MaterialId, VoxelCoord, WorldPointQ8};

pub const MAX_FEATURE_INSTANCES: usize = 16;
pub const MAX_ROUTE_WAYPOINTS: usize = 64;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObjectId(pub u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SpeciesId(pub u16);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum FeatureKind {
    Topsoil,
    Subsoil,
    Stratum,
    KarstCave,
    Aquifer,
    IronVein,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureInstance {
    pub id: u32,
    pub kind: FeatureKind,
    pub bounds: AabbQ8,
    pub host_material: MaterialId,
    pub depth_q8: i32,
    pub orientation_q16: [i32; 4],
    pub generator_key: u64,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum WaterKind {
    River,
    Lake,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WaterBodyDef {
    pub id: u32,
    pub kind: WaterKind,
    pub surface_y_q8: i32,
    pub footprint: Vec<WorldPointQ8>,
    pub bed_profile_key: u64,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum ObjectKind {
    TreeA,
    TreeB,
    Bush,
    Boulder,
    Stump,
    Rock,
    Ruin,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuantizedTransform {
    pub translation: WorldPointQ8,
    pub yaw_quarter_turns: u8,
    pub uniform_scale_q8: u16,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum VoxelObjectShape {
    Tree {
        trunk_radius_q8: u16,
        trunk_height_q8: u16,
        canopy_radii_q8: [u16; 3],
    },
    Bush {
        radii_q8: [u16; 3],
    },
    Boulder {
        radii_q8: [u16; 3],
        perturbation_key: u64,
    },
    Stump {
        radius_q8: u16,
        height_q8: u16,
    },
    Rock {
        radii_q8: [u16; 3],
        perturbation_key: u64,
    },
    SparseStamp {
        asset_key: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectPlacement {
    pub id: ObjectId,
    pub kind: ObjectKind,
    pub transform_q: QuantizedTransform,
    pub species: Option<SpeciesId>,
    pub shape: VoxelObjectShape,
    pub anchor: VoxelCoord,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuinPoi {
    pub placement: ObjectPlacement,
    pub stair_bottom: WorldPointQ8,
    pub stair_top: WorldPointQ8,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum RouteTag {
    Meadow,
    Forest,
    River,
    Lake,
    CliffTop,
    RockShelves,
    RuinStairBottom,
    RuinStairTop,
    CaveMouth,
    Aquifer,
    OreVein,
    CaveFloor,
    SignatureCarveHillside,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteWaypoint {
    pub order: u8,
    pub point: WorldPointQ8,
    pub tags: Vec<RouteTag>,
}

/// The ordered, canonically tagged traversal authored in a curated manifest.
///
/// This is distinct from the read-only [`crate::TraversalRoute`] snapshot API.
pub type CuratedRoute = Vec<RouteWaypoint>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StampRun {
    pub start_linear: u32,
    pub len: u16,
    pub palette_index: u8,
    pub density: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SparseVoxelStamp {
    pub key: String,
    pub size_voxels: [u16; 3],
    pub pivot_voxel: [i16; 3],
    pub palette: Vec<MaterialId>,
    pub runs: Vec<StampRun>,
    pub tags: BTreeMap<String, VoxelCoord>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CuratedManifest {
    pub seed: u64,
    pub parameters_digest: [u8; 32],
    pub generated_by: String,
    pub features: Vec<FeatureInstance>,
    pub water_bodies: Vec<WaterBodyDef>,
    pub objects: Vec<ObjectPlacement>,
    pub ruin: RuinPoi,
    pub route: CuratedRoute,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ManifestError {
    FeatureCountExceedsMaximum {
        actual: usize,
        maximum: usize,
    },
    RouteWaypointCountExceedsMaximum {
        actual: usize,
        maximum: usize,
    },
    FeatureIdsNotStrictlyAscending,
    WaterBodyIdsNotStrictlyAscending,
    ObjectIdsNotStrictlyAscending,
    RouteOrdersNotStrictlyAscending,
    RouteTagsNotStrictlyAscending {
        order: u8,
    },
    RuinObjectIdMustBeZero,
    RuinPlacementMustUseRuinKind,
    RuinPlacementMustUseSparseStamp,
    ObjectIdZeroIsReserved,
    ObjectPlacementMayNotUseRuinKind,
    ObjectPlacementMayNotUseSparseStamp,
    StampHasEmptyDimension,
    StampPivotOutOfBounds,
    StampVolumeExceedsAddressSpace,
    StampPaletteIsEmpty,
    StampPaletteContainsUnsupportedMaterial,
    StampRunHasZeroLength,
    StampRunPaletteIndexOutOfBounds,
    StampRunDensityDoesNotMatchMaterial,
    StampRunExceedsVolume,
    StampRunsNotStrictlyAscending,
    StampTagOutOfBounds {
        tag: String,
    },
    ObjectRawBoundsUnavailable {
        object_id: ObjectId,
    },
    ObjectDependencyBoundsOverflow {
        object_id: ObjectId,
    },
    ObjectIndexCellsExceeded {
        object_id: ObjectId,
        actual: u16,
        maximum: u8,
    },
    ObjectDependencyBricksExceeded {
        object_id: ObjectId,
        actual: u16,
        maximum: u16,
    },
    ObjectIndexCellCapacityExceeded {
        actual: u16,
        maximum: u16,
    },
    ObjectSampleCellCapacityExceeded {
        actual: u16,
        maximum: u8,
    },
    ObjectEditCandidatesExceeded {
        actual: u16,
        maximum: u16,
    },
    ObjectEditAffectedExceeded {
        actual: u16,
        maximum: u8,
    },
    HorizonTreeCellCapacityExceeded {
        actual: u16,
        maximum: u16,
    },
    ObjectIndexRetainedBytesExceeded {
        actual: u64,
        maximum: u32,
    },
    ObjectShapeOverlap {
        lower_id: ObjectId,
        higher_id: ObjectId,
        first_voxel: VoxelCoord,
    },
    ObjectRuinOverlap {
        object_id: ObjectId,
        first_voxel: VoxelCoord,
    },
}

impl fmt::Display for ManifestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid curated manifest: {self:?}")
    }
}

impl Error for ManifestError {}

impl CuratedManifest {
    /// Validates bounded collections and their canonical serialized order.
    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.features.len() > MAX_FEATURE_INSTANCES {
            return Err(ManifestError::FeatureCountExceedsMaximum {
                actual: self.features.len(),
                maximum: MAX_FEATURE_INSTANCES,
            });
        }
        if self.route.len() > MAX_ROUTE_WAYPOINTS {
            return Err(ManifestError::RouteWaypointCountExceedsMaximum {
                actual: self.route.len(),
                maximum: MAX_ROUTE_WAYPOINTS,
            });
        }
        if !strictly_ascending(&self.features, |feature| feature.id) {
            return Err(ManifestError::FeatureIdsNotStrictlyAscending);
        }
        if !strictly_ascending(&self.water_bodies, |water| water.id) {
            return Err(ManifestError::WaterBodyIdsNotStrictlyAscending);
        }
        if !strictly_ascending(&self.objects, |object| object.id) {
            return Err(ManifestError::ObjectIdsNotStrictlyAscending);
        }
        if !strictly_ascending(&self.route, |waypoint| waypoint.order) {
            return Err(ManifestError::RouteOrdersNotStrictlyAscending);
        }
        for waypoint in &self.route {
            if !strictly_ascending(&waypoint.tags, |tag| *tag) {
                return Err(ManifestError::RouteTagsNotStrictlyAscending {
                    order: waypoint.order,
                });
            }
        }
        if self.ruin.placement.id != ObjectId(0) {
            return Err(ManifestError::RuinObjectIdMustBeZero);
        }
        if self.ruin.placement.kind != ObjectKind::Ruin {
            return Err(ManifestError::RuinPlacementMustUseRuinKind);
        }
        if !matches!(
            self.ruin.placement.shape,
            VoxelObjectShape::SparseStamp { .. }
        ) {
            return Err(ManifestError::RuinPlacementMustUseSparseStamp);
        }
        for object in &self.objects {
            if object.id == ObjectId(0) {
                return Err(ManifestError::ObjectIdZeroIsReserved);
            }
            if object.kind == ObjectKind::Ruin {
                return Err(ManifestError::ObjectPlacementMayNotUseRuinKind);
            }
            if matches!(object.shape, VoxelObjectShape::SparseStamp { .. }) {
                return Err(ManifestError::ObjectPlacementMayNotUseSparseStamp);
            }
        }
        Ok(())
    }
}

impl SparseVoxelStamp {
    /// Validates compact RLE stamp data without expanding any voxel collection.
    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.size_voxels.contains(&0) {
            return Err(ManifestError::StampHasEmptyDimension);
        }
        if self
            .pivot_voxel
            .iter()
            .zip(self.size_voxels)
            .any(|(&pivot, edge)| pivot < 0 || i32::from(pivot) >= i32::from(edge))
        {
            return Err(ManifestError::StampPivotOutOfBounds);
        }
        if self.palette.is_empty() {
            return Err(ManifestError::StampPaletteIsEmpty);
        }
        if !self
            .palette
            .iter()
            .all(|material| matches!(*material, AIR | CUT_STONE))
        {
            return Err(ManifestError::StampPaletteContainsUnsupportedMaterial);
        }
        let volume = self
            .size_voxels
            .iter()
            .try_fold(1_u32, |product, edge| product.checked_mul(u32::from(*edge)))
            .ok_or(ManifestError::StampVolumeExceedsAddressSpace)?;
        let mut previous_end = 0;
        for run in &self.runs {
            if run.len == 0 {
                return Err(ManifestError::StampRunHasZeroLength);
            }
            if usize::from(run.palette_index) >= self.palette.len() {
                return Err(ManifestError::StampRunPaletteIndexOutOfBounds);
            }
            let material = self.palette[usize::from(run.palette_index)];
            if (material == AIR && run.density != 0)
                || (material == CUT_STONE && run.density != u8::MAX)
            {
                return Err(ManifestError::StampRunDensityDoesNotMatchMaterial);
            }
            let Some(end) = run.start_linear.checked_add(u32::from(run.len)) else {
                return Err(ManifestError::StampRunExceedsVolume);
            };
            if end > volume {
                return Err(ManifestError::StampRunExceedsVolume);
            }
            if run.start_linear < previous_end {
                return Err(ManifestError::StampRunsNotStrictlyAscending);
            }
            previous_end = end;
        }
        for (tag, coordinate) in &self.tags {
            if coordinate.x < 0
                || coordinate.y < 0
                || coordinate.z < 0
                || coordinate.x >= i32::from(self.size_voxels[0])
                || coordinate.y >= i32::from(self.size_voxels[1])
                || coordinate.z >= i32::from(self.size_voxels[2])
            {
                return Err(ManifestError::StampTagOutOfBounds { tag: tag.clone() });
            }
        }
        Ok(())
    }
}

fn strictly_ascending<T, K: Ord>(values: &[T], key: impl Fn(&T) -> K) -> bool {
    values.windows(2).all(|pair| key(&pair[0]) < key(&pair[1]))
}
