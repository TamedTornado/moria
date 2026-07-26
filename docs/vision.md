# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world product: generated natural surface and deep underground volumes whose authoritative truth is mutable voxel matter, consumable by external games through public interfaces—not a game.

The complete product mandate is the full substrate (generation, matter behavior families, navigation/spatial support, and consumer contract below). The **first delivery slice** is narrower but settled: a full reusable generation layer that produces curated natural surface and contentful deep-Z geology; a partial matter layer (operable sparse world, non-authoritative view, surface dressing, voxel-object placement and presentation, static water bodies); and an initial dig/place/query surface. Cellular automata, flowing fluids, structural integrity, granular settling, and richer object lifecycle are postponed past that slice—not removed from the product.

A **required first delivery adjacent to Moria** (not product identity) is a walkable generated-world validation harness: a playable/downloadable proof and benchmark consumer exercising generation, streaming, meshing, editing, voxel-truth collision, persistence, and performance through public interfaces only.

## Purpose

Moria exists so multiple games can share one material world foundation instead of each rebuilding terrain, dig/build mutability, deep-Z space, and matter-facing observation and mutation. The substrate stands alone with zero LLM or “System” dependency: game rules live above it; the substrate supplies world matter, physics-relevant behavior, queries, events, and mutation. What players see is continuous and natural-looking while remaining fully editable material—not a heightmap with props.

## Product boundary

**In product (Moria):**
- The reusable voxel-world substrate and its public consumer surface as Rust crate(s): geology-first generation of natural surface and contentful deep-Z volumes; material voxel truth and its non-authoritative view; mutability; streaming; persistence (edit-delta terrain truth plus durable object and entity/script-state journals); physics/collision against voxel truth; voxel-derived mutation-aware navigation and continuous-3D movement support; command-mediated queries, mutations, and events.
- Matter-behavior outcome families of the complete substrate: voxel-backed interactive objects and matter-responsive dressing; fluid behavior and material interactions; ambient fire, wetness, weather, and time behavior; granular behavior; structural support and collapse; dynamic matter and object lifecycle—depth deferred, ownership not reassigned.
- Compatibility seams the substrate requires so higher layers can attach later—without implementing those layers.

**Adjacent, required first delivery (not product identity):**
- A walkable-world executable that validates the fused first-slice and performance claims through the same public interfaces available to an external game. Character control, camera, routes, content, chrome, workloads, platforms, and machine-specific gates belong to that harness—not Moria’s identity.

**Out of repository / downstream:**
- The actual game(s) that will consume Moria, plus game rules and future System, LLM, spell, gas, combat, AI, and building *gameplay* layers—including agent policy, path-following AI, and movement rules that consume navigation data.

## Required product outcomes

- **Reusable Rust substrate:** External consumers integrate Moria as crate(s) with no privileged in-repo path; the adjacent validation harness is bound by the same public surface.
- **Material world truth in continuous space:** Fully mutable voxel volume (destroy, move, place anywhere, including deep underground); surface and underground read as a normal world; mesh and dressing are regenerated views. Simulation, queries, and physics/collision resolve against voxel truth—not the mesh.
- **Full reusable generation layer:** From seed and parameters, the substrate produces a curated natural surface (terrain, water bodies, vegetation placement cues) and contentful deep-Z geology—stratified rock bands, walkable cave voids, resource and aquifer features—with lazy materialization of untouched volume and attachment metadata for points of interest. Substrate-owned; ships in the first slice; not deferred to harness or future games.
- **Consumer contract and navigation support:** Mutation is command-mediated; consumers observe via stale/coarse mirror state and events. Dig/place-class operations and matter queries ship through that public surface. The substrate derives mutation-aware navigation data from voxel occupancy and supports continuous 3D movement classes against world truth; agents, AI, path policy, and designation UX remain consumer-owned.
- **Complete matter-behavior families:** Beyond the first slice, the substrate owns interactive voxel objects and matter-responsive dressing, fluid behavior and material interactions, ambient fire/wetness/weather/time behavior, granular behavior, structural support and collapse, and dynamic matter/object lifecycle (depth remains design).
- **GPU-resident operable worlds and durable state:** Large regions stay tractable via GPU-resident representation, sparse/lazy materialization, and streaming of active neighborhoods. Persistence truth is worldgen function plus edit deltas; the complete substrate also journals durable object and entity/script state per region. The first slice must reload the same seed plus deltas and restore exactly. No LLM/System dependency; no game policy.

## Future products and enabling implications

Future *consumers* (not current product scope) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. Those games own rules, UX, controllers, content, presentation, and policy.

High-level enabling implications:
- Games can price and script the same dig/build/query verbs differently without forking truth.
- Cross-mode reuse of scarred worlds is enabled by treating edits and object/entity journals as durable truth.
- Continuous-3D and deep-Z navigation support lets consumers present levels or free traversal without forking structure.

## Non-goals

- Implementing playable game(s), game rules, combat, stats, AI agents, or entity ecosystems beyond substrate matter/object infrastructure and durable entity/script-state journals.
- System/LLM features, spells, gas/pricing policy, intent stacks, or substrate dependency on those.
- Building *gameplay* layers (work orders, designation UX, mechanism game logic, economy, agent path policy) even where seams may be planned.
- Absorbing harness-owned controllers, cameras, routes, content, platforms, or performance gates into Moria’s identity.
- Multiplayer shipping or a frozen inventory of every matter subsystem’s depth.

## Confirmed vision constraints

- Product form: Rust crate or small family of tightly scoped Rust crates.
- Consumer boundary: no privileged game- or harness-only path into world truth; public interfaces only.
- Runtime character: GPU-resident voxel-world substrate; zero LLM/System dependency.
- First-slice boundary: full generation layer (curated natural surface, contentful deep-Z geology, lazy materialization, POI attachment metadata); partial matter (static water, dressing, object placement/presentation; no CA, flowing fluids, integrity, granular settle, or richer object lifecycle yet); dig/place/query surface; exact restore from seed plus edit deltas.
- Adjacent delivery: walkable validation harness is required as first delivery, outside product identity, and unprivileged.

## Deferred design decisions

- Capability depth and mechanisms within each matter-behavior family after Product One.
- Representation, meshing, streaming, and persistence mechanisms; voxel scale and LOD strategy; public API shape and crate split.
- Navigation data shape, movement-class depth, and consumer consumption patterns—without reassigning ownership.
- Whether multiplayer-ready command authority is carried early.
- Harness-owned content, controls, platforms, workloads, and acceptance measures.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md:** Names Moria as the reusable GPU-resident voxel-world Rust substrate and the walkable executable as a separate validation consumer—not a game layer.
- **docs/seeds/project-boundary.md:** Binding boundary—product is substrate crate(s); game out of repo; harness unprivileged; game/System/LLM/spell/gas/combat/AI/building layers out of scope (does not cancel required adjacent delivery).
- **docs/seeds/product-one-seed.md:** Settles first-slice substance and required adjacent walkable proof; full generation (natural surface, deep-Z geology, lazy gen, POI metadata), partial matter, dig/place/query, exact seed-plus-deltas restore; harness-owned controls and gates stay adjacent.
- **docs/seeds/voxel-world-substrate.md:** Complete substrate outcomes—natural look over voxel truth, full mutability, deep Z, geology-first generation, matter-behavior families, command/mirror/events, voxel-truth physics, mutation-aware navigation and continuous-3D support, object and entity/script-state journals, GPU-resident engine without LLM dependency.
