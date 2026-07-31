//! Consumer-authorized configuration and bounded resource schemas.

mod budgets;
mod types;

pub use crate::facade::BudgetGroup;
pub use budgets::*;
pub use types::*;
