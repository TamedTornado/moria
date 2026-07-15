# Issue 80 — Close same-LOD and transition seams

References: `docs/tdd/rendering.md §Chunk seams and LOD`; `docs/tdd/systems.md §Terrain meshing systems`.

## Properties that must hold

- For every adjacent same-LOD pair, boundary positions/normals match and canonical spatial ownership emits no duplicate primitive.
- For every adjacent different-LOD pair, fine-side skirt closes cracks, tucks toward solid <=0.05 m and never contributes collision truth.

## Entity configurations to test

- All six faces, edits moving crossing across brick/tile boundary, flat and complex surfaces, each adjacent LOD pair, empty replacement/removal.
- Same-LOD rebuild of one/both neighbors; LOD change with and without dither support.

## Edge cases and type boundaries

- Face-neighbor dirtying must include every affected seam; stale seam/result rejected by token/revision/LOD.

## Error paths

- Replacement retains old normal-world visual until new ready/removal; no blank chunk and no collision change from skirt.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

