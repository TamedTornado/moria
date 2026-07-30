# Issue 417 — Implement pollable receipt state machines

References: `interfaces.md` TECH-021 and its lifecycle matrix; issue M-023.

## Valid transitions

- Common GPU work: `Reserved -> Admitted -> Preparing -> Encoded -> Submitted -> GpuComplete -> Decoding -> Ready`, with failure/cancel terminal branches specified by each family.
- Genesis: `Verifying -> Materializing -> Submitting -> ExportingReplayHeader -> Ready|Failed`.
- Tick: `Reserved -> Admitted -> Preparing -> Encoded -> Submitted -> GpuComplete -> Decoding -> Ready|Failed`.
- Interest: `Queued -> Applying -> Ready|Failed`; cancellation before Applying yields Cancelled.
- Query: `Queued -> WaitingForReadiness -> Encoded -> Submitted -> Mapping -> Decoding -> Ready|Failed|Cancelled`.
- Observation resnapshot: `Queued -> Pinning -> Querying -> Encoding -> Ready|Failed|Cancelled`.
- Checkpoint: `Queued -> Pinning -> Reading -> StoringBlobs -> CommittingManifest -> Ready|Failed|Cancelled`.
- Correction: `Queued -> RestoringPrivate -> ReplayingPrivate -> ValidatingFinal -> ExportingCorrectionBranch -> Publishing -> Ready|Failed|Cancelled`.
- Restore: `Loading -> Verifying -> Rebuilding -> RestoringParticipants -> ExportingReplayHeader -> Publishing -> Ready|Failed|Cancelled`.
- Public replay: `LoadingOwnedRecords -> VerifyingHeader -> ReplayingPrivate -> ComparingExpected -> ExportingReplayHeader -> ExportingReplayPrefix -> Publishing -> Ready|Failed|Cancelled`.
- Recovery: `Queued -> CreatingGeneration -> LoadingAnchor -> Replaying -> Comparing -> Ready|Failed|Cancelled`.
- Shutdown: `ClosingAdmission -> Draining -> FinalCheckpoint -> Releasing -> Ready|Failed`.
- Query alone may expose `WaitingForReadiness` with `ProgressBlocker::Query`; all other families have `blocker == None`.

## Invalid transitions and guards

- Reject any transition out of `Ready`, `Failed`, or `Cancelled`; repeated poll returns the same shared terminal allocation.
- Cancellation guards: tick/genesis/shutdown are not consumer-cancellable; pre-encoding query/interest/resnapshot cancels; post-submission suppresses delivery and drains; correction becomes `NotCancellable` after branch invocation; restore/replay/recovery follow their specified private-work cutoffs.
- Old-generation completions may release resources but cannot transition to ready/published.

## Lifecycle and concurrency

- Clone receipts before and after terminalization; all clones observe one state/result and retain one record.
- Drop all handles before/after submission and prove admitted work persists while terminal capacity releases only under the cache/handle rules.
- Race cancel with encode/submit/completion: exactly one cutoff result wins, resource permits return only after last GPU/map/provider use, and no double terminalization occurs.
- Saturate terminal count/byte pools; new result-producing admission returns `Full` without unbounded growth.

Explicit retry always creates a new operation/receipt except subscription resume.
