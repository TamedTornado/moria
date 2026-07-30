# Issue 484 — Track GPU completion and device generations

References: `gpu-runtime.md` TECH-037; issue M-066.

## Valid transitions

- `Encoded -> Submitted(SubmissionIndex) -> GpuComplete -> MapPending -> Mapped -> Decoded -> terminal success/failure`.
- Mapping bytes is legal only after successful callback; mapped views drop before unmap; permit/slot returns after decode/discard and acknowledgement.

## Invalid transitions and guards

- Submit index is not completion. Reading before map, unmapping with live view, GPU use while mapped, decoding short/misaligned data, or publishing before decode is rejected.
- Device loss makes the generation terminal. Old callbacks/envelopes can release resources only.

## Lifecycle and concurrency

- Cancel at each state: before submit removes queued work; after submit suppresses delivery but preserves lifetime tracking.
- Lose device with jobs in every state. One GenerationLost control record plus each job’s reserved terminal envelope drains without overwrite; new generation cannot reuse old tokens.
- Race duplicate completion, mapping callback, shutdown, and bridge acknowledgement; each resource releases exactly once and no terminal state regresses.
- Timeouts record diagnostics/environmental failure only and never select a canonical result.
- Verify telemetry for submitted-to-complete, complete-to-map, decode, oldest age, bytes, and cancellation point.
