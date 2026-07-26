# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer world and matter foundation for games—not a game, demo experience, or content pack. Adjacent validation may exercise it through the same public interfaces an external game would use; that harness is not the product identity.

## Purpose

Games that need a natural-looking outdoor and underground world still want every visible surface to be real, diggable, placeable matter—not a heightmap skin with props. Moria exists so those games share one substrate for generation, mutable material, spatial queries, and mutation, while game rules, presentation, and policy stay outside the crate. The substrate must stand alone with **zero dependency on any LLM or “System” layer**.

## Product boundary

**In product:** the reusable world/matter substrate and its public integration surface for generation, occupancy and material truth, meshing-as-view, mutation, queries, streaming-oriented residency, and edit-aware persistence—implemented so external consumers have no privileged internal paths.

**Adjacent, not identity:** a walkable-world executable, if shipped, is only a **validation harness** for the substrate. It must call the same public APIs as a third-party game. Its controller, character, camera, route, debug presentation, authored seed content, and performance gate suite are harness concerns, not substrate product scope (see Q1).

**Downstream / out of repository:** the actual game(s). Game rules and the System, LLM, spell, gas, combat, AI, and building **game** layers are not implemented here. Compatibility seams may be designed where substrate outcomes require them; those layers are not delivered in Moria.

**Consumer-owned even when a harness uses them:** gameplay, UX, input schemes, characters, cameras, authored regions and POIs as content, HUD/debug chrome, and any game-specific pricing or policy over verbs.

## Required product outcomes

A competent design must make these true for the current product:

1. **Natural world, voxel truth.** Surface terrain can read as ordinary landscape (terrain, water bodies, vegetation and clutter consistent with materials) while all authoritative geometry and occupancy remain voxel matter—not decorative meshes outside the material world.
2. **Mutable everywhere, including deep Z.** Any material cell can be destroyed or placed through substrate verbs; underground volume is first-class content space (strata, voids, buried material), not a flat floor under a skybox.
3. **Geology-capable generation, on demand.** Worlds are produced as layered geology and structure that support honest digging, not a heightmap with rock painted underneath; large regions stay tractable via sparse residency and lazy materialization of untouched volume.
4. **Mesh is a view.** Rendering derives from matter and updates when matter changes; physics, queries, and gameplay-facing truth use the voxel world, not the render mesh as authority.
5. **Public mutation and query boundary.** Nothing above the matter core touches voxels ad hoc: consumers dig, place, and inspect only through the published verb/query surface—the same surface every external game and any in-repo harness must use.
6. **Residency and scars.** Active neighborhoods stream in and out; untouched volume stays cheap; player (or tool) edits persist as deltas over generation so reloads restore material change without treating the whole world as authored assets.

## Future products and enabling implications

Future **consumers** (not current Moria scope) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent adventure, and pure sandbox modes. They motivate, but do not import, gameplay, content, controllers, or presentation into this product.

**Enabling implications** the substrate is intended to support over time (not a committed feature roadmap in this brief): multi-game reuse of one matter stack; richer ambient and structural simulation (flow, fire, granular settle, integrity); object-scale vegetation that can later couple to rigid motion; and policy-pluggable verb pricing so different games share verbs without sharing rules. Long-horizon build, room, mechanism, and agent-labor fantasies remain consumer or later-layer work unless a future approved vision expands the substrate boundary.

## Non-goals

- Shipping the actual game, ARPG loop, fortress mode, or descent roguelike in this repository.
- Implementing System/LLM features, spells, gas economy, combat, AI, or building-game layers (UI, blueprints-as-gameplay, work orders, mechanism logic).
- Treating the validation harness’s character, camera, demo route, seed postcard, or benchmark theater as substrate requirements.
- Making the substrate depend on or embed an LLM.
- Full fluid dynamics, weather/season simulation, growth ecosystems, or multiplayer services as current-product promises (future enabling at most).

## Confirmed vision constraints

- **Rust crate ecosystem:** primary delivery is a Rust library surface (crate or small crate family) for game integration—not an ecosystem-neutral binary product.
- **Consumer isolation:** any in-repo harness or future game uses only public substrate interfaces; no privileged game-only paths inside the substrate.
- **GPU-resident world path with portable GPU stack:** load-bearing GPU work targets a portable `wgpu`/WGSL-style path (no native Metal fork in load-bearing layers); design must remain viable on Apple unified-memory GPUs used for development (including no reliance on 64-bit buffer atomics).
- **Standalone substrate:** zero LLM/System dependency; those attach only as external clients later.
- **Explicit exclusions** listed under Product boundary and Non-goals are binding now.

## Deferred design decisions

- Voxel scale, brick/layout parameters, meshing algorithm choice, and LOD/impostor strategy.
- Exact crate split, API shape, and how generation vs matter vs API packages are partitioned.
- How much of richer simulation (multi-tier fluids, CA, integrity, granular, object felling) lands in the first engineering slice versus later substrate increments.
- Persistence format details, streaming ring policy, and quantitative performance budgets (frame time, memory, dig-to-remesh latency) and the harness workloads that measure them.
- Material palette depth, object-layer capacity limits, and worldgen parameter authorship tools.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1. Is a walkable-world validation executable a required current delivery, or only permitted?**

- **Proposed safe answer:** **Permitted and recommended**, not a mandatory product deliverable. The current product promise is the substrate crate(s); a harness may exist to validate public APIs but is not required to ship for the substrate to be considered complete.
- **If answered differently:** Making the harness **mandatory** keeps substrate identity but adds a required adjacent delivery (still without absorbing its controller, content, or gates into product identity). Treating the **walkable demo itself as the product** would replace crate-substrate identity with a playable vertical slice and pull consumer-owned presentation into scope.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident voxel-world substrate consumed as a Rust crate and separates the walkable-world executable as harness, not game layer.
- **`docs/seeds/project-boundary.md`:** Binding product and repository boundary—substrate crates in, actual game out; harness-only executable; no privileged paths; game/System/building layers out of scope.
- **`docs/seeds/product-one-seed.md`:** First-slice and demo motivation for a walkable proof; supplies outcome pressure (natural region, dig proof, deep-Z run) and non-goals, without transferring harness content or acceptance theater into substrate identity.
- **`docs/seeds/voxel-world-substrate.md`:** Long-form substrate intent—natural look over mutable voxels, deep Z, generation, matter, and multi-game reuse—used for product outcomes and enabling implications, not as a mechanism or milestone catalog.
