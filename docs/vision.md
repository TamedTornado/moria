# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for downstream games—not a game, not a demo title, and not a gameplay stack. A separate walkable-world executable is a required adjacent validation harness and first delivery artifact; it is not part of product identity.

## Purpose

Give future games a shared material world: continuous three-dimensional terrain and geology that reads as an ordinary natural landscape while remaining fully mutable voxel matter underneath. Adventure, fortress, sandbox, and related games can consume the same foundation without baking game rules, economy, or presentation into the crate.

## Product boundary

**In product**

- Reusable substrate crates and public interfaces to generate, stream, inspect, collide with, mutate, observe, mesh-view, and persist a voxel world.
- World-as-matter: sparse GPU-resident volume, geology-oriented generation, derived surface views, mutation and observation surfaces, matter physics and simulation, mutation-safe derived traversal data, ambient time/season/weather effects on matter, and durable state including substrate-owned movable objects.

**Out of product / adjacent**

- The actual game is a separate downstream consumer, outside this product’s identity and repository product scope.
- A walkable-world executable is a required adjacent delivery and validation harness. It must use the same public interfaces available to an external game and must not own privileged paths inside the substrate. Character control, camera, curated demo route, debug presentation, and benchmark workload remain harness-owned, but are required parts of that adjacent first delivery.
- Game rules and the System, LLM, spell, gas, combat, AI, movement policy, and building layers are out of scope. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

- **Ordinary continuous world over voxel truth.** Consumers can present a continuous, smooth, ordinary-looking world; cuts remain faithful material edits. Meshes are views of voxel truth. Through public interfaces, matter can be destroyed, moved, or placed throughout the volume; underground is first-class continuous 3D with coherent subsurface structure. Worlds materialize from seed-driven generation independently and lazily; large volumes stay tractable via sparse residency.
- **Substrate-owned matter and ambient world.** The product owns movable matter; voxel-backed interactive objects and matter-consistent surface dressing; multi-tier fluids; fire ecology; granular behavior; and structural integrity with collapse. Time, seasons, and weather drive material responses—wetness, water-body change, growth or snow, and fire ecology. These are full-substrate responsibilities; first delivery proves only the reduced depth below.
- **Commands, queries, events, and collision.** Consumers mutate and inspect through a public command surface (including dig and place), a query mirror that may lag live GPU state, and events that make change observable. Collision uses voxel occupancy so interaction matches diggable truth after remesh.
- **Mutation-safe traversal data.** Navigation information is derived from mutable bricks and supports 3D movement classes (walk, climb, fly, burrow, swim). Dirty mutation invalidates affected navigation without a global rebuild. Downstream AI and movement policy remain consumer-owned.
- **Exact restoration of substrate-owned state.** Persistent truth is generation, voxel edit deltas, and journals for substrate-owned movable voxel objects (moved or felled state). Reload restores that world exactly. Gameplay-entity persistence remains consumer-owned.
- **Measurable interactive performance.** Interactive performance of the load-bearing substrate is a product obligation, exercised and reported through the required harness. The load-bearing crate stays portable across GPU backends rather than forked to one machine or API.

**First-slice delivery (settled)**

The first delivery is a walkable-world harness that proves one curated seed-generated, continuously traversable natural region through public interfaces: full sparse/lazy geology generation; GPU incremental meshing; dressing and static voxel-object presentation; static water only; public dig/place and mirror queries; voxel-truth collision; exact save/load; and reported performance. The harness’s controller, camera, continuous demo route, debug presentation, and benchmark workload are required parts of that adjacent delivery, not substrate identity. Product One does **not** run CA, fire, tier-2/3 fluids, integrity, granular settling, or non-stretch felling; those remain full-substrate outcomes, not open depth questions and not first-slice proof obligations.

## Future products and enabling implications

Future products (System-driven ARPG, fortress/colony, Moria-style descent, pure sandbox) are **downstream consumers**. They own gameplay, UX, controllers, characters, content, presentation, economy, and game policy.

- Shared world foundation: same matter, generation, traversal data, and observation surfaces across genres.
- Persistence and streaming shaped so player scars, substrate-owned object change, and multi-mode region reuse are possible at the world-data level.
- Game policy (pricing, labor, combat, LLM direction) injects above the substrate.

## Non-goals

- Shipping a playable game, campaign, or genre ruleset in this product
- System/LLM features, spells, gas, combat, AI, agent labor, or movement policy as product scope
- Building UI, blueprints, work orders, rooms-as-gameplay, and mechanism logic
- Importing harness presentation choices into substrate identity (while still requiring the adjacent first-slice harness)
- Treating first-slice exclusions (CA, fire, flow fluids, integrity, granular settle, non-stretch felling) as permanent removal of those substrate outcome families

## Confirmed vision constraints

- **Rust crate delivery.** Consumers integrate through public crate interfaces.
- **GPU-resident substrate.** Core world residency and heavy world work are GPU-resident by product intent.
- **Portable load-bearing GPU path.** The substrate crate stays cross-backend portable; a machine-specific fork of load-bearing layers is not acceptable.
- **Strict consumer isolation.** Adjacent consumers have no privileged access into voxel state; harness and games share the same public surface.
- **Required adjacent first-slice harness.** The walkable-world executable ships as the fixed first-delivery proof above; it does not redefine product identity.
- **Standalone engine layer.** No LLM or System runtime dependency; listed game systems stay out even when seams are anticipated.

## Deferred design decisions

- Exact crate split and internal module boundaries (beyond consumer isolation)
- Voxel resolution, meshing approach, LOD, and material payload layout
- How substrate depth beyond the fixed first slice is sequenced after that delivery
- Streaming policy, persistence encoding details, and quantitative performance budgets or benchmark environments
- Multiplayer authority design beyond keeping a command/query/event-shaped boundary open

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **`README.md`** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and the walkable-world executable as the separate validation harness for generation, streaming, meshing, editing, collision, persistence, and performance.
- **`docs/seeds/project-boundary.md`** — Binding identity: product is the substrate crates; the real game is downstream; harnesses use public interfaces; game/System/LLM/spell/gas/combat/AI/building layers stay out.
- **`docs/seeds/product-one-seed.md`** — Fixes first delivery: required adjacent walkable-world harness proving one curated seed-generated continuous natural region with sparse/lazy geology, GPU incremental meshing, dressing and static voxel objects, static water, dig/place and mirror queries, voxel-truth collision, exact save/load, and reported performance; harness controller/camera/route/debug/benchmark are part of that delivery. Excludes running CA, fire, tier-2/3 fluids, integrity, granular settle, and non-stretch felling from that slice.
- **`docs/seeds/voxel-world-substrate.md`** — Full substrate outcomes: continuous look over voxel truth, universal mutability, deep Z, seed-derived geology, matter physics, ambient time/season/weather material response, mutation-safe navigation and 3D movement classes, generation-plus-deltas plus object journals, multi-game reuse—without transferring game layers into this brief.
