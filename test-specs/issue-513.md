# Issue 513 — Prove historical observation and gap recovery headlessly

References: `validation.md` TECH-060 observation fixtures; `interfaces.md` TECH-025; issue M-118.

## State-machine matrix

- For every subscription and retained sequence range, polling must either return the exact matching records, expose the exact lost interval as a gap, or report closed.
- Admit subscriptions for Now/OldestRetained/After, fixed finite membership, all kind/spatial/world-event filters; reject every invalid request/cursor.
- `poll -> Items|Gap|Closed`; `Gap -> resnapshot pending -> Ready -> resume -> Items|Gap`; close/drop/shutdown end in Closed.

## Historical configurations

- Append create/move/material/presentation/retire records, move and retire the volume again, reclaim old directory state, then poll. Matching must use immutable append-time membership/bounds/revision facts only.
- Verify future volumes do not enter old membership and world events require explicit inclusion.
- Carry correlation through ordinary outcomes, expire it with overwrite, and assert `correlation_lost` through gap/resnapshot/resume.

## Concurrency and failure paths

- Trigger count-only and byte-only overwrite. In each case, pin a resnapshot and append enough records during it to produce both possible outcomes: exact suffix or a second honest gap.
- Reject wrong-stream, beyond-tail, backward-before-trustworthy, unproduced, and closed cursors without moving cursor.
- Failed/cancelled resnapshot leaves cursor unchanged and releases pins/capacity.
