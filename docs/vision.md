# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is the world foundation external games consume—not a game, demo, or presentation layer.

## Purpose

Provide a shared material-world layer where terrain, geology, and matter are one mutable truth, so downstream games can stand on a natural-looking, fully diggable, deep-Z world without each reimplementing that foundation. The substrate stands alone with no game-rules or LLM dependency.

## Product boundary

**In product:** the reusable substrate and its public integration surface—the outcomes that let consumers obtain a continuous, natural-looking, fully material, deep-Z world and mutate it through public interfaces only.

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness. It must use the same public interfaces available to an external game and must not own privileged or game-specific paths. Its controller, character, camera, content, route, presentation, and performance gates are harness concerns, not product scope.

**Out of this repository:** the actual game and game-owned layers—rules, System/LLM, spells, gas policy, combat, AI, and building/gameplay. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Future products and enabling implications

Described future consumers include a System-driven ARPG, a fortress/colony-style game, a descent-style adventure, and pure sandbox tools. Enabling implications at vision altitude: a geology-first mutable deep world and matter-level interaction reusable across those genres. Gameplay, UX, controllers, authored content, and presentation remain consumer-owned.

## Non-goals

- Shipping a game, character fantasy, or audience-facing gameplay product as Moria itself
- Implementing System/LLM, spell, gas, combat, AI, or building/game layers in this repository
- Treating harness-specific content, controls, or acceptance scenarios as substrate requirements

## Confirmed vision constraints

- Adjacent consumers, including any validation harness, have no privileged access: they use only public substrate interfaces.
- Game rules and the System, LLM, spell, gas, combat, AI, and building layers remain outside this product.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a mandatory current delivery alongside the substrate, or only a permitted adjacent artifact?

*Proposed answer:* Permitted and useful for proving the substrate, but not part of product identity; only the substrate is required current product.

*If different:* Making the harness mandatory expands delivery commitment beyond the crate-bound substrate without changing what Moria *is*; forbidding it would drop the seeds’ stated validation path.

## Seed synthesis

- **README.md** — Named Moria as the reusable GPU-resident voxel-world Rust substrate and cast the walkable-world executable as a separate consumer/validation harness; compatible detail remains subordinate design input.
- **docs/seeds/project-boundary.md** — Fixed current identity on the substrate crate(s), excluded the actual game and game-owned layers from this repository, and required any harness to share public interfaces only; crate-split mechanics stay downstream.
- **docs/seeds/product-one-seed.md** — Motivated first-slice outcomes (natural region, mutability proof, walkable validation) without transferring harness demo content into product identity; its specs remain subordinate design input.
- **docs/seeds/voxel-world-substrate.md** — Supplied long-horizon substrate goals (natural look, full mutability, deep Z, multi-game reuse, GPU-resident foundation) and future-consumer relationships without moving game layers into current scope; mechanism inventory remains subordinate design input.
