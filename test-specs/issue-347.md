# Issue 347 — Recovery PR #320: Enforce curation facade boundaries

References: `docs/tdd/api.md` §Boundary; `docs/tdd/overview.md` §Package and module structure; `docs/tdd/implementation-plan.md` §Gate F1.

## Properties that must hold

- For every build with Cargo feature `curation`, the public facade exposes exactly the pure contracts `derive_manifest(seed, config, stamp) -> Result<CuratedManifest, CurationError>` and `validate_manifest(config, manifest) -> Result<CurationReport, CurationError>` needed by `moria-curate`.
- For all facade calls, results depend only on explicit seed/config/stamp/manifest values and never access a live `WorldStore`, delta map, ECS world, render state, cache residency, or private generation/index type.
- For every shipped demo/benchmark build, the curation feature is not enabled/called and no extra authoritative access becomes available.

## Entity configurations to test

- External consumer builds with feature enabled and disabled; `moria-curate generate`, `check`, and `prove-forest`; demo and benchmark feature graphs.
- Valid/invalid config, stamp, identity, and manifest passed through the facade; repeat derivation/validation in changed call/thread order and compare canonical values/errors.
- Compile-fail attempts to import private curation/generation/index modules or pass a live store/delta/render object.

## Edge cases

- Feature-disabled crate root must not expose curation symbols; feature-enabled facade must not expose additional live-world operations.
- Pure derivation may construct bounded temporary data but must not retain global mutable state across calls.

## Error paths

- Typed `CurationError` propagates through the CLI with no duplicate alternate generator/validator that can disagree or weaken checks.
- Facade wiring cannot fabricate F1 performance/machine evidence; `prove-forest` must still use the production runtime validator and complete gate contract.

