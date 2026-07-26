# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a **Rust crate** or a small family of tightly scoped Rust crates. It is an engine layer for material worlds—not a game, not a demo title, and not the future System/ARPG or fortress product.

## Purpose

Moria exists so multiple games can share one world foundation: a natural-looking surface world over fully mutable voxel matter, with deep underground as real content, and with matter, physics, queries, and mutation living below game rules. The substrate must stand alone with **no LLM or System dependency**. Games own policy, content, and play; Moria owns the material world they sit on.

## Product boundary

**In product**

- The voxel-world substrate: generation, matter representation and simulation responsibilities, meshing as a non-authoritative view, public mutation/query surfaces, streaming, and persistence of editable world truth.
- Integration as Rust crate(s) intended for consumption by an external game.

**Out of product (adjacent or downstream)**

- The actual game (any ARPG, fortress, descent, or sandbox title) lives in a separate repository and is not part of Moria.
- Game rules and game layers: System/LLM, spells, gas policy, combat, stats, AI, and building-as-gameplay (UI, blueprints-as-play, mechanisms-as-game systems, work orders, economy).
- A walkable-world executable **may** exist as an **adjacent** validation harness. It is not the product identity. If present, it must use the **same public interfaces** available to an external game—no privileged or game-specific substrate paths. Whether shipping that harness is a current delivery obligation is open (**Q1**). Its character, camera, controller, authored demo route, presentation, workload, and performance gates are not product scope.

Compatibility seams may be designed where substrate outcomes require them; excluded layers must not be implemented here.

## Required product outcomes

1. **Material truth, normal look.** Consumers get continuous terrain that reads as an ordinary world (hills, forest, water, cliffs, meadows) while the voxel grid remains authoritative. The render mesh is a regenerated view—never the source of physics, queries, or saves.
2. **Mutable everywhere.** Any voxel matter can be destroyed, moved, or placed through substrate verbs. Dig and place are first-class proofs of material honesty; edited faces must read as cut or placed matter, not as props on a static shell.
3. **Deep Z and geology-first worlds.** Underground is content: strata, caves, ore, and related geological structure. Generation is geology (not a painted heightmap), with lazy materialization and sparse treatment of uninteresting volume so large regions remain feasible.
4. **Matter-coherent surface life.** Interactable vegetation and similar world objects are voxel-backed matter. Lightweight surface dressing is driven by underlying voxel state so look and interaction stay aligned with material truth.
5. **Substrate physics responsibilities.** Fluids (from still bodies through flow behavior), structural integrity and collapse, granular settle, and fire/ambient matter rules are substrate capabilities for games to rely on—not game systems authored in this product. Delivery depth and sequence are design choices.
6. **Safe reuse boundary, persistence, and packaging.** Nothing outside the matter core touches voxels directly: public verbs, queries, and events are the consumer path. World truth is generation plus edit deltas; active regions stream. The product ships as GPU-resident Rust crate(s) for external games.

## Future products and enabling implications

Future **consumers** (not this product) include a System-driven ARPG, a DF-style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate a reusable matter/query/mutation substrate; they do not pull gameplay, controllers, authored content, presentation, gas policy, or LLM features into Moria.

A walkable-world harness may exercise generation, streaming, meshing, editing, collision, persistence, and performance against public APIs (**Q1**). Its demo content and acceptance scenario are not substrate scope.

## Non-goals

- Implementing any full game, combat loop, entity AI, System/LLM features, spells, or gas/economy policy.
- Owning building-as-gameplay, fortress designation UX, or semantic game systems (rooms-as-play, work orders, town ledgers).
- Treating the walkable demo’s character, route, art direction, or benchmark scene as the product definition.
- Shipping a Minecraft-cube surface aesthetic as the primary look (debug raw-voxel views may exist; the product promise is material truth under a non-cube reading).

## Confirmed vision constraints

- Product form: reusable **Rust** crate or small family of tightly scoped Rust crates.
- Runtime character: **GPU-resident** voxel-world substrate.
- Consumer isolation: any in-repo harness or external game uses **public interfaces only**; no privileged substrate paths.
- Independence: substrate stands alone with **zero LLM/System dependency**.
- Repository scope: the actual game is **not** in this repository; excluded game layers are not implemented here.

## Deferred design decisions

- How far each substrate physics and generation outcome is taken in a given delivery slice, and in what order.
- Exact public API shape, crate split within the family, and internal representation choices (resolution, meshing approach, LOD, object scaling).
- Whether and how an adjacent harness is structured if delivered; platforms, backends, and numeric performance gates for validation.
- Open technical tradeoffs left by the substrate seeds (e.g. voxel resolution, distant terrain strategy)—design-phase measurement, not vision identity.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world executable a **mandatory current adjacent deliverable**, or only **permitted** as a validation harness?

- **Proposed answer:** Permitted only—not a required ship item of the current product. If built, it remains an adjacent consumer of public APIs and does not redefine product identity.
- **If different:** Making it mandatory does not change substrate identity or outcomes, but adds a required adjacent deliverable the repository must produce beside the crate(s); product-one-style demo content still stays out of product scope unless separately approved.

## Seed synthesis

- **README.md** — Names Moria as a GPU-resident voxel-world substrate consumed as a Rust crate, and positions a walkable-world executable as a separate validation consumer rather than a game layer.
- **docs/seeds/project-boundary.md** — Binding identity and boundary: Rust crate substrate; game out of repo; harness only via public APIs if present; game/System/building layers out of scope.
- **docs/seeds/product-one-seed.md** — Motivates first-slice proof of a material walkable world and dig/place honesty; supplies harness/demo detail that must not transfer into substrate identity; clarifies early non-goals for game systems.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look over voxel truth, universal mutability, deep-Z geology, matter-linked dressing/objects, fluids/integrity/granular/fire responsibilities, verb/query boundary, streaming and delta persistence) without making game layers or mechanism inventories part of this brief.
