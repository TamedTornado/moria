# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a **Rust crate** (or a small family of tightly scoped Rust crates). It is the engine layer for mutable material worlds—not a game, not a demo product, and not a gameplay stack.

## Purpose

Moria exists so multiple games can share one world foundation: natural-looking, fully diggable terrain with deep underground as real content, and clean public interfaces for matter, queries, mutation, and related world services. The substrate must stand alone with **no LLM or System dependency**. Game identity, rules, and presentation live in downstream consumers.

## Product boundary

**In product**

- The reusable substrate: authoritative voxel matter, generation hooks, meshing as a non-authoritative view, mutation and query surfaces, streaming and edit persistence, and substrate-level world services (collision against voxel truth, matter-coupled dressing/objects, and physics/fluid/support capabilities the engine owns for reuse).
- Integration as Rust crate(s) for external game consumers.

**Adjacent, not product identity**

- A **walkable-world executable** may exist in-repo as a validation harness that exercises the substrate through the **same public interfaces** an external game would use. Whether shipping that harness is part of current delivery is unresolved (**Q1**). Its character, controls, camera, authored demo route, content palette, presentation, scripted workloads, and machine-specific performance gates are harness-owned, not substrate scope.

**Out of product / repository**

- The actual game and all game rules.
- System / LLM, spells, gas policy, combat, AI, and building *game* layers (UI, labor, economy, designations). Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

Downstream design must make these true for the substrate:

1. **Authoritative mutable matter** — A GPU-resident voxel world is the source of truth; any volume can be destroyed, moved, or placed. Presentation meshes are regenerated views, never saved as authority.
2. **Natural surface, material truth** — Landscapes read as continuous natural worlds (terrain, vegetation, water bodies, cliffs) while dressing and voxel-backed interactables stay coupled to matter so look cannot desync from dig, burn, or similar change.
3. **Deep-Z geology at scale** — Underground is first-class content (strata, caves, ores, aquifers), produced by geology-first generation with lazy materialization and sparse representation so large regions stay tractable until touched.
4. **Public mutation and query boundary** — Consumers mutate and inspect the world only through public verbs and queries; nothing above the matter core touches voxels directly. This is the sandbox, multiplayer-readiness, and reuse boundary.
5. **Streaming, edit persistence, and collision truth** — Large worlds stream around activity; saves are worldgen plus edit deltas; locomotion and interaction can collide against voxel occupancy, not the render mesh alone.
6. **Multi-game reusable world services** — The same crate stack underpins ARPG, fortress/colony, descent, or sandbox consumers by providing matter, queries, mutation, and physics-relevant services (including support and fluid behavior the engine owns)—not by embedding any one game’s rules.

## Future products and enabling implications

Future **consumers** (not this product) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate substrate outcomes above; their gameplay, content, controllers, characters, animation, UX, and policies stay with those products.

Enabling implications only: keep layering so gas/pricing policy, LLM authorship, and game semantics plug in above the substrate without becoming substrate features.

## Non-goals

- Shipping an actual game or game-rule stack in this repository.
- Implementing System/LLM, spells, gas, combat, AI, or building-game layers.
- Treating the walkable-world harness’s demo fiction, UI, or acceptance scene as product requirements.
- Making the substrate depend on an LLM.

## Confirmed vision constraints

- Product form is a **Rust** crate or small family of crates for game integration.
- Any in-repo harness and all external games share one **public** consumer boundary; no privileged substrate paths for the harness.
- Substrate operation has **zero LLM dependency**.
- Matter is **GPU-resident** as part of product identity.

## Deferred design decisions

- Crate split and workspace layout (beyond the consumer boundary itself).
- Voxel resolution, meshing strategy, LOD, storage encodings, and kernel details.
- Depth and sequence of fluids, integrity, ambient sim, object felling, and related engine services.
- Harness content, controls, demo world, platforms, and performance acceptance (if a harness is delivered).
- Open measurement questions (e.g. voxel size tradeoffs) answered by design and validation, not by vision.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted only—the product promise is the substrate crates; a harness may exist and must use public APIs, but it is not mandatory to ship for the product to be complete.
- **If answered differently:** Requiring the harness keeps product identity as substrate-only but adds a mandatory adjacent deliverable (still without importing its demo content or controls into substrate scope). Treating it as out of repository entirely would remove even the permitted in-repo harness path described by the boundary seed.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and frames the walkable-world executable as a separate validation consumer rather than a game layer.
- **docs/seeds/project-boundary.md** — Settles product identity (substrate Rust crate(s)), excludes the actual game and listed game layers, permits a public-API-only validation harness, and forbids privileged harness paths.
- **docs/seeds/product-one-seed.md** — Describes a first walkable demo consumer/harness slice (content, controller, milestones, machine targets) that motivates validation of generation, meshing, edit, streaming, persistence, and collision without transferring that demo into substrate identity.
- **docs/seeds/voxel-world-substrate.md** — Authorizes the substrate’s outcome families (natural mutable worlds, deep-Z geology, generation at scale, matter/physics/query/mutation services, layering, LLM independence) at engine altitude for multi-game reuse.
