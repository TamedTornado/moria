# Issue 488 — Correct a retained suffix atomically

References: `content-persistence.md` TECH-048; issue M-049.

## Valid transitions

- `Queued -> RestoringPrivate -> ReplayingPrivate -> ValidatingFinal -> ExportingCorrectionBranch -> Publishing -> Ready`.
- Request replaces exactly target+1 through current present; expected hashes are either empty or exactly one per batch in contiguous order.
- Matching branch durability mandates one atomic rollback/log/stream/frontier/participant/receipt/observation publication.

## Invalid transitions and guards

- Reject target not retained/confirmed/strictly older, empty/short/extended/noncontiguous suffix, nonempty hash-count mismatch, log/sink/byte/correction capacity, or nondurable/in-flight current replay stream before pins or callbacks.
- Hash/participant/transition divergence before branch invocation aborts private state and preserves original.

## Lifecycle and concurrency

- Cancel before branch invocation: abort. After invocation: `NotCancellable`, then durable success must publish or provider failure terminally fails world with original frontier.
- Hold branch pending while queries/readers inspect old frontier; no rollback/log/replay/observation/presentation changes occur.
- On all failure paths assert byte-identical original bundle/log/stream/participants and `CorrectionError.committed == None`.
- On success physical prefix advances once, active suffix contains only corrected frames, old readers remain pinned, and only final dirty union schedules.
- Staged GPU tokens reclaim only after queue completion.
