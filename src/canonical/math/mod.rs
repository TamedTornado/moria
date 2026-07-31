//! Checked integer arithmetic for canonical placement values.

mod fixed;
mod placement;
#[cfg(test)]
mod wide;

pub use placement::{PlacementFixedFormat, PlacementScalar};

#[cfg(test)]
mod tests;
