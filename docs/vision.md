# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation for external games—not a game, not a gameplay stack, and not defined by any single demo scene.

## Purpose

Moria exists so multiple games can share one material world layer: landscapes that read as ordinary natural worlds while remaining fully diggable voxel truth, including deep underground as real content. The substrate provides matter, physics-relevant world services, queries, and mutation through public interfaces. It must stand alone with **no LLM or System dependency**. Game identity, rules, presentation, and policy live only in downstream consumers.

## Product boundary

**In product**

- The reusable substrate: authoritative voxel matter; geology-oriented generation consumers can drive; presentation meshes as non-authoritative views; public mutation and query surfaces; streaming and edit persistence; collision against voxel occupancy; and substrate-owned world services (matter-coupled surface dressing and interactable objects, plus fluid and support behavior the engine owns for reuse).
- Delivery and integration as Rust crate(s) for external game consumers.

**Adjacent, not product identity**

- A **walkable-world executable** may exist in this repository as a validation harness that exercises the substrate only through the **same public interfaces** available to an external game. Whether that harness is a required current delivery is unresolved (**Q1**). Its character, controller, camera, authored route, seed world content, presentation, scripted workloads, platforms, and performance gates are harness-owned and are not substrate scope.

**Out of product / repository**

- The actual game and all game rules.
- System / LLM, spells, gas policy, combat, AI, and building *game* layers (UI, labor, economy, designations, work orders). Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

Downstream design must make these true for the substrate:

1. **Authoritative mutable matter** — A GPU-resident voxel world is the source of truth; any volume can be destroyed, moved, or placed. Presentation is regenerated from matter and is never the saved authority.
2. **Natural surface, material truth** — Continuous natural landscapes (terrain, vegetation, water bodies, cliffs) while dressing and voxel-backed interactables stay coupled to matter so appearance cannot desync from dig or other material change.
3. **Deep-Z geology at scale** — Underground is first-class content (strata, caves, ores, aquifers and similar volume), from geology-first generation with lazy materialization and sparse representation so large regions stay tractable until touched.
4. **Public mutation and query boundary** — Consumers inspect and change the world only through public verbs and queries; nothing above the matter core owns privileged voxel paths. This is the reuse, sandbox, and multiplayer-readiness boundary.
5. **Streaming, edit persistence, and collision truth** — Large worlds stream around activity; persistence is generation plus edit deltas; locomotion and interaction can collide against voxel occupancy, not the render mesh alone.
6. **Multi-game reusable world services** — The same crate stack underpins ARPG, fortress/colony, descent, or sandbox consumers by supplying matter, queries, mutation, and physics-relevant services (including support and fluid behavior the engine owns)—without embedding any one game’s rules.

## Future products and enabling implications

Future **consumers** (not this product) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate the substrate outcomes above; their gameplay, content, controllers, characters, animation, UX, and policies remain with those products.

Enabling implications only: keep layering so gas/pricing policy, LLM authorship, and game semantics can plug in above the substrate without becoming substrate features.

## Non-goals

- Shipping an actual game or game-rule stack in this repository.
- Implementing System/LLM, spells, gas, combat, AI, or building-game layers here.
- Treating the walkable-world harness’s demo fiction, UI, controls, content, or acceptance scene as product requirements.
- Making the substrate depend on an LLM.

## Confirmed vision constraints

- Product form is a **Rust** crate or small family of tightly scoped crates for game integration.
- Any in-repo harness and all external games share one **public** consumer boundary; no privileged substrate paths for the harness.
- Substrate operation has **zero LLM dependency**.
- Matter is **GPU-resident** as part of product identity.

## Deferred design decisions

- Precise crate split and workspace layout (beyond the consumer boundary itself).
- Voxel resolution, meshing strategy, LOD, storage encodings, and kernel details.
- Delivery depth and sequence of fluids, integrity, ambient simulation, object felling, and related engine services.
- Harness content, controls, demo world, platforms, and performance acceptance (if a harness is delivered).
- Measurement questions (for example voxel-size tradeoffs) answered by design and validation, not by vision.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted only—the product promise is the substrate crates; a harness may exist and must use public APIs, but shipping it is not required for the product to be complete.
- **If answered differently:** Requiring the harness keeps product identity as substrate-only but adds a mandatory adjacent deliverable (still without importing its demo content or controls into substrate scope). Treating it as out of the repository entirely would remove even the permitted in-repo harness path described by the boundary seed.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and frames the walkable-world executable as a separate validation consumer for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md** — Settles product identity (substrate Rust crate(s)), excludes the actual game and listed game layers, permits a public-API-only validation harness, and forbids privileged harness paths.
- **docs/seeds/product-one-seed.md** — Describes a first walkable demo/harness slice (content, controller, milestones, machine targets) that motivates validation of generation, meshing, edit, streaming, persistence, and collision without transferring that demo into substrate identity or delivery commitment.
- **docs/seeds/voxel-world-substrate.md** — Authorizes the substrate’s outcome families (natural mutable worlds, deep-Z geology, generation at scale, matter/physics/query/mutation services, layering, LLM independence) at engine altitude for multi-game reuse.
