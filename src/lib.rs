//! Moria's public infrastructure facade.
//!
//! The scaffold deliberately exposes no product operations until their bounded
//! contracts are implemented. Consumers, including `moria-qualify`, import
//! this crate through the same public boundary.

pub mod canonical;
pub mod config;
pub mod facade;
pub mod prelude;
// The receipt engine is an implementation detail until the bounded world
// owner, replay sink, and complete TECH-070 result records are available.
// Exposing it earlier would let consumers observe results for operations that
// cannot yet be performed.
mod runtime;
