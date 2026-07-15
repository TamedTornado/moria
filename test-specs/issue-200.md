# Issue 200 — Wire acquired assets/manifests/asset_budgets.ron

References: `docs/tdd/assets.md §Production asset registries; §Import and validation pipeline`.

## Boundary contracts

- Install only `assets/manifests/asset_budgets.ron`; do not write the license registry or content.
- Parse schema_version=1 with deny_unknown_fields and exactly the same sorted 28 IDs/path/digests as license registry/content. max_file_bytes is positive and checked before decode; contract variant matches extension.
- Validate RON schema keys; GLB finite Q8 bounds/support, sorted names, clips and triangle caps; KTX2 dimensions/layers/mips/color/Basis; WGSL entries and forbids_i64_atomics=true.

## Multi-system scenarios

- Validate the production registry, record its SHA-256, and exercise every contract variant.
- Mutate one property at a time: missing/duplicate/unknown/order/path/digest; zero/one-below file cap; wrong variant; invalid bounds/support; duplicate names; wrong KTX2 facts; absent WGSL entry/false atomic guard.

## Failure propagation

- Any mismatch fails before content decode/use; neither truncation nor permissive fallback is allowed.
- Registry-byte changes invalidate prior per-content evidence and readiness.

## Ordering guarantees

- Canonical registry/cardinality and file-size guards run before decoder allocation; content format validation and cross-license equality precede readiness.

## Conformance-harness hook

Run through issue 232's public startup/failure-injection adapter and preserve this registry/content failure as a deterministic fixture.

