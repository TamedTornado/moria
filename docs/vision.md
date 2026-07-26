# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation for games—not a game, demo title, or content product.

## Purpose

Downstream games need a shared material world: terrain that looks like ordinary landscape, is fully diggable and rebuildable as voxel matter, and remains continuous in three dimensions including deep underground. Moria exists so multiple games (sandbox, descent, fortress-style, adventure) can consume one substrate for generation, matter, mutation, queries, and physics-facing world truth—without embedding game rules, economy, or an LLM “System” in the crate.

## Product boundary

**Belongs to Moria**

- The reusable substrate and its public integration surface for external Rust consumers.
- World generation, sparse GPU-resident matter storage, derived views (e.g. meshes), streaming, edit-aware persistence, and matter-facing mutation/query capabilities required for that substrate role.
- Optional compatibility seams demanded by substrate requirements, without implementing game systems behind those seams.

**Adjacent / not this product**

- The **actual game** is a separate downstream consumer and is **not** part of this repository.
- A **walkable-world executable** may exist as an adjacent validation consumer of the substrate; whether that artifact is a required repository delivery is unresolved (see Q1). It is not a game layer. If present, it must use the same public interfaces available to an external game—no privileged or game-specific substrate paths.
- Controller, character, camera, authored demo route/content, presentation polish, harness workloads, and acceptance numbers belong to that adjacent consumer (or later games), not to substrate identity.
- Game rules and future **System / LLM, spell, gas, combat, AI, and building** layers are out of scope here.

## Required product outcomes

1. **Natural look, voxel truth** — A continuous overworld that reads as ordinary landscape (terrain, water bodies, vegetation-scale presence) while all authoritative occupancy and material state live in the voxel substrate; render meshes and dressing are views regenerated from matter, never the source of truth for physics or mutation.
2. **Mutable everywhere** — Any region of matter can be destroyed, eroded, placed, or otherwise changed through substrate verbs so dig/build-anywhere is real end-to-end, including deep underground—not decorative shells over a fixed heightfield.
3. **Deep Z and geology-first generation** — Underground is first-class content. Generation produces coherent geology (columns/strata, caves, ores, saturated bands, natural surface features) as pure, seedable evaluation so bricks can materialize lazily; large regions idle cheaply via sparsity and homogeneous empty/solid regions.
4. **GPU-resident sparse world + streaming & scars** — Active world state is organized for GPU-resident operation at scale; streaming keeps working sets around activity; persistence is worldgen (seed/function) plus compact deltas for touched matter and related object/entity change—not full raw dumps of untouched volume.
5. **Matter-coupled dressing and interactables** — Surface clutter that is only visual stays driven by matter; things players expect to block, break, or burn are matter-backed (including voxel-object style vegetation/props) so presentation cannot desync from dig and fire-style response.
6. **Public verbs, queries, and honest collision** — Nothing above the matter core mutates voxels by reaching into storage; consumers use shared mutation/query interfaces. Collision and traversal sample voxel occupancy, not the pretty mesh. Physical response foundations (fluid presence, granular/support failure, related matter rules) stay available as substrate capability so dig/build stays honest, without shipping game policy.

## Future products and enabling implications

Future **games** (System/LLM ARPG, fortress/colony, Moria-style descent, pure sandbox) are **downstream consumers outside this product**. They motivate reuse, pricing/policy plug-ins, and semantic layers above the substrate; they do not pull gameplay, UX, content, or AI into Moria.

Enabling implications (not a committed delivery catalog): richer ambient and matter simulation, fortress- or adventure-oriented semantic hooks above matter, and multiplayer-ready command/mirror discipline may be designed against the same public verb/query boundary. Depth and sequence are design choices.

## Non-goals

- Shipping the playable game, game rules, System/LLM integration, spells, gas economy, combat, AI, or building-game layers in this repository.
- Treating the walkable-world harness as a game, or absorbing its character controls, demo route, scenery checklist, or benchmark gates into substrate scope.
- Making substrate correctness depend on an LLM or game-layer policy object.
- Replacing later design with a fixed algorithm, crate-graph, or milestone schedule in this brief.

## Confirmed vision constraints

- **Identity form:** Rust crate or small family of tightly scoped Rust crates for external game consumption.
- **Residency:** GPU-resident voxel-world substrate (per product seeds).
- **Consumer equality:** Any in-repo validation executable, if present, consumes only public substrate interfaces—same class of access as an external game.
- **Independence:** Substrate stands alone with zero LLM/System dependency.
- **Exclusion:** Game rules and System, LLM, spell, gas, combat, AI, and building layers are not implemented here; seams only where substrate needs require them.

## Deferred design decisions

- Exact crate split, module boundaries, and workspace layout (boundary intent is fixed; packaging is not).
- Voxel size, meshing/LOD strategy, object-layer capacity, fluid-model fidelity, and which matter-sim depths ship in which increment.
- Streaming-ring layout, persistence encoding, and performance/platform targets.
- Whether and how multiplayer authority is pursued beyond architecture that does not preclude it.
- Contents of any walkable-world harness (controller, seed region, benchmarks)—after Q1 settles delivery obligation only.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** of this repository, or only a **permitted adjacent validation artifact**?

- **Proposed safe answer:** Permitted adjacent validation artifact only; if present, it must exercise the substrate solely through public interfaces and remains outside product identity.
- **If answered differently:** Making it required adds a repository delivery obligation (a public-interface harness exists and is maintained) without turning Moria into a game or importing harness controls, content, presentation, or performance gates into substrate scope.

## Seed synthesis

- **`README.md`** — Names Moria as the reusable GPU-resident voxel-world substrate consumed as a Rust crate, and separates the walkable-world executable as consumer/harness rather than game layer.
- **`docs/seeds/project-boundary.md`** — Locks current product identity to the substrate crate family, excludes the game and listed game layers from the repo, and requires any harness to use the same public interfaces as an external game.
- **`docs/seeds/product-one-seed.md`** — Describes an adjacent walkable proof/demo consumer and the substrate outcomes it must stress; supplies motivation and first-slice depth without redefining Moria as that demo or transferring its controller, content, or gates into product identity (delivery mandate open in Q1).
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes substrate outcome families: natural presentation over mutable voxel truth, deep-Z geology generation, sparse GPU world, streaming/delta persistence, matter-coupled dressing/objects, verb/query layering, and physics-facing honesty for future games—without importing game layers or mechanism inventory into this brief.
