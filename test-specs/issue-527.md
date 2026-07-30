# Issue 527 — Implement the complete public-boundary scenario

References: `validation.md` TECH-064 scenario 1; issue M-092.

## Multi-system lifecycle

1. Through public facade only, configure providers/participants, publish Genesis, and confirm ticks.
2. Upsert/withdraw interest; run sample/region/trace/overlap/sweep queries; subscribe/filter/poll observations and force/resnapshot/resume a gap.
3. Read telemetry, checkpoint, correct a suffix, cold restore, public-replay exported records, poison one expected value for earliest divergence, request supported recovery, and shut down.

## Properties

- Every accepted step returns exact frontier/root/revision/result/stream position and every rejection preserves ownership.
- Checkpoint/restore/replay/correction durability ordering follows their public receipts; no live/private intermediate state is observable.
- Divergence artifact identifies the earliest poisoned tick and bounded exact prefix.

## Failure and closure paths

- Exercise at least one admission, provider, capacity, cancellation, device, and shutdown error with exact public type.
- Compile/run with no private imports, test-only features, storage handles, raw buffers, or mutation hooks.
- Missing/cold truth is never empty; presentation cannot substitute for canonical/query evidence.
- Scenario invokes every TECH-070 callable exactly through its supported boundary and demonstrates terminal receipt/stream continuation.
