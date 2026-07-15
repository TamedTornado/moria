# Issue 107 — Assemble and validate mutation-feasibility reports

References: `docs/tdd/data-model.md §Feasibility evidence`; `docs/tdd/benchmarks.md §Pre-implementation feasibility evidence`.

## Properties that must hold

- For every MutationFeasibilityReport, schema/identity/build/machine/backend/resolution/manifest/F1 hashes and exactly three workload roles plus query evidence and all stage keys are complete.
- For every validation, nonfinite/missing/identity mismatch or any timing, fairness, throughput, frame, reconciliation, cap or zero-work violation produces sorted failures and passed=false.

## Entity configurations to test

- One fully passing synthetic report; remove/corrupt each field/stage independently; duplicate/missing workload role; optional legitimate ruin/water zero branch.
- Wrong M4/Metal/release/2560x1440 and stale F1/git/world/manifest identities.

## Edge cases and type boundaries

- Validation precedes atomic output and nonzero exit; invalid report may be written only as truthful failed artifact.

## Error paths

- No rounded distribution can hide the direct <=1 ms discovery+eligibility check or expected/ready inequality.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

