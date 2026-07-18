# Issue 339 — Recovery PR #308: Enforce benchmark evidence contracts

References: `docs/tdd/data-model.md` §Feasibility evidence and §Benchmark data; `docs/tdd/benchmarks.md` §JSON report contract, §Metric definitions, and §Run lifecycle.

## Properties that must hold

- For every `ForestFeasibilityReport`, `MutationFeasibilityReport`, and `BenchmarkReport`, schema literal, RFC3339 timestamp, identities, required/null fields, finite distributions, sorted failure reasons/vectors/maps, numeric caps, and `passed == failure_reasons.is_empty()` must validate together.
- For every mutation-feasibility workload, all required stage keys (`admission` through `reconciliation`) exist even for legitimate zero-work branches; expected barrier items equal distinct renderer-ready/removed items and progress/fairness/Horizon/query fields satisfy their role-specific contracts.
- For every benchmark scenario, completed flythrough and mutation-workloads reports obey distinct nullability/save/round-trip rules; missing driver metadata is explicit null with `driver_metadata_available:false`.

## Entity configurations to test

- Complete passing and failing F1/F2 artifacts; completed flythrough; completed mutation-workloads; early failure before build/world/machine collection; resident measurement absent/present; substitution approval absent/present.
- Missing top-level key, missing stage, missing object-index evidence, NaN/infinity, unsorted/duplicate reasons or tags, wrong schema/enum spelling, stale digest, wrong profile/machine/backend/resolution, barrier count mismatch, and fabricated zero metric.
- Exact threshold values and one failing unit/epsilon beyond every integral/timing cap.

## Edge cases

- Ledger-only graphics evidence cannot set `product_target_proven:true`; without resident proof or approved substitution, overall pass is false with `resident_graphics_memory_unproven`.
- A no-work branch records count zero and elapsed time but cannot satisfy stages required to be nonzero for the catastrophic or aggregate workload.

## Error paths

- Validation returns deterministic sorted failure reasons and never normalizes incomplete/invalid evidence into a passing artifact.
- JSON round trip preserves explicit nulls, integer widths, exact schema/enum spellings, and all mandatory top-level keys.
- TDD gap: `data-model.md` declares `BenchmarkReport.build: BuildProfile`, while `benchmarks.md` requires `build:null` when an early failure occurs before provenance collection. It also declares `ScenarioName::{Flythrough, CarveStorm}`, while the public CLI/report contract names the second final scenario `mutation-workloads`. Tests must pin the JSON contract required by `benchmarks.md` for missing build provenance and scenario spelling, but the Rust field/enum declarations require a TDD amendment before tests can demand one exact in-memory representation.
