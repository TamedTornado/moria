# Issue 60 — Wire moria-curate through the public curation feature

References: `docs/tdd/api.md §Boundary`; `docs/tdd/overview.md §Package and module structure`; `docs/tdd/implementation-plan.md §Gate F1`.

## Boundary contracts

- With feature `curation` enabled, the crate root exposes exactly `derive_manifest(seed: u64, config: &RegionConfig, stamp: &SparseVoxelStamp) -> Result<CuratedManifest, CurationError>` and `validate_manifest(config: &RegionConfig, manifest: &CuratedManifest) -> Result<CurationReport, CurationError>`; disabled builds expose neither.
- `moria-curate generate`, `check`, and completed `prove-forest` call only those pure public functions. Demo and benchmark builds do not enable/call the feature, and no facade operation accepts a live store, delta map, or render state.

## Multi-system scenarios

- External package builds with feature on/off; demo and bench without it; valid and invalid config/stamp/manifest; repeated command runs.
- Run generate/check repeatedly and compare canonical bytes/typed reports; run prove-forest with the production validator/index and assert this wiring itself fabricates no acceptance evidence.

## Failure propagation

- Typed facade errors propagate to CLI without duplicated validation/generation; no evidence is fabricated by this wiring issue.
- Compile-fail imports of private generation/index modules and attempts to access a live world/store/delta/render path.

## Ordering guarantees

- Feature selection occurs at compile time; config/stamp input validation precedes deterministic derivation, canonical comparison and runtime validation. Feature-disabled shipped consumers remain buildable and behaviorally unchanged.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
