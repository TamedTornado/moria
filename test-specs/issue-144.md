# Issue 144 — Implement grounded, airborne, and paddling locomotion

References: `docs/tdd/states.md §Player locomotion state`; `docs/tdd/systems.md §Collision and movement systems`.

## Valid transitions

- `Grounded + consumed jump OR no support -> Airborne`. Assert target state and entry/exit effects.
- `Airborne + downward walkable solid contact and velocity<=0 -> Grounded`. Assert target state and entry/exit effects.
- `Grounded + qualifying water overlap/surface band -> Paddling`. Assert target state and entry/exit effects.
- `Airborne + qualifying water overlap/surface band -> Paddling`. Assert target state and entry/exit effects.
- `Paddling + no qualifying water + solid exit support -> Grounded`. Assert target state and entry/exit effects.
- `Paddling + no qualifying water + no solid support -> Airborne`. Assert target state and entry/exit effects.

## Invalid transitions

- Airborne + jump; Paddling + jump/gravity; Grounded + nonqualifying water; Airborne + upward/nonwalkable contact; any transition to underwater/climbing/crouching/damage state.
- Rejected event preserves mode except normal velocity integration; at most one transition/tick.

## Lifecycle ordering, guards, and concurrency

- When multiple guards pass, priority is paddling entry, grounded landing, then airborne; test all pair/triple combinations.
- Paddling requires actual public WaterSample/water_volume, zeros downward velocity, pins configured offset, caps speed 3 m/s; water-edge exit adds no impulse. Concurrent water/solid changes use one coherent revision.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

