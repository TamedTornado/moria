# Issue 546 — Aggregate facade and headless evidence

References: `validation.md` TECH-069 and TECH-060; `interfaces.md` TECH-021/070; issue M-130.

## Properties

- For every report, the closed row catalog is exactly: `complete_consumer_facade`, `admission_lifecycle`, `genesis_query`, `observation_history`, `base_provider`, `store_replay`, `configuration_boundaries`, `participant_policy`, `external_facade`, `public_closure`, `canonical_rust_lint`, `canonical_wgsl_lint`, `private_correction`, `completion_bridge`.
- Each row binds only its stated M-077 or M-116–M-128 artifact and retains typed row ID, immutable digest, callable/owner/bound/lifecycle coverage, expected/actual result, and validity.
- Domain validity is true iff every required row is present once, identity/digest-valid, complete, available, and passing.

## Entity configurations

- Build one fully valid set in arbitrary input order and require canonical report row order and stable report digest.
- For each row independently: omit, duplicate, reorder, corrupt digest, change contract/source/fixture identity, mark skipped/unavailable/incomplete/failed, or substitute another row’s valid artifact.
- Cross-wire compile-only facade evidence into runtime lifecycle row and vice versa; both fail.

## Error paths

- Undefined callable, owner, budget bound, lifecycle phase, admission/retry/generation state, or missing headless case invalidates the exact owning row and whole domain.
- TECH-059-independent private-correction/bridge rows remain mandatory and cannot be supplied by oracle/GPU evidence.
- Unknown row/version/field and trailing report data fail closed; no partial report is labeled valid.
