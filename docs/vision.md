# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or small family of tightly scoped Rust crates. It is the engine-layer foundation that makes a natural-looking, fully mutable material world available to games and other consumers—not a game, demo, or content product.

## Purpose

Provide a shared world substrate where terrain, geology, and deep underground volume are real mutable matter behind a public query and mutation surface, so multiple game modes can share one world truth without embedding game rules in the world layer.

## Product boundary

This product owns the substrate outcome: generated and streamed voxel matter, material truth under a non-authoritative presentation view, consumer-facing mutation and query interfaces, and a hard separation so adjacent consumers use only public interfaces.

Not this product: any actual game; game rules; System or LLM features; spells, gas, combat, AI, or building gameplay layers; and authored game content, characters, controllers, cameras, presentation, or demo acceptance scenarios. A walkable-world executable, if present, is an adjacent validation harness that must exercise the substrate only through the same public interfaces an external game would use.

## Future products and enabling implications

Future consumers include a System-driven ARPG, DF-style fortress or colony play, a Moria-style deep descent, and pure sandbox modes. The substrate should enable continuous deep-Z play, mutable matter suitable for dig and build fantasies, and clean layering so those games remain above the substrate. Their gameplay, content, controls, and policy stay consumer-owned.

## Non-goals

- Implementing game systems such as combat, AI, spells, gas economy, building UX, or gameplay mechanisms
- Shipping a finished game or game-mode content in this repository
- Treating validation-harness routes, characters, or performance gates as part of substrate identity

## Confirmed vision constraints

- Adjacent consumers, including any validation harness, have no privileged access and use the same public interfaces as external games.
- Game rules and System, LLM, spell, gas, combat, AI, and building layers are not implemented here; compatibility seams may exist only where substrate needs demand them.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a mandatory current delivery alongside the substrate, or only a permitted adjacent artifact?

- **Proposed answer:** Mandatory as an adjacent validation delivery that consumes public interfaces only; it does not redefine product identity, and its character, controls, content, route, and performance gates remain outside substrate scope.
- **If different:** If only permitted, current delivery can be substrate crates alone without committing to any harness executable. If harness demo content, controller, or performance are treated as product scope, the product becomes a walkable demo rather than a reusable substrate.

## Seed synthesis

- **README.md:** Named Moria as the reusable GPU-resident voxel-world Rust-crate substrate and positioned the walkable-world executable as a separate consumer and validation harness, not a game layer.
- **docs/seeds/project-boundary.md:** Binding correction that the product is the substrate crate or crates, the actual game is outside this repository, a harness may exist only as a public-interface consumer, and game or System layers are out of scope; packaging detail remains subordinate design input.
- **docs/seeds/product-one-seed.md:** Contributed the first consumer-shaped validation story—a generated region with dig and place proof and walkable traversal—and the motivation for an early substrate slice; its demo content, controller, milestones, and performance numbers remain subordinate design and acceptance input, not current product identity.
- **docs/seeds/voxel-world-substrate.md:** Contributed the long-horizon substrate purpose—natural look over voxel truth, deep-Z, and a reusable matter, physics, and query foundation for multiple game genres—and future-consumer relationships; mechanism inventories and open technical questions remain subordinate design input.
