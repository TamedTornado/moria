# Issue 96 — Enforce edit fairness, capacity pins, and staged readiness

References: `docs/tdd/api.md §Activation and inspection`; `docs/tdd/implementation-plan.md §Gate F2`.

## Valid transitions

- `Accepted queued + scheduler selection -> bounded staging window`. Assert the target state and all specified entry/exit effects.
- `Staged + fixed boundary -> committed window pinned`. Assert the target state and all specified entry/exit effects.
- `Committed + primary keys acknowledged -> primary-ready progress (request remains nonterminal if other work exists)`. Assert the target state and all specified entry/exit effects.
- `All batches committed + all final keys acknowledged -> terminal reconciled and capacity released`. Assert the target state and all specified entry/exit effects.
- `Runnable but unselected -> aging credit increases; eventual selection within 500 ms`. Assert the target state and all specified entry/exit effects.

## Invalid transitions

- Pin whole uncommitted volume; release reserved capacity before terminal; terminal from primary-ready; eviction/hiding discharges pin; interactive priority indefinitely skips other runnable request.
- Invalid action preserves accounting and records invariant failure.

## Lifecycle ordering, guards, and concurrency

- Saturate queue: over-cap rejects before acceptance, every accepted request retains capacity and reaches terminal; staging obeys 4 ms/frame budget.
- Controlled fixed time with interactive, eight colony, catastrophic and zero fixed-tick frames; deterministic interleaving, >=32 changed bricks/s and <=500 ms runnable wait. Concurrent acknowledgements may only advance matching request/revision.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

