//! Canonical generated metadata retained by world opening and curation.

mod model;

pub use model::{
    CuratedManifest, FeatureInstance, FeatureKind, ManifestError, ObjectId, ObjectKind,
    ObjectPlacement, QuantizedTransform, RouteTag, RouteWaypoint, RuinPoi, SparseVoxelStamp,
    SpeciesId, StampRun, VoxelObjectShape, WaterBodyDef, WaterKind,
};
