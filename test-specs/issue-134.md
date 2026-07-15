# Issue 134 — Rebuild active content after load

References: `docs/tdd/api.md §Save/load protocol`; `docs/tdd/states.md §Chunk lifecycle`.

## Boundary contracts

- Post-swap union invalidation feeds collision/query materialization and terrain/water/dressing/per-placement/Horizon rebuild barrier; LoadWorldCompleted{request_id,revision,changed_voxels} is sole success terminal.
- Presentation-disabled mode waits on truth readiness only; enabled mode additionally waits on full active render queue acknowledgements.

## Multi-system scenarios

- Load adds/removes/changes object dependency and Horizon membership, water bank, ruin/terrain seams and dressing anchors; empty load; headless load.
- First view after load and eviction/reactivation must reflect post-swap deltas.

## Failure propagation

- Stale token/source revision or missing/duplicate ack cannot complete load; rebuild failure after swap is fatal, not ordinary rollback.
- No base card may resurrect for loaded edited tree; one terminal message only.

## Ordering guarantees

- Atomic swap -> invalidate union -> rebuild current revision -> renderer ack (if enabled) -> Completed -> Idle. Queries remain coherent throughout.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

