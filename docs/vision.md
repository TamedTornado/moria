# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer library that owns world matter, generation hooks, queries, mutation, and related world services so external games can build on a shared material world without embedding game rules in the substrate.

A walkable-world executable may exist in this repository as an adjacent validation harness. Whether that harness is a required repository delivery is unresolved (see Q1). It is not the product identity and is not a game layer.

## Purpose

Games that need a diggable, buildable, deep, natural-looking world should not each reimplement voxel truth, streaming, meshing, editing, collision, and persistence. Moria exists so multiple downstream games—and a harness that proves the claim—can share one standalone matter substrate whose world remains fully material under a normal-looking surface, with deep Z as real play space, and with zero dependency on an LLM or game System.

## Product boundary

**In product:** the reusable substrate and its public consumer-facing interfaces; world generation and materialization services the substrate exposes; matter representation and mutation; view generation from voxel truth; world queries; collision against voxel truth; streaming and persistence of world state; and any seams the substrate needs so game layers can attach later without being implemented here.

**Adjacent, not product identity:** a walkable-world validation harness (character, camera, demo route, seed-world presentation, debug controls, harness workloads and gates). If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific implementation paths.

**Downstream / out of this repository:** the actual game; game rules; System/LLM, spell, gas, combat, AI, and building *game* layers; authored gameplay content, UX, controllers, and presentation policy belonging to a title.

## Required product outcomes

- **Reusable Rust substrate.** External and in-repo consumers integrate through public crate interfaces only; no privileged harness path is part of the product contract.
- **Material world truth.** The world is fully mutable voxel matter end to end—not a heightmap with non-material props. Surface presentation may look continuous and natural; the mesh (or other view) is derived, not authoritative.
- **Natural surface over deep geology.** Generated regions read as ordinary outdoor terrain while underground depth (caves, strata, diggable volume) is first-class content space, not a decorative floor.
- **Engine services for matter.** The substrate provides the world services games need at the matter layer: representation, generation/materialization, queries, mutation (including dig and place), collision against voxel truth, meshing/view update after edits, streaming, and persistence—without embedding game rules, pricing, or LLM control.
- **Standalone engine.** The substrate runs with no LLM/System dependency; future System or game logic is a client of the same interfaces, not a substrate feature.
- **Harness-exercisable public boundary.** Terrain generation, streaming, meshing, editing, collision, persistence, and performance must be exercisable through public product interfaces so an adjacent harness (if delivered) can validate them without becoming the product.

## Future products and enabling implications

Future **consumers**, not current product: a System-driven ARPG, a fortress/colony game, a Moria-style descent experience, pure sandbox titles, and any other game that injects its own rules, content, and presentation above the substrate.

Enabling implications already owned at substrate altitude (delivery depth is design): a fully material mutable world; deep-Z geology; matter-facing queries and mutations; derived views; streaming and edit-aware persistence—so those consumers can add gameplay without re-owning world truth. Gameplay systems, agents, economy, spells, gas policy, combat, AI, and building *gameplay* remain consumer-owned.

A first “walkable world” proof may exist as an adjacent harness demonstrating that the world is real matter under a good-looking surface; its controller, route, and demo content are not substrate features.

## Non-goals

- Implementing the shipped game or any game-rules layer in this repository.
- System/LLM features, spells, gas/pricing policy, combat, AI, or building-game systems (blueprints-as-gameplay, work orders, mechanism gameplay, etc.).
- Treating the validation harness’s character, UI, demo content, or acceptance theater as product scope.
- Making the substrate depend on an LLM or game System to function.

## Confirmed vision constraints

- **Ecosystem:** product surface is a Rust crate or small family of Rust crates for game consumers.
- **Residency model:** the voxel-world substrate is GPU-resident as part of product identity.
- **Consumer isolation:** any in-repo harness or external game uses the same public interfaces; privileged internal paths for “the demo” are forbidden.
- **No LLM dependency:** the substrate stands alone as an engine layer.
- **Layering:** compatibility seams may be designed where substrate requirements demand them; excluded game layers must not be implemented here.

## Deferred design decisions

- Crate split, APIs, storage layout, meshing and simulation techniques, and how generation, streaming, and persistence are structured.
- Capability delivery order and depth (which matter services land in which slice).
- Whether and how an adjacent harness is scoped, staged, or measured (workloads, platforms, content, performance gates).
- Open technical tradeoffs left by the seeds (e.g. resolution, LOD, object-scale limits, fluid fidelity, multiplayer readiness) that do not change product identity.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required repository delivery** alongside the substrate crates, or only **permitted** as an adjacent validation artifact?

- **Proposed safe answer:** Permitted (and useful) as an adjacent harness that consumes public APIs; not part of product identity. Repository “done” for the product is defined by the substrate’s public outcomes, not by a shippable third-person demo.
- **If answered differently:** Making the harness mandatory adds a required adjacent deliverable (still not product identity) and ties repo acceptance to a walkable proof; keeping it merely permitted allows substrate-only delivery without a playable executable.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident Rust substrate and positions the walkable-world executable as a separate consumer/harness for generation, streaming, meshing, editing, collision, persistence, and performance.
- **`docs/seeds/project-boundary.md`:** Binds product identity to the reusable Rust substrate, keeps the real game out of repo, allows a public-API-only harness, and excludes game/System/LLM/spell/gas/combat/AI/building layers from implementation here.
- **`docs/seeds/product-one-seed.md`:** Describes a first walkable proof and harness-shaped demo (controller, seed region, debug dig/place, targets); used only to clarify adjacent validation and first-slice motivation, not to redefine the product as that demo.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies substrate purpose and outcome-level responsibilities (natural surface, full mutability, deep Z, matter/physics/queries/mutation, standalone engine) and names future game consumers without pulling their gameplay into current scope.
