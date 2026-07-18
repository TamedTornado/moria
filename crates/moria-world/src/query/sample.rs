//! Immutable value types returned by read-only world observations.

use crate::{
    material_present, solid_collision, water_volume, MaterialId, MaterialRegistry, RouteWaypoint,
    Voxel, VoxelCoord, WaterBodyDef,
};

/// One sampled current-truth voxel together with the revision that produced it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorldSample {
    pub coordinate: VoxelCoord,
    pub material: MaterialId,
    pub density: u8,
    pub state: u8,
    pub material_present: bool,
    pub solid_collision: bool,
    pub water_volume: bool,
    pub revision: u64,
}

impl WorldSample {
    pub(super) fn from_voxel(
        coordinate: VoxelCoord,
        voxel: Voxel,
        materials: &MaterialRegistry,
        revision: u64,
    ) -> Self {
        Self {
            coordinate,
            material: voxel.material,
            density: voxel.density,
            state: voxel.state,
            material_present: material_present(voxel),
            solid_collision: solid_collision(voxel, materials),
            water_volume: water_volume(voxel),
            revision,
        }
    }
}

/// Static-surface metadata for the curated water body at a horizontal point.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WaterSample {
    pub body_id: u32,
    pub surface_y_q8: i32,
    pub revision: u64,
}

impl WaterSample {
    pub(super) const fn from_body(body: &WaterBodyDef, revision: u64) -> Self {
        Self {
            body_id: body.id,
            surface_y_q8: body.surface_y_q8,
            revision,
        }
    }
}

/// Ordered, immutable generated route metadata.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TraversalRoute {
    waypoints: Vec<RouteWaypoint>,
}

impl TraversalRoute {
    #[allow(
        dead_code,
        reason = "world lifecycle constructs the immutable route after manifest validation"
    )]
    pub(crate) fn new(waypoints: Vec<RouteWaypoint>) -> Self {
        Self { waypoints }
    }

    #[must_use]
    pub fn waypoints(&self) -> &[RouteWaypoint] {
        &self.waypoints
    }
}

/// The currently resident streaming distance band for an active brick.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ActiveBand {
    Near,
    Middle,
    Far,
    Horizon,
}

/// A synchronous read request failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryError {
    NotReady,
    OutOfBounds,
    InvalidInput,
    LimitExceeded(QueryLimitKind),
    SnapshotExpired,
}

/// A fixed public bound exceeded before a query traverses world truth.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryLimitKind {
    RayDistance,
    RayVoxelVisits,
    CapsuleRadius,
    CapsuleHeight,
    SweepDisplacement,
    SweepCandidateWork,
    ResultCount,
    ColumnRuns,
    DiagnosticBricks,
    DiagnosticCells,
    DiagnosticChunks,
    DiagnosticFocuses,
}
