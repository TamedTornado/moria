# Issue 505 — Exercise headless admission and lifecycle state machines

References: `validation.md` TECH-060; `interfaces.md` TECH-021; issue M-116.

## Properties

- For all headless fixtures, `MinimalPlugins` plus deliberately selected Moria plugins and explicit `App::update()` drive progress; no wall-clock wait, window, renderer loop, or no-op authority backend is used.
- Every accepted operation has one receipt/operation record and every rejection returns ownership unchanged.

## State-machine matrix

- Exercise every valid/invalid transition, phase, cancellation cutoff, terminal state, retry rule, generation outcome, and shutdown disposition for Genesis, Tick, Interest, Query, Observation resnapshot, Checkpoint, Correction, Restore, Replay, Recovery, and Shutdown.
- Poll repeatedly and through clones; terminal values remain idempotent. Drop receipts before/after submission; admitted work and bounded result retention remain correct.
- Race cancel, completion, device loss, and shutdown at each boundary; exactly one terminal result and exact-once resource release occur.

## Error and edge paths

- Pattern-match arithmetic/budget/participant/provider/device/shutdown/internal tick failures as exact `FailedNoAdvance` records.
- Saturate queues and terminal caches; assert `Full`/backpressure without growth.
- Missing `RenderApp` is `BackendUnavailable` and never installs a fake GPU path.
- Dirty shutdown reports retained truth and abandoned receipts; invalid transitions leave state unchanged.
