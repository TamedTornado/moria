# Issue 425 — Integrate Moria with the Bevy renderer device

References: `gpu-runtime.md` TECH-031 and TECH-036 bridge inequality; issue M-059.

## Boundary contract

- `MoriaPlugin` installs focused feature plugins. Main world owns facade queues/receipts/root metadata; `RenderApp` alone owns device-bound buffers, pipelines, submissions, and mappings.
- The Bevy path obtains exactly the existing `RenderDevice` and `RenderQueue`; instrument device creation to prove no second adapter/device request occurs.
- Plugin finish inserts the same `Arc<RenderCompletionBridge>` into both worlds. Every extracted job reserves one of 32 fixed cells and carries `(JobId, WorldId, DeviceGeneration, attempt_nonce)`.

## Multi-system scenarios and ordering

- Extract only bounded descriptors, immutable canonical bytes, and generation/root deltas; large payloads move through owned staging permits.
- Saturate job cells while retaining two dedicated control cells. The next job is backpressured before extraction; no cell is overwritten or dropped.
- Initialize recovery-sensitive resources in `RenderStartup`, replace generation, and prove old internal root/participant tokens cannot resolve on the new device.

## Failure propagation

- Missing `RenderApp` yields `BackendUnavailable`; it must not install a no-op canonical backend.
- Duplicate, unknown, cancelled, or old-generation completions release only lifetime resources and never publish.
- Device loss and shutdown enqueue/drain fixed control and per-job records before either bridge clone is removed.
- A bridge accounting violation fails closed with the last trustworthy frontier.

No public API may expose wgpu handles or internal generation tokens.
