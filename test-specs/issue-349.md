# Issue 349 — Recovery PR #319: Harden bounded ray contracts

References: `docs/tdd/api.md` §Read-only world observations/ray limits and result semantics; `docs/tdd/data-model.md` §Coordinate and scalar conventions.

## Properties that must hold

- For every legal ray, deterministic 3-D DDA visits no more than 448 voxels over no more than 16,384 Q8 (64 m), returns zero or the nearest matching hit, and produces identical voxel/point/normal/distance/revision across cache, activation, order, and thread variations.
- For all masks, `SOLID` selects only `solid_collision`, `WATER` only `water_volume`, and their union either; no render mesh or `material_present` shortcut may determine a hit.
- For every legal ray, traversal uses bounded/no work-sized allocation and integer-safe stepping across negative coordinates and region boundaries.

## Entity configurations to test

- Axis-aligned and positive/negative diagonal rays; origins inside a matching voxel, exactly on voxel faces/edges/corners, just inside each region face, and near the origin's negative floor-division boundaries.
- Distances zero, one voxel, exact 64 m; a path requiring exactly 448 visits; active versus inactive procedural cells; committed delta changing the first hit; SOLID/WATER/union masks.
- Compare every small-world case to exhaustive ordered voxel intersection and verify tie-breaking/quantized normal.

## Edge cases

- Zero or unnormalized Q16 direction and empty mask return `InvalidInput`; 16,385 Q8 returns `LimitExceeded(RayDistance)`; any traversal requiring a 449th visit returns `LimitExceeded(RayVoxelVisits)` rather than a partial/no-hit answer.
- Endpoint/step arithmetic overflow and an origin/traversal outside region return deterministic typed error and never wrap to another coordinate.

## Error paths

- Validation failures occur before DDA/world sampling and never return a truncated `Option<WorldHit>`.
- A cache miss/inactive brick must use bounded procedural truth; it cannot return `NotReady` after world readiness merely because detail is absent.

