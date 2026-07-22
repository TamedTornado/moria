#![allow(
    dead_code,
    reason = "streaming execution consumes these private state transitions in later slices"
)]

#[cfg(test)]
mod tests {
    use crate::{ActiveBand, BrickCoord, HorizonCellKey};

    use super::{ChunkLifecycle, ChunkPurpose, HorizonLifecycle, StreamLod};

    fn install_chunk(
        lifecycle: &mut ChunkLifecycle,
        brick: BrickCoord,
        lod: StreamLod,
        revision: u64,
    ) -> u64 {
        let token = lifecycle.request(brick, lod, revision, ChunkPurpose::Visual);
        assert!(lifecycle.start_materializing(brick, token, ChunkPurpose::Visual));
        assert!(lifecycle.start_meshing(brick, token, revision, lod, ChunkPurpose::Visual));
        assert!(lifecycle.install(brick, token, revision, lod, ChunkPurpose::Visual));
        token
    }

    fn install_horizon(
        lifecycle: &mut HorizonLifecycle,
        cell: HorizonCellKey,
        source_revision: u64,
    ) -> u64 {
        let token = lifecycle.request(cell, source_revision);
        assert!(lifecycle.start_building(cell, token));
        assert!(lifecycle.install(cell, token, source_revision));
        token
    }

    #[test]
    fn stale_chunk_token_revision_and_lod_results_cannot_become_resident() {
        let brick = BrickCoord::new(125, 32, 125).unwrap();
        let mut lifecycle = ChunkLifecycle::default();
        let old = lifecycle.request(brick, StreamLod::Near, 4, ChunkPurpose::Visual);
        let current = lifecycle.request(brick, StreamLod::Middle, 5, ChunkPurpose::Visual);

        assert!(!lifecycle.install(brick, old, 4, StreamLod::Near, ChunkPurpose::Visual));
        assert!(lifecycle.start_materializing(brick, current, ChunkPurpose::Visual));
        assert!(lifecycle.start_meshing(
            brick,
            current,
            5,
            StreamLod::Middle,
            ChunkPurpose::Visual
        ));
        assert!(lifecycle.install(brick, current, 5, StreamLod::Middle, ChunkPurpose::Visual));
        assert_eq!(lifecycle.resident_band(brick), Some(ActiveBand::Middle));
    }

    #[test]
    fn purpose_change_invalidates_in_flight_work_with_unequal_near_maximum_values() {
        let brick = BrickCoord::new(125, 32, 125).unwrap();
        let revision = u64::MAX - 3;
        let mut lifecycle = ChunkLifecycle {
            next_token: u64::MAX - 2,
            ..Default::default()
        };

        let visual = lifecycle.request(brick, StreamLod::Near, revision, ChunkPurpose::Visual);
        assert!(lifecycle.start_materializing(brick, visual, ChunkPurpose::Visual));
        let collision =
            lifecycle.request(brick, StreamLod::Near, revision, ChunkPurpose::Collision);

        assert_eq!(visual, u64::MAX - 1);
        assert_eq!(collision, u64::MAX);
        assert_ne!(collision, revision);
        assert!(!lifecycle.start_meshing(
            brick,
            visual,
            revision,
            StreamLod::Near,
            ChunkPurpose::Visual,
        ));
        assert!(!lifecycle.install(
            brick,
            visual,
            revision,
            StreamLod::Near,
            ChunkPurpose::Visual,
        ));
        assert!(!lifecycle.start_materializing(brick, collision, ChunkPurpose::Visual));
        assert!(lifecycle.start_materializing(brick, collision, ChunkPurpose::Collision));
        assert!(lifecycle.start_meshing(
            brick,
            collision,
            revision,
            StreamLod::Near,
            ChunkPurpose::Collision,
        ));
        assert!(!lifecycle.install(
            brick,
            collision,
            revision,
            StreamLod::Near,
            ChunkPurpose::Visual,
        ));
        assert!(lifecycle.install(
            brick,
            collision,
            revision,
            StreamLod::Near,
            ChunkPurpose::Collision,
        ));
    }

    #[test]
    fn resident_edit_meshes_directly_while_preserving_prior_presentation() {
        let brick = BrickCoord::new(125, 32, 125).unwrap();
        let mut lifecycle = ChunkLifecycle::default();
        install_chunk(&mut lifecycle, brick, StreamLod::Near, 4);

        let edit = lifecycle.request(brick, StreamLod::Near, 5, ChunkPurpose::CommittedEdit);

        assert_eq!(lifecycle.resident_band(brick), Some(ActiveBand::Near));
        assert!(lifecycle.start_meshing(
            brick,
            edit,
            5,
            StreamLod::Near,
            ChunkPurpose::CommittedEdit,
        ));
        assert_eq!(lifecycle.resident_band(brick), Some(ActiveBand::Near));
        assert!(lifecycle.install(brick, edit, 5, StreamLod::Near, ChunkPurpose::CommittedEdit,));
    }

    #[test]
    fn pins_applied_before_request_prevent_chunk_and_horizon_eviction() {
        let brick = BrickCoord::new(125, 32, 125).unwrap();
        let cell = HorizonCellKey::new(0, 0);
        let mut chunks = ChunkLifecycle::default();
        let mut horizon = HorizonLifecycle::default();

        chunks.pin(brick);
        horizon.pin(cell);
        install_chunk(&mut chunks, brick, StreamLod::Near, 1);
        install_horizon(&mut horizon, cell, 1);

        assert!(!chunks.begin_evict(brick));
        assert!(!horizon.begin_evict(cell));
    }

    #[test]
    fn replacements_keep_the_previous_presentation_until_the_new_revision_installs() {
        let brick = BrickCoord::new(125, 32, 125).unwrap();
        let cell = HorizonCellKey::new(0, 0);
        let mut chunks = ChunkLifecycle::default();
        let mut horizon = HorizonLifecycle::default();

        install_chunk(&mut chunks, brick, StreamLod::Near, 4);
        install_horizon(&mut horizon, cell, 8);

        let replacement_chunk = chunks.request(brick, StreamLod::Middle, 5, ChunkPurpose::Visual);
        let replacement_horizon = horizon.request(cell, 9);
        assert_eq!(chunks.resident_band(brick), Some(ActiveBand::Near));
        assert_eq!(horizon.presented_revision(cell), Some(8));
        assert!(!horizon.is_resident(cell));

        assert!(chunks.start_materializing(brick, replacement_chunk, ChunkPurpose::Visual));
        assert!(chunks.start_meshing(
            brick,
            replacement_chunk,
            5,
            StreamLod::Middle,
            ChunkPurpose::Visual
        ));
        assert!(chunks.install(
            brick,
            replacement_chunk,
            5,
            StreamLod::Middle,
            ChunkPurpose::Visual
        ));
        assert!(horizon.start_building(cell, replacement_horizon));
        assert!(horizon.install(cell, replacement_horizon, 9));
        assert_eq!(chunks.resident_band(brick), Some(ActiveBand::Middle));
        assert_eq!(horizon.presented_revision(cell), Some(9));
    }

    #[test]
    fn completed_chunk_token_cannot_restart_work_or_install_twice() {
        let brick = BrickCoord::new(125, 32, 125).unwrap();
        let mut lifecycle = ChunkLifecycle::default();
        let token = install_chunk(&mut lifecycle, brick, StreamLod::Near, 4);

        assert!(!lifecycle.start_materializing(brick, token, ChunkPurpose::Visual));
        assert!(!lifecycle.start_meshing(brick, token, 4, StreamLod::Near, ChunkPurpose::Visual));
        assert!(!lifecycle.install(brick, token, 4, StreamLod::Near, ChunkPurpose::Visual));
    }

    #[test]
    fn horizon_cannot_install_before_building() {
        let cell = HorizonCellKey::new(0, 0);
        let mut lifecycle = HorizonLifecycle::default();
        let token = lifecycle.request(cell, 8);

        assert!(!lifecycle.install(cell, token, 8));
        assert!(lifecycle.start_building(cell, token));
        assert!(lifecycle.install(cell, token, 8));
    }

    #[test]
    fn pinned_chunk_and_horizon_cell_cannot_evict_and_eviction_preserves_lifecycle_reactivation() {
        let brick = BrickCoord::new(125, 32, 125).unwrap();
        let cell = HorizonCellKey::new(0, 0);
        let mut chunks = ChunkLifecycle::default();
        let mut horizon = HorizonLifecycle::default();
        let chunk_token = install_chunk(&mut chunks, brick, StreamLod::Near, 1);
        let horizon_token = install_horizon(&mut horizon, cell, 1);

        chunks.pin(brick);
        horizon.pin(cell);
        assert!(!chunks.begin_evict(brick));
        assert!(!horizon.begin_evict(cell));
        chunks.unpin(brick);
        horizon.unpin(cell);
        assert!(chunks.begin_evict(brick));
        assert!(horizon.begin_evict(cell));
        assert!(chunks.finish_evict(brick));
        assert!(horizon.finish_evict(cell));
        assert!(chunks.resident_band(brick).is_none());
        assert!(!horizon.is_resident(cell));

        let next_chunk_token = chunks.request(brick, StreamLod::Near, 1, ChunkPurpose::Visual);
        let next_horizon_token = horizon.request(cell, 2);
        assert_ne!(next_chunk_token, chunk_token);
        assert_ne!(next_horizon_token, horizon_token);
        assert!(chunks.start_materializing(brick, next_chunk_token, ChunkPurpose::Visual));
        assert!(chunks.start_meshing(
            brick,
            next_chunk_token,
            1,
            StreamLod::Near,
            ChunkPurpose::Visual
        ));
        assert!(chunks.install(
            brick,
            next_chunk_token,
            1,
            StreamLod::Near,
            ChunkPurpose::Visual
        ));
        assert!(horizon.start_building(cell, next_horizon_token));
        assert!(horizon.install(cell, next_horizon_token, 2));
    }

    #[test]
    fn stale_horizon_results_cannot_replace_a_newer_partition() {
        let cell = HorizonCellKey::new(0, 0);
        let mut lifecycle = HorizonLifecycle::default();
        let old = lifecycle.request(cell, 8);
        let current = lifecycle.request(cell, 9);

        assert!(!lifecycle.install(cell, old, 8));
        assert!(lifecycle.start_building(cell, current));
        assert!(lifecycle.install(cell, current, 9));
        assert!(lifecycle.is_resident(cell));
    }
}
// Tokened private lifecycles for chunk and Horizon derived presentation.

use std::collections::BTreeMap;

use crate::{ActiveBand, BrickCoord, HorizonCellKey};

/// The requested geometric detail for a chunk-derived payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum StreamLod {
    Near,
    Middle,
    Far,
    Horizon,
}

/// The requirement a chunk-derived payload fulfills.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ChunkPurpose {
    Visual,
    Collision,
    Traversal,
    CommittedEdit,
}

impl StreamLod {
    const fn band(self) -> ActiveBand {
        match self {
            Self::Near => ActiveBand::Near,
            Self::Middle => ActiveBand::Middle,
            Self::Far => ActiveBand::Far,
            Self::Horizon => ActiveBand::Horizon,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ChunkPhase {
    Absent,
    Requested {
        token: u64,
    },
    Materializing {
        token: u64,
    },
    Meshing {
        token: u64,
        revision: u64,
        lod: StreamLod,
    },
    Resident {
        revision: u64,
        lod: StreamLod,
        meshing_token: Option<u64>,
    },
    EvictPending,
}

#[derive(Clone, Copy, Debug)]
struct ChunkEntry {
    phase: ChunkPhase,
    token: u64,
    revision: u64,
    lod: StreamLod,
    purpose: ChunkPurpose,
    presented: Option<PresentedChunk>,
}

#[derive(Clone, Copy, Debug)]
struct PresentedChunk {
    revision: u64,
    lod: StreamLod,
}

/// Owns desired chunk work and rejects results that no longer match it.
#[derive(Default, Debug)]
pub(crate) struct ChunkLifecycle {
    next_token: u64,
    entries: BTreeMap<BrickCoord, ChunkEntry>,
    pins: BTreeMap<BrickCoord, u16>,
}

impl ChunkLifecycle {
    #[allow(
        clippy::collapsible_if,
        reason = "the workspace targets Rust 2021, where the suggested let-chain requires edition 2024"
    )]
    pub(crate) fn request(
        &mut self,
        brick: BrickCoord,
        lod: StreamLod,
        revision: u64,
        purpose: ChunkPurpose,
    ) -> u64 {
        if let Some(entry) = self.entries.get(&brick) {
            if entry.lod == lod
                && entry.revision == revision
                && entry.purpose == purpose
                && !matches!(entry.phase, ChunkPhase::Absent | ChunkPhase::EvictPending)
            {
                return entry.token;
            }
        }
        let token = self.next_token();
        let presented = self.entries.get(&brick).and_then(|entry| entry.presented);
        let phase = match self.entries.get(&brick).map(|entry| entry.phase) {
            Some(ChunkPhase::Resident {
                revision: resident_revision,
                lod: resident_lod,
                ..
            }) if resident_lod == lod => ChunkPhase::Resident {
                revision: resident_revision,
                lod: resident_lod,
                meshing_token: Some(token),
            },
            _ => ChunkPhase::Requested { token },
        };
        self.entries.insert(
            brick,
            ChunkEntry {
                phase,
                token,
                revision,
                lod,
                purpose,
                presented,
            },
        );
        token
    }

    pub(crate) fn start_materializing(
        &mut self,
        brick: BrickCoord,
        token: u64,
        purpose: ChunkPurpose,
    ) -> bool {
        let Some(entry) = self.entries.get_mut(&brick) else {
            return false;
        };
        if entry.token != token
            || entry.purpose != purpose
            || !matches!(entry.phase, ChunkPhase::Requested { .. })
        {
            return false;
        }
        entry.phase = ChunkPhase::Materializing { token };
        true
    }

    pub(crate) fn start_meshing(
        &mut self,
        brick: BrickCoord,
        token: u64,
        revision: u64,
        lod: StreamLod,
        purpose: ChunkPurpose,
    ) -> bool {
        let Some(entry) = self.entries.get_mut(&brick) else {
            return false;
        };
        if entry.token != token
            || entry.revision != revision
            || entry.lod != lod
            || entry.purpose != purpose
            || !(matches!(
                entry.phase,
                ChunkPhase::Materializing { token: phase_token } if phase_token == token
            ) || matches!(
                entry.phase,
                ChunkPhase::Resident {
                    meshing_token: Some(phase_token),
                    ..
                } if phase_token == token
            ))
        {
            return false;
        }
        entry.phase = ChunkPhase::Meshing {
            token,
            revision,
            lod,
        };
        true
    }

    pub(crate) fn install(
        &mut self,
        brick: BrickCoord,
        token: u64,
        revision: u64,
        lod: StreamLod,
        purpose: ChunkPurpose,
    ) -> bool {
        let Some(entry) = self.entries.get_mut(&brick) else {
            return false;
        };
        if entry.token != token
            || entry.revision != revision
            || entry.lod != lod
            || entry.purpose != purpose
        {
            return false;
        }
        if !matches!(
            entry.phase,
            ChunkPhase::Materializing { token: phase_token } if phase_token == token
        ) && !matches!(
            entry.phase,
            ChunkPhase::Meshing {
                token: phase_token,
                revision: phase_revision,
                lod: phase_lod,
            } if phase_token == token && phase_revision == revision && phase_lod == lod
        ) {
            return false;
        }
        entry.presented = Some(PresentedChunk { revision, lod });
        entry.phase = ChunkPhase::Resident {
            revision,
            lod,
            meshing_token: None,
        };
        true
    }

    pub(crate) fn pin(&mut self, brick: BrickCoord) {
        let pins = self.pins.entry(brick).or_default();
        *pins = pins.checked_add(1).expect("chunk pin count cannot wrap");
    }

    pub(crate) fn unpin(&mut self, brick: BrickCoord) {
        if let Some(pins) = self.pins.get_mut(&brick) {
            *pins = pins.saturating_sub(1);
            if *pins == 0 {
                self.pins.remove(&brick);
            }
        }
    }

    pub(crate) fn begin_evict(&mut self, brick: BrickCoord) -> bool {
        if self.pins.contains_key(&brick) {
            return false;
        }
        let Some(entry) = self.entries.get_mut(&brick) else {
            return false;
        };
        if matches!(entry.phase, ChunkPhase::Absent | ChunkPhase::EvictPending) {
            return false;
        }
        entry.phase = ChunkPhase::EvictPending;
        true
    }

    pub(crate) fn finish_evict(&mut self, brick: BrickCoord) -> bool {
        if self.pins.contains_key(&brick) {
            return false;
        }
        let Some(entry) = self.entries.get_mut(&brick) else {
            return false;
        };
        if !matches!(entry.phase, ChunkPhase::EvictPending) {
            return false;
        }
        entry.presented = None;
        entry.phase = ChunkPhase::Absent;
        true
    }

    pub(crate) fn resident_band(&self, brick: BrickCoord) -> Option<ActiveBand> {
        self.entries
            .get(&brick)
            .and_then(|entry| entry.presented.map(|presented| presented.lod.band()))
    }

    fn next_token(&mut self) -> u64 {
        self.next_token = self
            .next_token
            .checked_add(1)
            .expect("streaming token cannot wrap");
        self.next_token
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HorizonPhase {
    Absent,
    Requested { token: u64, source_revision: u64 },
    Building { token: u64, source_revision: u64 },
    Resident { token: u64, source_revision: u64 },
    EvictPending,
}

#[derive(Clone, Copy, Debug)]
struct HorizonEntry {
    phase: HorizonPhase,
    token: u64,
    source_revision: u64,
    presented_revision: Option<u64>,
}

/// Parallel lifecycle for atomically-installed Horizon object-cell partitions.
#[derive(Default, Debug)]
pub(crate) struct HorizonLifecycle {
    next_token: u64,
    entries: BTreeMap<HorizonCellKey, HorizonEntry>,
    pins: BTreeMap<HorizonCellKey, u16>,
}

impl HorizonLifecycle {
    #[allow(
        clippy::collapsible_if,
        reason = "the workspace targets Rust 2021, where the suggested let-chain requires edition 2024"
    )]
    pub(crate) fn request(&mut self, cell: HorizonCellKey, source_revision: u64) -> u64 {
        if let Some(entry) = self.entries.get(&cell) {
            if entry.source_revision == source_revision
                && !matches!(
                    entry.phase,
                    HorizonPhase::Absent | HorizonPhase::EvictPending
                )
            {
                return entry.token;
            }
        }
        let token = self.next_token();
        let presented_revision = self
            .entries
            .get(&cell)
            .and_then(|entry| entry.presented_revision);
        self.entries.insert(
            cell,
            HorizonEntry {
                phase: HorizonPhase::Requested {
                    token,
                    source_revision,
                },
                token,
                source_revision,
                presented_revision,
            },
        );
        token
    }

    pub(crate) fn start_building(&mut self, cell: HorizonCellKey, token: u64) -> bool {
        let Some(entry) = self.entries.get_mut(&cell) else {
            return false;
        };
        if entry.token != token || !matches!(entry.phase, HorizonPhase::Requested { .. }) {
            return false;
        }
        entry.phase = HorizonPhase::Building {
            token,
            source_revision: entry.source_revision,
        };
        true
    }

    pub(crate) fn install(
        &mut self,
        cell: HorizonCellKey,
        token: u64,
        source_revision: u64,
    ) -> bool {
        let Some(entry) = self.entries.get_mut(&cell) else {
            return false;
        };
        if entry.token != token || entry.source_revision != source_revision {
            return false;
        }
        if !matches!(
            entry.phase,
            HorizonPhase::Building {
                token: phase_token,
                source_revision: phase_revision,
            } if phase_token == token && phase_revision == source_revision
        ) {
            return false;
        }
        entry.presented_revision = Some(source_revision);
        entry.phase = HorizonPhase::Resident {
            token,
            source_revision,
        };
        true
    }

    pub(crate) fn pin(&mut self, cell: HorizonCellKey) {
        let pins = self.pins.entry(cell).or_default();
        *pins = pins.checked_add(1).expect("Horizon pin count cannot wrap");
    }

    pub(crate) fn unpin(&mut self, cell: HorizonCellKey) {
        if let Some(pins) = self.pins.get_mut(&cell) {
            *pins = pins.saturating_sub(1);
            if *pins == 0 {
                self.pins.remove(&cell);
            }
        }
    }

    pub(crate) fn begin_evict(&mut self, cell: HorizonCellKey) -> bool {
        if self.pins.contains_key(&cell) {
            return false;
        }
        let Some(entry) = self.entries.get_mut(&cell) else {
            return false;
        };
        if matches!(
            entry.phase,
            HorizonPhase::Absent | HorizonPhase::EvictPending
        ) {
            return false;
        }
        entry.phase = HorizonPhase::EvictPending;
        true
    }

    pub(crate) fn finish_evict(&mut self, cell: HorizonCellKey) -> bool {
        if self.pins.contains_key(&cell) {
            return false;
        }
        let Some(entry) = self.entries.get_mut(&cell) else {
            return false;
        };
        if !matches!(entry.phase, HorizonPhase::EvictPending) {
            return false;
        }
        entry.presented_revision = None;
        entry.phase = HorizonPhase::Absent;
        true
    }

    pub(crate) fn is_resident(&self, cell: HorizonCellKey) -> bool {
        self.entries
            .get(&cell)
            .is_some_and(|entry| matches!(entry.phase, HorizonPhase::Resident { .. }))
    }

    pub(crate) fn presented_revision(&self, cell: HorizonCellKey) -> Option<u64> {
        self.entries
            .get(&cell)
            .and_then(|entry| entry.presented_revision)
    }

    fn next_token(&mut self) -> u64 {
        self.next_token = self
            .next_token
            .checked_add(1)
            .expect("Horizon token cannot wrap");
        self.next_token
    }
}
