# Issue 445 — Order canonical ticks and conflicts deterministically

References: `architecture.md` TECH-011; issue M-020.

## Valid transitions

- `Genesis + tick 0 -> Confirmed(0)`; `Confirmed(t) + tick t+1 -> Confirmed(t+1)` when checked addition succeeds.
- Inputs sort lexicographically by `(phase, source_id, source_sequence)` using fixed phases 0–4. Participant effects use high-bit participant source and `(ParticipantId, local_sequence)`.
- Participant preparation reads only `SourceState(n)` and phase-zero input; eligible commands then compose on staged state in canonical order.

## Invalid transitions and guards

- Before-next, after-next, already pending, duplicate canonical key, tick overflow, missing content/participant/arithmetic resource, and same-tick dependency follow their exact admission or `FailedNoAdvance` paths.
- A failed command emits a deterministic failed outcome with no writes/revision; a tick-global failure publishes no outcomes/root/token.

## Lifecycle, ordering, and concurrency

- Permute producer threads, insertion/completion orders, worker counts, cache layouts, and physical slots for identical sealed bytes; outcomes/root/hash must be byte-identical.
- Overlap direct and participant effects; later canonical order observes earlier staged cells. Stale preconditions fail only the named effect.
- Concurrent participant completions occupy fixed ID slots and combine in ID order; completion timing cannot select authority.
- No DAG, handoff, conflict callback, implicit sequence, or automatic retry is accepted.
