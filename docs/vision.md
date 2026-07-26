# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it as a library; Moria is the world-and-matter engine layer, not a game.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface over fully mutable voxel truth, continuous deep underground, and public mutation, query, and extension surfaces that keep game rules above the substrate. The material world must behave as a coherent world—not only look like one—and stand alone with no LLM or game-policy dependency.

## Product boundary

**In product**
- The reusable substrate: geology-first generation, sparse GPU-resident matter, non-authoritative visual presentation of that matter, mutation and query APIs, public surfaces for consumers to author world and material definitions, matter-coupled world behavior, derived navigation aids, and persistence/streaming of substrate-owned world truth (terrain edits and object lifecycle).
- A clean public integration surface so external games use the same verbs, queries, events, and extension surfaces as hand authoring and in-repo consumers.

**Adjacent / not product identity**
- A walkable-world executable is a **required adjacent first proof**. It validates generation, streaming, meshing, editing, collision, persistence, and performance, and is the project’s first downloadable/benchmark deliverable. It is not a game layer and does not define product identity. It must consume the substrate only through public interfaces. Character, controller, content, presentation, demo route, workloads, platforms, and numeric gates remain consumer-owned.
- The actual game (or games) is a separate downstream consumer, not part of this repository.

**Out of this product**
- Game rules; System/LLM features; spells; gas/pricing policy; combat; AI; and building/gameplay layers (UX, controllers, authored content, presentation policy, work orders, and game-specific modes).
- Compatibility seams may be designed where substrate outcomes require them; higher layers are not implemented here.

## Required product outcomes

A downstream design must make these true of the substrate:

1. **Normal-looking material world** — Surface terrain, water, vegetation, and geology read as a natural world; the voxel grid is authoritative matter, not the intended look. Rendered surfaces and non-identity clutter are views of that truth and stay coherent under edit.
2. **Coherent world behavior** — Interactable matter supports burn/break/growth coherence, granular settle, wetness, and fire ecology. Time and seasons drive light, growth, and snow; weather drives wetness, water levels, ignition, and fire ecology. Weather and time are not required to drive breaking or granular settling. Depth beyond the first-proof slice is design; outcomes are not cancelled by that proof.
3. **Mutable everywhere, including deep Z** — Any material can be destroyed, moved, or placed through depth. Caves, strata, ore, and underground volume are first-class. Dig/place are first-class substrate verbs.
4. **Geology-first generation with on-demand cost** — Worlds are produced as geology that materializes matter on demand so large regions stay sparse until touched, and dig-down reveals true materials, voids, and aquifers. The first proof requires the full reusable geology-generation slice (columns, strata, caves, ore, lazy materialization, POI metadata) as a delivered substrate asset, not a stub.
5. **Matter services, public integration, and consumer extensibility** — The substrate provides matter, physics-relevant response, queries, and mutation. Interactable voxel-backed objects, fluid bodies and flow support, structural support and collapse, and navigation data derived from matter are substrate responsibilities; game policy is not. Nothing above the matter boundary touches voxels directly. Consumers—including the walkable-world proof—use the same public interfaces as an external game; no privileged paths. Consumers and optional System clients author world and material definitions through the same public extension surfaces as hand authoring. Collision and queries run against voxel truth, not the render mesh.
6. **Persistence, streaming, and standalone reuse** — Truth is worldgen plus edit deltas: repeatable seed-backed regeneration of untouched world plus exact restoration of saved edit deltas. Object and entity lifecycle is journaled per region so mutable matter objects are not lost across saves. Cross-mode reuse is authorized for edit deltas; further cross-mode reuse of journals needs additional authority. Active regions stream while cold regions stay cheap. Zero LLM dependency. The same crate stack supports ARPG, fortress, descent, or sandbox consumers without forking the world layer.
7. **Walkable first-proof outcome slice** — The required adjacent first proof must deliver a fixed outcome-level slice: a generated natural, continuous material world (surface through deep underground) with visible dig/place proof of fully mutable matter, backed by the full reusable geology-generation slice and matter/meshing/dressing outcomes that make that claim undeniable. Controller, character, demo content, route, presentation, and numeric gates stay consumer-owned; substrate outcome depth for that proof is not open-ended design allocation.

## Future products and enabling implications

Future consumers (not current product): a System-driven ARPG, a fortress/colony game, a Moria-style descent, and pure sandbox tools. They motivate substrate outcomes (mutable geology, deep Z, matter verbs, integrity and fluids, coherent ambient/matter behavior, POI/structure metadata, extensible definitions) but own gameplay, content, controllers, characters, presentation, and policy. The walkable-world first proof is an adjacent validation consumer, not a future game; its fixed outcome slice is bound above, while character, camera, demo content, route, and performance gates stay consumer-owned.

## Non-goals

- Shipping the actual game, or implementing System/LLM, spell, gas, combat, AI, or building gameplay layers here.
- Treating walkable-proof UX, demo content, scripted routes, or hardware benchmark targets as substrate requirements.
- Making the substrate depend on an LLM or on any one game’s economy, pricing, or rules.
- Redefining product identity as the walkable demo rather than the reusable substrate.

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates for library consumption.
- Consumer boundary is mandatory: adjacent artifacts and external games share the public interface; privileged in-repo paths are disallowed.
- A walkable-world executable is a required adjacent first proof; outside product identity; public APIs only.
- Substrate must stand alone without LLM dependency.
- GPU-resident world/matter operation is part of the product promise (detail deferred).
- Load-bearing substrate layers stay on wgpu/WGSL for portability; a native-Metal fork of those layers is prohibited.

## Deferred design decisions

- Precise crate/package split and workspace layout (boundary intent is fixed; structure is design).
- Voxel resolution, LOD, meshing strategy, storage encodings, and sim tier depth beyond the fixed first-proof slice.
- Sequencing of further long-horizon substrate outcomes after the first-proof slice.
- Discrete-GPU performance budgets and validation workloads (proof- and design-owned unless later bound).
- How far multiplayer-ready command authority is taken early.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident voxel-world substrate (Rust crate) and the walkable-world executable as a separate non-game validation consumer.
- **docs/seeds/project-boundary.md** — Binds product identity to the reusable Rust substrate, excludes the actual game and higher game layers, and requires public-interface-only harness consumption without making the harness optional.
- **docs/seeds/product-one-seed.md** — Fixes the walkable first proof’s outcome slice (natural continuous material world, dig/place proof, full geology-generation slice) and the wgpu/WGSL portability constraint without redefining product identity or importing demo content.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families: natural look over voxel truth, mutability and deep Z, geology generation, coherent matter behavior with separated ambient couplings, matter services, public layering, and seed-backed regeneration plus exact edit-delta restoration with delta-scoped cross-mode reuse.
