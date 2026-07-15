# Issue 128 — Gate edits and disk requests during load

References: `docs/tdd/api.md §Save/load protocol`; `docs/tdd/states.md §World lifecycle/transaction`.

## Valid transitions

- `Idle + accepted LoadWorldRequest -> Loading{Staging} and exactly one LoadWorldStarted`. Assert target state and entry/exit effects.
- `Staging + preaccepted edits drained + valid staged map -> Loading{SwapPending}`. Assert target state and entry/exit effects.
- `Staging + any pre-swap failure -> Idle + exactly one LoadWorldFailed, unchanged truth/revision`. Assert target state and entry/exit effects.
- `Any loading phase + new edit -> same phase + synchronous SubmitError::LoadInProgress`. Assert target state and entry/exit effects.
- `Any loading phase + save/load -> same phase + typed Busy`. Assert target state and entry/exit effects.

## Invalid transitions

- Idle->SwapPending/Rebuilding without Started/Staging; begin staging before preaccepted edits finish; accept another disk op; duplicate Started/terminal.
- Invalid requests preserve transaction phase/accounting and do not touch truth.

## Lifecycle ordering, guards, and concurrency

- Race preaccepted edit commits with load acceptance: all accepted-before-Started finish, none after acceptance enter queue.
- Concurrent request ordering is deterministic. Pre-swap cancellation/task/decode failure returns Idle; after swap ordinary rollback is forbidden and invariant failure is fatal.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

