# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it as library code. It is an engine-layer world substrate—not a game, demo product identity, or content package.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface world on fully mutable voxel truth under a universal material-representation boundary, with deep underground as first-class space, geology that digs honestly and regenerates deterministically from seed and coordinates, and public mutation and query surfaces. The substrate stands alone with **no LLM or System dependency**. Game rules and harness UX live above it; substrate-owned visual-world outcomes remain part of the product.

## Product boundary

**In product:** the reusable substrate—deterministic geology-first generation; GPU-resident matter; the universal material-representation boundary (burn/break/block matter is voxel-backed; non-interactive dressing stays anchored to voxel truth); meshing and material-derived presentation as non-authoritative views of that truth; dig/place and related matter mutation; matter behavior (fluids, granular, integrity, ambient fire/weather/time responses, interactive voxel-backed objects); physics and spatial queries against voxels; mutation-aware navigation foundations; streaming and sparsity; persistence of reproducible generation truth plus edit deltas and object/entity journals; and the public integration surface (commands in, stale mirror plus events out, object model).

**Out of product:** the actual game and this repository’s game layer; game rules; System/LLM features; spells; gas policy; combat; AI (agents, labor, game behavior); and building *game* layers (blueprints-as-gameplay, work orders, mechanisms-as-game systems, fortress/ARPG policy). Compatibility seams may exist where substrate needs demand them; those layers are not implemented here.

**Adjacent, not identity:** a walkable-world executable *may* exist as a validation harness for generation, streaming, meshing, editing, collision, persistence, and performance. If present, it must use the **same public interfaces** as an external game—no privileged or game-specific paths. Harness-owned character, camera, controllers, authored demo route/content, game/harness UX and presentation, scripted workloads, machine profiles, and performance gates are not product scope. Whether that harness is a **mandatory current delivery** or only **permitted** is unresolved (see Q1); until then it is neither required nor ruled out in this brief.

## Required product outcomes

1. **Rust-consumable substrate.** Downstream integrators use Moria as crate(s), not as an embedded game binary or privileged in-repo fork.
2. **Natural look, voxel truth, and universal material representation.** Worlds read as continuous terrain (hills, forests, water, cliffs, caves), not a block aesthetic. Everything capable of burning, breaking, or blocking is voxel-backed; non-interactive dressing is not independent matter—it remains anchored to voxel truth and stays mutation-synchronized. Smooth material-derived rendering is a substrate outcome; the mesh is a regenerated view. Physics, collision, and gameplay-facing queries run against voxel matter.
3. **Mutable material world and living matter.** Matter can be destroyed, placed, and edited through the public API. Interactive voxel-backed objects, granular behavior, fluids, ambient time/weather/fire behavior, and material-dependent structural failure belong to the substrate so the material world reacts consistently to damage, support, water, fire, weather, and other matter changes. Omitting some behaviors from a first validation slice does not demote them from product outcomes.
4. **Deterministic geology-first generation and deep Z.** Worlds are generated as geology (strata, channels, caves, underground features). Base-world generation is a pure function of coordinates and world seed: the same seed and coordinates always yield the same base matter, so regions can materialize independently and lazily while large extents idle cheaply. The underground is content, not a false floor.
5. **Scale, streaming, and full persistence.** Streaming and sparsity keep large regions tractable. Persistence truth is that reproducible base generation plus edit deltas, *and* journals for moved voxel objects and entity/script state; reloading the same seed with the same deltas restores the same world—not terrain scars alone, and not generation that may drift between runs.
6. **Clean consumer coupling and navigation foundations.** Consumers issue commands and observe via a mirror that may be stale, plus events; the public surface also owns the object model. Nothing above the matter surface holds private authoritative voxel paths. The substrate provides mutation-aware, voxel-derived navigation data and queries for continuous-3D traversal classes; agents, labor, and game AI stay consumer-owned.

## Future products and enabling implications

Downstream **consumers** (not this product) include a System-driven ARPG, a DF-style fortress/colony game, a Moria-style descent experience, and pure sandboxes. Enabling implications: shared diggable natural worlds, deep-Z play, living matter reactions, and a common command/mirror/event surface. Their gameplay, UX, controllers, characters, content, and policy are **not** current-product scope.

Product One’s “walkable world” is an adjacent **demo/validation shape**—not a redefinition of Moria’s identity. First-slice depth is design/delivery, not a narrower product identity.

## Non-goals

- Shipping game systems: combat, stats, AI, System/LLM, spells, gas economy, weather/season *game* loops, building UI and fortress/ARPG policy.
- Treating harness character control, camera, demo seed content, or benchmark numbers as substrate product requirements.
- Excluding substrate-owned material-derived rendering, synchronized dressing, matter simulation, navigation data, or object journals because a first harness slice sits above them.
- Implementing excluded upper layers “for later” inside this product under another name.

## Confirmed vision constraints

- **Ecosystem:** product surface is Rust crate(s) for library consumption.
- **Residency:** the world substrate is GPU-resident in intent.
- **Coupling:** public integration is commands in and a potentially stale mirror plus events out.
- **Interface equality:** any in-repo validation consumer uses only public substrate interfaces available to an external game.
- **No LLM dependency:** substrate operation does not require System/LLM features.
- **Repository boundary:** the actual game is a separate downstream consumer, not part of this product’s identity.

## Deferred design decisions

- Exact crate split, APIs, algorithms, data layouts, voxel size, LOD, object-layer scaling, and persistence encodings.
- Depth and sequence of matter-sim families versus any first validation slice—not whether those families are substrate responsibilities.
- Harness-only choices: controllers, demo content, platforms, performance thresholds, and benchmarks—if a harness is delivered.
- Implementation stack details unless later bound as product promise.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **mandatory current delivery** alongside the substrate, or only a **permitted** adjacent validation artifact?

- **Proposed safe answer:** Permitted only. Ship identity stays the substrate crate(s); a harness may exist and must use public APIs, but is not required for the product to be itself.
- **If different:** Making it mandatory keeps substrate identity but adds a required adjacent deliverable (without importing its controls, content, or performance gates). Ruling it out entirely removes even permission to house a same-interface harness here.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident voxel-world substrate Rust crate and positions the walkable-world executable as a separate harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Fixes current identity (substrate crate(s)), excludes the game and upper layers, and requires public-interface consumption for any harness.
- **`docs/seeds/product-one-seed.md`:** Adjacent walkable-world validation/demo slice; reinforces reload of the same seed plus deltas; supplies motivation without reassigning harness content or device targets to product identity.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes substrate outcome families: natural look over voxel truth, the universal voxel-backed vs dressing boundary, full mutability, living matter, deep Z, deterministic seed-and-coordinate geology generation, streaming, persistence as generation plus deltas, mutation-aware navigation, GPU command/mirror/event coupling, and material-derived visual outcomes—without importing mechanisms or game layers.
