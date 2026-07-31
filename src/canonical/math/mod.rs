//! Checked integer arithmetic for canonical placement values.

mod fixed;
mod wide;

pub use fixed::{FixedI32, floor_div, floor_shift_right};
pub use wide::WideI64;
