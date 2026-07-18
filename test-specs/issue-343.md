# Issue 343 — Recovery PR #310: Complete object provenance evaluation

References: `docs/tdd/data-model.md` §Geological feature and §Objects, ruin, and dressing; `docs/tdd/rendering.md` §Terrain extraction and §Registered objects and forest scale; `docs/tdd/systems.md` §Object and dressing systems.

## Properties that must hold

- For every in-bounds coordinate, base evaluation returns one deterministic `VoxelSource::Terrain | Object(id) | Ruin(id)` according to documented precedence; current deltas replace voxel value only and never reassign base provenance.
- For every current solid cell, `solid_presentation_owner` returns exactly `TerrainChunk` for terrain/ruin provenance or `NonRuinObject(id)` for non-ruin object provenance; every nonsolid cell returns no owner without changing collision/query truth.
- For every supported object kind/transform, analytic shape evaluation is deterministic: tree/bush/stump primitives, perturbed boulder/rock ellipsoids, and quarter-turn sparse ruin stamps produce their specified material/provenance, including ruin air-carves.

## Entity configurations to test

- Each `ObjectKind`, min/max quantized dimensions/scales, quarter-turn orientations, region-bound clipping, terrain/object and object/ruin boundaries, cave/geology/water intersections, and exact delta reversion.
- Object-only, terrain-only, ruin-only, and boundary-spanning edits; current solid/nonsolid values over each base source; full erasure and later restoration.
- Two deliberately overlapping non-ruin shapes and non-ruin versus every authored ruin coordinate to verify pre-readiness rejection.

## Edge cases

- Lowest-ID overlap precedence may make candidate evaluation total, but an opened manifest must reject all such overlaps; runtime output must never rely on precedence to mask an invalid accepted object.
- Checked transform/subtraction at min/max region faces must not wrap or alias.

## Error paths

- Owner-filtered terrain and object primitive sets must be disjoint and their union equal global extraction; stale/wrong-ID payloads are rejected.
- Invalid overlap returns stable IDs and first voxel before `WorldReady` and spawns no object root/terrain chunk from rejected truth.

