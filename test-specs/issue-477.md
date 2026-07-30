# Issue 477 — Bound GPU job pools and staging lifetimes

References: `gpu-runtime.md` TECH-036 and TECH-037; issue M-065.

## Valid transitions

- `Pending permit -> Encoded -> Submitted -> GPU complete -> Mapped -> Decoded/Discarded -> Unmapped/Recalled -> Permit returned`.
- One permit owns all input, scratch, output, staging, diagnostics, bridge cell, and byte reservations for its job.

## Invalid transitions and guards

- Reject/queue at the declared pending or in-flight boundary; no hidden queue or opportunistic allocation may grow.
- Permit cannot return at encode/submit, while mapped, before decode/discard, before staging-belt recall, or while provider/bridge acknowledgement remains.

## Lifecycle and concurrency

- Saturate every count and byte ceiling independently for canonical, query/readback, checkpoint, materialization, presentation, participant, and bridge resources.
- Hold mappings, callbacks, root pins, receipt delivery, and bridge acknowledgement in all combinations; high-water telemetry equals actual reserved resources.
- Cancel before encoding and after submission; former releases immediately, latter suppresses delivery but drains safely.
- Pair every staging-belt `finish` with a submitted encoder and later recall; inject failure between each milestone.
- Race completion/device loss/shutdown with permit release and prove exact once-only return, no ABA reuse, and no old-generation publication.
