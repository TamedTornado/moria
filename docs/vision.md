# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer library for natural, fully material voxel worlds—not a game and not a gameplay shell.

**Product One** is the **first delivery slice** of that substrate: it must prove full generation, sparse/lazy residency, GPU incremental meshing, matter-consistent dressing and static water, voxel-object placement and rendering, dig/place/query boundaries, streaming, and persistence. That slice does **not** include active cellular/material simulation or fire, dynamic fluids, structural integrity, granular settling, or full object dynamics. Those families remain required product outcomes of the substrate; their mechanisms and further delivery order are design concerns.

## Purpose

Games need a shared foundation where the surface looks like ordinary terrain, the underground is real content, and every visible solid is mutable matter—not decorative geometry. Moria exists so multiple games (sandbox, fortress, descent, ARPG) can share one matter-and-world stack: generation, material truth, mutation, observation, persistence, and world/physics foundations, with game rules living entirely above the substrate.

## Product boundary

**This product owns:** the reusable substrate crates and the public interfaces external games use for world generation, matter storage and residency, meshing-as-view, mutation and observation, material and voxel-object foundations, world simulation foundations assigned to the substrate, and persistence/streaming of world and object/entity state.

**Adjacent, not identity:** a walkable-world executable *may* exist as a validation harness. It is a separate consumer. Whether shipping it is part of current delivery is open (see Q1). If present, it must use the same public interfaces as an external game and must not own privileged or game-specific substrate paths.

**Downstream, not this repository:** the actual game and any game rules; System/LLM features; spell, gas, combat, and AI layers; building-game policy and UX (blueprints as gameplay, work orders, mechanisms as game entities). Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

**Harness-owned when a harness exists:** character control, camera, authored demo route, presentation polish, debug tools, scripted benchmark scenes, and machine-specific performance gates.

## Required product outcomes

- **Reusable GPU integration surface.** Downstream consumers issue commands and observe a coarse, potentially stale mirror plus events—not unrestricted or implicitly synchronous access to GPU-resident truth. Nothing above the matter boundary mutates voxels directly; dig, place, and related operations go through that command/query/event surface via public Rust-crate interfaces.
- **Natural continuous material world.** Generated terrain reads as continuous natural ground (hills, strata, water, living cover) while remaining fully material: dig, place, and related matter operations work anywhere the world has content, including first-class deep underground (geology, caves, voids, subsurface materials), not a flat floor under a heightmap shell. Collision, queries, and mutation run against material occupancy; extracted meshes are regenerated views, never world authority.
- **Tractable large worlds.** Regions stay practical through sparse residency, lazy materialization from generation, and streaming-oriented active sets so idle volume does not force full voxel residency.
- **Matter-consistent presentation and objects.** Interactable solids and surface dressing stay driven by the material world. Voxel objects support placement, registration, rendering, and interactive object behavior the substrate owns (growth, felling/conversion, impact with the matter world); dressing stays consistent with material state.
- **World simulation foundations.** The substrate provides material simulation, dynamic fluids, ambient simulation (time, weather-driven effects, fire ecology at world scale), structural failure, granular behavior, and mutation-safe navigation data derived from the material world—so consumers need not reimplement those foundations.
- **Durable world and entity restoration.** Generated voxel truth plus edit deltas restore unedited and scarred matter; object and entity lifecycle state persists separately from voxel deltas; restored state supports cross-run reuse. The stack has zero LLM dependency; higher systems attach only as external consumers of the same interfaces.

## Future products and enabling implications

Future **consumers** (not current product) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony mode, and a Moria-style descent experience. They motivate a substrate that stays game-policy-free while exposing matter, queries, mutation, observation, and the simulation and persistence foundations above. Gameplay, content, controllers, presentation, economy, and AI remain consumer-owned. The substrate outcome families in this brief are product responsibilities, not merely long-horizon implications of those games; only consumer-owned layers and further product lines beyond the substrate stay future.

## Non-goals

- Implementing a shippable game, ARPG, fortress sim, or descent roguelike in this repository
- System/LLM runtime, spells, gas policy, combat, agent AI, or building-game layers
- Treating the walkable-world harness’s character, route, UI, content palette, or acceptance numbers as substrate scope
- Making decorative-only terrain or a pure heightmap world the product truth model
- Implying that object, entity, or simulation state is stored only as voxel edit deltas

## Confirmed vision constraints

- Delivered as a **Rust crate** or small family of tightly scoped Rust crates for Rust-game consumers
- **GPU-resident** world substrate with **command-in / coarse mirror-plus-events-out** observation (potentially stale mirror)
- Any in-repo harness or external game uses **only public interfaces**—no privileged substrate paths
- **Zero LLM dependency** inside the substrate
- **Out of product scope:** game rules; System/LLM; spell, gas, combat, AI; building layers (seams only where required)
- **Product One slice exclusions:** active CA/fire, dynamic fluids, integrity, granular settling, and full object dynamics are out of the initial slice while remaining required substrate outcome families

## Deferred design decisions

- Precise crate split, APIs, data layouts, algorithms, and meshing approach
- Voxel scale, LOD strategy, object-layer capacity, and depth of each simulation family per subsequent release
- Streaming policy, persistence encoding (including how object/entity journals relate to matter deltas), and synchronization patterns
- Whether and how a validation harness is structured, and all harness content, controls, platforms, and performance gates
- Order and mechanisms for delivering simulation families after the Product One slice

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world validation harness **required current delivery**, or only **permitted** as an adjacent artifact?

- **Proposed safe answer:** Permitted only. Current product identity and required delivery are the substrate crates and public interfaces; a harness may exist later or alongside but is not assumed mandatory until design commits to it.
- **If answered “required”:** Delivery must include a harness executable that validates the substrate through public APIs, still outside product identity—without importing its controller, content, presentation, or performance gates into substrate scope.

## Seed synthesis

- **`README.md`** — Names Moria as the GPU-resident Rust-crate substrate and separates the walkable-world executable as consumer/harness, not game layer.
- **`docs/seeds/project-boundary.md`** — Fixes product identity (reusable substrate crates), consumer vs harness boundary, public-interface-only access, and exclusion of game/System/building layers.
- **`docs/seeds/product-one-seed.md`** — Defines Product One as the first substrate delivery slice (prove generation, residency, meshing, dressing, static water, object place/render, dig/place/query, streaming, persistence; exclude active CA/fire, dynamic fluids, integrity, granular settling, full object dynamics). Demo player, content route, and platform gates stay harness/consumer-side.
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes full substrate outcome families (natural material world, deep Z, mutability, generation, interactive objects, material/fluid/ambient/structural/granular simulation, mutation-safe nav, gen+delta and object/entity persistence, GPU command/mirror/events, standalone engine role) without importing mechanism inventory or consumer games into identity.
