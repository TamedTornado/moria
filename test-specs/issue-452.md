# Issue 452 — Manage bounded interest and readiness lifecycle

References: `interfaces.md` TECH-022; issue M-024.

## Valid transitions

- Per capability/range: `Cold -> Requested -> Materializing -> Ready -> Retiring -> Cold`; `Materializing -> Failed -> Requested` only by explicit retry.
- Upsert atomically installs/replaces one consumer-owned `InterestId`; withdrawal atomically makes the installed request eligible for retirement.

## Invalid transitions and guards

- Reject empty capability set, unknown/retired volume, duplicate/unsorted listed IDs after normalization failure, duplicate live ID misuse, invalid bounds, and exact required brick count exceeding `min(max_resident_bricks, world budget)`.
- If `allow_partial == false`, no clipped coverage is installed. If true, ready result reports exact requested/covered bounds and `complete`.
- Future volume creation never expands existing finite membership.

## Lifecycle and concurrency

- Test one volume and sorted unique listed selectors at zero/maximum membership and capacity boundaries.
- Withdraw while root/query/checkpoint/observation/artifact/GPU pins or dirty scars remain: transition enters/awaits retiring but truth is not evicted until all guards pass.
- Race upsert and withdrawal for one ID; accepted control-queue order produces one complete installed state, never mixed fields.
- Interest readiness and preload may materialize/pin content but cannot alter simulation-domain bytes/hash; only a tick activation may do so.
- Device loss preserves installed interest but reports truth unavailable; shutdown cancels unapplied controls.
