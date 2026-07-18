# Issue 340 — Recovery PR #309: Require validated benchmark output before success

References: `docs/tdd/api.md` §Benchmark CLI and output; `docs/tdd/benchmarks.md` §Run lifecycle and §JSON report contract; `docs/tdd/states.md` §Benchmark runner states.

## Properties that must hold

- For every benchmark invocation, process exit code `0` is possible only after scenario completion, report validation, sibling-temp write, flush, and atomic rename all succeed for a report whose `passed` value is true.
- For every runtime/contract/report failure after argument parsing, the runner must attempt a truthful report with `passed:false`, explicit nulls/missing-metric reasons, and exit `1`; it must never reinterpret validation failure as success.
- For every argument failure (unknown/missing arguments, non-curated seed, invalid resolution, unwritable output detected before app construction), the runner exits `2` with no report.

## Entity configurations to test

- Each scenario with a complete passing report; each scenario with a mandatory field missing, false workload minimum, non-finite metric, asset fallback, or absent machine profile.
- Runtime failure before and after partial metric collection; watchdog timeout; report serialize, temp-create, write, flush, validate, and rename failures.
- Existing valid output file plus each pre-rename failure, proving it remains byte-identical.

## Edge cases

- A syntactically valid JSON report with `passed:true` but scenario-required null data must fail validation and exit `1`.
- `passed:false` artifacts can be written successfully but never produce exit `0`; a passing in-memory report that cannot be durably renamed also cannot exit `0`.

## Error paths

- Failure output must not fill unavailable metrics with zero; the last state/waypoint/request and missing fields are preserved where available.
- Human summary is printed only after safe JSON output and cannot override validator or write outcome.

