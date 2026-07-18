# Issue 352 — Recovery PR #323: Enforce all F2 query probe budgets

References: `docs/tdd/api.md` §Read-only world observations performance proof; `docs/tdd/data-model.md` §Feasibility evidence/`QueryCostEvidence`; `docs/tdd/implementation-plan.md` §Gate F2.

## Properties that must hold

- For every F2 query probe, all workload classes and budgets validate independently: each frame-critical call p99 `<=1.0 ms` and max `<=4.0 ms`; normal bundle p99 `<=2.0 ms`; column p99 `<=1.0 ms`; metadata-page p99 `<=1.0 ms`; two-brick cell-page p99 `<=4.0 ms` and max `<=8.0 ms`.
- For all valid evidence, the probe records 256 distinct previously unsampled inactive-forest calls, 1,000 normal bundles, 128 maximum columns/metadata pages, 128 two-brick cell pages, exact-limit inputs, and observed work maxima/candidate counters.
- For every failed individual budget, sample count, public query, or complexity maximum, `QueryCostEvidence` and the enclosing mutation-feasibility report remain complete but `passed:false`; another passing aggregate/distribution cannot mask it.

## Entity configurations to test

- One all-green evidence fixture; then vary exactly one field at a time to exceed each of the seven p99/max budgets while all other metrics pass.
- Exact threshold values; the smallest representable value above each; p99 passing with max failing; max passing with p99 failing; normal bundle passing while one constituent call fails and vice versa.
- Correct M4/Metal/release/2560x1440/digest identity and each wrong identity; active/cold coordinates; duplicate cold coordinate; missing distribution/counter; public query error; NaN/infinity.

## Edge cases

- Quantile calculation at exact sample counts must use the report's deterministic distribution method and must not round an over-budget value down to pass.
- Warm-cache repetitions are a negative control and cannot replace the 256 distinct cold inactive calls; retries cannot discard slow valid samples.

## Error paths

- Missing/incomplete/over-limit evidence yields stable sorted failure reasons naming the specific call class and p99/max contract.
- Query probe failure prevents `BenchmarkState::QueryCostProbe -> Running`, writes a failed mutation-feasibility report, and never fabricates zero values or continue into mutation workloads.

