# Issue 115 — Add the sparse cut-stone ruin and staircase

References: `docs/tdd/assets.md §Ruin stamp`; `docs/tdd/data-model.md §Objects, ruin, and dressing`; `docs/tdd/rendering.md §Registered objects`.

## Properties that must hold

- For every transformed stamp coordinate, sorted sparse runs/palette/quarter-turn sampling yields exact cut-stone or explicit air truth and provenance Ruin(id).
- For every ruin revision—including exact reversion—solid presentation remains voxel-derived TerrainChunk with matching collision and connected supported stairs.

## Entity configurations to test

- All four Y quarter turns; run/palette/tag boundaries; stair_bottom/top/entrance; activation, dig/place/erase/revert across chunk seam.
- Supported/disjoint placement and deliberate object/stamp/air-carve conflicts.

## Edge cases and type boundaries

- Unsorted/overlap/out-of-bounds run, invalid material/tag/orientation/digest or untraversable stair rejects before readiness.

## Error paths

- No ruin GLB, collision mesh, authored intact root or privileged stair entity; stale chunk/seam result cannot install.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

