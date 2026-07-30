# Issue 551 — Implement correction-branch durability scenarios

References: `validation.md` TECH-066 correction scenarios; issue M-102.

## Lifecycle scenarios

- Confirm an original suffix, request a complete replacement through the same present, replay private roots/participants, and hold the one correction-branch append pending.
- Before durability assert old frontier, rollback deque, active log/history, physical replay position, observations, participants, and presentation dirty set are unchanged.
- On matching durability assert one atomic publication: corrected suffix only is active, physical prefix advances one sequence, active-history digest drops superseded records, and final dirty union schedules.

## Guard and cancellation cases

- Expected-hash vectors: empty, exact valid, each poisoned index, short, and excess. Short/excess reject before pins/callback/sink; poison fails before advancing past that private tick.
- Cancel before invocation succeeds and aborts; cancel after invocation is `NotCancellable`.
- Admit correction while an ordinary tick append is pending: return `PersistenceBackpressure` before participant/branch work.

## Failure/concurrency paths

- Branch sink failure leaves original world byte-identical, terminally fails world, and exposes matching receipt/correction/lifecycle failure with committed none and exact replay metadata.
- Hold old readers/checkpoint pins through successful splice; their old roots remain valid until drain.
- Every staged CPU/GPU token and private byte/callback permit releases at its declared last use.
