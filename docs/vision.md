# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for downstream games—not a game, not a demo title, and not a gameplay stack. A separate walkable-world executable is a required adjacent validation harness and first delivery artifact; it is not part of product identity.

## Purpose

Give future games a shared material world: continuous three-dimensional terrain and geology that reads as an ordinary natural landscape while remaining fully mutable voxel matter underneath. The substrate exists so adventure, fortress, sandbox, and related games can consume the same world foundation without baking game rules, economy, or presentation into the crate.

## Product boundary

**In product**

- Reusable substrate crates and the public interfaces through which consumers generate, stream, inspect, collide with, mutate, observe, mesh-view, and persist a voxel world.
- World-as-matter responsibilities: sparse GPU-resident material volume, geology-oriented generation, derived surface views, mutation and observation surfaces, matter physics and simulation outcomes listed below, and durable world state suitable for large regions.

**Out of product / adjacent**

- The actual game is a separate downstream consumer and is not part of this product’s identity or repository product scope.
- A walkable-world executable is a required adjacent delivery and validation harness. It must consume the substrate through the same public interfaces available to an external game and must not own privileged or game-specific paths inside the substrate. Its purpose is to prove terrain generation, streaming, meshing, editing, collision, persistence, and performance via those public interfaces. Character control, camera, curated routes and scenery, debug presentation, scripted workloads, and harness- or machine-specific acceptance gates remain harness-owned, not substrate scope.
- Game rules and the System, LLM, spell, gas, combat, AI, and building layers are out of scope. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

Downstream design must make these product-level outcomes true:

- **Continuous ordinary look, honest cuts.** Consumers can present a continuous, smooth, ordinary-looking world; when matter is cut or carved, the visible result remains a faithful material edit. Render meshes are views of voxel truth, not a separate authoritative world.
- **Mutable deep volume.** Through public interfaces, matter can be destroyed, moved, or placed throughout the volume; underground space is first-class continuous 3D content with coherent subsurface structure, not a heightmap skin over fake fill.
- **Seed-derived geology at scale.** Worlds materialize from seed-driven generation so any region can be produced independently and lazily; large volumes stay tractable via sparse residency rather than requiring the full raw field in memory.
- **Substrate-owned matter behavior.** The product owns movable matter; voxel-backed interactive objects and matter-consistent surface dressing; fluid behavior; fire and ambient material change; granular behavior; and structural integrity with collapse. Initial delivery may prove a reduced depth of these outcomes; the outcomes remain committed substrate responsibility, not future-game ownership.
- **Commands, queries, and events.** Consumers mutate and inspect the world through a public command surface (including dig and place), a query mirror that may lag live GPU state, and events that make world change observable—not only silent post-mutation reads. Spatial interaction uses voxel occupancy so collision matches diggable truth after remesh.
- **Exact restoration and measurable performance.** Persistent truth is generation plus edit deltas; reload restores the edited world exactly. Interactive performance of the load-bearing substrate is a product obligation and is exercised and reported through the required harness. The load-bearing crate remains portable across GPU backends rather than forked to one machine or API.

## Future products and enabling implications

Described future products (System-driven ARPG, fortress/colony play, Moria-style descent, pure sandbox) are **downstream consumers**, not this product. They own gameplay, UX, controllers, characters, authored content, presentation, economy, and game policy.

Enabling implications (not consumer scope transfer):

- Shared world foundation so multiple game genres reuse the same matter, generation, and observation surfaces.
- Persistence and streaming shaped so long-lived player scars and multi-mode reuse of a region are possible at the world-data level.
- Game policy (pricing, labor, combat, LLM direction) injects above the substrate; the substrate does not implement those layers.

## Non-goals

- Shipping a playable game, campaign, or genre ruleset in this product
- System/LLM features inside the substrate
- Spells, gas metering, combat, AI, and agent labor as product scope
- Building UI, blueprints, work orders, rooms-as-gameplay, and mechanism logic
- Importing harness controls, demo scenery, machine-specific gates, or marketing milestones into substrate identity
- Treating reduced first-slice delivery depth as permanent exclusion of substrate matter outcomes

## Confirmed vision constraints

- **Rust crate delivery.** Consumers integrate through public crate interfaces.
- **GPU-resident substrate.** Core world residency and heavy world work are GPU-resident by product intent.
- **Portable load-bearing GPU path.** The substrate crate stays cross-backend portable; a machine-specific fork of load-bearing layers is not acceptable.
- **Strict consumer isolation.** Adjacent consumers have no privileged access into voxel state; harness and games share the same public surface.
- **Required adjacent harness delivery.** The walkable-world executable ships as the validation and first-delivery proof of public-interface outcomes above; it does not redefine product identity.
- **Standalone engine layer.** No LLM or System runtime dependency; listed game systems stay out even when seams are anticipated.

## Deferred design decisions

- Exact crate split and internal module boundaries (beyond the consumer-isolation outcome)
- Voxel resolution, meshing approach, LOD, and material payload layout
- Delivery depth and sequence of matter-simulation families across the first proof slice versus later substrate depth
- Streaming policy, persistence encoding, and quantitative performance budgets or benchmark environments
- Multiplayer authority design beyond keeping a command/query/event-shaped boundary open

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **`README.md`** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and identifies the walkable-world executable as the separate validation harness for generation, streaming, meshing, editing, collision, persistence, and performance.
- **`docs/seeds/project-boundary.md`** — Binding identity and exclusion: product is the substrate crates; the real game is downstream; any harness must use public interfaces; game/System/LLM/spell/gas/combat/AI/building layers stay out. Permits the harness without negating its delivery status from other seeds.
- **`docs/seeds/product-one-seed.md`** — Establishes the walkable-world harness as the required first delivery and benchmarked proof; authorizes smooth ordinary look with honest dig/place cuts, dig/place and mirror queries in the first slice, exact restoration, measurable interactive performance, and portable GPU crate delivery. Its character, route, scenery, milestones, and machine-specific gates remain harness-owned; its exclusions limit first-slice depth, not long-term substrate matter ownership.
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes full substrate outcome families: ordinary continuous look over voxel truth, universal mutability, deep Z, seed-derived geology, matter physics and simulation (movable matter, voxel objects and dressing, fluids, fire/ambient change, granular, integrity/collapse), command/query/event observation, generation-plus-deltas persistence, and multi-game reuse—without transferring game layers or mechanism inventory into this brief.
