# Issue 94 — Capture benchmark timing, allocation, streaming, and machine metrics

References: `docs/tdd/benchmarks.md §Metric definitions`; `docs/tdd/data-model.md §Benchmark data`.

## Properties that must hold

- For every capture window, warmup 300 frames is excluded from frame distributions but retained in allocations/startup; distributions compute exact min/p50/p95/p99/max and one-percent-low from finite samples.
- For every metric set, scenario/profile/resolution/world/build/assets/machine identity remains attached; distinct machine profiles are never merged.

## Entity configurations to test

- 0/1/even/odd sample distributions, tied values, slowest 1%; window focus loss/occlusion; driver present and unavailable.
- Ledger-only, valid resident measurement, unified/discrete machine, repeated profile normalization.

## Edge cases and type boundaries

- Empty/nonfinite/missing samples fail applicable metric instead of fabricating zero; occluded/minimized interval invalidates run.

## Error paths

- Ledger always marks overhead untracked and cannot set resident target proven; malformed/partial machine profile fails validation.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

