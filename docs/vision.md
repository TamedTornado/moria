# Moria — Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**: a Rust crate (or small family of tightly scoped Rust crates) that owns the material world layer for continuous three-dimensional play. Downstream games consume it through public interfaces. They do not live in this repository.

The substrate’s job is the world of matter: geology-backed generation, mutation, observation, events, voxel-backed objects, matter-level behavior, meshing and surface dressing as non-authoritative views, sparse streaming, and persistence. Consumers issue **commands in** and receive a **stale mirror plus events out**. Nothing above the matter layer touches voxels directly.

This repository also delivers a **walkable-world executable** as a **required first-delivery validation harness**, not as product identity. That harness is a separate public-API consumer: it proves generation, streaming, meshing, editing, collision against voxel truth, persistence, and performance. Character control, camera, curated demo content, presentation chrome, and machine-specific acceptance gates are harness concerns—not substrate identity.

## Purpose

Moria exists so multiple games can share one authoritative material world instead of each rebuilding terrain, dig/build mutability, deep underground space, and matter-facing observation and mutation. The world must read as a normal natural surface while remaining fully mutable voxel truth all the way down. Game rules, economy, combat, AI, and presentation live above this layer. The substrate stands alone with **zero LLM or “System” dependency**.

## Product boundary

**In product (Moria)**

- The reusable substrate and its public Rust consumer surface.
- Geology-first generation of natural surface and contentful deep-Z volumes; material occupancy; mutation; non-authoritative mesh/dressing views; streaming; persistence of voxel edit deltas and substrate-owned object/entity state.
- Command-mediated queries, mutations, and events; physics/collision against voxel truth; mutation-safe navigation derived from voxels.
- Substrate-owned matter behavior families and placement primitives (including reusable stamps / structure representation needed for generation and mutation), at product altitude—not game building UX.
- Compatibility seams the substrate requires for later game layers—without implementing those layers.

**Required adjacent delivery (not identity)**

- A walkable-world executable that exercises the fused first slice through the same public interfaces an external game would use. Controls, camera, seed-region content, presentation, workloads, platforms, and numeric gates belong to that harness.

**Out of product / repository**

- The actual game(s) that will consume Moria.
- Game rules and System, LLM, spell, gas, combat, AI, and building **gameplay** layers (work orders, designation UX, mechanism entity logic, room/economy policy, authored game content).
- Privileged or harness-only paths into world truth.

## Required product outcomes

1. **Reusable Rust matter world** — External games integrate public crate API(s) with no in-tree privilege and no LLM dependency. Load-bearing GPU work stays on a portable path (wgpu/WGSL family); the crate must not fork load-bearing layers to a single vendor API. Named devices and exact backend choices are not product identity.

2. **Material truth, regenerated views** — The voxel volume is authoritative matter. Mesh and dressing are regenerated views—never saved truth. Physics, collision, and world-facing queries run against voxel truth, not the render mesh. **Categorical interactable rule:** everything that can burn, break, or block is voxel-backed; only passive dressing (grass, flowers, ground clutter) may lack individual voxel identity, and it remains a pure function of underlying voxel state so it cannot desync.

3. **Natural surface, mutable everywhere, deep Z** — Generated regions read as ordinary outdoor worlds while remaining fully mutable anywhere, including deep underground. Generation is geological (strata, caves, ores, aquifers, biomes) with lazy materialization so large sparse regions stay tractable. Any voxel can be destroyed, moved, or placed.

4. **Command / stale-mirror consumer contract** — Consumers change and inspect the world only through public verbs, queries, and events. Observation is a potentially stale mirror plus events, not live authoritative CPU-side voxel access. Dig/place-class operations and matter queries ship on that surface from the first slice.

5. **Matter-behavior families (full substrate mandate)** — Fluids (static bodies and richer flow/splash coupling), structural support and failure, granular settle, thin ambient time/weather/fire ecology, and physical lifecycles for voxel-backed objects (including growth and falling/rigid conversion). Delivery sequence and depth after the first slice are design; exclusion from the first cut does not reassign ownership to consumers. Pricing and win conditions stay out of the crate.

6. **Mutation-safe navigation across continuous 3D** — Navigation and traversal data derived from voxel occupancy stay consistent under mutation and support continuous three-dimensional movement classes. Agent AI, path policy, and designation UX remain consumer-owned.

7. **Streamable, persistent, operable at scale** — Active neighborhoods stream; untouched world stays cheap via sparse/lazy residency. Persistence truth is worldgen function plus edit deltas (plus object/entity journals in the complete product) so scars and substrate-owned object state restore across runs. First-slice promise includes interactive frame behavior, bounded edit-to-remesh latency, cold-start into a walkable world, sparse GPU residency, compact save with exact material restoration from seed plus deltas, and comparable benchmark evidence. Exact thresholds, machines, and workloads are harness/adjacent-delivery detail.

### First substrate slice (binding delivery contour, not the full product ceiling)

**Ships in the first delivery:** full generation for a curated proof region (natural surface, contentful deep-Z geology, lazy materialization, POI attachment metadata); partial matter (sparse occupancy, GPU incremental meshing, surface dressing, voxel-object placement and presentation without felling/rigid conversion, static water bodies); dig/place and mirror queries; streaming; persistence with exact restore from seed plus deltas; performance outcomes above; and the adjacent walkable harness on the public API.

**Out of the first cut, still product outcomes later:** active cellular automata / fire, flowing fluids, structural integrity, granular settle, vegetation growth, seasonal/ambient coupling, and non-stretch felling / rigid conversion.

## Future products and enabling implications

Future **consumers** (not this product) include a System-driven ARPG, fortress/colony play, a Moria-style descent experience, and pure sandboxes. They own gameplay, UX, controllers, characters, content, pricing policy, presentation, mechanism scripts, and room/economy semantics.

**Enabling implications only:** the same public matter, generation, mutation, navigation, streaming, and persistence surface should let those games author content and apply rules without privileged world paths. Cross-run reuse of scarred worlds (deltas as durable truth) and continuous-3D / deep-Z support are substrate enablers, not deferred consumer features. Multiplayer-ready command/mirror separation is a design posture, not a committed shipping milestone.

## Non-goals

- Shipping a playable game, combat loop, stats, AI agents, or multiplayer service in this repository.
- Implementing System/LLM features, spells, gas/pricing policy, or building-as-gameplay layers (UI, work orders, mechanism game logic, room assignment, economy).
- Defining product identity as the walkable demo, a specific seed region, an audience clip, or machine-specific performance gates.
- Making mesh or dressing authoritative for simulation or saves.
- Any LLM dependency inside the substrate, or forking load-bearing GPU layers to a single vendor/machine.

## Confirmed vision constraints

- Delivery form: Rust crate or small family of tightly scoped Rust crates, with a Cargo workspace boundary between reusable substrate and validation harness.
- GPU-resident world substrate; load-bearing GPU layers stay on portable wgpu/WGSL—no native Metal fork in those layers.
- Zero required LLM/System dependency; gas/pricing and game policy are consumer-injected if present at all.
- Commands in; stale mirror plus events out; no privileged consumer path, including the harness.
- Voxel matter is truth; burnable/breakable/blocking interactables are voxel-backed.
- Product One pins first-delivery depth; it does not shrink long-horizon substrate identity to that slice alone.
- Walkable harness is a required current adjacent delivery and must remain an unprivileged public-API consumer.

## Deferred design decisions

- Capability depth, tuning, and delivery order within each matter-behavior family beyond the first-slice contour.
- Representation, meshing approach, streaming layout, delta encoding, sim scheduling, voxel scale, and LOD strategy.
- Exact public API shape and crate family split (consumer boundary intent is fixed).
- Navigation data shape and movement-class depth—without reassigning ownership.
- How far multiplayer-oriented command/mirror patterns are carried early.
- Harness-owned content, controls, platforms, workloads, and numeric acceptance measures.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds agree on product identity (the substrate crate), first-slice substance (Product One), harness role (required adjacent validation consumer, not identity), and what is reference versus binding for this milestone.

## Seed synthesis

| Seed | Contribution to this vision |
|---|---|
| **`README.md`** | Names Moria as the reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/validation harness—not a game layer. |
| **`docs/seeds/project-boundary.md`** | Binding boundary: product is substrate crate(s); game out of repo; harness is unprivileged public-API consumer; game/System/LLM/spell/gas/combat/AI/building layers out of scope. Workspace split is concrete; exact crate layout is design. |
| **`docs/seeds/product-one-seed.md`** | Binding first-delivery contour and “done” proof: full generation for a curated region, partial matter (static water, dressing, object placement without felling), dig/place + mirror queries, streaming, exact seed-plus-delta restore, performance family, portable wgpu/WGSL path, and the walkable harness as product-shaped validation. Explicitly keeps CA/fire, flowing fluids, integrity, granular settle, growth, and weather out of the first cut without removing them from longer-horizon substrate identity. Harness controls, camera, route, content, and machine gates stay adjacent. |
| **`docs/seeds/voxel-world-substrate.md`** | Architecture reference and long-horizon substrate mandate: natural look over voxel truth, categorical object vs dressing rule, deep-Z geology, matter-behavior families, command/mirror coupling, mutation-safe multi-class navigation, streaming/persistence, multi-game reuse without LLM. Only portions selected by Product One are required for the first milestone; game-layer features (mechanisms-as-gameplay, rooms/economy, System authorship) are not imported into current scope. |

### Omitted or demoted source material (intentionally)

- Specific demo content (tree species, ruin stamp, palette list, route beats, debug-key lists) — harness/fixture detail.
- Algorithm and storage catalogs (brick sizes, bit layouts, meshing algorithms, CA pass inventories) — technical design.
- Platform atomic-width rules and named GPU class targets — engineering/harness constraints refining portable operation, not product identity.
- Building UI, blueprints-as-gameplay, mechanisms, rooms, work orders, gas pricing, System/LLM attachment behavior — future consumer layers or seams, not current product outcomes.
- Open engineering questions (25 cm vs 12.5 cm final call, LOD strategy, object spatial index, fluid pressure solve) — deferred design, not vision identity questions.

---

**Summary:** Moria is the reusable GPU-resident voxel-world substrate—matter, generation, mutation, and a reactive natural world behind a command / stale-mirror / event boundary. Product One is the first proof: a substrate slice plus walkable public-API harness that looks like a natural world and digs like matter. Games live elsewhere.
