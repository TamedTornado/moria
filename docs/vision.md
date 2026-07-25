# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is the engine-layer foundation for material worlds: mutable voxel truth under a natural-looking surface, deep underground as first-class space, and public interfaces so external games can query and change that world. It is not a game and not the walkable demo itself.

## Purpose

Games that need a fully material world—not a heightmap with props—need a shared substrate that owns geology-backed generation, matter, mutation, and views derived from voxel truth. Moria exists so those games can share one stable world layer without embedding game rules, AI, or presentation policy in the engine.

## Product boundary

**In product:** the reusable substrate and its public consumer surface—world generation for a continuous material world, matter representation and mutation (including dig/place-class change), derived non-authoritative views of voxel truth, and the streaming/persistence of world truth that keeps a large mutable region viable. High-level matter and world responsibilities that seeds assign to the substrate stay here even when future games motivate them.

**Out of product:** the actual game (a separate downstream consumer, not this product); game rules; System/LLM, spell, gas, combat, AI, and building layers. Controllers, characters, cameras, authored demo content and routes, presentation polish, and acceptance workloads belong to adjacent harnesses or downstream games. Compatibility seams may be designed where substrate needs demand them; those layers are not implemented here.

**Adjacent validation:** a walkable-world executable may sit beside the crate stack as a consumer under test. It must use the same public interfaces available to an external game. Its controls, content, and gates are not product identity (see Q1).

## Future products and enabling implications

Described future consumers include a System-backed ARPG, a fortress/colony-style game, a Moria-style descent experience, and pure sandbox modes. They own gameplay, UX, content, characters, and policy. Enabling implications supported at vision altitude: a world that reads as natural while remaining fully mutable matter; deep-Z geology and underground play space; substrate-owned mutation and query foundations that later games can price and present differently. Long-horizon matter capabilities (further simulation, integrity, richer fluids, building-oriented verbs) are enabling implications for those consumers, not a committed current roadmap in this brief.

## Non-goals

- Shipping an actual game, ARPG, fortress mode, or System/LLM integration as this product.
- Implementing combat, stats, AI, spells, gas economy, building UI, blueprints, or mechanism policy here.
- Defining the product by harness character controls, demo routes, content palettes, or performance acceptance scenes.

## Confirmed vision constraints

- Integration form is the Rust crate ecosystem: external games consume the substrate as crate dependents.
- Any in-repo validation executable uses only public substrate interfaces—no privileged or game-specific access paths.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **mandatory current delivery** beside the substrate, or only **permitted** as an adjacent harness?
*Proposed safe answer:* Permitted (and useful to prove the substrate), but not required for product completeness; if shipped, it remains harness-only and outside product identity.
*How another answer changes the brief:* If mandatory, an adjacent walkable harness becomes required current delivery without absorbing its controls, content, presentation, or acceptance detail into the substrate product; if permitted only, crate-only substrate delivery can be complete without shipping that executable.

## Seed synthesis

- **README.md** named Moria and fixed identity as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, casting the walkable-world executable as a separate validation consumer rather than a game layer; listed harness concerns stay subordinate design input.
- **docs/seeds/project-boundary.md** bound the product to the substrate crate family, excluded the actual game and named game layers from this product, permitted a non-privileged validation harness, and made the consumer boundary non-optional; workspace or crate-split mechanics remain downstream.
- **docs/seeds/product-one-seed.md** supplied the product-shaped claim that the world is fully material and mutable and motivated an early walkable proof; region content, controller, debug presentation, performance gates, and milestones are subordinate harness/design input and do not redefine product identity.
- **docs/seeds/voxel-world-substrate.md** contributed long-horizon substrate purpose—natural look over voxel truth, everywhere-mutable deep-Z world, substrate-not-game layering for multiple future games—and high-level world/matter responsibilities; algorithms, storage layouts, layer inventories, and open technical questions remain subordinate design input.
