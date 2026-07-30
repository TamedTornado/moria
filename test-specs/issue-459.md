# Issue 459 — Retain bounded shared rollback frontiers

References: `architecture.md` TECH-014; issue M-032.

## Valid transitions

- Keep a separate Genesis root plus a deque of confirmed frontiers. Each successful tick installs one new confirmed frontier and may evict only an over-capacity unpinned oldest confirmed frontier.
- Restoring an exact retained frontier swaps root/metadata/participant tokens without enumerating material bricks.

## Invalid transitions and guards

- Genesis requires capacity for at least 20 confirmed frontiers. Runtime logical-budget preflight rejects a tick rather than violating that minimum.
- A frontier cannot leave while pinned by live state, rollback, replay/correction, query, checkpoint, participant/artifact lease, or GPU submission.
- Genesis is never counted or decoded as a confirmed-tick entry.

## Lifecycle and concurrency

- Retain 32 ticks, restore depths 1/5/10/20/32, and compare root hash, metadata, participants, input/outcome digests, and tick position exactly.
- Use overlapping and disjoint brick/volume changes to prove additional logical state is proportional to changed paths and untouched paths are shared.
- Hold every pin family independently across capacity eviction; only unpinned eligible roots reclaim.
- Race install/restore/readers: each reader observes one immutable bundle; install cost remains registry/participant handle scope, not whole-world traversal.
- Byte pressure produces exact no-advance/capacity failure and preserves all reachable frontiers.
