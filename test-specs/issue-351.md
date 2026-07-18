# Issue 351 — Recovery PR #324: Enforce revision-safe streaming lifecycles

References: `docs/tdd/states.md` §Chunk lifecycle; `docs/tdd/systems.md` §Focus and streaming systems; `docs/tdd/data-model.md` §Relationship invariants.

## Valid transitions

- Terrain: `Absent -> Requested{token} -> Materializing{token} -> Meshing{token,revision,lod} -> Resident{revision,lod}`; a compact no-surface result may transition `Materializing -> Resident` metadata-only.
- A resident edit transitions back to `Meshing` at the new revision while retaining old presentation until matching replacement/removal installation.
- Unfocused, outside-hysteresis, unpinned terrain transitions `Resident -> EvictPending -> Absent`; eviction removes detail/presentation but not deltas.
- Horizon: `Absent -> Requested{token,source_revision} -> Building{token,source_revision} -> Resident{token,source_revision} -> EvictPending -> Absent`.
- Horizon member edit/load supersedes the token/source revision and rebuilds one coherent sorted partition while retaining prior logical presentation until render acknowledgement.

## Invalid transitions

- Any result with stale token, wrong brick/cell key, wrong relevant revision, wrong desired LOD/band/purpose, or superseded source revision must be discarded and cannot enter `Resident`.
- Pinned terrain/Horizon state cannot enter `EvictPending`; phase skipping cannot install two resident render entities/logical Horizon owners.
- Invalid transitions preserve truth/deltas/placements and record invariant/stale-result telemetry as specified.

## Lifecycle ordering, guards, and concurrency

- Complete normal, metadata-only, resident-edit, eviction/reactivation, Far-to-Horizon, Horizon edit, and post-load rebuilding sequences with results delivered in original, reverse, and interleaved completion order.
- Race focus removal with edit pin: pin wins until matching render acknowledgement, then eviction may proceed.
- Change desired LOD and content revision while materialization/meshing/building are in flight; only the newest token+revision+desired representation installs.
- Reactivate an edited evicted Horizon cell and assert it repartitions from current deltas rather than cached resident membership; `LoadWorldCompleted` waits for post-swap active cells.

## Properties, edge cases, and errors

- For all resident terrain layers there is at most one installed entity per `(brick, visual layer)`; for all resident Horizon cells every assigned tree is exactly once in base-card or derived/tombstone membership.
- Token/revision `u64` overflow must fail rather than wrap and accidentally match stale work.
- Empty mesh/removal and fully removed-tree tombstone are revision-bearing successful results and must satisfy barriers only after render extraction/GPU free/queue acknowledgement.

