# Issue 336 — Recovery PR #304: Complete curated metadata contracts

References: `docs/tdd/data-model.md` §Seed, region, and generated metadata; `docs/tdd/assets.md` §Curated manifest; `docs/tdd/api.md` §Boundary.

## Properties that must hold

- For every accepted `CuratedManifest`, `seed` and `parameters_digest` match the validated `RegionConfig`/ruin stamp identity; objects are sorted by `ObjectId`, all collections use canonical deterministic order, all coordinates/bounds are in region bounds, features are at most 16, and route waypoints are at most 64.
- For all accepted manifests, metadata contains generated feature bounds, exactly the required river/lake definitions, stable object placements including exactly one ruin, and every required traversal/benchmark tag; it contains no expanded voxel landscape or derived render truth.
- For every repeated derivation from identical seed/config/stamp, canonical values and serialized bytes are identical regardless of evaluation order, thread count, or cache state.

## Entity configurations to test

- Minimal valid and checked-in full manifest; empty/missing required feature kinds; duplicate/unsorted IDs; 16/17 features; 64/65 route points; out-of-bounds feature/object/water/route coordinates; one/multiple/no ruins.
- Matching identity, changed seed, changed authoritative config, changed ruin stamp, and presentation-only change.
- Route with each required semantic tag removed individually and with duplicate tags/waypoints in noncanonical order.

## Edge cases

- Coordinates on min-inclusive and max-exclusive faces; Q8 integer extrema that would overflow transformed bounds; empty optional metadata versus missing required metadata.
- Canonical generation comment may differ only as documented generation text; semantic canonical serialization and byte comparison must remain deterministic.

## Error paths

- Identity, ordering, cardinality, bounds, or required-feature failure must return a typed `CurationError`/manifest error before `WorldReady`; no partial manifest/index/store is retained.
- Public curated values must expose no mutable `WorldStore`, delta, render state, or authoritative voxel slice.

