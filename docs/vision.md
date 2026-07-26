# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer library for natural-looking, fully material voxel worlds—not a game and not this repository’s future game content.

A walkable-world executable may exist only as an adjacent validation harness (see Q1). It is not the product identity.

## Purpose

Moria exists so multiple games—and validation consumers—can sit on one shared world foundation: geology-backed terrain, continuous deep underground space, mutable matter, and consumer-facing queries and mutation, without embedding game rules, economy policy, or LLM behavior in the substrate.

The product promise is substrate reuse: the same crate stack can support adventure, fortress/colony, descent, or pure sandbox experiences while remaining independently useful with zero LLM dependency.

## Product boundary

**In product**

- The reusable voxel-world substrate and its public integration surface as Rust crates.
- World, matter, generation, mutation, query, meshing/view, streaming, and persistence responsibilities that make that substrate consumable.
- Compatibility seams the substrate itself needs so higher layers can attach later—without implementing those layers here.

**Adjacent / not this product**

- The actual game and all game-rule layers (System/LLM, spells, gas policy, combat, AI, building-as-gameplay).
- Any walkable-world executable: a separate consumer that exercises the substrate through the **same public interfaces** available to an external game. It must not own privileged or game-specific implementation paths inside the substrate.
- Controllers, cameras, characters, authored demo routes, presentation polish, content packs, and acceptance scenarios for demos or harnesses—consumer-owned unless later design places them outside this brief’s product identity.

**Repository boundary**

- This repository is the substrate (and, if present, an adjacent harness that only uses public APIs). The game lives downstream.

## Required product outcomes

A competent design must make these consumer-visible guarantees true:

1. **Natural material world** — Downstream consumers get a world that *reads* as ordinary outdoor terrain (rolling ground, water bodies, vegetation presence, cliffs, caves) while remaining a fully material voxel truth underneath; render geometry is a derived view, not the authority for interaction.
2. **Mutable everywhere, deep Z included** — Matter can be destroyed, placed, and inspected throughout the volume, including deep underground. Continuous 3D play space is first-class, not a decorative floor under a heightfield.
3. **Geology-first generation consumers can trust** — Worlds are produced so digging and descent reveal coherent subsurface structure (strata, voids, resources, water-bearing bands), materializing cost only where the world is touched.
4. **Matter operations through a public surface** — Consumers mutate and query the world only through the substrate’s public verbs and queries. Adjacent consumers, including any harness, have no privileged path around that boundary.
5. **Living substrate services for reuse** — The product provides the engine-level world services games need to share: surface dressing and voxel-backed interactable objects, fluid presence beyond empty air, structural honesty of solid matter, streaming of active regions, and persistence as generation plus edit history—so games author policy and content above, not reimplement the world core.
6. **Standalone GPU-resident library** — The substrate runs as a GPU-resident Rust library stack with no LLM or game-policy dependency required for its core operation.

## Future products and enabling implications

Future **consumers** (not current product scope) include a System-driven ARPG, a fortress/colony-style builder, a Moria-style descent experience, and pure sandbox modes. They own gameplay, UX, content, economy, AI, and presentation.

**Enabling implications** (substrate responsibilities that make those consumers possible, not a committed game roadmap): deep mutable geology; public matter mutation and queries; streaming and scar-friendly persistence; and seams for higher-layer rules without baking those rules in. Game-specific systems remain excluded even when they motivate the substrate.

## Non-goals

- Shipping the actual game, its rules, or its content in this product.
- Implementing System/LLM behavior, spells, gas/economy policy, combat, agent AI, or building-as-gameplay layers here.
- Treating a demo character, third-person controls, authored postcard route, or marketing clip as substrate requirements.
- Giving any in-repo harness privileged access to voxel truth.

## Confirmed vision constraints

- **Form:** Rust crate or small family of tightly scoped Rust crates.
- **Consumer isolation:** external games and any in-repo harness use the same public interfaces; privileged game-specific paths are forbidden.
- **No game layers in-repo:** game rules and the listed future gameplay layers are out of scope; seams may be designed, not implemented as those layers.
- **Standalone:** core substrate operation does not depend on an LLM or a particular game policy.
- **GPU-resident world substrate:** product identity includes GPU-resident operation of the world/matter foundation.

## Deferred design decisions

- Precise crate split and workspace layout (boundary intent is fixed; packaging is design).
- Delivery depth and sequence of generation, matter, fluid, integrity, object, and persistence capabilities.
- Meshing strategy, storage layout, voxel scale, LOD, and sim algorithms.
- Whether and how a validation harness is built, and any harness-only content, controls, platforms, or performance gates.
- Multiplayer, platform backends, and concrete API surface shape.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **mandatory current delivery** of this effort, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted only—the product is the substrate; a harness may be added later or in parallel but is not required to claim the product is delivered.
- **If answered differently:** Making the harness mandatory keeps product identity as the substrate but adds a required adjacent deliverable that must still consume only public APIs; it does not move controllers, demo content, or performance gates into substrate scope.

## Seed synthesis

- **`README.md`** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and separates the walkable-world executable as a non-game validation consumer.
- **`docs/seeds/project-boundary.md`** — Binding identity and boundary: substrate-as-crates, game out of repository, harness-only public-API consumption, and exclusion of game-rule layers.
- **`docs/seeds/product-one-seed.md`** — First-slice / demo motivation for a walkable proof of material terrain; informs why dig/place and natural terrain matter, without defining product identity or importing harness-owned controls, content, or gates.
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes the substrate’s outcome families (natural look over voxel truth, full mutability, deep Z, geology-backed generation, matter services, public layering, multi-game reuse, LLM-free stand-alone operation) at vision altitude, not as a mechanism inventory.
