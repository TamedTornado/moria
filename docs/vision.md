# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it as library code. It is an engine-layer world substrate—not a game, demo app, or content package.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface world whose look rests on fully mutable voxel truth, with deep underground as first-class space, generation that digs honestly, and public mutation and query surfaces games call without reimplementing the world. The substrate stands alone with **no LLM or System dependency**. Game rules, progression, and presentation live above it.

## Product boundary

**In product:** the reusable substrate—world generation foundations, GPU-resident matter, meshing as a non-authoritative view of voxel truth, dig/place and related matter mutation, physics and queries against voxels, streaming and sparsity so large regions stay tractable, persistence as generation plus edit deltas, and the public integration surface for external consumers.

**Out of product:** the actual game and this repository’s game layer; game rules; System/LLM features; spells; gas policy; combat; AI; and building *game* layers (blueprints-as-gameplay, work orders, mechanisms-as-game systems, fortress/ARPG policy). Compatibility seams may exist where substrate needs demand them; those layers are not implemented here.

**Adjacent, not identity:** a walkable-world executable *may* exist as a validation harness for generation, streaming, meshing, editing, collision, persistence, and performance. If present, it must use the **same public interfaces** as an external game—no privileged or game-specific paths. Harness-owned character, camera, controllers, authored demo route/content, presentation, scripted workloads, machine profiles, and performance gates are not product scope. Whether that harness is a **mandatory current delivery** or only **permitted** is unresolved (see Q1); until then it is neither required nor ruled out in this brief.

## Required product outcomes

1. **Rust-consumable substrate.** Downstream integrators use Moria as crate(s), not as an embedded game binary or privileged in-repo fork.
2. **Natural look, voxel truth.** Worlds read as continuous terrain (hills, forests, water, cliffs, caves), not a block aesthetic; the mesh is a regenerated view. Physics, collision, and gameplay-facing queries run against voxel matter.
3. **Mutable material world.** Matter can be destroyed, placed, and edited through the public API; dig/place prove the world is fully material, not decorative geometry over a heightmap.
4. **Geology-first generation and deep Z.** Worlds are generated as geology (strata, channels, caves, underground features), materializing lazily so large extents idle cheaply; the underground is content, not a false floor.
5. **Scale and continuity of scars.** Streaming and sparsity keep large regions tractable; persistence is worldgen function plus edit deltas so mutations survive reload without saving untouched volume as dense voxels.
6. **Clean consumer boundary.** Nothing above the matter surface touches voxels directly; all consumers—including any harness—go through public verbs and queries. The substrate provides matter, generation, mutation, queries, and related physics foundations without owning game policy.

## Future products and enabling implications

Downstream **consumers** (not this product) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate why the substrate stays reusable and game-agnostic. Enabling implications: shared diggable natural worlds, deep-Z play, and a common mutation/query surface. Their gameplay, UX, controllers, characters, animation, authored content, gas/pricing policy, and acceptance scenarios are **not** current-product scope.

Product One’s “walkable world” is an adjacent **demo/validation shape** (one region, a character, proof digs)—not a redefinition of Moria’s identity. First-slice depth for generation, matter, fluids, or integrity is a design/delivery concern, not a narrower product identity.

## Non-goals

- Shipping game systems: combat, stats, AI, System/LLM, spells, gas economy, weather/season *game* loops, building UI and fortress/ARPG policy.
- Treating harness character control, free-orbit camera, demo seed content, or benchmark numbers as substrate product requirements.
- Implementing excluded upper layers “for later” inside this product under another name.

## Confirmed vision constraints

- **Ecosystem:** product surface is Rust crate(s) for library consumption.
- **Residency:** the world substrate is GPU-resident in intent (matter and related work live on GPU-oriented paths as the product model).
- **Interface equality:** any in-repo validation consumer uses only public substrate interfaces available to an external game.
- **No LLM dependency:** substrate operation and value do not require System/LLM features.
- **Repository boundary:** the actual game is a separate downstream consumer, not part of this product’s identity.

## Deferred design decisions

- Exact crate split, APIs, algorithms, data layouts, voxel size, LOD, object-layer scaling, fluid/integrity/CA depth and sequence, and persistence encodings.
- Whether and how far first delivery goes on generation, meshing, matter sim, and persistence (including any Product One milestone order).
- Harness-only choices: controllers, demo region contents, platforms, performance thresholds, and benchmark scenes—if a harness is delivered.
- Implementation stack details (e.g. graphics backend, device-specific limits) unless later bound as product promise.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **mandatory current delivery** alongside the substrate, or only a **permitted** adjacent validation artifact?

- **Proposed safe answer:** Permitted only. Ship identity stays the substrate crate(s); a harness may exist and must use public APIs, but is not required for the product to be itself.
- **If different:** Making it mandatory keeps product identity as the substrate but adds a required adjacent deliverable (still without importing its controls, content, or performance gates into substrate scope). Treating it as out of scope entirely would remove even permission to house a same-interface harness here.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident voxel-world substrate Rust crate and positions the walkable-world executable as a separate harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Fixes current identity (substrate crate(s)), excludes the game and upper layers, and requires public-interface consumption for any harness.
- **`docs/seeds/product-one-seed.md`:** Describes an adjacent walkable-world validation/demo slice and first-proof depths; supplies motivation and non-goals without reassigning harness content or device targets to product identity.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes substrate outcome families (natural look over voxel truth, full mutability, deep Z, geology-first generation, streaming/persistence, reusable layered engine without LLM) at outcome altitude for this brief.
