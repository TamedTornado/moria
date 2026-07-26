# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for natural, fully material 3D worlds—not a game.

## Purpose

Games need a world that looks continuous and natural while remaining fully mutable voxel matter all the way underground, without baking game rules, LLM features, or a single title into the foundation. Moria exists so multiple downstream games can share the same generation, matter, mutation, query, streaming, and persistence capabilities through a clean public API, with zero LLM dependency in the substrate itself.

## Product boundary

- **In product:** the reusable substrate crates and the public interfaces they expose for world generation, matter representation, meshing as a non-authoritative view, editing, collision and queries against voxel truth, streaming, and persistence.
- **Out of product identity:** the actual game, game rules, and the System / LLM, spell, gas, combat, AI, and building game layers. Those are downstream consumers or later layers; compatibility seams may be designed where substrate needs require them, but those layers are not implemented here.
- **Adjacent, not identity:** a walkable-world executable may exist in this repository as a validation harness. If present, it must consume the substrate only through the same public interfaces available to an external game—no privileged or game-specific paths. Whether that harness is a required current delivery remains open (see Q1). Its controller, character, content, presentation, routes, workloads, and acceptance gates are not product scope.
- **Consumer-owned:** gameplay, UX, controllers, authored content, presentation, and game-specific policy stay with adjacent products unless a later explicit boundary says otherwise.

## Required product outcomes

- **Reusable Rust integration:** external games (and any harness) integrate as ordinary consumers of the public crate API; nothing above the substrate touches voxels outside verbs and queries.
- **Natural look, voxel truth:** the world reads as continuous natural terrain (hills, forest, water, cliffs, underground) while all interaction and simulation authority remains voxel matter; the extracted mesh is a regenerated view, never the source of truth.
- **Mutable everywhere:** matter can be destroyed, placed, and edited through the public API so cuts and scars are real geology, not decorative geometry.
- **Deep Z as first-class space:** underground volumes (caves, strata, buried features) are playable continuous 3D space, not a shallow floor under a heightmap.
- **Geology-first generation:** worlds are produced as seed-driven geology (columns, strata, caves, materials) with lazy materialization so large regions stay tractable until touched.
- **Streaming, sparsity, and persistence:** large regions stream and idle cheaply via sparse residency; durable truth is generation plus edit history so changed worlds can be saved and restored.

## Future products and enabling implications

Downstream consumers—not current product—include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox titles. The substrate enables them by supplying shared matter, mutation, queries, generation, deep-Z worlds, and a policy-free API surface. Their gameplay, content, controllers, characters, presentation, and economic or combat rules are out of scope here. Long-horizon matter behaviors that would further those games (extended fluid, fire, integrity, building machinery, ambient weather) are enabling implications for design depth, not a committed current roadmap catalog in this brief.

## Non-goals

- Shipping a complete game or game rules in this repository.
- Implementing System / LLM, spells, gas policy, combat, AI, or building game layers here.
- Treating harness-owned demo content (seed postcard route, third-person character, debug presentation, scripted acceptance scenes) as substrate product scope.
- Embedding LLM dependence into the world substrate.

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates intended for Rust-game consumption.
- The substrate is GPU-resident and must stand alone with zero LLM dependency.
- Any in-repo validation executable, if delivered, is a peer consumer of public interfaces only—no privileged substrate paths.
- Game-rule and listed future game layers stay out of implementation in this product even when seams are prepared for them.

## Deferred design decisions

- Exact crate split, internal layering, algorithms, data layouts, and meshing strategy.
- Capability depth and delivery sequence for generation, matter simulation tiers, vegetation/objects, fluids, integrity, and ambient systems.
- Voxel resolution, LOD, streaming-ring policy, and persistence encoding details.
- Whether and how a walkable validation harness is shaped (content, controls, platforms, performance gates)—after Q1 settles delivery status.
- Open substrate engineering tradeoffs (e.g. fidelity vs cost) left for measurement-driven design.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required current delivery** of this repository, or only **permitted** as an adjacent artifact?

- **Proposed safe answer:** Permitted and valuable, but not required to define product completeness; the product is complete as consumable substrate crates with public APIs. If a harness is built, it remains an adjacent consumer under the public-interface rule.
- **If answered differently:** Making the harness mandatory adds an adjacent delivery obligation (still outside product identity) that design must plan and validate; it does not move controller, content, or presentation into the substrate, but it does change what “done” means for the repository’s current program of work.

## Seed synthesis

- **README.md:** States current product identity as the GPU-resident Rust voxel-world substrate and separates the walkable executable as consumer/harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md:** Binding boundary correction—product is the reusable Rust substrate; game is out of repo; harness may exist only via public APIs; game-rule and listed future layers are out of scope here.
- **docs/seeds/product-one-seed.md:** Motivates a first walkable proof and dig/place honesty for the substrate, and details an adjacent demo consumer (region, character, presentation, targets); those consumer specifics do not expand product identity.
- **docs/seeds/voxel-world-substrate.md:** Authorizes substrate purpose and outcome families (natural look, full mutability, deep Z, reusable layering, generation, matter, streaming/persistence) and names future game consumers without transferring their rules into this product.
