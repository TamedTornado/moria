# Issue 492 — Recover fail-closed after device loss

References: `gpu-runtime.md` TECH-038; issue M-067.

## Valid transitions

- Device loss: `Ready -> RecoveringParticipant` for retry policy, or `Ready -> Failed` for any fail-world GPU participant.
- Explicit recovery: `Queued -> CreatingGeneration -> LoadingAnchor -> Replaying -> Comparing -> Ready` only when the rebuilt bundle equals the retained last frontier.

## Invalid transitions and guards

- Admission requires expected frontier byte-equal to last trustworthy, exact visible store/key anchor, and replay limits within recovery budgets.
- Reject missing/incompatible checkpoint/genesis, excessive suffix, unsupported capability, canonical-math failure, participant restore failure, or hash/RNG/commitment mismatch.
- Old-generation results cannot install or make the world ready.

## Lifecycle and concurrency

- Lose device at candidate, checkpoint readback, participant, query, mapping, and publication-adjacent phases. All candidates fail no-advance and last trustworthy frontier identity remains.
- Restore snapshot/replay bytes into new staged tokens, replay confirmed in-memory suffix, compare every retained tick, then republish the same frontier without canonical byte change.
- Cancel before/after new-generation submission; later cancel drains and leaves RecoveringParticipant.
- Failure under retry policy stays recoverable for a new explicit request; no timer retry/CPU fallback/alternate store occurs. Fail-world remains terminal.
