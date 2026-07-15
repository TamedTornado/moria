# Issue 232 — Establish virtual-world reference-model conformance harness

References: `docs/tdd/overview.md §Verification strategy; docs/tdd/data-model.md §Relationship invariants; docs/tdd/api.md; docs/tdd/systems.md; docs/tdd/states.md; docs/tdd/implementation-plan.md`.

## Boundary contracts

- The test-only dense model independently implements small bounded base truth, registered-feature precedence, four voxel bytes, region bounds and base-relative overlays; it shares no sparse-store, feature-index, dirty-discovery, scheduler or codec implementation.
- Production cases enter through public MoriaWorldPlugin/WorldRead/WorldEditWrite/focus/save-load messages and normal named schedules. Tests may control fixed time, task completion, render acknowledgements and external I/O failures only.
- Define adapters for scalar/column/ray/capsule truth, affected dependency IDs, dirty/derived keys, lifecycle messages, persistence snapshot and terminal acknowledgement sets.

## Multi-system scenarios

- Generate bounded worlds and operation histories containing atomic/progressive dig/place, overlap, no-op, exact reversion and region-edge clipping. After every committed batch—not only at the end—compare every authoritative voxel byte and all observable query/feature results.
- Run long seeded histories, then compare after inactive caching, eviction/reactivation, exact save/load and untouched regeneration.
- Metamorphically vary request insertion/hash order, worker/task completion order, legal progressive batch partition, zero/one/many fixed ticks, active/inactive residency and persistence boundary; authoritative/public final results must remain equal.
- Cross boundaries through query/collision, dependency discovery, invalidation, owner/Horizon/dressing/water keys, stale token/revision rejection and exact terminal barrier counts.

## Failure propagation

- Inject queue full, stale/duplicate/out-of-order results and renderer acknowledgements, missing/partial task completion, write/flush/rename/decode failure, checksum/identity mismatch and restart/retry. No partial/stale derived state may become authoritative or terminally complete.
- A failed load before swap preserves truth/revision; after swap an invariant failure is fatal. A rejected edit emits no lifecycle; an accepted edit cannot disappear.
- Record seed, operation sequence, completion schedule and fault point for every failure. Shrink counterexamples while preserving semantics and promote minimized cases to deterministic regression fixtures.

## Ordering guarantees

- Public production ordering is validation/readiness -> admission -> bounded staging -> atomic commit -> invalidation/snapshot -> derived install/render ack -> primary/final completion; the oracle compares at every observable boundary.
- Provide one documented command with bounded fast CI and extended deterministic profiles. Production-scale timings remain F1/F2 assertions, but both profiles use the same instrumentation/code paths.

