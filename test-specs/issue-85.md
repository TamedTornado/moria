# Issue 85 — Implement revision-safe Horizon object cells

References: `docs/tdd/states.md §Chunk lifecycle/Horizon`; `docs/tdd/data-model.md §Objects, ruin, and dressing`.

## Valid transitions

- `Absent + Horizon focus -> Requested{token,source_revision}`. Assert the target state and all specified entry/exit effects.
- `Requested + build start -> Building{same token,source_revision}`. Assert the target state and all specified entry/exit effects.
- `Building + complete partition and render acknowledgement -> Resident{token,source_revision}`. Assert the target state and all specified entry/exit effects.
- `Resident + member edit/load -> Building{new token,new source_revision}, retaining old logical presentation`. Assert the target state and all specified entry/exit effects.
- `Resident + no focus and unpinned -> EvictPending -> Absent`. Assert the target state and all specified entry/exit effects.
- `VoxelDerived member + exact removal of all dependency deltas -> Intact on rebuilt partition`. Assert the target state and all specified entry/exit effects.

## Invalid transitions

- Mismatched token/revision build -> Resident; pinned -> EvictPending; retiring Far roots before complete current Horizon partition; reusing evicted filtered payload by cell key alone.
- State and installed membership remain current; stale output is discarded.

## Lifecycle ordering, guards, and concurrency

- Every resident cell has sorted disjoint base_card_ids/derived IDs whose union equals all assigned visible trees exactly once; ineligible/erased IDs appear only in derived, with empty tombstone allowed.
- Race edit/load/eviction/build: edit/load supersedes token, pin defeats eviction, LoadWorldCompleted waits for active cell revision, reactivation repartitions current deltas.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

