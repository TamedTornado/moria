pub mod bounded;

/// The fallibly allocated immutable reference-counted slice used by [`OwnedBytes`].
pub use triomphe::Arc as SharedArc;

pub use bounded::{
    BoundedBytes, BoundedBytes64, BoundedOwnerError, BoundedPushRejected, BoundedUtf8, BoundedVec,
    BytesConstructionRejected, OwnedBytes, VecConstructionRejected,
};
