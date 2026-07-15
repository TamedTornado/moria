# Issue 45 — Define world identity and curated metadata contracts

References: `docs/tdd/data-model.md §Seed, region, and generated metadata`; `docs/tdd/api.md §Boundary`.

## Input validation and properties

- For all equivalent canonical authoritative config+ruin-stamp bytes, parameters_digest is identical; changing either changes it, while presentation-only changes do not.
- For every accepted manifest, objects and collections are canonical, IDs stable, route length <=64 and feature evaluators <=16; public values expose no mutable store/brick/voxel slice.

## Transformation correctness and entity configurations

- Same semantic RON in canonical form; one-byte config/stamp changes; presentation-only changes; empty/max feature and route collections.
- Sorted, unsorted, duplicate-ID, out-of-bounds and identity-mismatched manifests.

## Edge cases and type boundaries

- Reject noncanonical/duplicate/over-cap metadata with stable typed witnesses before readiness.

## Error paths

- External compile-fail fixture attempts WorldStore, BrickRecord, mutable voxel-slice, and private-module access.

## Rendering states

- Metadata itself renders nothing. Test loading, valid-populated, empty-optional-collection, and fatal-validation lifecycle states; invalid metadata must never spawn partial world presentation.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
