# Issue 538 — Implement persistence, replay, capability, and presentation failure-matrix slices

References: `validation.md` TECH-065; issue M-099.

## Properties

- For every required persistence, replay, capability, or presentation failure, the matrix must retain the exact last trustworthy/durable/derived-state outcome without substitution.
- Each row specifies exact error layer/code, retryability, receipt/world state, committed effect, observation/evidence, durable/dirty state, and retained last trustworthy frontier.
- Presentation/evidence/capability results cannot substitute for canonical or persistence truth.

## Required rows/configurations

- Store put/get/manifest/commit failure; corrupt/incomplete checkpoint; wrong lineage/exact root/material/source; rollback outside window.
- Replay header/identity/expected poison, gap/order/branch/history mismatch, sink failure at genesis/tick/correction/public replay.
- Unsupported adapter/device capability, device loss, candidate injected determinism failure.
- Presentation queue/output/shader/upload/stale-generation pressure/failure.

## Failure invariants

- No scar disappears and no failed restore/replay/correction publishes partial state.
- Exact requested provider is reported and no fallback occurs.
- Unsupported capability is distinct from device loss/determinism violation.
- Presentation failures preserve canonical hash/collision and report only derived status/telemetry.
- Internal storage handles never appear; completion timing cannot select authority.
