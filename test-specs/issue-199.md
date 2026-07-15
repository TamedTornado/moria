# Issue 199 — Wire acquired assets/manifests/asset_licenses.ron

References: `docs/tdd/assets.md §Production asset registries; §Import and validation pipeline`.

## Boundary contracts

- Install only `assets/manifests/asset_licenses.ron`; do not write the budget registry or any content asset.
- Parse schema_version=1 with deny_unknown_fields. Entries are sorted by stable_id and cover exactly the 28 declared non-registry content IDs once. Each immutable path and lowercase 64-hex content digest matches installed bytes and the budget registry.
- Validate nonblank provenance: in-house generator/author/source rules, or absolute HTTPS source, author, valid SPDX expression, existing repository-relative license text and modification rules.

## Multi-system scenarios

- Validate the full 28-entry production registry and record its own SHA-256 in AssetValidationReport.
- For each rule, mutate one fixture: missing/duplicate/unknown/out-of-order ID, path/digest mismatch, extra field, blank/TBD/placeholder provenance, HTTP URL, invalid SPDX, missing license text, invalid empty modifications/source.

## Failure propagation

- Any registry, content, or cross-registry mismatch is fatal in every profile; no partial registry or fallback is exposed.
- Changing registry bytes invalidates old wire-in evidence even if content is unchanged.

## Ordering guarantees

- Parse/canonical/cardinality validation precedes filesystem digest/provenance checks; cross-registry equality precedes per-content readiness.

## Conformance-harness hook

Run through issue 232's public startup/failure-injection adapter and preserve this registry/content failure as a deterministic fixture.

