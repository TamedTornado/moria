# Issue 82 — Derive revision-anchored dressing

References: `docs/tdd/data-model.md §Objects, ruin, and dressing`; `docs/tdd/rendering.md §Dressing`.

## Properties that must hold

- For every dressing instance, anchor is current-revision upward surface (normal y>=0.75), topsoil, nonwater and nonexcluded; key/transform derivation is deterministic.
- For every anchor revision change, old batch is removed immediately before deterministic replacement; dressing never enters authoritative deltas/save.

## Entity configurations to test

- Topsoil threshold slopes just below/at 0.75; water, ceiling, masonry, empty, object-excluded, dug/moved/reverted surface; Near/Middle/Far/Horizon.
- Same seed/revision repeated generation; edit/load and eviction/reactivation.

## Edge cases and type boundaries

- Missing/stale anchor or stale task produces no instance; failed replacement cannot leave old floating dressing visible.

## Error paths

- Instances share mesh/material handles; density/batch caps reject before oversized GPU buffers.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

