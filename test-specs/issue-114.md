# Issue 114 — Generate carved river and lake bodies

References: `docs/tdd/config.md §Geology and feature constraints`; `docs/tdd/data-model.md §Water body`.

## Properties that must hold

- For every accepted body, exactly one river and lake stay in bounds and satisfy width/depth or diameter/depth; carved bed/basin is evaluated before static water assignment.
- For every voxel, water exists only inside footprint between bed and surface, is material-present/water-volume, and never solid.

## Entity configurations to test

- River widths/depth extrema and bank edge; lake diameter/depth extrema; bed, one below/at surface, footprint boundary and overlaps with terrain/cave.
- Cold/active evaluation and edits exposing/covering predefined volume.

## Edge cases and type boundaries

- Undersized/out-of-bounds/noncarved/multiple/missing bodies fail curation with stable witness.

## Error paths

- No dynamic fluid resource/system, flow, pressure or neighbor update is introduced.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

