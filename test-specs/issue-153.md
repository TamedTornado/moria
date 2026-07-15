# Issue 153 — Implement the demo application flow

References: `docs/tdd/states.md §Demo state map`.

## Valid transitions

- `Boot + asset/config requests registered (within one update) -> LoadingWorld`. Assert target state and entry/exit effects.
- `LoadingWorld + WorldReady + all active-view/control guards -> Playing`. Assert target state and entry/exit effects.
- `LoadingWorld + fatal startup error -> FatalError`. Assert target state and entry/exit effects.
- `Playing + explicit Load edge/request -> SuspendedForLoad`. Assert target state and entry/exit effects.
- `SuspendedForLoad + LoadWorldCompleted/active view ready -> Playing`. Assert target state and entry/exit effects.
- `SuspendedForLoad + LoadWorldFailed(nonfatal) -> Playing with unchanged world/error HUD`. Assert target state and entry/exit effects.
- `Any nonfatal state + unrecoverable world error -> FatalError`. Assert target state and entry/exit effects.

## Invalid transitions

- Boot->Playing/Suspended; LoadingWorld->Suspended; Playing->LoadingWorld; Suspended->LoadingWorld/Fatal on ordinary load error; FatalError->any ordinary state; duplicate entry request/event.
- Reject/preserve state, no duplicate load or readiness effects.

## Lifecycle ordering, guards, and concurrency

- Playing requires full readiness/control timing; OnExit Loading and load completion clear latches. Suspended entry clears velocity/latches/focus; only Playing enables interaction.
- Concurrent load terminal/fatal: fatal wins safely. Save stays Playing. Demo freeze is UX only—library load tests must remain safe with moving external consumer.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

