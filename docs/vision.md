# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or small family of tightly scoped Rust crates. It provides a natural-looking, fully material world layer—mutable everywhere, continuous in three dimensions including deep underground—so external games can consume matter, world generation foundations, queries, and mutation without owning the world engine.

## Purpose

Games that dig, build, flood, collapse, and explore continuous terrain need a shared foundation where the voxel grid is the truth of the world and the visible surface still reads as ordinary landscape—not a heightmap with props. Moria exists so that foundation is an engine product with a hard consumer boundary: one substrate, many games, zero dependency on any particular game rules layer or LLM “System.”

## Product boundary

**In product:** the reusable world substrate and its public interfaces; optional workspace separation so a validation executable (if present) is only a consumer of those interfaces.

**Out of product / adjacent:** the actual game(s); any walkable-world character, camera, authored demo route, or presentation chosen only to exercise the substrate; game rules and future System, LLM, spell, gas, combat, AI, and building *game* layers.

**Ownership rule:** gameplay, UX, controllers, authored content, presentation policy, and game-specific pricing or designation policy remain consumer-owned. Compatibility seams may be designed where substrate requirements demand them; those upper layers are not implemented here.

## Future products and enabling implications

Described future consumers include an ARPG that uses a System/LLM layer, a fortress/colony-style builder, a Moria-style descent adventure, and pure sandbox modes. They motivate a substrate that can stand alone for natural surface worlds, deep-Z geology and voids, and honest dig/build matter behavior.

**Enabling implications (not a committed feature roadmap):** consumers should eventually be able to rely on this crate stack for GPU-resident mutable worlds that look natural, stream and persist as world truth plus edits, and expose matter mutation and queries without privileged game-only paths. Product-one’s “walkable world” is an adjacent validation/demo consumer of the substrate, not a second product identity in this repository.

## Non-goals

- Shipping a game, combat loop, AI, economy, or LLM/System runtime in this repository.
- Treating the validation harness’s character, controls, seed postcard, or performance gates as the product definition.
- Implementing game-layer building, spell, gas, or policy systems here (even when future consumers need them).

## Confirmed vision constraints

- Substrate is GPU-resident and intended for consumption as Rust crate(s); workspace boundary between substrate and any harness is required at the product level.
- Any included walkable executable is a validation harness only and must use the same public interfaces available to an external game.
- Substrate must stand alone with no LLM/System dependency; game rules and listed game layers stay out of this product.

## Assumptions proposed for approval

1. **Current identity is the substrate, not “the walkable demo.”** Product-one describes a first validation shape and proof points; it does not redefine Moria as a third-person game product.
2. **Long-horizon matter/world behaviors** named across the substrate seed (rich fluids, integrity, ambient ecology, multi-game semantic toys) are enabling implications for future consumers, not automatic commitments of the current delivery slice.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** of this repository, or only **permitted** as an adjacent consumer?

- **Proposed safe answer:** Permitted (and strongly useful), not mandatory for product identity; the crates and public substrate boundary define “done” for Moria itself.
- **If different:** Making the harness mandatory keeps identity on the substrate but expands repository delivery to always include a runnable consumer; importing its controller, content, or acceptance scenario as product scope would turn Moria into a demo-game product and contradict the stated boundary.

## Seed synthesis

- **`README.md`:** Named the product Moria; fixed identity as reusable GPU-resident voxel-world substrate consumed as a Rust crate; cast the walkable executable as harness, not game layer.
- **`docs/seeds/project-boundary.md`:** Binding boundary—substrate is the product; real game is out of repo; harness may exist only via public APIs; Cargo workspace consumer split required; System/LLM/spell/gas/combat/AI/building layers out of scope.
- **`docs/seeds/product-one-seed.md`:** First consumer-shaped proof (natural generated region, mutability proof, no game systems); clarified non-goals for an early slice; treated dig/place as proof of material world; supplied demo/harness detail held out of vision scope pending Q1.
- **`docs/seeds/voxel-world-substrate.md`:** Outcome goals—normal-looking surface, mutable everywhere, deep Z, substrate-not-game, multi-game reuse, GPU-resident, zero LLM dependency; future consumer fantasies and detailed mechanisms treated as design/enabling context, not current inventory.
