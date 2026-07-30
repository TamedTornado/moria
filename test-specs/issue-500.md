# Issue 500 — Drive staged GPU participant operations

References: `architecture.md` TECH-016; `interfaces.md` TECH-029; `collision-presentation.md` TECH-054; issue M-071.

## Boundary contract

- `prepare_device` creates only generation-local rebuildable pipelines/resources. Active canonical state lives in immutable source/destination token pools.
- Exercise encode Genesis/Tick/RestoreSnapshot/Reconstruct/ExportSnapshot with the exact group-zero ABI, fixed sinks, and one balanced validation scope.

## Multi-system scenarios

- Implement an external GPU participant for both rollback strategies and failure policies; verify genesis, ordinary tick, correction, cold restore, checkpoint export, device loss, and explicit recovery.
- Read source token/artifact/input and write distinct destination/effects/events/status; validate output on GPU/ordinary bounded readback before canonical command processing.
- Complete participants in varied order and prove one-phase ID/local-sequence ordering.

## Failure propagation

- Reject source/destination alias, mixed attempt/generation, missing/duplicate status, count/byte/unused-slot overflow, invalid effect/event/schema, wrong commitment/RNG/snapshot metadata, stale generation, incompatible pipeline, and map/decode failure.
- `encode_* == Ok` is not completion; only validated status/readback may produce PreparedPrivate.
- Failure policy selects retryable no-advance or failed world, never CPU swap/skip/stale token/partial state.
- Cancel/fail/drain uninstalled tokens through last queue use; old-generation results never install.
