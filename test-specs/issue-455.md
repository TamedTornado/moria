# Issue 455 — Normalize canonical simulation-domain membership

References: `interfaces.md` TECH-030; issue M-030.

## Valid transitions

- `Inactive + ActivateRegion -> normalized active union`; `Active + DeactivateRegion -> normalized remaining union`.
- Intervals are normalized per `(volume, activity_class)` by stable endpoint sort with the TDD’s start-before-end tie rule and emit each covered brick once.

## Invalid transitions and guards

- Admission requires live volume, brick-aligned half-open range, exact base root/optional subtree digest, and all content/collision dependencies pinned.
- Missing dependency before admission returns `DependencyNotReady`; corruption after admission fails the whole tick with no domain change.
- Interest, camera, cache, I/O completion, or presentation events cannot trigger either canonical transition.

## Lifecycle and concurrency

- Exercise disjoint, adjacent, nested, identical, and partially overlapping regions across same/different activity classes; compare normalized bytes with an interval oracle.
- Permute activation input arrival, materialization completion, and presentation schedules for identical sealed bytes; union and world hash remain identical.
- Roll back across activate/deactivate sequences and recover the exact prior union/content commitments.
- Concurrent readers pin old roots and observe the old complete union until tick publication.
- Invalid/empty/reversed ranges, wrong content identity, retired volume, and capacity overflow reject without a partial union.
