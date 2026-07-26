# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** for games, delivered as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer world of matter—not a game, not a genre demo, and not an LLM system.

## Purpose

Moria exists so multiple games can share one material world foundation: terrain that **reads as a normal natural world** while remaining **fully mutable voxel truth**, including **deep underground** play. Game rules, progression, and presentation live above the substrate; the substrate supplies matter, physics, queries, and mutation without depending on any game or LLM.

## Product boundary

**In product**

- The reusable voxel-world substrate and its public integration surface for external Rust consumers.
- World generation, matter representation, mutation, spatial queries, matter/physics the substrate owns for a reusable material world, and world persistence/streaming—stated as product outcomes, not as a feature inventory.
- Compatibility seams the substrate needs so downstream games can attach without forking privileged paths.

**Adjacent, not product identity**

- A walkable-world executable, if present, is only a **validation harness** for the substrate. It must use the same public interfaces an external game would use. Whether that harness is a required repository delivery is open (see Q1). Its controller, camera, character, authored demo route, content palette, debug presentation, benchmark scene, and performance gates are not product scope.
- The actual game (and any genre-specific mode) is a **separate downstream consumer**, outside this repository.

**Out of this product**

- Game rules and the System, LLM, spell, gas, combat, AI, and building **game** layers. Seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

- **Natural-looking material world.** Generated terrain and surface dressing present as hills, forests, water, cliffs, and related natural structure; the voxel grid is truth, not the intended cube aesthetic. Rendered surface is a regenerated view; gameplay-facing physics and queries run against voxel matter.
- **Mutable everywhere, deep Z first-class.** Any voxel can be destroyed, moved, or placed through substrate mutation. Underground volume is real content (caves, strata, ore, voids), not a decorative floor under a heightmap.
- **Geology-first generation.** Worlds are produced as layered geology and related natural structure so digging reveals consistent material truth, with lazy materialization so untouched volume need not occupy full brick storage.
- **Matter, physics, queries, mutation.** Consumers change and inspect the world only through the substrate’s verb/query boundary—not by touching raw voxel storage. Dig and place are first-class mutation; mesh is never authoritative truth.
- **Persistence and streaming.** Truth is worldgen plus edit deltas; active regions stream without requiring the whole region resident as raw voxels.
- **Multi-game foundation, zero LLM dependency.** The same crate stack is intended to underpin ARPG, fortress/colony, descent, and sandbox consumers. The substrate stands alone with no LLM requirement; gas/pricing and similar policies are consumer-injected, not hard-wired game rules.

## Future products and enabling implications

Future **consumers** (not current product): a System/LLM-backed ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox modes.

Enabling implications already owned at substrate altitude (delivery depth is design): continuous 3D mutable geology; matter rules that can later support fire, fluids, granular settle, and structural integrity; vegetation and interactable natural objects as matter-backed where interaction is required; placement/mutation suitable for later building games; column-friendly structure so Z-slice views and fortress-style tooling can sit above without rewriting the world model. Gameplay, UX, controllers, authored content, presentation, and game-specific policy remain consumer-owned.

## Non-goals

- Implementing the actual game, its rules, combat, stats, AI, spells, gas economy, or System/LLM features in this repository.
- Treating the validation harness’s character, camera, demo route, fixtures, or performance poster as the product definition.
- Making the substrate depend on an LLM or embed a particular game’s pricing, progression, or building-policy layer.
- Importing harness or future-game content, controls, presentation, or acceptance scenarios into the substrate product promise.

## Confirmed vision constraints

- **Rust crate ecosystem.** The product is consumed as a Rust crate or small family of tightly scoped Rust crates.
- **Strict consumer boundary.** Adjacent consumers (including any walkable-world harness) have no privileged or game-specific implementation path into the substrate; public interfaces are the only integration surface.
- **Substrate, not game.** Clean layering: game rules live above; Moria provides world/matter capability underneath.
- **GPU-resident world substrate.** Core world residency and simulation posture is GPU-resident as part of product identity (mechanism details are design).

## Deferred design decisions

- Exact crate split within the workspace boundary; internal layering and APIs.
- Voxel resolution, LOD, meshing approach selection, storage layout, and related engine parameters.
- Which substrate-capable matter/physics behaviors ship in which delivery depth; milestone order and acceptance thresholds.
- Harness-only concerns if a harness exists: demo region contents, controls, presentation, platforms, and benchmark protocol.
- Multiplayer and long-horizon engine modules beyond the vision-level command/query boundary.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world executable a **required repository delivery** alongside the substrate crates, or only a **permitted** adjacent validation artifact?

- **Proposed safe answer:** Permitted only—not part of product identity. If humans later require it as a repo delivery, it remains an adjacent harness that exercises public substrate interfaces; its controls, content, presentation, and performance gates still do not become product outcomes.
- **If answered “required delivery”:** The repository must ship a harness that validates the public substrate boundary, but product identity stays the reusable crates; harness-specific scenarios stay out of Required product outcomes.
- **If answered “permitted only”:** Substrate crates alone define current delivery; any harness is optional adjacent work.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and frames the walkable-world executable as a separate validation consumer rather than a game layer.
- **docs/seeds/project-boundary.md** — Settles product identity (substrate crates), excludes the actual game and named game layers from the repo, and requires any harness to use public interfaces only.
- **docs/seeds/product-one-seed.md** — Motivates first-slice proof points (generated walkable region, dig/place as mutability proof) as harness/demo depth; does not redefine the reusable product as that demo’s content, controls, or performance poster.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate purpose and outcome altitude: natural-looking mutable world, deep Z, geology-first generation, matter/physics/queries/mutation, persistence/streaming, multi-game reuse, and zero LLM dependency—without transferring game-layer ownership into Moria.
