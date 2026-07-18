//! Consumer-owned streaming focus sources.

use std::collections::BTreeMap;

use bevy::prelude::*;

use crate::WorldPointQ8;

/// The reason a consumer needs streaming detail at a point.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FocusPurpose {
    Traversal,
    Camera,
    Inspection,
    Mutation,
}

/// One stable consumer-owned streaming focus source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FocusSource {
    pub id: u32,
    pub position: WorldPointQ8,
    pub purpose: FocusPurpose,
}

/// Replaces the source with the same ID, or adds a new source.
#[derive(Clone, Copy, Debug, Message, Eq, PartialEq)]
pub struct SetFocusSource(pub FocusSource);

/// Removes the focus source identified by `id`.
#[derive(Clone, Copy, Debug, Message, Eq, PartialEq)]
pub struct RemoveFocusSource {
    pub id: u32,
}

/// Private ordered focus index. Its generation invalidates diagnostic pages.
#[derive(Default, Resource)]
pub(crate) struct FocusState {
    sources: BTreeMap<u32, FocusSource>,
    generation: u64,
}

impl FocusState {
    pub(crate) fn sources(&self) -> &BTreeMap<u32, FocusSource> {
        &self.sources
    }

    pub(crate) const fn generation(&self) -> u64 {
        self.generation
    }
}

pub(crate) fn apply_focus_messages(
    mut focus_state: ResMut<FocusState>,
    mut set_sources: MessageReader<SetFocusSource>,
    mut remove_sources: MessageReader<RemoveFocusSource>,
) {
    let mut changed = false;
    for SetFocusSource(source) in set_sources.read().copied() {
        changed |= focus_state.sources.insert(source.id, source) != Some(source);
    }
    for RemoveFocusSource { id } in remove_sources.read() {
        changed |= focus_state.sources.remove(id).is_some();
    }
    if changed {
        focus_state.generation = focus_state
            .generation
            .checked_add(1)
            .expect("focus generation cannot wrap");
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::{
        FocusPurpose, FocusSource, FocusState, RemoveFocusSource, SetFocusSource,
        apply_focus_messages,
    };
    use crate::WorldPointQ8;

    #[test]
    fn focus_messages_replace_by_id_and_only_invalidate_on_real_changes() {
        let mut app = App::new();
        app.init_resource::<FocusState>()
            .add_message::<SetFocusSource>()
            .add_message::<RemoveFocusSource>()
            .add_systems(Update, apply_focus_messages);
        let source = FocusSource {
            id: 4,
            position: WorldPointQ8::new(1, 2, 3),
            purpose: FocusPurpose::Camera,
        };

        app.world_mut().write_message(SetFocusSource(source));
        app.update();
        assert_eq!(app.world().resource::<FocusState>().generation(), 1);

        app.world_mut().write_message(SetFocusSource(source));
        app.update();
        assert_eq!(app.world().resource::<FocusState>().generation(), 1);

        app.world_mut()
            .write_message(RemoveFocusSource { id: source.id });
        app.update();
        let state = app.world().resource::<FocusState>();
        assert_eq!(state.generation(), 2);
        assert!(state.sources().is_empty());
    }
}
