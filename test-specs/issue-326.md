# Issue 326 — Recovery PR #246: Register tree horizon card validation fixture

References: `docs/tdd/assets.md` §Directory and inventory, §Textures, §Production asset registries, and §Import and validation pipeline; `docs/tdd/config.md` §Rendering and environment configuration.

## Properties that must hold

- For every ordinary `cargo test -p moria-world` and repository `cargo test` run, the tree-horizon-card validation fixture must be discovered and executed without a manual test-name, opt-in feature, or headed window; a fixture that exists on disk but is not part of either normal test graph is a failure.
- For every accepted `assets/vegetation/tree_horizon_cards.ktx2`, its declaration, license entry, and budget entry must agree on path and SHA-256, the file must fit its positive byte budget, and the decoded KTX2 metadata must match the registry's dimensions, layers, complete mip count, color space, and `basis_payload: true`.
- For all accepted horizon cards, the payload must expose the declared color/normal/opacity layers. Structural validation must not silently replace a missing or invalid card with the development fallback and report success.

## Entity configurations to test

- Run the registered fixture against the checked-in card, then against isolated mutations of magic/header, level index, Basis payload declaration, mip count, layer count, dimensions, digest, and registry path.
- Exercise a complete production registry, a missing card entry, duplicate/out-of-order entries, and a card whose license and budget digests disagree.
- Verify the missing-card development state emits the declared readiness warning and disables only Horizon tree presentation; benchmark/release acceptance must reject that fallback. Treat species-silhouette quality as headed visual evidence, not as a claim made by this headless registration fixture.

## Edge cases

- Test the exact declared byte limit and one byte above it before decoding.
- Test the smallest mip and the end of the final level so truncated payloads cannot pass metadata-only validation.

## Error paths

- Any schema, digest, size, KTX2, mip/layer, or Basis failure must be a typed asset-validation failure and must not report the card as ready.
- A deliberately failing fixture input must make both normal Cargo invocations report the fixture failure, proving registration rather than merely validating through a helper that no test invokes.
