# TDD simplicity eval case: overengineered adapter amendment

## Identity

- Case ID: `moria-overengineered-adapter-amendment`
- Baseline revision: `2ab6796ddeba169545a5922461b2e0908ccd2ad2`
- Candidate revision: `dab1a1a5f7a60e409ef054b2bcbdf5d6468a51e9`
- Artifact under evaluation: `docs/tdd/`

## Authority

Preserve the three generic substrate capabilities requested during human review
of Moria PR 395:

1. Atomic GPU-resident fracture into independently moving child volumes.
2. CPU-authored multi-fidelity activity regions, with coarse simulation outside
   and full physics inside the selected regions.
3. A bounded, opaque, asynchronous GPU-to-CPU adapter egress channel.

Moria must expose substrate hooks for those capabilities. It must not implement
physics, damage, weapons, gameplay semantics, or a game-specific event model.

## Observed concern

The candidate revision responded to the authority with a large architectural
amendment. The experiment asks whether each intervention causes a drafting
agent to preserve the required hooks while removing, reusing, or narrowing
machinery that is not necessary at the substrate boundary.

Document length alone is not a failure. A smaller revision fails if it drops a
required capability, hides an obligation behind vague deferral, or weakens
ownership, boundedness, ordering, lifecycle, or failure contracts needed for a
usable substrate interface.
