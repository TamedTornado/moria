# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation for games—not a game, demo title, or content product. A walkable-world executable is a **required adjacent delivery** that validates the substrate through public interfaces; it is not part of substrate identity.

## Purpose

Downstream games need a shared material world: terrain that reads as ordinary landscape, is fully diggable and rebuildable as voxel matter, and remains continuous in three dimensions including deep underground. Moria exists so multiple games (sandbox, descent, fortress-style, adventure) can consume one substrate for generation, matter, mutation, queries, persistence, physics-facing world truth, and related world simulation—without embedding game rules, economy, or an LLM System in the crate.

## Product boundary

**Belongs to Moria (the substrate)**

- The reusable substrate, its public integration surface for external Rust consumers, and compatibility seams demanded by substrate requirements (without implementing game systems behind those seams).
- World generation, sparse GPU-resident matter, derived views (e.g. meshes), streaming, edit-aware persistence with exact restoration, matter-facing mutation/query capabilities, matter simulation families, and mutation-safe spatial derivation.

**Adjacent / not this product**

- The **actual game** is a separate downstream consumer and is **not** part of this repository. Game rules and future **System / LLM, spell, gas, combat, AI, and building** layers are out of scope here.
- The **walkable-world executable** is a **required adjacent Product One delivery**: a validation consumer that proves generation, streaming, meshing, editing, collision, persistence, and performance through the same public interfaces available to an external game—no privileged or game-specific substrate paths. It is not a game layer and does not define substrate identity.
- Controller, character, camera, authored demo route/content, presentation polish, harness workloads, machine-specific benchmark gates, and acceptance numbers belong to that adjacent consumer (or later games), not to the substrate.

## Required product outcomes

1. **Natural look, voxel truth** — A continuous overworld that reads as ordinary landscape (terrain, water bodies, vegetation-scale presence) while authoritative occupancy and material state live in the voxel substrate; render meshes and dressing are views regenerated from matter, never the source of truth for physics or mutation.
2. **Mutable everywhere** — Any region of matter can be destroyed, eroded, placed, or otherwise changed through substrate verbs so dig/build-anywhere is real end-to-end, including deep underground—not decorative shells over a fixed heightfield.
3. **Deep Z and geology-first generation** — Underground is first-class content. Generation produces coherent geology (columns/strata, caves, ores, saturated bands, natural surface features) as pure, seedable evaluation so matter can materialize lazily; large regions idle cheaply via sparsity and homogeneous empty/solid regions.
4. **GPU-resident sparse world + streaming & exact restore** — Active world state is organized for GPU-resident operation at scale; streaming keeps working sets around activity. Persistent truth is world generation (seed/function) plus edit deltas and related object/entity journals for touched matter—not full raw dumps of untouched volume. Reloading the same seed plus deltas must restore the world exactly.
5. **Matter-coupled dressing, objects, and lifecycle** — Surface clutter that is only visual stays driven by matter; things consumers expect to block, break, burn, or move are matter-backed (including voxel-object style vegetation/props) so presentation cannot desync from dig and matter response. The substrate provides matter movement and object lifecycle behavior (placement through physical response and re-integration with the world).
6. **Public authority boundary and honest world response** — Consumers mutate and observe only through shared command, query, and event interfaces: commands in, a stale mirror and events out; nothing above the matter core reaches into voxel storage. Collision and traversal sample voxel occupancy, not the mesh. The substrate provides active fluid behavior, ambient time/weather and fire ecology, structural/granular failure, and mutation-safe spatial derivation (navigation data from voxels, invalidated after mutation, supporting distinct 3D movement classes)—without shipping game policy.

## Future products and enabling implications

Future **games** (System/LLM ARPG, fortress/colony, Moria-style descent, pure sandbox) are **downstream consumers outside this product**. They motivate reuse, pricing/policy plug-ins, and semantic layers above the substrate; they do not pull gameplay, UX, content, or AI into Moria.

The walkable-world harness is the first **adjacent validation consumer**, not a future product and not a narrowing of substrate identity. Product One’s first slice may omit some matter-simulation depths while proving generation, meshing, dig/place, streaming, and persistence; those omissions do not remove the broader outcome families above. Delivery sequence and mechanism depth remain design choices.

## Non-goals

- Shipping the playable game, game rules, System/LLM integration, spells, gas economy, combat, AI, or building-game layers in this repository.
- Treating the walkable-world harness as a game, or absorbing its character controls, demo route, scenery checklist, or machine-specific benchmark gates into substrate scope.
- Making substrate correctness depend on an LLM or game-layer policy object.
- Replacing later design with a fixed algorithm, crate-graph, or milestone schedule in this brief.

## Confirmed vision constraints

- **Identity & residency:** Rust crate (or small family of tightly scoped Rust crates); GPU-resident voxel-world substrate for external game consumption.
- **Consumer equality:** The required walkable-world validation executable consumes only public substrate interfaces—same class of access as an external game.
- **Authority / observability:** Consumers couple through commands in and a stale mirror plus events out; no direct voxel access from above the matter core.
- **Portable GPU backend:** Load-bearing layers stay on wgpu/WGSL; no native-Metal fork. Portability across Vulkan/DX12-class backends is part of the crate’s purpose. Machine-specific limits and benchmark gates are not product constraints.
- **Independence:** Substrate stands alone with zero LLM/System dependency.
- **Exclusion:** Game rules and System, LLM, spell, gas, combat, AI, and building layers are not implemented here; seams only where substrate needs require them.

## Deferred design decisions

- Exact crate split, module boundaries, and workspace layout (boundary intent is fixed; packaging is not).
- Voxel size, meshing/LOD strategy, object-layer capacity, and fluid/integrity/ambient simulation fidelity and which depths ship in which increment.
- Streaming-ring layout, persistence encoding details, and performance targets (including machine-specific harness gates).
- Whether and how multiplayer authority is pursued beyond architecture that does not preclude it.
- Contents of the walkable-world harness (controller, seed region, presentation, workloads)—not whether the harness exists.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **`README.md`** — Names Moria as the reusable GPU-resident voxel-world substrate consumed as a Rust crate, and identifies the walkable-world executable as the separate validation consumer for generation, streaming, meshing, editing, collision, persistence, and performance.
- **`docs/seeds/project-boundary.md`** — Locks current product identity to the substrate crate family, excludes the game and listed game layers from the repo, and requires any harness to use the same public interfaces as an external game.
- **`docs/seeds/product-one-seed.md`** — Pins the first adjacent walkable proof as a required delivery stressing the substrate through public interfaces; defines first-slice depth (exact seed-plus-delta restore, wgpu/WGSL portability) without redefining Moria as that demo or importing harness controls, content, or machine gates.
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes full substrate outcome families: natural presentation over mutable voxel truth, deep-Z geology generation, sparse GPU world, streaming and generation-plus-delta persistence, matter-coupled dressing/objects and lifecycle, command/mirror/event consumer coupling, active fluids, ambient time/weather/fire ecology, structural/granular failure, and mutation-safe spatial derivation—without importing game layers or mechanism inventory into this brief.
