# Issue 158 — Drive and validate production mutation workloads

References: `docs/tdd/benchmarks.md §Mutation workloads and heavy save` and `§Mutation progress and reconciliation`.

## Properties that must hold

- For every request in interactive, eight-stream colony and catastrophic workloads, exact Accepted/batches/primary/reconciled trace exists with monotonic fields, committed/not-yet/outside truth samples, and threshold validation.
- For every final workload, at least 256 bricks remain different from base across colony/catastrophic volumes; eight streams overlap in flight and catastrophic is progressive.

## Entity configurations to test

- Documented materials/locations/object dependencies; disjoint/overlapping and reversible edits; no-op control; signature clear path; scheduler contention.
- All latency/throughput/fairness/reconciliation/frame limits at exact threshold and one failing sample.

## Edge cases and type boundaries

- Any rejection, missing stage, stale install, threshold/frame/coverage failure or insufficient defacement fails workload without private reset.

## Error paths

- No game designation/worker AI/spell semantics or direct store/presentation mutation is allowed.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.
