# Issue 497 — Isolate presentation pressure and failure

References: `collision-presentation.md` TECH-058; issue M-076.

## Valid transitions

- Under normal budget, dirty chunk queues/builds/uploads/installs through TECH-056.
- Under pressure, permitted outcomes are coalesce, retire out-of-interest, retain/hide stale per policy, reject, or mark derived chunk failed.

## Invalid transitions and guards

- Presentation may not claim canonical/query/checkpoint/rollback permits, evict pinned truth, block tick on shader readiness, or feed mesh/timing into canonical input/hash.
- At default at most three presentation jobs are in flight; count/byte/job output bounds are hard.

## Lifecycle and concurrency

- Saturate queued chunks, resident chunks/bytes, in-flight jobs, vertices, indices, job bytes, and dressing record capacities independently.
- Inject shader, output overflow, upload, entity-install, stale-generation, and cancellation failures while confirming ticks and collision queries.
- Move/edit/rollback/withdraw interest during pressure; work coalesces/discards according to source revisions without canonical side effects.

## Rendering states

- Assert exact requested/current/stale/failed counts, truth-to-view revision lag, rebuild cause, overflow, queue age, and commit-to-current latency.
- Canonical root/outcome/collision bytes remain identical to a run with presentation disabled/corrupted/discarded.
- Visual review may assess presentation but cannot satisfy these truth-isolation assertions.
