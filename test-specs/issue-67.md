# Issue 67 — Plan active distance bands from focus sources

References: `docs/tdd/config.md §Streaming and task configuration`; `docs/tdd/states.md §Chunk lifecycle`.

## Valid transitions

- `Absent + desired focus -> Requested{new token}`. Assert the target state and all specified entry/exit effects.
- `Requested + worker start -> Materializing{same token}`. Assert the target state and all specified entry/exit effects.
- `Materializing + boundary mesh needed -> Meshing{token,revision,lod}`. Assert the target state and all specified entry/exit effects.
- `Materializing + compact no-surface result -> Resident metadata-only`. Assert the target state and all specified entry/exit effects.
- `Meshing + matching install -> Resident{revision,lod}`. Assert the target state and all specified entry/exit effects.
- `Resident + edit -> Meshing while prior visual remains`. Assert the target state and all specified entry/exit effects.
- `Resident + no focus beyond hysteresis and unpinned -> EvictPending -> Absent`. Assert the target state and all specified entry/exit effects.
- `Any requested/materializing/meshing + superseding focus/edit -> same phase with new token; old result rejected`. Assert the target state and all specified entry/exit effects.
- `Horizon Absent + desired focus -> Requested{token,source_revision} -> Building{token,source_revision} -> Resident{token,source_revision}`. Assert one coherent sorted partition is installed.
- `Horizon Resident + member edit/load -> Building{new token,new source_revision}`. Assert the prior logical presentation remains until the matching partition is acknowledged.
- `Horizon Resident + no focus and unpinned -> EvictPending -> Absent`. Assert presentation/GPU buffers are removed but deltas/base descriptors remain.

## Invalid transitions

- Pinned state + eviction request; mismatched token/revision/LOD/source_revision + Resident; unchanged focus + duplicate enqueue; any phase skip that installs two logical residents.
- Verify state/queues unchanged except invariant telemetry and no delta/placement loss.

## Lifecycle ordering, guards, and concurrency

- Distances exactly at 64/160/320/720 m and ±12 m hysteresis, all focus purposes and priority order; collision broad phase always queryable.
- Concurrent edit pin and focus removal: pin wins; after acknowledgement, eviction may proceed. Moving away removes detail/presentation only.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
