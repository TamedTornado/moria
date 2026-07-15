# Issue 164 — Extend the public benchmark application with production scenarios

References: `docs/tdd/overview.md §Package and module structure`; `docs/tdd/states.md §Benchmark runner states`; `docs/tdd/benchmarks.md §Run lifecycle`.

## Boundary contracts

- Thin benchmark preserves feasibility-mutation and adds exactly one selected flythrough or mutation-workloads lifecycle through public world/persistence APIs.
- No private reset/store/mesh/accounting/codec path or scenario metric fabrication is linked.

## Multi-system scenarios

- End-to-end flythrough: Loading/Warmup/Running/Reporting. Mutation: Loading/Warmup/Running/Saving/RoundTrip/Reporting. Existing feasibility lifecycle regression.
- Watchdog/runtime/lifecycle/report/save/round-trip failure at each boundary and strict CLI selection.

## Failure propagation

- Failures write truthful complete/null report/exit 1 where possible; argument errors no report/exit 2; validation gates exit 0.
- Dependency/compile inspection rejects private moria-world access and alternate persistence.

## Ordering guarantees

- State transitions exactly match states.md; explicit events only, 300 warmup, atomic output last.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

