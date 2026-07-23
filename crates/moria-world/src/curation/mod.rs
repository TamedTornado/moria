//! Canonical generated metadata retained by world opening and curation.

#[cfg(feature = "curation")]
mod facade;
#[cfg(feature = "curation")]
mod generate;
mod model;

#[cfg(feature = "curation")]
const PRODUCT_ONE_MAX_HORIZON_TREE_MEMBERS_PER_CELL: u16 = 1_024;

#[cfg(feature = "curation")]
pub use facade::{CurationError, CurationReport, derive_manifest, validate_manifest};
pub use model::{
    CuratedManifest, CuratedRoute, FeatureInstance, FeatureKind, ManifestError, ObjectId,
    ObjectKind, ObjectPlacement, QuantizedTransform, RouteTag, RouteWaypoint, RuinPoi,
    SparseVoxelStamp, SpeciesId, StampRun, VoxelObjectShape, WaterBodyDef, WaterKind,
};
