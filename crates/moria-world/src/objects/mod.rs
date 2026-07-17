//! Pure registered-object shape evaluation and extraction dependencies.

mod shapes;

pub use shapes::{
    OBJECT_EXTRACTION_STENCIL, VoxelOffset, dependency_contains, raw_shape_bounds,
    raw_shape_contains, sample_object_shape, sample_sparse_stamp,
};
