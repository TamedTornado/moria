# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is the engine-layer foundation for natural-looking, fully material worlds that downstream games can generate, query, mutate, and simulate against—not a game, demo title, or presentation layer.

## Purpose

Moria exists so multiple future games can share one trustworthy matter world: terrain that reads as a normal landscape, remains voxel truth all the way down, and supports deep underground play and free dig/build mutation without each game re-owning world representation. The substrate must stand alone with no LLM or game-rules dependency.

## Product boundary

**In product:** the reusable substrate—world generation as geology, GPU-resident matter, public query and mutation surfaces, and the physics/matter outcomes that make a material world honest for any consumer. Intended integration is as Rust library crates.

**Out of product / adjacent:** the actual game lives in a separate repository and is a downstream consumer. A walkable-world executable, if present, is only a validation harness and must use the same public interfaces as an external game—never privileged or game-specific paths. Gameplay, UX, controllers, cameras, characters, authored demo content and routes, presentation, and game-specific policy belong to consumers or the harness, not to Moria’s product identity.

**Excluded layers:** game rules and the System, LLM, spell, gas, combat, AI, and building *game* layers are not implemented here. Compatibility seams may exist where substrate requirements demand them; those layers remain consumer-owned.

## Future products and enabling implications

Future consumers include a System/ARPG, a fortress/colony-style game, a descent-style adventure, and pure sandbox modes. Moria enables them by providing shared matter, mutation, queries, and related world-substrate outcomes. Their gameplay, content, controllers, presentation, and policies are not current scope. Long-horizon substrate depth motivated by those games is an enabling implication, not a committed current roadmap.

## Non-goals

- Shipping a playable game, ARPG, fortress mode, or narrative experience in this repository
- Owning controllers, characters, cameras, combat, AI, economy, or building UX/policy
- Making the System/LLM, spells, or gas metering part of the substrate product
- Treating a validation harness’s demo content, workload, or performance gates as the product itself

## Confirmed vision constraints

- Adjacent consumers (including any validation harness) have no privileged access: they use the same public interfaces as an external game.
- The product is exposed for Rust crate consumption; the real game is outside this repository.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** beside the substrate, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted and encouraged for proving public interfaces, but not mandatory for the substrate product to be considered complete; product identity stays the crate substrate.
- **If different:** Making the harness mandatory keeps identity on the substrate but expands *current delivery* to include an adjacent executable that must ship and stay interface-honest; it still does not import demo controls, content, or acceptance details into product scope.

## Seed synthesis

- **`README.md`:** Fixed the product name and identity as a reusable GPU-resident voxel-world substrate for Rust crate consumption, with the walkable-world executable as a separate validation consumer—not a game layer; compatible detail remains subordinate.
- **`docs/seeds/project-boundary.md`:** Settled repository and consumer boundary (substrate in-repo; game out; harness non-privileged; game/System/building layers excluded); crate-split and seam mechanics stay downstream.
- **`docs/seeds/product-one-seed.md`:** Motivated first-consumer proof of a material walkable world and dig/place honesty, but its controller, seed content, presentation, milestones, and performance gates are harness/consumer material—not current product identity; compatible substrate asks remain subordinate design input.
- **`docs/seeds/voxel-world-substrate.md`:** Supplied substrate purpose (normal look, full mutability, deep Z, reusable matter/physics/query/mutation foundation) and future-consumer relationships; mechanism inventories and open technical choices remain subordinate design input.
