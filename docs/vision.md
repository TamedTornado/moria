# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** for Rust consumers: a small family of tightly scoped crates that own world matter, generation, mutation, queries, and related physical/material behavior. It is an engine-layer product, not a game. A walkable-world executable may exist only as an adjacent validation harness; whether that harness is a current delivery is open (see Q1).

## Purpose

Give downstream games a shared foundation for large, natural-looking worlds that remain fully material and mutable—including deep underground—so adventure, fortress, sandbox, and similar titles can sit on one substrate without each reimplementing the world layer. The substrate must stand alone with no dependency on an LLM or “System” layer.

## Product boundary

**This product owns:** the reusable voxel-world substrate and its public integration surface for external Rust games and tools.

**Adjacent, not identity:** a walkable-world executable, if present, is only a validation consumer of that surface. It must use the same public interfaces available to an external game and must not own privileged or game-specific substrate paths.

**Downstream / out of repository:** the actual game (or games) that consume Moria.

**Not this product:** game rules; System/LLM features; spells; gas policy; combat; AI; building-as-gameplay layers; player controllers; cameras; authored demo content; presentation and UX chosen by a consumer or harness.

Compatibility seams may be designed where substrate requirements demand them; those excluded layers must not be implemented here.

## Required product outcomes

- **Reusable Rust substrate.** Downstream games and tools consume Moria as crate(s), not as a shipped title; game policy stays above the substrate, with no LLM/System requirement.
- **Natural look, material truth.** Surface worlds can read as continuous natural terrain while remaining fully backed by mutable voxel matter; render meshes are views, not authority for physics, queries, or mutation.
- **Mutation and query everywhere.** Matter can be destroyed, placed, and inspected through the public verb/query surface only—no private voxel write paths for consumers or harnesses.
- **Deep Z and geology-first generation.** Underground volume is first-class content; worlds generate as coherent geology so dig-down and cut faces are honest, with unvisited volume materializing on demand.
- **GPU-resident matter world.** The live world representation and its core substrate work are GPU-resident as part of the product promise to consumers.
- **Large-world practicality.** Sparse residency, streaming around activity, and persistence as generation plus edit deltas keep large regions workable without loading an entire raw grid.

## Future products and enabling implications

Described future **consumers** (not this product) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style deep-descent experience, and pure sandbox modes. Product One’s walkable demo is a consumer-shaped validation story, not a second product identity.

**Enabling implications** (substrate-level, not consumer features): deep mutable geology and dig/place; query/verb symmetry usable by agents and tools alike; matter-consistent surface dressing and interactable voxel-backed objects; room for richer material simulation (fluids, fire, granular settle, structural integrity) and placement/stamp flows so fortress- and adventure-class games can attach later without re-owning the world. Gameplay, content, controllers, presentation, and mode-specific policy remain consumer-owned.

## Non-goals

- Shipping the actual game, its rules, or its content pipeline inside this product
- Implementing System/LLM, spell, gas, combat, AI, or building gameplay layers here
- Treating harness- or demo-specific characters, routes, debug UX, or trailer content as substrate scope
- Making decorative non-material geometry the authority for collision, queries, or mutation

## Confirmed vision constraints

- **Ecosystem:** product is a Rust crate or small family of tightly scoped Rust crates.
- **Consumer boundary:** any in-repo validation executable, if built, consumes only public interfaces; privileged game-specific substrate paths are forbidden.
- **Scope exclusion:** game rules and future System, LLM, spell, gas, combat, AI, and building layers are out of scope for implementation here.
- **Independence:** the substrate must function with zero LLM dependency.

## Deferred design decisions

- Delivery depth and sequence of substrate capabilities (how far generation, meshing, matter simulation, objects, fluids, integrity, and related behavior go in each release)
- Precise crate split, internal layering, algorithms, data layouts, and performance budgets
- Voxel resolution, LOD, streaming-ring policy, and object-layer scaling choices
- Whether and how ambient simulation (weather, seasons, growth) appears in the substrate versus consumers
- Harness design (if delivered): platforms, scenes, controls, workloads, and acceptance gates

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery**, or only **permitted** as an adjacent artifact when useful?

- **Proposed answer:** Permitted only—it may exist as an adjacent validation harness and is not part of product identity or a settled mandatory delivery.
- **If answered differently:** A required harness stays outside product identity but becomes a scheduled current delivery that must exercise public interfaces; design must plan for its existence without importing any specific controller, character, content, route, presentation, or performance gate into the substrate product itself.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding boundary—substrate crates are the product; the real game is out of repo; harnesses (if any) share public interfaces; game/System/LLM/spell/gas/combat/AI/building layers stay out of scope.
- **`docs/seeds/voxel-world-substrate.md`:** Defines substrate purpose and outcomes—natural material worlds, full mutability, deep Z, geology-first generation, GPU-resident matter, layering with game policy above, and future multi-genre consumers.
- **`docs/seeds/product-one-seed.md`:** First-slice / demo consumer story (region, avatar, dig proof, targets); motivates validation needs and slice depth without expanding current product identity beyond the substrate.
