# Issue 140 — Resolve player movement against voxel truth

References: `docs/tdd/api.md §Player and camera action contracts`; `docs/tdd/systems.md §Collision and movement systems`.

## Properties that must hold

- For every 1/60 tick from a valid start, resulting capsule is in bounds, nonoverlapping solid voxels, within speed/acceleration/integrated-displacement bounds and uses <=4 contacts/substeps at <=0.125 m.
- For every collision, movement reads only current public solid capsule queries; water/render triangles are not walls and a committed opening is traversable next tick.

## Entity configurations to test

- Ground run/sprint, acceleration/deceleration, gravity/jump, air steering, slopes just below/at/above 48°, 0.25 m step and >0.30 m obstacle, slide corners, four contacts.
- Route shelves, ruin stairs, caves, region edges, newly dug cell and invalid overlapping start.

## Edge cases and type boundaries

- Invalid start triggers bounded one-voxel recovery/debug invariant; unresolved overlap never returns a silently valid step.

## Error paths

- Query limit/error produces safe unchanged/bounded response and typed diagnostic, not mesh fallback or tunneling.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

