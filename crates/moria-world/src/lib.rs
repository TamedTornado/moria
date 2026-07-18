//! Public facade for the reusable sparse voxel world.
//!
//! Consumer packages depend on this crate only through its explicit public API.

use bevy::prelude::*;

pub mod config;
pub mod config_validation;
pub mod curation;
pub mod generation;
mod lifecycle;
pub mod objects;
pub mod presentation;
mod query;
mod storage;
pub mod telemetry;
pub mod terrain;
pub mod testing;

pub use config::{
    ActionBinding, BandConfig, BenchmarkConfig, BiomeConfig, BoundsConfig, CameraConfig,
    CapsuleConfig, CaveConfig, CaveLightConfig, CollisionClass, GeologyConfig, InputAction,
    InputConfig, MaterialDef, MaterialId, MaterialRegistry, MutationConfig, ObjectGenConfig,
    PresentationConfig, RangeQ8, RegionConfig, RenderingConfig, StreamingConfig, SurfaceClass,
    TerrainGenConfig, WaterGenConfig, WindowConfig, parameters_digest_from_bytes,
};
pub use config_validation::{
    ConfigValidationError, validate_input_config, validate_material_registry,
    validate_presentation_config, validate_region_config,
};
pub use curation::{
    CuratedManifest, CurationGenerateError, FeatureInstance, FeatureKind, ManifestError, ObjectId,
    ObjectKind, ObjectPlacement, QuantizedTransform, RouteTag, RouteWaypoint, RuinPoi,
    SparseVoxelStamp, SpeciesId, StampRun, VoxelObjectShape, WaterBodyDef, WaterKind,
    canonical_manifest_ron, generate_manifest,
};
pub use generation::{
    AabbQ8, BiomeId, ColumnRun, ColumnSample, ProceduralClass, RunKind, WorldBounds, WorldIdentity,
    WorldSeed, biome_at, classify_brick, evaluate_base_voxel, evaluate_column,
};
pub use lifecycle::{
    SubmitError, WorldEditCommand, WorldEditWrite, WorldLifecycle, WorldLifecycleInvariantError,
    WorldLifecyclePhase, WorldLifecycleTransition, WorldOpenError,
};
pub use objects::{
    DependencyGridCell, DependencyGridCellKey, HorizonCellKey, OBJECT_EXTRACTION_STENCIL,
    ObjectIndexConfig, ObjectIndexRecord, ObjectSpatialIndex, SampleGridCell, SampleGridCellKey,
    VoxelOffset, build_object_index, dependency_contains, horizon_tree_ids, placement_ids_in,
    raw_shape_bounds, raw_shape_contains, sample_object_shape, sample_sparse_stamp,
    validate_object_shape_disjointness,
};
pub use query::{
    ActiveBand, CapsuleQ8, MAX_CAPSULE_HALF_SEGMENT_Q8, MAX_CAPSULE_RADIUS_Q8,
    MAX_OVERLAP_CANDIDATE_TESTS, MAX_QUERY_HITS, MAX_SWEEP_CANDIDATE_TESTS,
    MAX_RAY_DISTANCE_Q8, MAX_RAY_VOXEL_VISITS, MAX_SWEEP_DISPLACEMENT_Q8,
    MIN_CAPSULE_RADIUS_Q8, MatchedQueryMask, QueryError, QueryLimitKind, QueryMask, SweepResult,
    TraversalRoute, Vec3Q8, WaterSample, WorldHit, WorldNormal, WorldRayQ8, WorldRead,
    WorldSample,
};
pub use storage::{
    AIR, BRICK_EDGE_VOXELS, BrickCoord, CUT_STONE, ColumnCoord, CoordinateError, GRANITE, GRAVEL,
    IRON_ORE, LEAF, LIMESTONE, Q8_UNITS_PER_METER, SAND, SANDSTONE, SHALE, SUBSOIL, TOPSOIL,
    VOXEL_EDGE_Q8, Voxel, VoxelCoord, WATER, WOOD, WorldPointQ8, material_present, solid_collision,
    water_volume,
};
pub use terrain::{SolidPresentationOwner, VoxelSource, solid_presentation_owner};

/// Installs the reusable world feature set.
///
/// Feature plugins are added here as their public contracts are implemented.
pub struct MoriaWorldPlugin;

impl Plugin for MoriaWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldLifecycle>();
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::MoriaWorldPlugin;

    #[test]
    fn public_world_plugin_installs_in_a_headless_app() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins)
            .add_plugins(MoriaWorldPlugin);
        app.update();
    }
}
