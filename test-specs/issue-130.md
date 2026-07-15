# Issue 130 — Extend streaming across the complete route

References: `docs/tdd/overview.md §High-level architecture`; `docs/tdd/rendering.md §Scene hierarchy and bands`.

## Boundary contracts

- The complete-route generation/store/read slice feeds existing band planner, materialization, terrain/water/object/Horizon/dressing extract/install path without alternate truth.
- All installed outputs retain existing token/revision/LOD/owner contracts and shared asset resource.

## Multi-system scenarios

- Travel every complete route feature through Near/Middle/Far/Horizon; cross each boundary/hysteresis both directions; underground occlusion behavior.
- Edit/load, evict, reactivate and transition a feature/object; compare current public truth and unique logical owner.

## Failure propagation

- Stale/duplicate output, task failure or missing feature asset cannot create a resident owner or lose deltas.
- Untouched uniform wilderness remains metadata-only; repeated instances do not allocate unique mesh/material assets.

## Ordering guarantees

- Public truth revision precedes derived rebuild; transitions retain old until current new ready; reactivation never resurrects stale presentation.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

