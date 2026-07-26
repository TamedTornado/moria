# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for material voxel worlds—not a game, not a playable product identity, and not an LLM-dependent system.

## Purpose

Moria exists so multiple games and tools can share one material world layer: generated natural terrain and deep volume that is fully editable matter, presented as continuous landscape, streamed and persisted at scale, and queried and collided against as truth—while game rules, presentation policy, controllers, and authored gameplay remain owned by each consumer.

## Product boundary

- **In product:** the reusable substrate—generation of material voxel volumes, non-authoritative meshing/view of that truth, mutation (edit), queries and collision against voxels, streaming, and persistence—exposed only through public consumer interfaces.
- **Adjacent, not identity:** a walkable-world executable, if delivered, is only a validation harness that exercises those public interfaces (terrain generation, streaming, meshing, editing, collision, persistence, performance). Its character, controller, camera, demo content, route, and performance gates are not the product.
- **Out of product:** the actual game(s); game rules; System/LLM; spells; gas; combat; AI; and building-as-gameplay layers. Compatibility seams may be designed where the substrate requires them; those layers are not implemented here.
- **Consumer-owned:** gameplay, UX, controllers, authored content, presentation policy, and game-specific rules or pricing.

## Required product outcomes

1. **Rust-integrable substrate.** External and in-repo consumers depend on Moria through ordinary public crate interfaces, with no privileged internal path reserved for a first-party game.
2. **Material world truth.** Supported space is a fully mutable voxel volume (dig and place are first-class), not a heightmap with non-matter props standing in for terrain.
3. **Natural presentation of truth.** Generated surface and underground worlds can read as ordinary continuous terrain while simulation, collision, and queries remain on voxels; the mesh is a regenerated view, never authority or save truth.
4. **Deep volume as first-class space.** Underground and vertical extent are real material space consumers can enter and alter, not a decorative floor or skybox.
5. **Generate, stream, and persist.** Large regions can materialize and stream without keeping the full raw volume always resident, and durable edit state can be saved and restored.
6. **Standalone engine layer.** The substrate has zero dependency on LLM or System features and stays reusable across sandbox, adventure, fortress-style, and other consumers.

## Future products and enabling implications

- **Downstream games** (Moria-style descent, fortress/colony, System-driven ARPG, pure sandbox) are separate future consumers. They motivate reuse and seams; they do not pull gameplay, content, or controls into current scope.
- **Longer-horizon substrate growth** (richer matter simulation such as flowing fluids, fire/CA, granular settle, structural integrity; dynamic voxel objects; broader building/mechanism APIs; semantic structure and nav helpers) may later enable those games. They are enabling implications or later design—not present product identity or a committed roadmap merely because a broad design seed describes them.
- **Walkable-world harness** remains an adjacent proof vehicle for substrate claims, never a second product identity or game layer.

## Non-goals

- Shipping a game, combat loop, AI cast, or economy in this repository
- Implementing System/LLM, spells, gas pricing, or building-game UX and policy here
- Absorbing harness character, demo content, presentation, or benchmark scenarios into the product promise
- Requiring or embedding an LLM in the substrate

## Confirmed vision constraints

- Delivered as a Rust crate or small family of tightly scoped Rust crates
- GPU-resident voxel-world substrate
- Any validation harness must consume the same public interfaces available to an external game
- Game rules and System/LLM/spell/gas/combat/AI/building layers are not implemented in this product
- Substrate stands alone with zero LLM dependency

## Deferred design decisions

- Precise crate and workspace packaging (the consumer boundary is fixed; the split is design)
- Capability depth and delivery sequence inside the substrate (initial proof slice versus later matter systems)
- Algorithms, data layouts, voxel scale, meshing approach, streaming topology, and persistence encoding
- Performance budgets, target hardware, and graphics-stack choices
- Harness-only UX, demo-world content, routes, and acceptance scenarios

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** (must ship as an adjacent validation harness that uses public APIs) or only **permitted** in the repository?

- **Proposed safe answer:** Required as an adjacent validation harness that proves the substrate through public interfaces; it is not part of product identity and does not import game features.
- **If different:** If only permitted, substrate crates alone may satisfy current delivery with no obligation to ship a walkable executable. If the walkable world is treated as the product itself, identity shifts from reusable substrate to a playable demo product.

## Seed synthesis

- **README.md:** Defines Moria as the GPU-resident Rust substrate and the walkable executable as a separate harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md:** Binds identity to reusable Rust crate(s), excludes the actual game and named game layers, and requires any harness to share public interfaces with external games.
- **docs/seeds/product-one-seed.md:** Motivates an initial substrate proof (natural generated region, dig/place, meshing, streaming, persistence) via a walkable demo; harness controls, content, and platform gates stay out of product identity.
- **docs/seeds/voxel-world-substrate.md:** Supplies long-horizon substrate purpose (material world, deep Z, multi-game reuse, standalone engine) and future enabling matter systems without making that full inventory current scope.
