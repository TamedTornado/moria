# Issue 157 — Drive the complete scripted flythrough

References: `docs/tdd/benchmarks.md §Scripted flythrough`; `docs/tdd/config.md §Benchmark configuration`.

## Properties that must hold

- For every run, fixed 120 s C1 Catmull-Rom public-focus route remains in bounds/collision-free and visits each ordered tag continuously for >=1 measured second without teleport.
- For every completed flythrough, all bands and required frame/startup/allocation/streaming/world/build/asset/machine/resolution identities are recorded; isolated clean slot size is 0.

## Entity configurations to test

- Spline endpoints/joins and tight cave probes; each tag and band boundary; 1080p/1440p named profiles; clean absent slot.
- 300-frame warmup followed by measured route; focus/occlusion changes.

## Edge cases and type boundaries

- Missing tag/band/duration, invalid probe, fallback, focus invalidation, incomplete profile or teleport fails even if FPS passes.

## Error paths

- Metric/task failure creates truthful failed report; it cannot skip segment or merge machines.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

