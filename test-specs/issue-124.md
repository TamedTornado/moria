# Issue 124 — Curate the complete feature route and manifest

References: `docs/tdd/assets.md §Curated manifest`; `docs/tdd/config.md §Biome, object, and route constraints`.

## Properties that must hold

- For every regeneration from canonical seed/config/stamp, bytes and parameters digest match; all required features, object counts/species/spacing/canopy bins, water, ruin and route tags validate together.
- For every adjacent route waypoint segment, configured capsule traversal is continuous through meadow, forest, river, lake, cliff, ruin, cave, aquifer, ore and floor without teleport.

## Entity configurations to test

- Checked-in manifest; independently fail each area/count/species/spacing/bin/clearance/overlap/index/water/geology/cave/ruin/route contract.
- Signature target before/after radius-3 m occupancy oracle and identity change.

## Edge cases and type boundaries

- Failures return deterministic typed witness/first coordinate and never weaken another constraint or accept partial content.

## Error paths

- Intentional authoritative identity change invalidates incompatible development save; presentation-only changes do not.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

