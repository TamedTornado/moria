//! Pure registered-object shape evaluation and extraction dependencies.

mod index;
mod shapes;
mod validation;

pub use index::{
    DependencyGridCell, DependencyGridCellKey, HorizonCellKey, ObjectIndexConfig,
    ObjectIndexRecord, ObjectSpatialIndex, SampleGridCell, SampleGridCellKey, build_object_index,
    dependency_ids_at, horizon_tree_ids, placement_ids_in, sample_object_ids_at,
};

pub use shapes::{
    OBJECT_EXTRACTION_STENCIL, VoxelOffset, dependency_contains, evaluate_base_voxel_with_objects,
    raw_shape_bounds, raw_shape_contains, sample_object_shape, sample_sparse_stamp,
};
pub use validation::validate_object_shape_disjointness;
