//! Convenient public imports for Moria consumers.
//!
//! This module remains small and re-exports the public facade owners.

pub use crate::facade::{
    BoundedBytes, BoundedBytes64, BoundedOwnerError, BoundedPushRejected, BoundedUtf8, BoundedVec,
    BytesConstructionRejected, OwnedBytes, VecConstructionRejected,
};
