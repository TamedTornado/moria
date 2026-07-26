# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation for external games—not a game, demo fantasy, or content product. A walkable-world validation harness is a required adjacent first delivery that consumes this substrate; it is not product identity.

## Purpose

Moria exists so multiple game styles can share one material world foundation: a natural-looking surface over fully mutable voxel truth, with first-class deep underground, without embedding game rules or any LLM dependency. The substrate provides matter, physics, queries, and mutation; game policy lives above it. First delivery proves that foundation via a public-interface harness and a reusable substrate slice.

## Product boundary

**This product owns** the reusable world substrate and its public consumer integration surface in Rust: geology-first generation, material world state, unified material behavior, construction-enabling semantics, mutation-safe navigation and continuous-3D traversal support, faithful world/object residency and persistence, and world presentation as a view of that matter (smooth voxel-derived terrain and structures; separate voxel-backed interactive objects; voxel-anchored non-voxel dressing). Physics, spatial queries, and collision operate against voxel truth; the mesh is a non-authoritative view.

**Adjacent or downstream (not this product’s identity):**

- The actual game is a separate consumer and is not part of this repository.
- Game rules and the System, LLM, spell, gas, combat, AI, and building *game* layers are out of scope here (compatibility seams may be designed where substrate needs demand them; those layers are not implemented in Moria).
- A walkable-world executable is a **required adjacent first delivery** and validation harness. It is not the product under design; it must consume the substrate only through the same public interfaces available to an external game.
- Harness- or game-owned work stays outside product identity: character controllers, cameras, authored demo routes and seed fixtures, UI, gameplay and labor policy, game presentation choices, device-specific limits, and numerical performance gates.

**Boundary rule:** In-repo consumers use the same public interfaces available to an external game—no privileged or game-specific substrate paths. Separately, higher layers integrate only through the substrate’s command/query/event boundary with consumer-supplied pricing policy.

## Required product outcomes

1. **Natural material world with correct representations.** Surface look is a view of material truth: terrain and structures use smooth voxel-derived presentation; interactive vegetation and objects are voxel-backed with their own representations; non-voxel clutter is voxel-anchored dressing responsive to the underlying world.
2. **Mutable everywhere with deep Z.** Any voxel matter can be destroyed, moved, or placed through substrate mutation surfaces, all the way down; underground volume (depth, strata, caves and voids) is first-class content, not a shallow floor under a heightmap skin.
3. **Voxel truth is authoritative for physics, queries, and collision.** The presentation mesh is a regenerated view only—never authoritative. Physics, spatial queries, and collision operate against voxel occupancy and world truth. The required harness proves collision against occupancy, not the render mesh.
4. **Unified material behavior and construction-enabling semantics.** World-matter behavior includes voxel-backed interactive objects, voxel-anchored responsive dressing, disturbed fluids, thin ambient fire/weather/time behavior, granular movement, and structural failure. Placement, stamps, structure descriptions, mechanism participation, and queryable spatial structure enable build-style consumers. First-slice depth does not narrow longer substrate responsibility; consumer gameplay, labor, UI, and content remain outside.
5. **Traversable, seed-reproducible, faithfully restorable worlds.** Mutation-safe navigation and continuous-3D traversal support; seed-reproducible geology with scalable lazy residency; load restores saved world and object changes exactly (generated truth plus edit deltas and object/entity journals).
6. **Reusable engine boundary and required first slice.** Matter, physics, queries, and mutation for external games; higher layers use a command/query/event boundary with consumer-supplied pricing; zero LLM dependency; consumers use only public interfaces. First delivery: full generation layer, specified partial matter layer, dig/place and mirror surfaces, plus adjacent walkable generated-region harness validating terrain generation, streaming, meshing, editing, collision, persistence, and performance. Controllers, cameras, routes, fixtures, hardware, and numerical gates remain harness-owned.

## Future products and enabling implications

**Future / external consumers** (not current Moria scope): a System-driven ARPG, fortress/colony-style games, Moria-style descent experiences, pure sandboxes, and later products on a walkable world. Their gameplay, content, controllers, characters, animation, and presentation remain consumer-owned. The Product One harness is a required adjacent first delivery that proves the substrate; it does not narrow substrate identity to that slice’s depth.

**Enabling implications** (depth and sequence are design, not a roadmap): optional higher intelligence or authoring clients on the same public surfaces; seams for multiplayer- or script-ready attachment without those systems being substrate features.

## Non-goals

- Shipping the actual game, its rules, or its content in this repository.
- Implementing System/LLM, spells, gas policy, combat, AI, or building game-layer systems here.
- Absorbing harness- or demo-owned specifics (controllers, cameras, routes, fixtures, device limits, numerical gates) into substrate identity.
- Making substrate correctness or operation depend on an LLM.
- Treating milestone depth or first-slice exclusions of the adjacent harness as a narrowing of longer substrate identity.

## Confirmed vision constraints

- **Form factor:** Rust crate or small family of tightly scoped Rust crates intended for game consumption.
- **GPU-resident substrate posture:** world matter the engine owns is designed to live GPU-resident as part of product identity.
- **Portable GPU backends in load-bearing layers:** load-bearing crate layers stay on wgpu/WGSL; no platform-native GPU fork (e.g. native Metal); cross-backend portability (including Vulkan/DX12) is the crate’s point. Device-specific limits and benchmark thresholds remain harness- or design-owned.
- **Mandatory public consumer boundary:** in-repo validation consumers do not receive privileged implementation paths versus external games.
- **Mandatory higher-layer integration boundary:** higher layers mutate or read world truth only through command/query/event surfaces; pricing policy is consumer-supplied.
- **Excluded game layers stay unimplemented** here even when seams are designed for them; **no LLM dependency** for the substrate to operate.

## Deferred design decisions

- Exact crate split, APIs, encodings, and enforcement layout for public and internal boundaries.
- Capability depth and build order inside the substrate beyond the required first-slice outline (storage, meshing, simulation tiers, resolution, streaming/persistence detail).
- How far multiplayer, scripting, or higher-layer seams are realized early.
- Adjacent harness acceptance thresholds, platforms, fixture protocol, controller/camera/route design, and numerical performance gates.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and identifies the walkable-world executable as a separate consumer/validation harness with stated validation domains rather than a game layer.
- **`docs/seeds/project-boundary.md`:** Locks current product identity to the substrate and Rust crate boundary; keeps the real game and higher game layers out of repo; establishes the harness as adjacent and bound to external public interfaces without canceling Product One’s delivery force.
- **`docs/seeds/product-one-seed.md`:** Pins the required first repository delivery—full generation layer, specified partial matter layer, dig/place and mirror surfaces, walkable generated-region harness, benchmarks, and exact save/load restore—plus portable wgpu/WGSL load-bearing layers and collision against voxel occupancy; harness controls, content, devices, and numerical gates stay adjacent.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes substrate purpose and outcomes—correct representations, universal mutability, deep Z, mesh-as-view with voxel-truth physics/queries, unified material behavior, construction-enabling semantics, mutation-safe continuous-3D traversal, seed-reproducible geology with lazy residency and generated-truth-plus-deltas restoration, and the command/query/event boundary with consumer pricing—leaving mechanisms and sequencing to design.
