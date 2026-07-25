# Project vision

## What we are building now

**Moria** is a reusable Rust voxel-world substrate consumed only through public crate interfaces. This repository ships that substrate—and adjacent validation that proves the public boundary—not a game or game prototype.

## Purpose

Games and tools need a shared foundation for generated, mutable, authoritative voxel worlds. Moria exists so downstream consumers rely on one deterministic world substrate instead of reimplementing world identity, material truth, bounded change, residency, and persistence per title.

## Product boundary

**This product** owns the reusable world substrate: worlds that can be created and identified, generated deterministically from versioned parameters and seed, observed and mutated as authoritative material truth, streamed under bounded residency with visible lifecycle, persisted as material deltas, diagnosed read-only, and surface-extracted so consumers can present the world without owning substrate internals.

**Adjacent validation** (headless fixtures and a small visual executable) must exist and may only use the same public interfaces. It is not a privileged consumer, not a game prototype, and does not define product content, controls, or presentation policy.

**Downstream / separate products** own gameplay, UX, controllers, characters, animation, authored levels, production content, and all game-specific rules—including any later Product One title.

## Future products and enabling implications

**Product One** (separate repository, after the substrate ships) may place a third-person explorer in a generated region with hills, forest, river, and cave, using skeletal animation and a curated cliff-to-cave traversal. That narrative is future-consumer pressure only.

**Enabling implication:** Moria must remain useful to multiple such exploration-oriented and other games that need generated mutable worlds, without absorbing their characters, routes, assets, or presentation into this product.

## Non-goals

- Implementing game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content.
- Shipping Product One, or any player-facing demo that is a game, inside this repository.
- Treating derived meshes or diagnostics as authoritative world state, or exposing storage, meshing, or scheduler internals to consumers.

## Confirmed vision constraints

- Generation is deterministic for the same versioned parameters and seed; mutation is admitted through a bounded command API and committed atomically; persistence restores the same authoritative material state.
- Streaming bounds resident work, exposes observable lifecycle states, and keeps stale background results from replacing newer truth; public failures are typed and observable.
- Validation and other external consumers share the public API only; performance reporting may include machine identity, but this vision sets no machine-specific correctness threshold.

## Assumptions proposed for approval

None. Seeds already settle product identity, validation adjacency, and exclusion of later game vision from current deliverables.

## Questions for human review

None. The supplied seeds describe one coherent current product (the reusable substrate) and clearly mark Product One and game systems as non-current.

## Seed synthesis

- **`README.md`:** Establishes the name Moria; points at the program brief as the binding current-product definition; notes that later-product text is embedded but not in-repo deliverable; marks the interface reference as supporting context that does not expand scope.
- **`docs/seeds/mixed-project-brief.md`:** Supplies current identity, public boundary, correctness and validation commitments, non-goals, and the Product One future-consumer passage with an explicit non-transfer of game systems into Moria.
- **`docs/seeds/substrate-interface-reference.md`:** Confirms the consumer-facing surface (world identity, bounded query/mutation, streaming states, delta persistence, non-entity registered objects, diagnostics) without adding deliverables; concrete interface shape is design-level, not vision expansion.
