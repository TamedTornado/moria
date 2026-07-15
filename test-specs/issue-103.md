# Issue 103 — Run interactive and colony mutation feasibility scenarios

References: `docs/tdd/implementation-plan.md §Gate F2`; `docs/tdd/benchmarks.md §Mutation progress and reconciliation`.

## Boundary contracts

- Scenario submits one public 3 m carve and up to eight public worker-sized streams over 32x32x16 m, consuming only lifecycle/telemetry/render acknowledgements.
- Every stage key and final expected==renderer-ready item count is mandatory.

## Multi-system scenarios

- Interactive signature: <=2 ms admission, <=100 ms first commit, primary p95/max <=250/500 ms, <=1 s reconciliation, clear capsule route, max frame <=33.3 ms.
- Eight streams with disjoint and overlapping edits: <=250 ms first commit each, >=32 bricks/s, <=500 ms runnable wait, <=30 s reconciliation; normal startup <5000 ms.

## Failure propagation

- Any rejection, fallback, missing stage/ack, starvation, timing/frame/traversal failure produces failed immutable evidence.
- No designation/worker AI or private reset/store/mesh shortcut; overlap must still yield deterministic final truth.

## Ordering guarantees

- Start from same validated baseline; scripted submissions deterministic, fair interleaving allowed; terminal only after all streams reconcile.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

