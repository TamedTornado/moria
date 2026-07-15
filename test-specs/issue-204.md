# Issue 204 — Wire acquired assets/config/input.ron

References: `docs/tdd/assets.md §Directory and inventory; §Format specifications; §Production asset registries; §Import and validation pipeline`.

## Boundary contracts

- Install exactly `assets/config/input.ron` at that unchanged runtime path; the issue writes no sibling content or either production registry.
- The loader validates RON centralized mapping for every documented keyboard/mouse/gamepad binding, dead zone 0.15 and sensitivity values to semantic Player/Debug/Ui actions.
- Both already-installed production registries must contain this stable ID exactly once with identical path/lowercase SHA-256; license provenance and budget contract must validate, and the wire-in evidence records content, license-registry, and budget-registry digests.

## Multi-system scenarios

- Load the valid file through AssetServer/AssetValidationPlugin, reach the relevant runtime presentation/config behavior, and assert no fallback for this ID.
- Independently mutate the fixture for: missing/duplicate/conflicting binding, unknown physical/semantic action, invalid dead zone/sensitivity. Each mutation must fail the precise schema/format/budget rule.
- Replace bytes without updating registries, then update only one registry: both cases must fail cross-registry/content digest validation.

## Failure propagation

- Missing, oversized, corrupt, path/digest/provenance/budget-mismatched or unknown content blocks readiness/release validation; it may not silently use stale installed data or mark wire-in complete.
- A failed validation leaves the prior file/registries logically unaccepted and spawns/loads no partial runtime representation for this ID.

## Ordering guarantees

- Production license and budget registries validate first; installed bytes are hashed before decode; schema/format/budget validation precedes loader readiness and runtime use.
- Reordered asynchronous asset completions cannot bypass either registry or produce duplicate handles. Repeated instances must use the shared handle path where applicable.

## Conformance-harness hook

Use issue 232's public startup path to prove valid content does not alter authoritative truth except for authoritative config/stamp/manifest inputs, and invalid content cannot expose a partially ready world.

