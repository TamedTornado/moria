//! Moria's public infrastructure facade.
//!
//! The scaffold deliberately exposes no product operations until their bounded
//! contracts are implemented. Consumers, including `moria-qualify`, import
//! this crate through the same public boundary.

pub mod canonical;
pub mod facade;
pub mod prelude;
