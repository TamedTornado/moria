# Project vision

## What we are building now

**Moria** is a reusable, **GPU-resident voxel-world substrate** delivered as a **Rust crate** (or a small family of tightly scoped Rust crates). It is the material-world foundation other products consume—not a game, not a character demo, and not an LLM runtime.

## Purpose

Games and tools need a world that *reads as ordinary nature* (terrain, forest, water, cliffs, caves) while remaining *fully material and mutable* all the way down. Moria exists so those consumers share one engine layer for matter, generation, mutation, queries, and presentation-as-view—without baking game rules, economy policy, or content authorship into the world stack. The substrate must stand alone with **zero System/LLM dependency**.

## Product boundary

**Belongs to Moria**
- The reusable voxel-world substrate and its public consumer-facing surface (verbs, queries, events, registries the seeds treat as engine API).
- World matter, geology-backed generation, sparse residency/streaming, non-authoritative visual extraction of voxel truth, and seed-plus-delta persistence that make a continuous 3D mutable world usable at overworld scale.
- Optional **compatibility seams** where substrate requirements demand them—seams only, not game systems.

**Adjacent, not identity**
- A **walkable-world executable**, if present, is only a **validation harness** and must use the **same public interfaces** as an external game. No privileged or game-specific substrate paths.
- The real game(s) live in **other repositories / downstream products**.

**Does not belong to Moria**
- Game rules; System/LLM; spells; gas/pricing policy; combat; AI; building/colony gameplay layers; authored campaign content; game UX, cameras, and character controllers.

## Required product outcomes

A downstream design must make these product-level guarantees true:

1. **Natural world, voxel truth** — Consumers can present a continuous, natural-looking surface world (hills, vegetation, water, rock, underground volume) whose render is a *view* of underlying mutable voxels, not a heightmap with detached props. Mutation remains coherent with what the player sees (e.g. cuts read as cut matter).
2. **Mutable everywhere, deep Z first-class** — Any material cell in the playable volume can be destroyed, altered, or placed through the public mutation path; underground volume (caves, strata, buried materials) is real content in continuous 3D, not a decorative floor.
3. **Geology-first generation with cheap idle worlds** — Worlds are produced as geology and related world structure, materializable **lazily** so untouched volume stays cheap; sparsity/homogeneous empty-or-solid regions are load-bearing, not a later optimization story.
4. **Public matter boundary** — Nothing above the substrate touches voxels directly. Adjacent consumers (including any harness) share one public command/query/event surface; there is no privileged in-tree game path.
5. **Interactive-scale residency** — The substrate is GPU-resident and streamable so a large region need not live as dense raw voxels in memory, while remaining responsive to local mutation (dirty-region remesh/view update as a quality promise, not a numeric SLA in this brief).
6. **Durable scars** — Truth is **worldgen function + edit deltas** (plus whatever object/event journal the design assigns to the substrate). Reload restores the same material world without saving untouched volume as full grids.
7. **Matter-linked surface life (substrate slice)** — Interactable world elements the seeds assign to the engine (e.g. voxel-backed objects such as trees/rocks, and dressing driven by voxel/surface state) participate as matter-linked content at least for placement and truthful presentation; full destruction/rigid coupling is not required for current identity. **Static water bodies** are in; flowing/fluid simulation beyond that is not a current mandate.

## Future products and enabling implications

**Future consumers** (not this product): a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent/adventure mode, and pure sandbox tools. Those own gameplay, UX, content, controllers, presentation policy, and acceptance scenarios.

**Enabling implications for the substrate** (high-level responsibilities the long-horizon seed assigns to the *engine*, not current delivery commitments or consumer features): richer matter simulation (fire/wetness-class cellular rules, multi-tier fluids, granular settle, structural integrity), fuller placement/stamp/blueprint *engine* verbs and mechanism-shaped entities, derived navigation aggregates, and ambient weather/time hooks. Design may stage these; this vision does not schedule them. Excluded **game** building/combat/System layers stay excluded even when a future title needs them.

## Non-goals

- Shipping “the game,” ARPG loop, fortress mode, or System/LLM inside this repository.
- Treating harness character controls, cameras, demo routes, seed postcard content, or benchmark theater as substrate identity.
- Implementing gas policy, combat, AI, building UI/work orders, or economy.
- Making decorative non-matter geometry authoritative for physics or mutation.
- Requiring an embedded scripting language in the first identity of the product (public engine API is enough).

## Confirmed vision constraints

- **Ecosystem**: Rust crate(s); cargo-level separation between reusable substrate and any validation harness; harness is a peer consumer of the public surface.
- **Portability**: load-bearing GPU work stays on **wgpu/WGSL**-class portability—not a native Metal-only fork in core paths.
- **Independence**: substrate has zero LLM/System runtime dependency.
- **Consumer equality**: no privileged access for in-repo tools versus external games.
- **Quality shape**: large-world sparsity/streaming and mutation-coherent remeshing are product-defining; exact fps/memory thresholds and hardware scoreboards are design/acceptance detail.

## Deferred design decisions

- Precise crate graph inside the substrate family and internal module boundaries.
- Voxel size, LOD/impostor strategy, and object-registry capacity strategy.
- How much of the generation pipeline ships before continent/climate stubs give way to full passes.
- Depth and order of later matter simulations (CA, flowing fluids, integrity, granular, felling/rigid coupling).
- Numeric performance budgets, benchmark workloads, and platform baselining.
- Whether and how multiplayer authority is pursued beyond keeping the command/mirror shape ready.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world **validation harness** a **required deliverable** of this repository now, or only a **permitted** adjacent consumer?

- **Proposed safe answer:** **Permitted** (and useful), not mandatory for product identity. If built, it must exercise the substrate only through public interfaces; its controller, character, demo route, content set, presentation, and performance gates stay harness-owned and out of substrate scope.
- **If answered differently:** Making the harness **mandatory** keeps substrate identity unchanged but adds an adjacent delivery commitment (a public-API validation executable must ship). It still must not import game-layer scope into Moria.

## Seed synthesis

- **`README.md`** — Names Moria as the GPU-resident Rust-crate substrate and frames the walkable-world executable as separate harness, not game layer.
- **`docs/seeds/project-boundary.md`** — Binding product identity, Rust/crate consumer boundary, public-interface rule for any harness, and exclusion of game/System/building layers.
- **`docs/seeds/product-one-seed.md`** — First-slice proof points (natural mutable world, dig/place honesty, geology, dressing/objects, static water, API discipline) and platform/portability notes; its character demo, postcard content, milestones, and numeric gates inform harness/validation aspiration, not substrate identity.
- **`docs/seeds/voxel-world-substrate.md`** — Long-horizon substrate responsibilities (matter world, generation, mutation, deep Z, reuse across game genres) and future enabling sims; game-facing layers remain consumers.
