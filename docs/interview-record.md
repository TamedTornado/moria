---
status: complete
coverage:
  Problem Statement: 2/2
next_id: 11
next_note_id: 4
---


## Problem Statement


### q9: What problem are we solving?
- status: answered
- answer: |-
  Moria's product is the reusable voxel-world substrate, exposed as a Rust crate
  or a small family of tightly scoped Rust crates.
  
  The actual game is a separate downstream consumer and is not part of this
  repository. Moria may include a walkable-world executable, but that executable
  is only a validation harness. It must consume the substrate through the same
  public interfaces available to an external game rather than owning privileged
  or game-specific implementation paths.
  
  This is an immediate, concrete reason to use a Cargo workspace boundary between
  the reusable substrate and its validation harness. The precise crate split is a
  technical-design decision; the consumer boundary is not optional.
  
  Game rules and the future System, LLM, spell, gas, combat, AI, and building
  layers are out of scope. Compatibility seams may be designed where the substrate
  requirements demand them, but those layers must not be implemented here.

### q10: What problem are we solving?
- status: answered
- answer: |-
  These documents are preserved publicly and verbatim from the source material
  provided on 2026-07-13:
  
  1. `product-one-seed.md` defines the binding substrate implementation and its
     walkable-world validation harness.
  2. `voxel-world-substrate.md` is the substrate architecture reference. Only the
     portions selected by the Product One seed are required for this milestone.
  
  `project-boundary.md` records the operator's binding clarification: Moria is a
  reusable crate consumed by a separate game, and the included executable is only
  a public-API validation harness.
  
  Moria is only the voxel-world substrate. Broader game, System, LLM, spell, gas,
  combat, AI, and building intent is deliberately absent and out of scope.


## General Notes


#### n1: Seeded from docs/seeds/product-one-seed.md

#### n2: Seeded from docs/seeds/project-boundary.md

#### n3: Seeded from docs/seeds/README.md
