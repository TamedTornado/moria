# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it as a library; Moria is the world-and-matter engine layer, not a game.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface over fully mutable voxel truth, continuous deep underground, and public mutation, query, and extension surfaces that keep game rules above the substrate. The material world must behave as a coherent world—not only look like one. It must stand alone with no LLM or game-policy dependency, so an ARPG, fortress game, descent roguelike, or sandbox can each sit on the same crate stack.

## Product boundary

**In product**
- The reusable substrate: geology-first generation, sparse GPU-resident matter, non-authoritative visual presentation of that matter, mutation and query APIs, public surfaces for consumers to author applicable world and material definitions, matter-coupled world behavior games need, derived navigation aids, and persistence/streaming of substrate-owned world truth (terrain edits and object lifecycle).
- A clean public integration surface so any external game uses the same verbs, queries, events, and extension surfaces that hand authoring and any in-repo consumer use.

**Adjacent / not product identity**
- A walkable-world executable is a **required adjacent first proof** of the substrate. It validates generation, streaming, meshing, editing, collision, persistence, and performance, and is the project’s first downloadable/benchmark deliverable. It is not a game layer and does not define product identity. It must consume the substrate only through public interfaces. Character, controller, content, presentation, demo route, workloads, platforms, and numeric gates remain consumer-owned.
- The actual game (or games) is a separate downstream consumer and is not part of this repository.

**Out of this product**
- Game rules; System/LLM features; spells; gas/pricing policy; combat; AI; and building/gameplay layers (UX, controllers, authored content, presentation policy, work orders, and game-specific modes).
- Compatibility seams may be designed where substrate outcomes require them; those higher layers are not implemented here.

## Required product outcomes

A downstream design must make these true of the substrate:

1. **Normal-looking material world** — Surface terrain, water, vegetation, and geology read as a natural world; the voxel grid is authoritative matter, not the intended look. Rendered surfaces and non-identity clutter are views of that truth and stay coherent under edit.
2. **Coherent world behavior** — The material world behaves, rather than merely looks, like a coherent world. Substrate matter outcomes include burn/break/growth coherence for interactable matter, granular settle behavior, wetness and fire ecology, and weather/time-driven ambient effects that drive those rules. Delivery depth and sequence are design choices; the outcomes are not cancelled by a first-slice proof.
3. **Mutable everywhere, including deep Z** — Any material can be destroyed, moved, or placed through depth. Caves, strata, ore, and underground volume are first-class world content. Dig/place are first-class substrate verbs.
4. **Geology-first generation with on-demand cost** — Worlds are produced as geology that materializes matter on demand so large regions stay sparse until touched, and dig-down reveals true materials, voids, and aquifers rather than painted underlay.
5. **Matter services, public integration, and consumer extensibility** — The substrate provides matter, physics-relevant response, queries, and mutation. Interactable voxel-backed objects, fluid bodies and flow support, structural support and collapse behavior, and navigation data derived from matter are substrate responsibilities at outcome level; game policy above them is not. Nothing above the matter boundary touches voxels directly. Consumers—including the walkable-world proof—use the same public interfaces available to an external game; no privileged paths. Consumers and optional System clients author applicable world and material definitions through the same public extension surfaces as hand authoring. Collision and queries run against voxel truth, not the render mesh.
6. **Persistence, streaming, and standalone reuse** — Truth is worldgen plus edit deltas. Substrate-owned object lifecycle (moved/felled objects and their state) is journaled so mutable matter objects are not lost across saves. Saved deltas and journals support reuse across runs or modes. Active regions stream while cold regions stay cheap. Zero LLM dependency. The same crate stack is intended to support a System ARPG, fortress/colony game, Moria-style descent, or pure sandbox without forking the world layer.

## Future products and enabling implications

Future consumers (not current product): a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent, and pure sandbox tools. They motivate substrate outcomes (mutable geology, deep Z, matter verbs, integrity and fluids, coherent ambient/matter behavior, metadata for POIs and structures, extensible definitions) but own their gameplay, content, controllers, characters, presentation, and policy.

The walkable-world first proof is an adjacent validation consumer of the substrate, not a future game. Its purpose is to make substrate claims undeniable through a walkable, downloadable, benchmarkable artifact; its character, camera, demo content, route, and performance gates stay consumer-owned.

## Non-goals

- Shipping the actual game, or implementing System/LLM, spell, gas, combat, AI, or building gameplay layers in this product.
- Treating walkable-proof UX, demo content, scripted routes, or hardware benchmark targets as substrate requirements or product identity.
- Making the substrate depend on an LLM or on any one game’s economy, pricing, or rules.
- Redefining product identity as the walkable demo rather than the reusable crate substrate.

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates for library consumption.
- Consumer boundary is mandatory: adjacent artifacts and external games share the public interface; privileged in-repo paths are disallowed.
- A walkable-world executable is a required adjacent first proof; it remains outside product identity and integrates only via public APIs.
- Substrate must stand alone without LLM dependency.
- GPU-resident world/matter operation is part of the product promise (implementation detail deferred).

## Deferred design decisions

- Precise crate/package split and workspace layout (boundary intent is fixed; structure is design).
- Voxel resolution, LOD, meshing strategy, storage encodings, and sim tier depth/sequence for coherent-behavior outcomes.
- Which substrate outcome depths ship in which milestone; first-slice proof vs full long-horizon capability.
- Platform backends, performance budgets, and validation workloads (proof- and design-owned unless later bound).
- How far multiplayer-ready command authority is taken in early delivery.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident voxel-world substrate (Rust crate) and identifies the walkable-world executable as a separate non-game validation consumer of that substrate.
- **docs/seeds/project-boundary.md** — Binds current product identity to the reusable Rust substrate, excludes the actual game and higher game layers from this repo, and requires public-interface-only consumption for the walkable-world harness without making that harness optional.
- **docs/seeds/product-one-seed.md** — Establishes the walkable-world executable as the first proof built alongside the substrate crates (including benchmark deliverable and downloadable demo) and motivates why mutability, meshing, geology, and streaming must be undeniable; does not redefine product identity or import demo content into substrate scope.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families at vision altitude: natural look over voxel truth, full mutability and deep Z, geology generation, coherent matter behavior (including burn/break/growth, granular, wetness, fire ecology, weather/time), matter services, public layering and consumer-authored definitions, and persistence/streaming with object journals and cross-run reuse—without carrying mechanism inventory into this brief.
