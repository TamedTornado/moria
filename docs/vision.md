# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate library. It exposes a public crate surface so games and other external programs can create worlds, stream and observe material truth, apply bounded edits, extract surfaces, and persist authoritative deltas—without owning any particular game.

## Purpose

Moria exists so multiple downstream consumers can share one deterministic, mutable, persistable voxel world rather than each reimplementing that foundation. The product’s value is a stable public boundary that keeps world authority in the substrate and keeps gameplay policy outside it.

## Product boundary

**In product:** the substrate crate and the world capabilities it owns for shared reuse—seed-based deterministic generation, sparse voxel material authority, bounded streaming with observable lifecycle, bounded mutation, surface extraction, persistence of authoritative deltas, and read-only diagnostics.

**Adjacent delivery, not product identity:** a minimal validation executable and headless fixtures that exercise the same public interfaces any external consumer would use. They prove the crate is usable; they are not a game prototype and hold no privileged world path.

**Out of product:** game rules, combat, inventory, AI, narrative, characters, animation, controllers, authored levels, production content, presentation policy, and any player-facing demo.

## Future products and enabling implications

A separate Product One repository may later ship a third-person explorer in a generated region (hills, mixed forest, river, cave) with curated traversal and skeletal animation. That product is a future consumer of Moria, not a current deliverable.

Enabling implication only: the substrate must remain generically useful so such a game can generate, stream, edit, mesh, and persist world material through public interfaces. Controllers, characters, animation, forest content, routes, and presentation stay with Product One.

## Non-goals

- Implementing or shipping any game, player controller, character, animation, or authored content in this repository.
- Privileged internal access for validation or demos; they use the public crate surface only.
- Treating derived meshes or diagnostics as authoritative world state.

## Confirmed vision constraints

- Consumers—including validation—integrate only through public crate interfaces; they must not reach into storage, meshing, or scheduler internals.
- For the same versioned parameters and seed, generation is deterministic; persistence restores authoritative material state; derived meshes and diagnostics never become that authority.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds agree on a single current product (the Rust substrate), place Product One as a later consumer, and treat validation as required adjacent delivery outside product identity.

## Seed synthesis

- `README.md` names Moria as the reusable voxel-world substrate, states that only current substrate commitments are repository deliverables, and frames the interface reference as non-expanding technical context; compatible detail stays subordinate to downstream design.
- `docs/seeds/mixed-project-brief.md` supplies the binding product identity, purpose, public consumer boundary, substrate-owned world outcomes, validation-as-adjacent-delivery posture, non-goals, and the Product One future-consumer separation; its operation-level correctness and fixture detail remain subordinate input to later design.
- `docs/seeds/substrate-interface-reference.md` reinforces the public integration surface and authority split (queries, commands, streaming lifecycle, delta persistence, diagnostics) without adding deliverables; its interface specifics remain subordinate design input.
