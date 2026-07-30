# Issue 462 — Reclaim generational GPU storage safely

References: `architecture.md` TECH-015; `gpu-runtime.md` TECH-037; issue M-034.

## Valid transitions

- `Referenced(slot,generation) -> Retiring -> DrainingLastUse -> Reclaimed(next_generation)`.
- Reclamation first removes new references, then waits for every root/reader/submission/map/decode use; generation increments before free-list return.
- Generation wrap transitions the slot to permanently retired, never reusable.

## Invalid transitions and guards

- Reject stale `(slot,generation)` before encoding. Never reclaim/reuse while live, rollback, query, checkpoint, replay, artifact, participant, mapping, or GPU submission pins remain.
- Old-generation completion may acknowledge last use but cannot install an artifact/root/result.

## Lifecycle and concurrency

- Hold each pin family independently and in combinations; release in every order and verify reuse only after the final declared pin plus queue/map completion.
- Race allocation with delayed completion and cancellation; no two live handles resolve to the same slot/generation.
- Perturb free-list order and physical histories; canonical identities/outcomes/hashes remain unchanged.
- Inject duplicate/late completions and generation wrap; exact once-only permit/cell release occurs without ABA publication.
- A mapped buffer remains ineligible for GPU reuse until views drop and unmap completes.
