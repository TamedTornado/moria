#![allow(
    dead_code,
    reason = "streaming execution consumes these private state transitions in later slices"
)]

#[cfg(test)]
mod tests {
    use crate::{ActiveBand, BrickCoord, HorizonCellKey};

    use super::{ChunkLifecycle, HorizonLifecycle, StreamLod};

    #[test]
    fn stale_chunk_token_revision_and_lod_results_cannot_become_resident() {
        let brick = BrickCoord::new(125, 32, 125).unwrap();
        let mut lifecycle = ChunkLifecycle::default();
        let old = lifecycle.request(brick, StreamLod::Near, 4);
        let current = lifecycle.request(brick, StreamLod::Middle, 5);

        assert!(!lifecycle.install(brick, old, 4, StreamLod::Near));
        assert!(lifecycle.install(brick, current, 5, StreamLod::Middle));
        assert_eq!(lifecycle.resident_band(brick), Some(ActiveBand::Middle));
    }

    #[test]
    fn pinned_chunk_and_horizon_cell_cannot_evict_and_eviction_preserves_lifecycle_reactivation() {
        let brick = BrickCoord::new(125, 32, 125).unwrap();
        let cell = HorizonCellKey::new(0, 0);
        let mut chunks = ChunkLifecycle::default();
        let mut horizon = HorizonLifecycle::default();
        let chunk_token = chunks.request(brick, StreamLod::Near, 1);
        let horizon_token = horizon.request(cell, 1);
        assert!(chunks.install(brick, chunk_token, 1, StreamLod::Near));
        assert!(horizon.install(cell, horizon_token, 1));

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

        let next_chunk_token = chunks.request(brick, StreamLod::Near, 1);
        let next_horizon_token = horizon.request(cell, 2);
        assert_ne!(next_chunk_token, chunk_token);
        assert_ne!(next_horizon_token, horizon_token);
        assert!(chunks.install(brick, next_chunk_token, 1, StreamLod::Near));
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
    },
    EvictPending,
}

#[derive(Clone, Copy, Debug)]
struct ChunkEntry {
    phase: ChunkPhase,
    token: u64,
    revision: u64,
    lod: StreamLod,
    pins: u16,
}

/// Owns desired chunk work and rejects results that no longer match it.
#[derive(Default, Debug)]
pub(crate) struct ChunkLifecycle {
    next_token: u64,
    entries: BTreeMap<BrickCoord, ChunkEntry>,
}

impl ChunkLifecycle {
    #[allow(
        clippy::collapsible_if,
        reason = "the workspace targets Rust 2021, where the suggested let-chain requires edition 2024"
    )]
    pub(crate) fn request(&mut self, brick: BrickCoord, lod: StreamLod, revision: u64) -> u64 {
        if let Some(entry) = self.entries.get(&brick) {
            if entry.lod == lod
                && entry.revision == revision
                && !matches!(entry.phase, ChunkPhase::Absent | ChunkPhase::EvictPending)
            {
                return entry.token;
            }
        }
        let token = self.next_token();
        let pins = self.entries.get(&brick).map_or(0, |entry| entry.pins);
        self.entries.insert(
            brick,
            ChunkEntry {
                phase: ChunkPhase::Requested { token },
                token,
                revision,
                lod,
                pins,
            },
        );
        token
    }

    pub(crate) fn start_materializing(&mut self, brick: BrickCoord, token: u64) -> bool {
        let Some(entry) = self.entries.get_mut(&brick) else {
            return false;
        };
        if entry.token != token || !matches!(entry.phase, ChunkPhase::Requested { .. }) {
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
    ) -> bool {
        let Some(entry) = self.entries.get_mut(&brick) else {
            return false;
        };
        if entry.token != token || entry.revision != revision || entry.lod != lod {
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
    ) -> bool {
        let Some(entry) = self.entries.get_mut(&brick) else {
            return false;
        };
        if entry.token != token || entry.revision != revision || entry.lod != lod {
            return false;
        }
        if !matches!(
            entry.phase,
            ChunkPhase::Requested { .. }
                | ChunkPhase::Materializing { .. }
                | ChunkPhase::Meshing { .. }
        ) {
            return false;
        }
        entry.phase = ChunkPhase::Resident { revision, lod };
        true
    }

    pub(crate) fn pin(&mut self, brick: BrickCoord) {
        if let Some(entry) = self.entries.get_mut(&brick) {
            entry.pins = entry
                .pins
                .checked_add(1)
                .expect("chunk pin count cannot wrap");
        }
    }

    pub(crate) fn unpin(&mut self, brick: BrickCoord) {
        if let Some(entry) = self.entries.get_mut(&brick) {
            entry.pins = entry.pins.saturating_sub(1);
        }
    }

    pub(crate) fn begin_evict(&mut self, brick: BrickCoord) -> bool {
        let Some(entry) = self.entries.get_mut(&brick) else {
            return false;
        };
        if entry.pins != 0 || matches!(entry.phase, ChunkPhase::Absent | ChunkPhase::EvictPending) {
            return false;
        }
        entry.phase = ChunkPhase::EvictPending;
        true
    }

    pub(crate) fn finish_evict(&mut self, brick: BrickCoord) -> bool {
        let Some(entry) = self.entries.get_mut(&brick) else {
            return false;
        };
        if !matches!(entry.phase, ChunkPhase::EvictPending) {
            return false;
        }
        entry.phase = ChunkPhase::Absent;
        true
    }

    pub(crate) fn resident_band(&self, brick: BrickCoord) -> Option<ActiveBand> {
        self.entries
            .get(&brick)
            .and_then(|entry| match entry.phase {
                ChunkPhase::Resident { lod, .. } => Some(lod.band()),
                _ => None,
            })
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
    pins: u16,
}

/// Parallel lifecycle for atomically-installed Horizon object-cell partitions.
#[derive(Default, Debug)]
pub(crate) struct HorizonLifecycle {
    next_token: u64,
    entries: BTreeMap<HorizonCellKey, HorizonEntry>,
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
        let pins = self.entries.get(&cell).map_or(0, |entry| entry.pins);
        self.entries.insert(
            cell,
            HorizonEntry {
                phase: HorizonPhase::Requested {
                    token,
                    source_revision,
                },
                token,
                source_revision,
                pins,
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
            HorizonPhase::Requested { .. } | HorizonPhase::Building { .. }
        ) {
            return false;
        }
        entry.phase = HorizonPhase::Resident {
            token,
            source_revision,
        };
        true
    }

    pub(crate) fn pin(&mut self, cell: HorizonCellKey) {
        if let Some(entry) = self.entries.get_mut(&cell) {
            entry.pins = entry
                .pins
                .checked_add(1)
                .expect("Horizon pin count cannot wrap");
        }
    }

    pub(crate) fn unpin(&mut self, cell: HorizonCellKey) {
        if let Some(entry) = self.entries.get_mut(&cell) {
            entry.pins = entry.pins.saturating_sub(1);
        }
    }

    pub(crate) fn begin_evict(&mut self, cell: HorizonCellKey) -> bool {
        let Some(entry) = self.entries.get_mut(&cell) else {
            return false;
        };
        if entry.pins != 0
            || matches!(
                entry.phase,
                HorizonPhase::Absent | HorizonPhase::EvictPending
            )
        {
            return false;
        }
        entry.phase = HorizonPhase::EvictPending;
        true
    }

    pub(crate) fn finish_evict(&mut self, cell: HorizonCellKey) -> bool {
        let Some(entry) = self.entries.get_mut(&cell) else {
            return false;
        };
        if !matches!(entry.phase, HorizonPhase::EvictPending) {
            return false;
        }
        entry.phase = HorizonPhase::Absent;
        true
    }

    pub(crate) fn is_resident(&self, cell: HorizonCellKey) -> bool {
        self.entries
            .get(&cell)
            .is_some_and(|entry| matches!(entry.phase, HorizonPhase::Resident { .. }))
    }

    fn next_token(&mut self) -> u64 {
        self.next_token = self
            .next_token
            .checked_add(1)
            .expect("Horizon token cannot wrap");
        self.next_token
    }
}
