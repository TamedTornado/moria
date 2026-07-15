# Issue 105 — Run catastrophic active-Horizon feasibility scenario

References: `docs/tdd/implementation-plan.md §Gate F2`; `docs/tdd/data-model.md §MutationWorkloadEvidence`.

## Boundary contracts

- One public Progressive DigSphere radius_q8=4096 targets F1's exact maximum-candidate center while an active Horizon focus exercises cards/derived/tombstones.
- Queries observe only whole committed batches; full volume is never allocated or pinned.

## Multi-system scenarios

- Require admission <=2 ms, first nonempty commit <=100 ms, primary p95/max <=250/500 ms, >=32 bricks/s, runnable wait <=500 ms, reconciliation <=30 s and every frame <=33.3 ms.
- Final evidence includes object rebuild, dressing remove/install, excluded base card, derived record/tombstone, GPU free/create and queue ack.

## Failure propagation

- Wrong F1 hash/target, asset fallback, overflow, rejection, stale install, starvation, missing branch or private reset fails before/within run and cannot be relabeled primary completion.
- Discovery+eligibility stress evidence must meet <=1 ms and all expected renderer items equal ready/removed.

## Ordering guarantees

- Progressive commits follow canonical order; primary may precede distant reconciliation; later revisions supersede older task output without resurrection.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

