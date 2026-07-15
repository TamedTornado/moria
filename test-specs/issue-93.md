# Issue 93 — Wire feasibility streaming through render installation

References: `docs/tdd/overview.md §High-level architecture`; `docs/tdd/rendering.md §Scene hierarchy`; `docs/tdd/implementation-plan.md §Gate F2`.

## Boundary contracts

- Focus/band plan feeds materialization -> immutable snapshots -> terrain/seam/object/Horizon/dressing/water extraction -> portable material install -> render acknowledgement; every payload carries key/token/source revision/LOD/owner.
- Eviction frees derived presentation/ledger entries but never store deltas or immutable placements.

## Multi-system scenarios

- Signature focus in Near plus stress focus with active Horizon; move through all bands and back; edit object/terrain/water/dressing owners then reconcile.
- Evict and reactivate edited content and compare current public truth and unique installed owners.

## Failure propagation

- Any stale/superseded stage result is rejected and rescheduled; partial install cannot become resident or acknowledged.
- Missing asset/shader fallback makes feasibility evidence fail; allocation/task errors cannot erase truth.

## Ordering guarantees

- Priority edit/collision/camera work precedes ordinary/prefetch; worker completion permutation cannot alter final representations or owners.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

