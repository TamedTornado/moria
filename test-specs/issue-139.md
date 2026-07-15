# Issue 139 — Populate full forest-scale object visuals

References: `docs/tdd/config.md §Biome, object, and route constraints`; `docs/tdd/rendering.md §Registered objects and forest scale`.

## Properties that must hold

- For every indexed placement visible in its policy band, visual ID/species/variant equals immutable manifest/stable hash and current dependency eligibility; no extra/missing ID.
- For all repeated objects, Near/Middle/Far/Horizon use shared handles, visibility/culling/LOD and never unique per-instance mesh/material allocation.

## Entity configurations to test

- Both species and every object kind; each LOD/band boundary; edited/ineligible and intact; dense maximum cell; eviction/reactivation.
- Compare manifest/index expected counts with resident logical roots and allocation IDs.

## Edge cases and type boundaries

- Unknown ID/variant, cap overflow or asset failure blocks/uses only declared fallback; never silently omits an indexed visible object.

## Error paths

- Stale eligibility/LOD result cannot install; dense diagnostics must show shared assets and bounded per-cell buffers.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

