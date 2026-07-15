# Issue 133 — Extend mutation across the complete route

References: `docs/tdd/api.md §World edit protocol`; `docs/tdd/rendering.md §Edit update path`.

## Boundary contracts

- Existing public atomic/progressive admission/commit drives complete-route truth and reconciles terrain, water, ruin, nonruin object, Horizon and dressing owners; lifecycle payload/API is unchanged.
- Consumers can neither edit voxel/delta directly nor inject payload/ack/reconciliation.

## Multi-system scenarios

- Sphere/box dig/place at each feature owner; cross water bank, ruin seam, object boundary, active Horizon; exact revert; progressive overlap and saturation.
- After every batch compare query/collision and after terminal compare owner/revision completeness.

## Failure propagation

- Stale/overlap/task failure follows existing reschedule/accounting; synchronous saturation/invalid input emits no lifecycle.
- Any omitted affected owner or premature primary/terminal result fails integration.

## Ordering guarantees

- Commit truth atomically in FixedUpdate; old presentation may remain; current replacements/removals ack; primary before possible terminal; final exactly once.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

