# Issue 330 — Recovery PR #284: Fix pine near GLB binary layout

References: `docs/tdd/assets.md` §Meshes and animation, §Production asset registries, and §Import and validation pipeline; `docs/tdd/rendering.md` §Registered objects and forest scale.

## Properties that must hold

- For every accepted `assets/vegetation/pine_near.glb`, the GLB container, JSON chunk, BIN chunk, buffers, buffer views, accessors, indices, and primitive ranges must be internally consistent and fully in bounds.
- For all pine-near primitives, positions/normals/UV0 and required tangents are finite, triangles are indexed with `u32`-compatible counts, and the asset obeys the registry's bounds, support origin, named primitives, byte limit, and near-tree limit of 12,000 triangles.
- For all repeated pine placements, the validated mesh/material handles must be shared; the asset must contain no world-space placement.

## Entity configurations to test

- Checked-in GLB; truncated BIN; misaligned/out-of-range buffer view; accessor whose count overruns its view; index outside vertex count; absent normal/UV; non-finite position; wrong origin/bounds; and 12,001 triangles.
- Two and many pine entities must reference the same declared asset handles while retaining distinct transforms/IDs.

## Edge cases

- Exact end-of-buffer access and exactly 12,000 triangles pass; one-byte and one-index overruns fail.
- Optional vertex color may be absent; required tangent is conditional on the declared normal-mapped material.

## Error paths

- Binary/layout failure must be detected before scene installation, yield a typed asset error/fallback warning as profile permits, and never spawn a partially decoded pine root.

