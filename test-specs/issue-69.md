# Issue 69 — Implement the bounded public query-cost probe

References: `docs/tdd/api.md §Read-only world observations performance proof`; `docs/tdd/data-model.md §QueryCostEvidence`.

## Properties that must hold

- For every probe run, the exact sample counts, distinct cold coordinates and observed work maxima are recorded; the probe uses only public reads.
- For each call class, p99/max budgets validate independently: frame-critical <=1/4 ms, normal bundle p99 <=2 ms, column/metadata p99 <=1 ms, two-brick cells p99/max <=4/8 ms.

## Entity configurations to test

- 256 distinct previously unsampled inactive-forest calls; 1000 normal bundles; 128 maximum column/metadata pages; 128 two-brick cell pages; exact-limit inputs.
- Warm cache repeated set as negative control; active and inactive coordinates; correct M4/release environment and wrong identities.

## Edge cases and type boundaries

- Repeated cold coordinates, missing samples/counters, over-limit work or nonfinite timing fails evidence rather than silently warming/retrying.

## Error paths

- Any public query error or incomplete distribution produces failed complete QueryCostEvidence.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

