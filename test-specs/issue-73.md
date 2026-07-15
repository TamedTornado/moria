# Issue 73 — Wire the feasibility generator, store, and public reads

References: `docs/tdd/overview.md §High-level architecture`; `docs/tdd/api.md §Boundary`; `docs/tdd/implementation-plan.md §Gate F2`.

## Boundary contracts

- An external consumer reaches identity/bounds/scalar/column/water/route/active-band, ray, capsule, focus, diagnostics and telemetry only through the public facade; store/index/generator types remain private.
- The composed generator validates world, manifest and F1 identities before constructing accessible storage/read services.

## Multi-system scenarios

- For the same samples in active, inactive, evicted/reactivated and edited states, compare every scalar/object/query/diagnostic result.
- Open through normal plugin schedules, establish readiness, then exercise every F2 read from a separate consumer crate.

## Failure propagation

- Identity/digest/F1 mismatch prevents readiness and every read/edit path; no partially built index/store escapes.
- Stale materialization/cache results cannot alter public values; query errors propagate unchanged and no private fallback is used.

## Ordering guarantees

- Validation -> private store/index -> readiness -> external reads. Random worker completion and cache residency must not change results.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

