# Issue 47 — Add feasibility and benchmark report schemas

References: `docs/tdd/data-model.md §Feasibility evidence` and `§Benchmark data`; `docs/tdd/benchmarks.md §JSON report contract`.

## Input validation and properties

- For every serialized report, schema literal, identities, required/null fields, finite distributions, sorted duplicate-free vectors/maps, and passed == failure_reasons.is_empty() are validated together.
- For all ledger-only evidence, product_target_proven remains false; resident proof or an approval ID cannot be inferred.

## Transformation correctness and entity configurations

- Minimal early failure, complete F1/F2, completed flythrough, completed mutation-workloads, absent driver metadata, resident measurement, and explicit substitution approval.
- Unsorted/duplicate reasons, NaN/±Inf, missing stage/key, fabricated zero, wrong digest/profile/machine/resolution, inconsistent pass bit.

## Edge cases and type boundaries

- Validation returns a deterministic failure set and never emits a passing artifact from incomplete data.

## Error paths

- JSON round trip preserves explicit nulls and exact schema/enumeration spellings.

## Rendering states

- Reports have no game rendering. Test human-summary/output states for early failure with explicit nulls, failed populated evidence, completed flythrough, completed mutation-workloads, F1 and F2; overflow/nonfinite data must appear only as failed evidence.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
