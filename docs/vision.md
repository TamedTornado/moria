# Project vision

## What we are building now

**Moria** is a reusable, **GPU-resident voxel-world substrate**, delivered as a **Rust crate** (or a small family of tightly scoped Rust crates). It is a material-world foundation for downstream games: geology-backed generation, sparse mutable voxels through deep Z, meshing and dressing as views of that matter, and public mutation/query surfaces. It is **not** a game, demo identity, or gameplay stack.

## Purpose

Give multiple future games one trustworthy world engine so a continuous volume of matter can read as an ordinary natural landscape, stay fully diggable and placeable, and keep simulation and collision on voxel truth rather than decorative geometry. Game rules, economies, AI, and presentation live above the substrate. The substrate stands alone with **no LLM/System dependency**.

## Product boundary

**In product**

- The reusable substrate: world generation as geology, sparse GPU-resident matter, smooth surface extraction as a non-authoritative view, vegetation/object matter hooks the substrate owns, static fluid bodies as matter, dig/place and related public verbs/queries, streaming, and persistence as worldgen-plus-edit-deltas.
- A clear integration boundary so external games consume only public crate interfaces.

**Adjacent (not product identity)**

- A **walkable-world executable**, if present, is only a **validation harness** for substrate capabilities (generation, streaming, meshing, editing, collision, persistence, performance). It must use the same public interfaces as an external game and must not own privileged or game-specific engine paths.
- Controllers, cameras, characters, authored demo routes, seed-world content lists, presentation polish, and harness acceptance scenarios are **consumer-owned**.

**Out of this product and repository**

- The actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and **building layers** (gameplay, UX, blueprints-as-work-orders, mechanisms-as-game-entities, room/economy policy).
- Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

1. **Material world, normal look** — Consumers can present rolling terrain, forests, water, cliffs, and caves whose look is driven by voxel truth (smooth extracted surfaces and matter-anchored dressing), not a heightmap with non-material props as the world model.
2. **Mutable everywhere; deep Z first-class** — Voxels can be destroyed or placed throughout the volume; underground is real content (strata, caves, ores, voids) in continuous 3D, not a skybox floor.
3. **Truth vs view** — Physics, collision, and queries run against voxel occupancy/matter; meshes are regenerated views—never authoritative and never the save format.
4. **Public verb/query boundary** — Dig, place, and mirror-style queries exist so nothing above the substrate touches voxels directly; the same surface is what games and any harness must use.
5. **Sparse residency, streaming, persistence** — Large regions stay tractable via lazy materialization and sparse storage; persisted truth is generation parameters/seed plus edit deltas, with streaming around active anchors.
6. **Reusable Rust integration** — Shipped for consumption as Rust crate(s), portable across external game projects, with zero required coupling to LLM/System features.

## Future products and enabling implications

Described **future consumers** (separate products, not this repo’s identity): a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent/adventure game, and pure sandboxes. They motivate a substrate that can later host richer matter behavior (flowing fluids, fire, granular settle, structural integrity, interactable vegetation objects, building-support queries) and semantic seams—without implementing those games, their UX, or excluded building/System layers here. A product-shaped walkable demo remains a **consumer/harness path** that proves the substrate; its route, character, and gates are not substrate scope.

## Non-goals

- Shipping the actual game, game rules, or excluded layers (System/LLM, spells, gas, combat, AI, building gameplay) in this repository.
- Treating the walkable harness as the product or granting it private engine paths.
- Committing long-horizon matter systems, multiplayer, or full fortress/ARPG feature sets as current product identity merely because broad design notes describe them.
- Importing harness hardware targets, controllers, seed content inventories, or benchmark scene scripts as substrate requirements.

## Confirmed vision constraints

- **Identity:** reusable voxel-world substrate as Rust crate(s); the game is a separate downstream consumer.
- **GPU-resident** matter/world foundation as part of the product promise.
- **Consumer boundary is mandatory:** no privileged harness or in-repo game path around the public API.
- **Standalone substrate:** no LLM/System dependency inside the product.
- **Excluded layers** listed under Product boundary are not implemented here (seams only where required).

## Deferred design decisions

- Exact crate split and workspace layout (enforce the consumer boundary; structure is design’s choice).
- Voxel resolution, LOD, object-layer capacity, meshing algorithm details, and related measurement tradeoffs.
- Which substrate matter subsystems ship in which delivery slice (beyond the outcome mandates above).
- Persistence encoding, streaming-ring policy, performance budgets, and benchmark environments.
- Depth of later-enabling APIs (fluids beyond static bodies, integrity, fire, building-support queries) once identity and boundary are fixed.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** **Required as a harness delivery** (repository ships a walkable validation consumer), but it stays **outside product identity**—no transfer of its character, controls, content, presentation, platform, or acceptance gates into the substrate.
- **If different:** If only permitted, shipping any walkable executable is optional and must not be treated as a repository commitment in planning.

## Seed synthesis

- **`README.md`** — Names Moria as the GPU-resident Rust voxel substrate and frames the walkable executable as a separate validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binds current product to reusable Rust crate(s), mandatory public-API consumer boundary, optional harness-only executable, and explicit exclusion of game/System/building layers from this repo.
- **`docs/seeds/product-one-seed.md`** — Motivates first validation pressure (material walkable world, dig/place proof, gen/mesh/stream/persist) and harness-shaped demo scope; its controllers, seed content, milestones, and machine gates do not define substrate identity.
- **`docs/seeds/voxel-world-substrate.md`** — Supplies substrate design goals (normal look, mutability, deep Z, reusable matter/physics/query foundation, standalone of LLM) and long-horizon enabling shape for future games without making those games or full system inventory current scope.
