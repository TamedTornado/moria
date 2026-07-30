# Issue 427 — Publish a verified pre-tick genesis frontier

References: `overview.md` TECH-002; `interfaces.md` TECH-017; issue M-013.

## Valid transitions

- `Configuring -> VerifyingGenesis -> Ready(FrontierPosition::Genesis)` only after registries, budgets, content proofs, participants, device capabilities, canonical math vectors, materialization, root hash, and replay header all succeed.
- The ready result has `position == Genesis`, `next_tick == 0`, and replay position sequence 0/next 1. No confirmed-tick rollback entry or outcome exists.

## Invalid transitions and guards

- Any validation, callback, allocation, participant, device, or replay-header failure transitions private construction to terminal failure and publishes no world/registry/root.
- No consumer callback or GPU page allocation occurs until all pure configuration/cross-reference/budget checks pass.
- A wrong/duplicate stream pair, exhausted retired-stream capacity, or missing provider rejects with the original builder; no sink invocation occurs.

## Lifecycle and concurrency

- Concurrent builders using the same `(ReplaySinkId, ReplayStreamKey)` permit only the first accepted construction; later attempts reject without replacing it.
- Failure before sequence-zero invocation releases world and stream reservations. Failure after invocation releases the world permit but commits the client-lifetime stream tombstone.
- Race replay-header completion with failure/cancellation: only matching durable completion can publish Genesis, and late/duplicate completion cannot revive construction.
- After successful genesis, tick zero is the only eligible batch; Genesis remains queryable and byte-distinct from `Confirmed(0)`.
