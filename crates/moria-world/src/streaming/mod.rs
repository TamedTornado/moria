//! Public streaming inputs and the private state they update.

mod focus;

pub use focus::{FocusPurpose, FocusSource, RemoveFocusSource, SetFocusSource};

pub(crate) use focus::{FocusState, apply_focus_messages};
