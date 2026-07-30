# Issue 408 — Implement canonical cells, bricks, and logical domains

References: `architecture.md` TECH-006; issue M-003.

## Properties and transformations

- For every valid `CellWire`, encoding is exactly four little-endian bytes. `material_id == 0` requires density `<= 0`; nonzero material requires a registered ID.
- Every dense brick has exactly 512 cells and 2,048 canonical bytes in x-major, then y, then z order. A uniform brick is semantically identical to expanding its four-byte cell 512 times.
- For all signed coordinates `a`, brick quotient/remainder obey `a = 8q+r` and `0 <= r < 8`, including `-1`, `-8`, and `-9`.
- Every accepted AABB is half-open and has `min < max` on all axes.

## Entity configurations

- Test empty material, nonoccupying `Never` material with positive matter density, and `SolidAbove` immediately below/at/above threshold.
- Test one-cell domains, each 8,191-cell side limit, pivots at 4,095-cell radius, negative-coordinate domains, and bricks crossing zero.
- Compare dense and uniform representations for all-empty and all-one-material bricks.

## Edge and error paths

- Reject positive-density empty cells, unknown nonzero materials, an empty/reversed axis, side 8,192, a corner beyond pivot radius, invalid brick byte length, and coordinate arithmetic overflow.
- Rejection must not normalize density, clamp bounds, allocate a brick, or mutate registry state.

Rendering is not part of TECH-006.
