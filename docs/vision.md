# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** for Rust consumers: a small family of tightly scoped crates that own world matter, generation, mutation, observation, and related physical/material behavior. It is an engine-layer product, not a game. A **walkable-world executable** is a required adjacent first delivery slice: a separate validation consumer of the public substrate surface, not part of product identity.

## Purpose

Give downstream games a shared foundation for large, natural-looking worlds that remain fully material and mutable—including deep underground—so adventure, fortress, sandbox, and similar titles can sit on one substrate without each reimplementing the world layer. The substrate must stand alone with no dependency on an LLM or “System” layer.

## Product boundary

**This product owns:** the reusable voxel-world substrate and its public integration surface for external Rust games and tools—matter, generation, material and ambient behavior, neutral construction and spatial primitives, navigation data derived from the world, and persistence of world truth across runs.

**Adjacent required delivery (not identity):** a walkable-world executable that validates terrain generation, streaming, meshing, editing, collision, persistence, and related substrate behavior through the same public interfaces available to an external game. It must not own privileged or game-specific substrate paths. Its controller, character, content, presentation, route, workload, platform, and performance gates are harness concerns, not substrate identity. For that first slice’s single-save (seed plus deltas) path, load restores the saved world exactly; that exactness does not extend to every substrate persistence artifact.

**Downstream / out of repository:** the actual game (or games) that consume Moria.

**Not this product:** game rules; System/LLM features; spells; gas policy; combat; AI; building-as-gameplay layers (work orders, blueprints-as-gameplay, mechanism policy); player controllers; cameras; authored demo content; presentation and UX chosen by a consumer or harness; multiplayer gameplay or networking implementation.

Compatibility seams may be designed where substrate requirements demand them; those excluded layers must not be implemented here.

## Required product outcomes

- **Reusable Rust substrate.** Downstream games and tools consume Moria as crate(s), not as a shipped title; game policy stays above the substrate, with no LLM/System requirement.
- **Natural look, material truth, deep Z.** Surface worlds can read as continuous natural terrain while remaining fully backed by mutable voxel matter; render meshes are views, not authority. Underground volume is first-class; worlds generate as coherent geology so dig-down and cut faces are honest, with unvisited volume materializing on demand. The live world representation and core substrate work are GPU-resident.
- **Mutation and bounded observation.** Matter can be destroyed, moved, or placed only through the public command surface. Consumers observe via a stale/coarse mirror plus events—not direct privileged voxel access and not a promise of necessarily current GPU truth.
- **Material and ambient behavior.** Vegetation and interactable objects stay materially consistent with the matter world; fluids, fire/wetness, granular settle, growth, and structural failure are substrate outcome families. Thin ambient simulation (time-of-day, seasons, weather-driven effects that make the surface world behave) is present on the substrate.
- **Construction, spatial semantics, and navigation.** Neutral placement and stamp affordances; structure/room metadata over enclosed volumes; mutation-safe 3D navigation data derived from the world. These are substrate primitives, not building gameplay.
- **Large-world practicality and persistence.** Sparse residency and streaming around activity keep large regions workable. Persisted truth is generation plus edit deltas, plus object/entity journals, supporting cross-run reuse. Exact load restoration is a walkable-world first-slice obligation for its single-save seed-plus-deltas path, not a substrate-wide guarantee on every journaled or persisted artifact.

## Future products and enabling implications

Described future **consumers** (not this product) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style deep-descent experience, and pure sandbox modes. The walkable-world harness is a validation consumer of the current substrate, not a second product identity.

**Enabling implications** (substrate-level, not consumer features): deep mutable geology and dig/place/move; command/mirror/event symmetry usable by agents and tools alike; material-behavior and ambient outcomes above so fortress- and adventure-class games can attach without re-owning the world; neutral placement, room semantics, and nav data so building and pathfinding games can layer policy above. Gameplay, content, controllers, presentation, and mode-specific policy remain consumer-owned.

## Non-goals

- Shipping the actual game, its rules, or its content pipeline inside this product
- Implementing System/LLM, spell, gas, combat, AI, or building gameplay layers here
- Implementing multiplayer gameplay or networking here
- Treating harness- or demo-specific characters, routes, debug UX, trailer content, platforms, or performance gates as substrate scope
- Making decorative non-material geometry the authority for collision, queries, or mutation

## Confirmed vision constraints

- **Ecosystem:** product is a Rust crate or small family of tightly scoped Rust crates.
- **Consumer boundary:** the in-repo walkable-world validation executable is a required adjacent first delivery and must consume only public interfaces; privileged game-specific substrate paths are forbidden.
- **GPU-backend portability:** load-bearing substrate layers must remain portable across Metal, Vulkan, and DX12; a Metal-only fork of those layers is forbidden.
- **Scope exclusion:** game rules and future System, LLM, spell, gas, combat, AI, and building layers are out of scope for implementation here.
- **Independence:** the substrate must function with zero LLM dependency.

## Deferred design decisions

- Delivery depth and sequence of substrate capability families (how far generation, meshing, material simulation, objects, fluids, integrity, ambient sim, and related behavior go in each release)
- Precise crate split, internal layering, algorithms, data layouts, and performance budgets
- Voxel resolution, LOD, streaming-ring policy, and object-layer scaling choices
- Implementation depth of ambient simulation and of each material-behavior family
- Harness design details: platforms, scenes, controls, content, workloads, and acceptance gates

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is **server-authoritative multiplayer-readiness** of the command architecture a required current-product constraint even though multiplayer is not built?

- **Proposed answer:** Yes—keep the public command/mirror/event surface multiplayer-ready by construction as a product constraint, without implementing multiplayer gameplay or networking.
- **If different:** Answering no removes multiplayer-readiness from product identity and constraints; the substrate would only need the command/mirror boundary for sandbox reuse and privileged-access exclusion, not as an explicit multiplayer-ready quality.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and positions the walkable-world executable as a separate validation consumer for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding boundary—substrate crates are the product; the real game is out of repo; the walkable harness remains a non-product consumer of public interfaces; game/System/LLM/spell/gas/combat/AI/building layers stay out of scope.
- **`docs/seeds/voxel-world-substrate.md`:** Defines substrate purpose and outcome families—natural material worlds; destroy/move/place mutation; command/mirror/event observation; deep Z and geology-first generation; GPU-resident matter; vegetation/objects, fluids, fire/wetness, granular, growth, structural failure; thin ambient sim; placement/stamp, room metadata, mutation-safe nav; persistence via generation plus deltas and journals with cross-run reuse (without substrate-wide exact restoration); multi-genre consumers above game policy; open whether multiplayer-readiness stays in scope statements.
- **`docs/seeds/product-one-seed.md`:** First-slice validation story that settles a concrete walkable-world delivery and exact save/load restoration for that slice’s single-save seed-plus-deltas path; requires load-bearing layer portability across Metal/Vulkan/DX12 (no Metal-only fork); narrows first-slice depth without removing substrate-owned outcome families from product identity.
