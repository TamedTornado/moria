# Issue 473 — Drive world failure and shutdown lifecycle

References: `interfaces.md` TECH-028 and TECH-021 shutdown row; issue M-029.

## Valid transitions

- `Configuring -> VerifyingGenesis -> Ready`; `Ready <-> Replaying`; `Ready/Replaying -> RecoveringParticipant -> Ready|Failed`; `Ready -> Failed`; and `Ready/Replaying/RecoveringParticipant/Failed -> ShuttingDown -> Closed`.
- Shutdown performs admission closure, unsubmitted tick abandonment, submitted-work drain, configured checkpoint/dirty reporting, observation/participant closure, then derived/canonical/device release.

## Invalid transitions and guards

- Reject ticks/queries/interests/checkpoints once shutting down; second shutdown returns already-shutting-down/closed and creates no second drain.
- Failed world rejects new ticks but retains the last trustworthy readable frontier where failure scope permits.
- Shutdown cannot create a tick or relabel dirty truth as durable.

## Lifecycle and concurrency

- Cover Genesis-only shutdown, Ready/Replaying/Recovering/Failed shutdown, required checkpoint success/failure, and explicit dirty-without-checkpoint.
- Race shutdown with reserved/unsubmitted and submitted tick, query, provider call, participant callback, replay append, and recovery. Each receipt follows its TECH-021 shutdown disposition.
- `ShutdownReport` contains exact last frontier/durable checkpoint, abandoned receipt IDs, every dirty root, optional checkpoint, and replay export failure before raw pins release.
- Verify release order and no old-generation completion publishes during drain.
