//! Consumer-facing, finite ownership types.

mod bounded;

pub use bounded::{
    BoundedBytes, BoundedBytes64, BoundedOwnerError, BoundedPushRejected, BoundedUtf8, BoundedVec,
    BytesConstructionRejected, OwnedBytes, VecConstructionRejected,
};
