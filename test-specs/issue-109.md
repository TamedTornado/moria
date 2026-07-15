# Issue 109 — Compose the feasibility-mutation benchmark application

References: `docs/tdd/overview.md §Package and module structure`; `docs/tdd/api.md §Benchmark CLI`; `docs/tdd/states.md §Benchmark runner states`.

## Boundary contracts

- Thin moria-bench parses strict feasibility-mutation args, records process start before App construction, uses MoriaWorldPlugin/public facade and hands complete evidence to sole report assembler/output path.
- Exactly one scenario lifecycle runs; no gate-time source generation or private world/reset/mesh/metric path exists.

## Multi-system scenarios

- End-to-end passing controlled app covers F1 verification, loading, 300 warmup, query probe, all three workloads, reporting and exit 0.
- Argument/F1/digest/query/workload/watchdog/runtime/report-validation/output failures at every phase.

## Failure propagation

- Argument failures exit 2/no report; runtime failures exit 1/truthful failed report; atomic write failure preserves prior file.
- Compile/dependency inspection rejects private imports and fabricated direct metric injection.

## Ordering guarantees

- Boot -> VerifyingFeasibilityInput -> Loading -> Warmup(300) -> QueryCostProbe -> Running -> Reporting -> Complete; any runtime phase may -> Failed, with no Saving/RoundTrip.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

