# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation: a natural-looking surface world over fully mutable voxel matter, with deep underground as first-class content, intended for external games to consume—not a game or playable title itself.

## Purpose

Moria exists so multiple games can share one honest material world: terrain that reads as ordinary landscape while remaining fully diggable and placeable matter all the way down. The substrate stands alone with no game-rules or LLM dependency, so future adventure, fortress, or sandbox titles can own their own rules above a common world foundation.

## Product boundary

**In product:** the reusable substrate and its public integration surface for generation, matter, queries, and mutation; any optional validation executable in this repository is a separate consumer of that surface.

**Out of product:** the actual game (a separate downstream product, not this repository); game rules; and System, LLM, spell, gas, combat, AI, and building layers. Compatibility seams may be designed where substrate needs demand them, but those layers are not implemented here.

**Adjacent, not identity:** a walkable-world executable, if present, is only a validation harness. It must use the same public interfaces available to an external game and must not own privileged or game-specific paths through the substrate. Harness controllers, characters, demo routes, authored seed content, presentation, and performance gates are consumer concerns, not the product definition.

## Future products and enabling implications

Downstream consumers include a System-backed ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style descent experience, and pure sandbox play. This vision does not commit their gameplay, content, UX, or policies.

High-level enabling implications supported by the seeds: games need a substrate that keeps voxel matter authoritative (with presentation as a non-authoritative view), supports geology-first world generation and deep continuous Z, and exposes mutation and query seams so dig, build, and simulation policy can live above the crate. Long-horizon matter behaviors that would unlock those games are enabling direction, not a committed current roadmap in this brief.

## Non-goals

- Shipping or owning the actual game, its rules, progression, or economy
- Implementing System/LLM, spells, gas policy, combat, AI agents, or building/gameplay layers in this product
- Treating harness-specific controls, demo content, cameras, or acceptance scenarios as substrate scope

## Confirmed vision constraints

- Adjacent consumers, including any validation harness, share only the public substrate interfaces; they get no privileged access paths.
- Game rules and the System, LLM, spell, gas, combat, AI, and building layers remain outside this product even when future games motivate substrate seams.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required current deliverable** of this repository, or only a **permitted** adjacent consumer?

- **Proposed answer:** Permitted and expected for proving the substrate, but not part of product identity; if the repo ships one, it remains an adjacent consumer of public APIs.
- **If different:** Making the harness mandatory means current delivery includes shipping that adjacent executable; making it optional means a substrate-only delivery can be complete without a walkable demo.

## Seed synthesis

- **README.md** — Named Moria as the reusable GPU-resident Rust-crate substrate and cast the walkable-world executable as a separate validation consumer, not a game layer; compatible harness detail stays subordinate to later design.
- **docs/seeds/project-boundary.md** — Locked current product identity to the substrate crate boundary, excluded the actual game and listed game layers, and required any harness to use public interfaces; crate-split mechanics remain downstream technical input.
- **docs/seeds/product-one-seed.md** — Motivated a first product-shaped proof of a material walkable world and listed demo content, controller, and acceptance detail as consumer/harness concerns; those specifics remain subordinate design input and do not redefine the product as a game.
- **docs/seeds/voxel-world-substrate.md** — Established the long-horizon substrate outcome (natural look over mutable voxel truth, deep Z, substrate-not-game reuse across future titles) and assigned matter/world foundation responsibility to this product; mechanism inventories and build order remain subordinate design input.
