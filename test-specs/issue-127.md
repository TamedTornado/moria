# Issue 127 — Extend public generation and reads across the complete route

References: `docs/tdd/overview.md §High-level architecture`; `docs/tdd/api.md §Read-only world observations`.

## Boundary contracts

- Complete strata/cave/water/ruin/object/route evaluators feed the existing private store and existing WorldRead API; no new privileged observation is added.
- Complete curated identity is validated before readiness and before any evaluator/store access.

## Multi-system scenarios

- For every route feature/material/source, compare active, inactive, evicted/reactivated and edited samples/columns/rays/capsules/water results.
- External consumer traverses route metadata and queries complete features only through facade.

## Failure propagation

- Missing/mismatched feature or digest blocks startup; evaluator failure cannot leak partial store/readiness.
- Cache/task/order variations preserve F2 scalar and error contracts; no private generator import fallback.

## Ordering guarantees

- Validate complete manifest -> register private evaluators -> construct store -> existing readiness -> public reads.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

