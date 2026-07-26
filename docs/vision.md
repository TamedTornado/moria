# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer library for natural, fully material voxel worlds—not a game and not a gameplay shell.

**Product One** is the **first delivery slice**: the substrate crates **plus** one required adjacent walkable-world validation harness (a generated region and a controllable character using public interfaces). That slice must prove generation for a curated region—columns, strata, caves, ore, lazy materialization, and POI metadata (continent pass may be stubbed to that region)—plus sparse/lazy residency, GPU incremental meshing, matter-consistent dressing and static water, voxel-object placement and rendering, dig/place/query boundaries, streaming, and persistence. It excludes active cellular/material simulation or fire, dynamic fluids, structural integrity, granular settling, and full object dynamics. Those families remain required substrate outcomes; depth and order are design concerns.

## Purpose

Games need a foundation where the surface reads as ordinary terrain, the underground is real content, and every visible solid is mutable matter—not decorative geometry. Moria exists so multiple games (sandbox, fortress, descent, ARPG) can share one matter-and-world stack: generation, material truth, mutation, observation, consumer-neutral authoring seams, persistence, and world/physics foundations, with game rules entirely above the substrate.

## Product boundary

**This product owns:** the reusable substrate crates and public interfaces for world generation, matter storage and residency, meshing-as-view, mutation and observation, material and voxel-object foundations, consumer-neutral content registries and placement seams, world simulation foundations assigned to the substrate, and persistence/streaming of world and object/entity state.

**Product One delivery (adjacent, not identity):** a walkable-world executable is a **required** validation harness for Product One. It is a separate consumer that must use the same public interfaces as an external game and must not own privileged or game-specific substrate paths. Delivery includes that harness as proof of the substrate—not as part of substrate identity.

**Downstream, not this repository:** the actual game and game rules; System/LLM features; spell, gas, combat, and AI layers; building-game policy and UX (blueprints as gameplay, work orders, mechanisms as game entities). Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

**Harness-owned (outside substrate identity):** character control, camera, authored demo route, presentation polish, debug tools, exact seed content and palette, scripted benchmark workloads, platforms, and numeric performance gates. At vision altitude the harness must still prove continuous traversal of a natural deep-Z material world, visible dig/place material truth, and validation of performance, streaming, and restoration through public interfaces.

## Required product outcomes

- **Reusable GPU integration surface.** Consumers issue commands and observe a coarse, potentially stale mirror plus events—not unrestricted or implicitly synchronous access to GPU-resident truth. Nothing above the matter boundary mutates voxels directly; dig, place, and related operations go through that public Rust-crate command/query/event surface.
- **Natural continuous material world.** Generated terrain reads as continuous natural ground while remaining fully material: dig, place, and related operations work anywhere content exists, including first-class deep underground, not a heightmap shell. Collision, queries, and mutation run against material occupancy; meshes are regenerated views, never authority.
- **Tractable large worlds and durable restoration.** Sparse residency, lazy materialization, and streaming-oriented active sets keep idle volume practical. Generated truth plus edit deltas restore unedited and scarred matter; object and entity lifecycle state persists separately; restored state supports cross-run reuse. Zero LLM dependency in the stack.
- **Matter-consistent presentation and objects.** Interactable solids and surface dressing stay driven by the material world. Voxel objects support placement, registration, rendering, and substrate-owned interactive object behavior (growth, felling/conversion, impact with matter); dressing stays consistent with material state.
- **Consumer-neutral authoring and extension.** External consumers define and place world, material, and object content through the same neutral public registries and placement seams (metadata, palettes, materials, objects/placements, related extension paths). Higher systems attach only as external clients of those seams. Product One includes POI metadata among them.
- **World simulation foundations.** Material simulation, dynamic fluids, ambient simulation (time, weather-driven effects, fire ecology at world scale), structural failure, granular behavior, and mutation-safe navigation data derived from the material world—so consumers need not reimplement those foundations.

## Future products and enabling implications

Future **consumers** (not current product) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony mode, and a Moria-style descent experience. They motivate a game-policy-free substrate exposing matter, queries, mutation, observation, neutral authoring seams, and the simulation and persistence foundations above. Gameplay, content, controllers, presentation, economy, and AI remain consumer-owned. Substrate outcome families here are product responsibilities, not merely long-horizon implications of those games.

## Non-goals

- Implementing a shippable game, ARPG, fortress sim, or descent roguelike in this repository
- System/LLM runtime, spells, gas policy, combat, agent AI, or building-game layers
- Absorbing the harness’s controller, route, UI, exact content, workloads, platforms, or numeric gates into substrate identity
- Making decorative-only terrain or a pure heightmap world the product truth model
- Implying Product One ships complete generalized continental generation (continent pass may be stubbed to the curated region)
- Implying object, entity, or simulation state is stored only as voxel edit deltas

## Confirmed vision constraints

- Delivered as a **Rust crate** or small family of tightly scoped Rust crates for Rust-game consumers
- **GPU-resident** substrate with **command-in / coarse mirror-plus-events-out** observation (potentially stale mirror)
- Any in-repo harness or external game uses **only public interfaces**—no privileged substrate paths
- **Zero LLM dependency** inside the substrate
- **Out of product scope:** game rules; System/LLM; spell, gas, combat, AI; building layers (seams only where required)
- **Product One slice:** required adjacent walkable harness; generation ships columns, strata, caves, ore, lazy materialization, and POI metadata with continent pass permitted stubbed to the curated region; active CA/fire, dynamic fluids, integrity, granular settling, and full object dynamics out of the initial slice while remaining required substrate outcome families

## Deferred design decisions

- Precise crate split, APIs, data layouts, algorithms, and meshing approach
- Voxel scale, LOD strategy, object-layer capacity, and depth of each simulation family per subsequent release
- Streaming policy, persistence encoding (including object/entity journals vs matter deltas), and synchronization patterns
- Harness technical structure and all harness-owned content, controls, presentation, platforms, workloads, and numeric gates
- Order and mechanisms for delivering simulation families after the Product One slice

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **`README.md`** — Names Moria as the GPU-resident Rust-crate substrate and separates the walkable-world executable as consumer/harness, not game layer.
- **`docs/seeds/project-boundary.md`** — Fixes product identity (reusable substrate crates), consumer vs harness boundary, public-interface-only access, and exclusion of game/System/building layers.
- **`docs/seeds/product-one-seed.md`** — Defines Product One as substrate plus required walkable demo (region + character): prove curated-region generation (continent stub permitted), residency, meshing, dressing, static water, object place/render, dig/place/query, streaming, persistence, and downloadable proof; exclude active CA/fire, dynamic fluids, integrity, granular settling, full object dynamics. Harness-owned controls, route, content detail, and numeric gates stay outside substrate identity.
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes full substrate outcome families (natural material world, deep Z, mutability, generation, interactive objects, consumer-neutral registries/placement seams, material/fluid/ambient/structural/granular simulation, mutation-safe nav, gen+delta and object/entity persistence, GPU command/mirror/events, standalone engine role) without importing mechanism inventory or consumer games into identity.
