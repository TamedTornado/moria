# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate (or a small family of tightly scoped Rust crates). It gives external games a material world that reads as continuous natural terrain while remaining fully mutable voxel truth—including deep underground space—through public generation, mutation, and query surfaces with no privileged consumer paths.

## Purpose

Moria exists so multiple games can share one honest material-world foundation instead of each rebuilding terrain, mutability, and underground space. The substrate must stand alone: zero LLM dependency and no game-rule layer. Proof that the world is matter under a smooth view—not a heightmap with props—may be shown by an adjacent walkable harness, but that harness is not the product identity.

## Product boundary

**In product:** the substrate library and its integration contract for external Rust consumers—world matter, geology-shaped generation, non-authoritative visual views of voxel truth, and public mutation and query surfaces.

**Out of product:** any actual game; game rules; System/LLM, spells, gas, combat, AI, and building-policy layers. A walkable-world executable, if present, is only a validation harness and adjacent consumer: it must use the same public interfaces available to an external game. Character control, camera, demo routes, authored content, presentation, and harness-specific acceptance scenarios remain consumer-owned.

## Future products and enabling implications

Future consumers include a System-driven ARPG, a fortress/colony-style game, and a Moria-style descent experience. They motivate a reusable, deep-Z, game-policy-free substrate. Their gameplay, content, controllers, characters, and presentation are not current scope. Broader matter-simulation depth that would serve those games is an enabling implication, not a committed roadmap here.

## Non-goals

- Shipping a playable game, game rules, or game-mode content in this repository
- Implementing System/LLM, spell, gas, combat, AI, or building-policy layers here
- Defining product identity by a harness character, camera, demo route, content set, or performance gate

## Confirmed vision constraints

- Adjacent consumers, including any validation harness, have no privileged or game-specific substrate paths—only public interfaces.
- The substrate has zero LLM dependency and does not implement game-rule layers (System, spells, gas, combat, AI, building policy).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a required current delivery alongside the substrate crate, or only a permitted adjacent artifact?

*Proposed safe answer:* Permitted and useful for proving the substrate, but not mandatory for the substrate product to be complete; if delivered, it stays outside product identity.

*Why it matters:* Mandatory delivery expands current outcomes to require an adjacent executable; permitted-only keeps “done” on the reusable crate and public contract.

## Seed synthesis

- **README.md** — Named Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate and cast the walkable-world executable as a separate validation consumer, not a game layer; further harness detail is subordinate design input.
- **docs/seeds/project-boundary.md** — Fixed current identity to the substrate crate family, excluded the actual game and game-rule layers from this repository, and required any harness to use public interfaces only; enforcement layout remains downstream.
- **docs/seeds/product-one-seed.md** — Motivated a product-shaped walkable proof of material mutability and continuous terrain, while character, route, content, and acceptance specifics describe harness/consumer work rather than substrate identity; slice and target detail remain design input.
- **docs/seeds/voxel-world-substrate.md** — Established the high-level substrate outcome (natural-looking world, full mutability, deep Z, GPU-resident, reusable engine layer for multiple games) without moving future game systems into current scope; mechanisms and inventories remain subordinate design material.
