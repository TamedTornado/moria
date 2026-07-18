//! Canonical generated metadata retained by world opening and curation.

mod generate;
mod model;

pub use generate::{CurationGenerateError, canonical_manifest_ron, generate_manifest};
pub use model::{
    CuratedManifest, FeatureInstance, FeatureKind, ManifestError, ObjectId, ObjectKind,
    ObjectPlacement, QuantizedTransform, RouteTag, RouteWaypoint, RuinPoi, SparseVoxelStamp,
    SpeciesId, StampRun, VoxelObjectShape, WaterBodyDef, WaterKind,
};
