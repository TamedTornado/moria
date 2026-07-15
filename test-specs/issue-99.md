# Issue 99 — Wire feasibility mutation through staged reconciliation

References: `docs/tdd/api.md §World edit protocol`; `docs/tdd/systems.md §Mutation systems`; `docs/tdd/implementation-plan.md §Gate F2`.

## Boundary contracts

- External WorldEditWrite admission feeds canonical stage/atomic commit -> dependency invalidation -> snapshots/extraction/install -> primary and final renderer acknowledgements; public lifecycle payloads keep exact request/revision/frame/progress fields.
- No consumer path may reset/write store, inject mesh, or signal completion directly.

## Multi-system scenarios

- Interactive atomic, eight overlapping colony streams, catastrophic progressive, object/Horizon/dressing, no-op, stale result, saturation and zero-fixed-tick input.
- After each commit compare public queries/collision inside committed, not-yet-committed and outside shape; then verify exact lifecycle and final ack equality.

## Failure propagation

- Synchronous rejection emits no lifecycle; accepted capacity/task failures cannot vanish and must retry/progress or fail gate truthfully.
- Stale/out-of-order render work cannot advance primary/terminal; queue saturation affects only not-yet-accepted request.

## Ordering guarantees

- Requests/batches deterministic; commits atomic; primary may precede final; final only after all accepted batches and renderer items. No starvation or per-frame unbounded work.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

