# Issue 449 — Reserve, build, and seal bounded tick batches

References: `interfaces.md` TECH-019; issue M-022.

## Valid transitions

- `Available -> Reserved(TickPermit) -> Sealed(SealedTickBatch) -> Submitted`; dropping an unsealed permit returns all queue/byte/pending-slot reservations.
- `push` appends owned input/correlation only within reserved count/byte limits. `seal` canonical-encodes, sorts, validates declared counts/unique keys, computes digest, and fixes correlation by canonical order.

## Invalid transitions and guards

- Only `frontier.next_tick()` may reserve/submit. Wrong world, before/after next, already pending, not ready, dependency not ready, full, closed, and invalid batch return exact codes/contexts.
- Push failure returns input and correlation unchanged. Seal failure returns the permit with all accumulated inputs. Submission rejection returns the exact sealed batch.
- Empty/count mismatch/duplicate key/encoding/reservation mismatch are distinct batch errors.

## Lifecycle and concurrency

- Test zero/exact/one-over input, encoded-byte, and correlation-byte limits; duplicate source/sequence on push and canonical-key duplication at seal.
- Race two reservations for the sole pending tick: exactly one permit wins.
- Correct a seal failure by modifying the returned permit, then seal successfully; dropping it instead releases all capacity.
- Receipt drop after accepted submission does not cancel work or release candidate resources early.
