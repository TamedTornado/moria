# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it as a library; Moria is the world-and-matter engine layer, not a game.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface over fully mutable voxel truth, continuous deep underground, and public mutation and query surfaces that keep game rules above the substrate. It must stand alone with no LLM or game-policy dependency, so an ARPG, fortress game, descent roguelike, or sandbox can each sit on the same crate stack.

## Product boundary

**In product**
- The reusable substrate: generation of material geology, sparse GPU-resident matter, presentation of that matter as a non-authoritative visual view, mutation and query APIs, matter-coupled simulation hooks games need (fluids, integrity-relevant behavior, interactable matter objects, derived navigation aids), and persistence/streaming of world truth as generation plus edit deltas.
- A clean public integration surface so any external game uses the same verbs, queries, and events the project itself would use.

**Adjacent / not product identity**
- A walkable-world executable may exist as a separate validation harness for terrain generation, streaming, meshing, editing, collision, persistence, and performance. It is not a game layer and must not define product identity. Whether shipping that harness is a required project delivery is open (see Q1).
- The actual game (or games) is a separate downstream consumer and is not part of this repository.

**Out of this product**
- Game rules; System/LLM features; spells; gas/pricing policy; combat; AI; and building/gameplay layers (UX, controllers, authored content, presentation policy, work orders, and game-specific modes).
- Compatibility seams may be designed where substrate outcomes require them; those higher layers are not implemented here.

## Required product outcomes

A downstream design must make these true of the substrate:

1. **Normal-looking material world** — Surface terrain, water, vegetation, and geology read as a natural world; the voxel grid is authoritative matter, not the intended look. Rendered surfaces and non-identity clutter are views of that truth and stay coherent under edit.
2. **Mutable everywhere, including deep Z** — Any material can be destroyed, moved, or placed through depth. Caves, strata, ore, and underground volume are first-class world content, not a floor under a skybox. Dig/place and placement are first-class substrate verbs.
3. **Geology-first generation with on-demand cost** — Worlds are produced as geology that materializes matter on demand so large regions stay sparse until touched, and dig-down reveals true materials, voids, and aquifers rather than painted underlay.
4. **Matter services for many games** — The substrate provides matter, physics-relevant response, queries, and mutation. Interactable voxel-backed objects, fluid bodies and flow support, structural support and collapse behavior, and navigation data derived from matter are substrate responsibilities at outcome level; game policy above them is not.
5. **Public-only integration** — Nothing above the matter boundary touches voxels directly. Consumers—including any in-repo harness—use the same public interfaces available to an external game; no privileged or game-specific substrate paths. Collision and queries run against voxel truth, not the render mesh.
6. **Persistence, streaming, and standalone reuse** — Truth is worldgen plus edit deltas; active regions stream while cold regions stay cheap. Zero LLM dependency. The same crate stack is intended to support a System ARPG, fortress/colony game, Moria-style descent, or pure sandbox without forking the world layer.

## Future products and enabling implications

Future consumers (not current product): a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent, and pure sandbox tools. They motivate substrate outcomes (mutable geology, deep Z, matter verbs, integrity and fluids hooks, metadata for POIs and structures) but own their gameplay, content, controllers, characters, presentation, and policy.

A walkable-world demo or Product One–style proof may exercise the substrate through public APIs; its character, camera, demo route, seed world content, and performance gates are consumer-owned and are not product scope. See Q1 for whether that harness is a required project delivery.

## Non-goals

- Shipping the actual game, or implementing System/LLM, spell, gas, combat, AI, or building gameplay layers in this product.
- Treating validation-harness UX, demo content, scripted routes, or hardware benchmark targets as substrate requirements.
- Making the substrate depend on an LLM or on any one game’s economy, pricing, or rules.
- Redefining product identity as the walkable demo rather than the reusable crate substrate.

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates for library consumption.
- Consumer boundary is mandatory: adjacent artifacts and external games share the public interface; privileged in-repo paths are disallowed.
- Substrate must stand alone without LLM dependency.
- GPU-resident world/matter operation is part of the product promise (implementation detail deferred).

## Deferred design decisions

- Precise crate/package split and workspace layout (boundary intent is fixed; structure is design).
- Voxel resolution, LOD, meshing strategy, storage encodings, and sim tier depth/sequence.
- Which substrate outcome depths ship in which milestone; first-slice vs full long-horizon capability.
- Platform backends, performance budgets, and validation workloads (harness- and design-owned unless later bound).
- How far multiplayer-ready command authority is taken in early delivery.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation executable a **required project delivery** alongside the substrate crates, or only a **permitted adjacent artifact**?

- **Proposed safe answer:** Permitted adjacent artifact only—project success is defined by the reusable substrate crates and their public API; a harness may exist to validate them but is not mandatory delivery.
- **If answered “required”:** Product identity stays the substrate, but the project must also ship a harness that consumes only public interfaces and exercises generation, streaming, meshing, editing, collision, persistence, and performance. Harness-owned controllers, content, presentation, and acceptance numbers still do not enter product scope.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident voxel-world substrate (Rust crate) and positions the walkable-world executable as a non-game validation consumer.
- **docs/seeds/project-boundary.md** — Binds current product identity to the reusable Rust substrate, excludes the actual game and higher game layers from this repo, and requires public-interface-only consumption for any harness.
- **docs/seeds/product-one-seed.md** — Describes a first walkable proof consumer and motivates why mutability, meshing, geology, and streaming must be undeniable; does not redefine product identity or import demo content into substrate scope.
- **docs/seeds/voxel-world-substrate.md** — Authorizes the substrate outcome families (natural look over voxel truth, full mutability, deep Z, geology generation, matter services, API layering, persistence/streaming, multi-game reuse) at vision altitude without carrying mechanism inventory into this brief.
