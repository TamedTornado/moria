# Issue 446 — Implement stable scan, compaction, and radix sort

References: `architecture.md` TECH-015; `gpu-runtime.md` TECH-035; issue M-033.

## Properties

- For every bounded mark vector, hierarchical exclusive scan equals a CPU oracle at every position; scatter is stable and reports exact `total`, `written`, and `overflowed`.
- Tile width is 128 with two elements per lane; inactive lanes participate in every barrier using identity.
- Four-bit LSD radix passes preserve input order among equal keys and produce stable full-key order.

## Configurations

- Test empty, one element, 255/256/257, exact tile, partial tile, multi-tile, multilevel hierarchy, maximum capacity, all-kept/all-dropped, stable duplicates, and collision-heavy keys.
- Compare CPU and WGSL scratch offsets, hierarchy levels, outputs, counts, and overflow flags exactly.

## Edge and error paths

- Precompute hierarchy sizes/offsets with checked arithmetic; insufficient scratch/output capacity fails before dispatch or aborts candidate without truncation.
- Over-dispatch guards every global index. Any nonuniform barrier path, cross-workgroup spin, atomic append ordering, counter wrap, or unchecked indirect dimension is rejected.
- Overflow never publishes a partial compacted canonical result.
