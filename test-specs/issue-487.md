# Issue 487 — Restore a durable checkpoint into a private world

References: `content-persistence.md` TECH-046; issue M-044.

## Valid transitions

- `Loading -> Verifying -> Rebuilding -> RestoringParticipants -> ExportingReplayHeader -> Publishing -> Ready`.
- Load exact key from exact store, verify manifest/blobs/chunks, rebuild private GPU roots/tokens, recompute bottom-up root, durably append checkpoint-anchored sequence-zero header, then publish once.

## Invalid transitions and guards

- Reject wrong store/key, contract/version/config/placement/lineage/root/material/provider/participant, missing/corrupt/short/long blob or replay range, unsupported capability, budget overflow, hash/commitment divergence, or checked next-tick overflow.
- No fallback store/content/participant and no migration/rebase is permitted.

## Lifecycle and concurrency

- Cold-process fixture destroys all live objects and restores only from manifest references.
- Hold replay header completion pending: no world is visible. Matching durability publishes saved `Confirmed(t)`, next tick t+1, replay sequence 0/next 1.
- Cancellation/failure before sink invocation releases stream pair/tombstone; after invocation drains and retires it, with no publication.
- First later tick appends at sequence one.
- Rejection returns builder/request unchanged; accepted failure drops private bundle and releases world permit after last device/store use.
