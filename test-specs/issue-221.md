# Issue 221 — Wire acquired assets/props/stump.glb

References: `docs/tdd/assets.md §Directory and inventory; §Format specifications; §Production asset registries; §Import and validation pipeline`.

## Boundary contracts

- Install exactly `assets/props/stump.glb` at that unchanged runtime path; the issue writes no sibling content or either production registry.
- The loader validates binary glTF support-centered within configured stump bounds, all required named variants, shared materials and <=3000 triangles per variant.
- Both already-installed production registries must contain this stable ID exactly once with identical path/lowercase SHA-256; license provenance and budget contract must validate, and the wire-in evidence records content, license-registry, and budget-registry digests.

## Multi-system scenarios

- Load the valid file through AssetServer/AssetValidationPlugin, reach the relevant runtime presentation/config behavior, and assert no fallback for this ID.
- Independently mutate the fixture for: missing/duplicate variant, support/bounds mismatch, unique material or any variant at 3001 triangles. Each mutation must fail the precise schema/format/budget rule.
- Replace bytes without updating registries, then update only one registry: both cases must fail cross-registry/content digest validation.

## Failure propagation

- Missing, oversized, corrupt, path/digest/provenance/budget-mismatched or unknown content blocks readiness/release validation; it may not silently use stale installed data or mark wire-in complete.
- A failed validation leaves the prior file/registries logically unaccepted and spawns/loads no partial runtime representation for this ID.

## Ordering guarantees

- Production license and budget registries validate first; installed bytes are hashed before decode; schema/format/budget validation precedes loader readiness and runtime use.
- Reordered asynchronous asset completions cannot bypass either registry or produce duplicate handles. Repeated instances must use the shared handle path where applicable.

## Conformance-harness hook

Use issue 232's public startup path to prove valid content does not alter authoritative truth except for authoritative config/stamp/manifest inputs, and invalid content cannot expose a partially ready world.

