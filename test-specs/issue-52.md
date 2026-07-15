# Issue 52 — Implement strict benchmark CLI and atomic report output

References: `docs/tdd/api.md §Benchmark CLI and output`; `docs/tdd/benchmarks.md §Run lifecycle` and `§JSON report contract`.

## Properties that must hold

- For every invocation, argument validation selects exactly one scenario/output/resolution/seed/proof combination and maps argument/runtime/pass outcomes to exit 2/1/0 exactly.
- For every report write, validated complete/null JSON is written to sibling temp, flushed, atomically renamed, then summarized; unavailable metrics stay null.

## Entity configurations to test

- All three scenarios; defaults and explicit resolutions; curated/noncurated seed; required/absent forest proof; existing/unwritable output.
- Runtime failure before and after partial metrics, validation failure, temp/write/flush/rename failure.

## Edge cases and type boundaries

- Unknown/missing/invalid arguments create no report and exit 2; runtime/contract failures attempt truthful failed report and exit 1.

## Error paths

- Prior output survives any pre-rename failure; pass exit 0 is impossible until report validation succeeds.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
