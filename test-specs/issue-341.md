# Issue 341 — Recovery PR #313: Harden bounded object index contracts

References: `docs/tdd/systems.md` §Generation systems/Pure evaluators; `docs/tdd/data-model.md` §Seed, region, and generated metadata; `docs/tdd/config.md` §Biome, object, and route constraints.

## Properties that must hold

- For every accepted index regardless of placement insertion order, dependency/sample tables and `placement_ids_in`, `horizon_tree_ids`, and changed-dependency queries return sorted, duplicate-free IDs equal to brute-force exact oracles.
- For all accepted placements/indexes: each placement touches at most 16 cells in each grid, dependency bounds cover at most 128 bricks, dependency/sample cells contain at most 1,024/64 members, supported edits return at most 256 broad/64 exact IDs, Horizon cells hold at most 1,024 trees, retained capacity is at most 16 MiB, and dependency-coordinate allocation is zero.
- For every scalar base sample, exactly one 4 m sample cell is visited and at most 64 analytic object shapes are exact-tested.

## Entity configurations to test

- Empty, single-placement, small oracle, and checked-in full manifest; placements on 4 m/32 m/64 m cell faces/corners; duplicate candidates through multiple cells; inactive/active/evicted/reactivated regions.
- Exact maximum and one-above fixtures for every cell/object/brick/candidate/member/byte cap, including checked capacity arithmetic overflow.
- Non-ruin/non-ruin and non-ruin/ruin-air-carve overlaps with multiple conflicts to verify ordered IDs and lexicographically first coordinate.

## Edge cases

- Bounds clipped at region faces, checked subtraction, empty query AABB, and query AABB spanning repeated cells must not wrap, duplicate, or scan the region.
- Horizon membership derives by one sorted pass and must not create/retain a third index.

## Error paths

- Any cap violation rejects with typed `ManifestError`/`ForestContractViolation` rather than truncating and retains no partial index.
- Overlap reports stable IDs/first voxel before `WorldReady`; no authored or derived object root is spawned from the rejected world.

