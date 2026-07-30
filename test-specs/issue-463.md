# Issue 463 — Coordinate immutable participant frontier tokens

References: `architecture.md` TECH-016; issue M-035.

## Valid transitions

- `Reserved -> Preparing/RestoringSnapshot/Reconstructing -> PreparedPrivate -> InstalledInFrontier`.
- Failure/cancel branch is `Failed -> Aborting -> DrainingLastUse -> Reclaimed`.
- Genesis token binds Genesis; tick `n` destination binds `Confirmed(n)` and source binds `SourceState(n)`.

## Invalid transitions and guards

- Only `PreparedPrivate` may enter a bundle, and installation occurs only with the substrate root/all participants in one pointer swap.
- Reject wrong participant/contract/frontier/root/commitment/generation, duplicate/late completion, missing/oversized state/snapshot/effects/events/artifacts, divergent RNG commitment, snapshot metadata on wrong strategy, or same-tick dependency.
- Cancellation is accepted only before submission; later it suppresses installation and drains.

## Lifecycle and concurrency

- Prove source token bytes/metadata never change during preparation and adapters have no global mutable canonical state.
- Complete participants in arbitrary order; fixed ID slots and ID-sorted commitments/RNG streams yield identical bundle/hash.
- In correction, restore several private tokens then fail: original live bundle remains byte-identical and all staged tokens reclaim after last use.
- Keep effect/event sinks separate; events reach confirmed receipt/replay only, never another participant or observation stream.
- Device loss invalidates staged GPU tokens; CPU tokens follow policy. No stale-token reuse, empty commitment, skip, or partial bundle is legal.
