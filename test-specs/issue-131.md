# Issue 131 — Swap staged deltas atomically at a fixed boundary

References: `docs/tdd/api.md §Save/load protocol`; `docs/tdd/states.md §World lifecycle/transaction`.

## Valid transitions

- `Loading{SwapPending} + fixed boundary + fully validated staged map -> Loading{Rebuilding}, replace complete delta map and increment revision exactly once`. Assert target state and entry/exit effects.
- `Loading{Rebuilding} + all active truth/presentation ready -> Idle (completion emitted by issue 134)`. Assert target state and entry/exit effects.
- `SwapPending + pre-swap validation failure -> Idle unchanged`. Assert target state and entry/exit effects.
- `Rebuilding + invariant failure -> WorldLifecycle::Failed (no ordinary rollback)`. Assert target state and entry/exit effects.

## Invalid transitions

- Partial/per-brick swap; query during mixed map; second swap/revision increment; Rebuilding->SwapPending/Staging; post-swap nonfatal rollback.
- Assert queries see wholly old or wholly loaded map and transaction/revision remain coherent.

## Lifecycle ordering, guards, and concurrency

- Old-only, loaded-only, overlapping and exact-reverted delta sets; affected keys are union of old and loaded dependencies including object/Horizon/dressing/water/seams.
- Race movement/query ticks with boundary: each tick sees one revision. Concurrent task results from prior revision are rejected.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

