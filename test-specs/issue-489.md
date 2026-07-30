# Issue 489 — Install only current presentation revisions

References: `collision-presentation.md` TECH-056; issue M-074.

## Valid transitions

- `Absent -> Building(source) -> Current(source)`.
- `Current(old) -> Stale(old) -> Building(new) -> Current(new)`.
- `Building/Current/Stale -> Failed(source,reason)` where the failure applies only to that derived source/job.

## Invalid transitions and guards

- Installation requires exact world, volume, brick, source revision, root hash, and device generation matching the current request.
- Mismatched/stale/future/old-generation completion drains/discards and cannot become Current.
- `PresentationCurrent` emits only after matching mesh upload and Bevy entity installation.

## Lifecycle and concurrency

- Edit, move, rollback, withdraw interest, and lose device during Building. Submitted work drains; unsubmitted dirty work coalesces to newest source; rollback queues only final corrected union.
- Exercise every configured stale display policy without any collision/hash change.
- Race two revisions completing out of order; only the newest requested exact source installs.

## Rendering states

- Absent: no installed chunk. Building: prior stale visibility follows policy. Current: source facts match. Stale: never labeled current. Failed: explicit reason/status while matter remains authoritative.
- Overflow, shader, upload, or entity failure cannot block tick publication or alter canonical truth.
