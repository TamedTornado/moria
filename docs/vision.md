# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates for consumption by external games. It is an engine-layer world product, not a game and not a repository that ships the eventual title’s rules or content.

## Purpose

Moria exists so multiple game styles can share one material world foundation: generated, fully mutable voxel matter; presentation that can read as a natural world while the grid remains truth; first-class deep vertical space; and public query and mutation surfaces—without embedding game rules, LLM systems, or a particular title’s policies in the substrate.

## Product boundary

**In product**

- The reusable voxel-world substrate and its public consumer interfaces.
- Substrate responsibilities for matter, world-side physical behavior hooks, spatial queries, and mutation so downstream games can dig, place, explore, stream, persist edits, and reason about the world.
- Compatibility seams only where substrate requirements demand them—not implementations of future game layers.

**Adjacent, not identity**

- A walkable-world executable may exist as a validation harness that exercises the substrate. Whether delivering that harness is part of the current commitment is unresolved (see Q1). If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific implementation paths.
- Harness-specific character control, authored demo route, presentation choices, scripted workloads, machine profiles, and performance gates belong to that adjacent artifact, not to substrate identity.

**Out of this product / repository**

- The actual game is a separate downstream consumer and is not part of this repository.
- Game rules and the future System, LLM, spell, gas, combat, AI, and building layers are out of scope (seams only where required; those layers are not implemented here).

## Required product outcomes

A competent design must make these consumer-visible guarantees true:

1. **Material world truth** — The world is fully material voxel matter: any location in scope can be destroyed, moved, or placed; simulation and gameplay truth run on that matter, not on decorative geometry outside it.
2. **Natural-looking presentation from voxel truth** — Consumers can present a surface world that reads as ordinary terrain (hills, vegetation presence, water bodies, rock) while the voxel grid remains the authority; the mesh or dressing is a view, regenerated from matter, not a second world model.
3. **Deep vertical play space** — Underground extent is first-class content space (caves, strata, descent), not a flat floor under a skybox.
4. **Generation and lazy presence** — Worlds can be produced as geology-oriented generation so regions materialize on demand; large extents need not be fully resident as raw voxels when untouched.
5. **Mutation, streaming, and persistence of scars** — Consumers can edit matter, stream active regions, and persist edits as deltas over generation so reloads restore player-altered truth.
6. **Substrate, not game; no LLM dependency** — The same substrate stack supports multiple games above it. Game rules, pricing policy, and LLM/System behavior live above Moria. The substrate stands alone with zero LLM dependency.

## Future products and enabling implications

Described future consumers—not current Moria products—include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox titles. They motivate reuse, not current gameplay ownership.

High-level enabling implications already in the substrate vision (delivery depth and sequence are design): continuous 3D mutable worlds suitable for surface and deep play; matter-driven interactions games can specialize; public verbs/queries so agents and tools share one mutation path; and edit-delta persistence so later modes can load prior scars. Gameplay, UX, controllers, authored content, presentation, and game-specific policy remain consumer-owned.

## Non-goals

- Shipping the actual game, its rules, content, combat, AI, or building-game systems in this repository.
- Implementing System/LLM features, spells, gas/pricing policy, or game-layer intent stacks inside the substrate.
- Treating a first walkable demo’s content, controller, camera, route, or benchmark gates as the product’s identity or permanent scope.
- Expanding this brief into architecture, crate graphs, algorithms, asset catalogs, or acceptance thresholds.

## Confirmed vision constraints

- **Rust crate consumer surface** — Integration is as a Rust crate or small family of tightly scoped Rust crates.
- **GPU-resident substrate** — The world substrate is intended to run GPU-resident for the matter-heavy path.
- **Equal public access** — Any in-repo validation executable and external games use the same public interfaces; no privileged harness-only world paths.
- **No LLM in the substrate** — Standalone engine layer with zero LLM dependency.
- **Game layers excluded** — System, LLM, spell, gas, combat, AI, and building layers are not implemented here.

## Deferred design decisions

- Precise crate split, internal layering, and API shape within the public-consumer boundary.
- Delivery depth and sequence of matter behaviors (e.g. fluids beyond static bodies, integrity, fire, granular settle, vegetation lifecycle, rigid coupling).
- Representation and meshing strategy, generation pipeline detail, streaming rings, and persistence encoding.
- Supported hardware/OS profiles, performance budgets, and validation workloads—including whether a harness is delivered and what it exercises (after Q1).
- Voxel resolution and related fidelity/cost tradeoffs.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** alongside the substrate crates, or only a **permitted adjacent validation artifact**?

- **Proposed safe answer:** Permitted adjacent validation artifact only. Current product commitment is the reusable substrate crates; a harness may exist later or in parallel but is not required to define “done” for Moria’s product identity.
- **If answered differently:** Requiring the harness adds a current delivery of a public-API walkable executable without moving controller, demo content, presentation, or performance gates into substrate identity; those remain harness-owned acceptance details for design.

## Seed synthesis

- **`README.md`** — Names Moria as the reusable GPU-resident voxel-world Rust crate and frames the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binding boundary: substrate crates are the product; the real game is out of repo; harness if present is public-API-only; game/System/LLM/spell/gas/combat/AI/building layers are out of scope.
- **`docs/seeds/product-one-seed.md`** — Adjacent first-slice “walkable world” demo/harness intent (content, controller, milestones, machine-specific targets); used to confirm validation may exercise generation, mutation, streaming, and persistence, not to redefine substrate identity or import demo scope.
- **`docs/seeds/voxel-world-substrate.md`** — Substrate purpose and outcome altitude: natural-looking material worlds, full mutability, deep Z, reusable matter/query/mutation foundation, GPU-resident path, multi-game reuse without LLM dependency; mechanism and milestone detail deferred to design.
