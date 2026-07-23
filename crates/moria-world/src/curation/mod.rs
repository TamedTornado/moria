//! Canonical generated metadata retained by world opening and curation.

#[cfg(feature = "curation")]
mod facade;
#[cfg(feature = "curation")]
mod generate;
mod model;
#[cfg(feature = "curation")]
mod stress;

#[cfg(feature = "curation")]
pub use facade::{CurationError, CurationReport, derive_manifest, validate_manifest};
pub use model::{
    CuratedManifest, FeatureInstance, FeatureKind, ManifestError, ObjectId, ObjectKind,
    ObjectPlacement, QuantizedTransform, RouteTag, RouteWaypoint, RuinPoi, SparseVoxelStamp,
    SpeciesId, StampRun, VoxelObjectShape, WaterBodyDef, WaterKind,
};
#[cfg(feature = "curation")]
pub use stress::CurationStressTarget;
