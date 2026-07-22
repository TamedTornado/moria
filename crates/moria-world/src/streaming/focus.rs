//! Consumer-owned streaming focus sources.

use std::collections::BTreeMap;

use bevy::prelude::*;

use crate::query::WorldReadState;
use crate::{BrickCoord, WorldPointQ8};

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
    by_brick: BTreeMap<BrickCoord, BTreeMap<u32, FocusSource>>,
    inactive_sources: BTreeMap<u32, FocusSource>,
    generation: u64,
}

impl FocusState {
    #[cfg(test)]
    pub(crate) fn sources(&self) -> &BTreeMap<u32, FocusSource> {
        &self.sources
    }

    pub(crate) fn sources_at(&self, brick: BrickCoord) -> Option<&BTreeMap<u32, FocusSource>> {
        self.by_brick.get(&brick)
    }

    pub(crate) fn inactive_sources(&self) -> &BTreeMap<u32, FocusSource> {
        &self.inactive_sources
    }

    pub(crate) const fn generation(&self) -> u64 {
        self.generation
    }

    pub(crate) fn set_brick_active(&mut self, brick: BrickCoord, active: bool) {
        let Some(sources) = self.by_brick.get(&brick) else {
            return;
        };
        if active {
            for id in sources.keys() {
                self.inactive_sources.remove(id);
            }
        } else {
            self.inactive_sources
                .extend(sources.iter().map(|(&id, &source)| (id, source)));
        }
    }

    fn remove_source_from_indexes(&mut self, source: FocusSource) {
        let Some(brick) = focus_brick(source) else {
            self.inactive_sources.remove(&source.id);
            return;
        };
        let remove_brick = {
            let sources = self
                .by_brick
                .get_mut(&brick)
                .expect("indexed focus brick must contain its source");
            sources.remove(&source.id);
            sources.is_empty()
        };
        if remove_brick {
            self.by_brick.remove(&brick);
        }
        self.inactive_sources.remove(&source.id);
    }

    fn insert_source_into_indexes(&mut self, source: FocusSource, active: bool) {
        let Some(brick) = focus_brick(source) else {
            self.inactive_sources.insert(source.id, source);
            return;
        };
        self.by_brick
            .entry(brick)
            .or_default()
            .insert(source.id, source);
        if !active {
            self.inactive_sources.insert(source.id, source);
        }
    }
}

pub(crate) fn apply_focus_messages(
    mut focus_state: ResMut<FocusState>,
    mut set_sources: MessageReader<SetFocusSource>,
    mut remove_sources: MessageReader<RemoveFocusSource>,
    world: Option<Res<WorldReadState>>,
) {
    let mut changed = false;
    for SetFocusSource(source) in set_sources.read().copied() {
        let previous = focus_state.sources.insert(source.id, source);
        if previous == Some(source) {
            continue;
        }
        if let Some(previous) = previous {
            focus_state.remove_source_from_indexes(previous);
        }
        let active = focus_brick(source).is_some_and(|brick| {
            world
                .as_deref()
                .is_some_and(|world| world.is_active_brick(brick))
        });
        focus_state.insert_source_into_indexes(source, active);
        changed = true;
    }
    for RemoveFocusSource { id } in remove_sources.read() {
        if let Some(source) = focus_state.sources.remove(id) {
            focus_state.remove_source_from_indexes(source);
            changed = true;
        }
    }
    if changed {
        focus_state.generation = focus_state
            .generation
            .checked_add(1)
            .expect("focus generation cannot wrap");
    }
}

fn focus_brick(source: FocusSource) -> Option<BrickCoord> {
    source
        .position
        .to_voxel_coord()
        .ok()
        .and_then(|point| point.to_brick_coord().ok())
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
