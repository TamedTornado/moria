# Issue 119 — Generate the connected karst cave route

References: `docs/tdd/config.md §Geology and feature constraints`; `docs/tdd/systems.md §Generation systems`.

## Properties that must hold

- For every accepted cave manifest, the fixed-point signed field creates one connected void from mouth near 0 m to floor -40±2 m with >=3 m clear width/height and configured slope/shelves.
- For every cave coordinate, cave void wins geology; declared aquifer/iron wall samples expose their materials without closing capsule connectivity.

## Entity configurations to test

- Mouth, each spline/chamber join, tightest width/height/slope, rock shelves, aquifer and ore crossings, exact floor tolerance endpoints.
- Capsule path under active/inactive generation and randomized evaluation order.

## Edge cases and type boundaries

- Disconnected, undersized, oversteep, wrong elevation or missing crossing fails curation with stable waypoint/witness.

## Error paths

- No authored cave mesh, teleport/level transition or nondeterministic float field may satisfy the route.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

