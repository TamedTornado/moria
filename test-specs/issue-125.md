# Issue 125 — Write the single save slot asynchronously and atomically

References: `docs/tdd/api.md §Save/load protocol`; `docs/tdd/states.md §World lifecycle`; `docs/tdd/data-model.md §Edit delta set and save file`.

## Valid transitions

- `Idle + accepted SaveWorldRequest -> Saving(snapshot/task reserved)`. Assert target state and entry/exit effects.
- `Saving + encode/write/flush/atomic rename success -> Idle + exactly one SaveWorldCompleted`. Assert target state and entry/exit effects.
- `Saving + encode/write/flush/rename failure -> Idle + exactly one SaveWorldFailed`. Assert target state and entry/exit effects.
- `Idle + concurrent disk request accepted first -> second request remains unaccepted and receives typed Busy`. Assert target state and entry/exit effects.

## Invalid transitions

- Second save/load while operation active; duplicate terminal/task result; success before rename; completion with mismatched request/snapshot counts.
- State/prior slot remains valid, no duplicate terminal event or leaked busy admission.

## Lifecycle ordering, guards, and concurrency

- Use controllable I/O to block and fail encode, create, write, flush and rename separately; fixed/render schedules continue while task is pending.
- Success reports exact final PathBuf/filesystem bytes/changed count after rename. Every failure preserves prior slot and removes or truthfully reports temp state.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

