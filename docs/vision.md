# Project vision

## What we are building now

**Moria** is a reusable, **GPU-resident voxel-world substrate** exposed as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation: matter, generation, mutation, and queries that downstream games consume. It is **not** a game, not a demo product identity, and not a System/LLM stack.

## Purpose

Moria exists so multiple games can share one continuous material world that **reads as a natural surface world** while remaining **fully diggable voxel truth**—including deep underground—without each title reimplementing geology, matter, streaming, and edit persistence. The substrate must stand alone with **zero LLM dependency**; game rules and presentation live above it.

## Product boundary

**This product owns**

- The reusable voxel-world substrate and its public consumer surface (Rust library integration).
- World-generation and matter responsibilities that make a natural, mutable, deep-Z world reusable: material truth, derived presentation of that truth, dig/place-class mutation, collision against voxel truth, streaming of active regions, and persistence of edits against generated baseline.
- Compatibility seams only where substrate requirements demand them—not implementations of game layers.

**Adjacent or downstream (not this product’s identity)**

- The actual game(s), including ARPG/System, fortress/colony, descent, or pure sandbox titles.
- Game rules, UX, controllers, cameras, authored demo routes, characters, and presentation policy.
- System/LLM, spells, gas/pricing policy, combat, AI, and building-as-gameplay layers.
- A walkable-world executable, if present, is only an **adjacent validation consumer** of the substrate; it is not the product. Whether shipping that harness is part of current repository delivery is **unresolved (Q1)**. While open, treat it only as an artifact that **may** exist, and only through the same public interfaces an external game would use—no privileged or game-specific substrate paths.

## Required product outcomes

Downstream design must make these true for the substrate product:

1. **Natural material world.** Consumers get a world that reads as ordinary outdoor terrain (hills, forest, water, cliffs, meadows) while everything visible is backed by mutable voxel matter—not decorative geometry outside the material world, and not a cube-aesthetic primary look.
2. **Mutable everywhere, deep Z first-class.** Any material volume can be destroyed, altered, or placed; underground (caves, strata, buried materials) is content, not a false floor.
3. **Geology-first generation.** Generation produces true layered ground and voids so digging reveals honest materials and spaces; the world is not a heightmap with rock painted underneath.
4. **Mesh as view.** Collision, queries, and gameplay-facing truth run on voxels; visual mesh is regenerated, non-authoritative, and not the save source of truth.
5. **Public mutation and query surface.** Nothing above the matter core touches voxels directly; external games and any in-repo harness use the same public verbs/queries/events. Adjacent consumers have no privileged access.
6. **Streamable, edit-persistent world.** Untouched generated volume stays cheap; player/system scars persist as edits against generation; active regions can be streamed for presence without treating the whole region as always-resident dense truth.
7. **GPU-resident substrate.** The world substrate is designed to live and work as a GPU-resident engine layer consumable from Rust.

## Future products and enabling implications

Future **consumers** (not current product scope) include a System/LLM ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and a pure sandbox. They own gameplay, content, controls, economy, and presentation.

**Enabling implications** (substrate remains the owner of engine capability; delivery depth is design’s problem): the same matter, generation, mutation, and query foundation should be reusable across those modes; gas/pricing and LLM direction attach as **game-layer policy/clients**, not substrate features; richer matter behaviors (multi-tier fluids, structural integrity, fire/ecology-style aggregate sim, voxel-backed vegetation objects, building stamps/mechanisms as engine verbs) are motivated so those games can sit on one stack—without importing their UX, combat, agents, or content into Moria now.

## Non-goals

- Implementing the actual game, its rules, combat, stats, AI, or player-facing building/crafting UX.
- System/LLM features, spells, gas metering/pricing, or any LLM dependency in the substrate.
- Treating the walkable demo’s character, camera, seed route, debug keys, or trailer content as product requirements.
- Shipping game layers (semantic fortress tooling, economy, designations) inside this repository’s product scope.
- Primary “Minecraft cube world” aesthetic as the intended surface look.

## Confirmed vision constraints

- **Rust library product:** exposed as a crate or small family of tightly scoped crates for game consumers.
- **GPU-resident** voxel-world substrate.
- **Consumer isolation:** any validation harness or external game uses only public substrate interfaces; no privileged in-repo game paths.
- **Standalone:** zero LLM dependency; the System is not a substrate feature.
- **Out of implementation here:** game rules and future System, spell, gas, combat, AI, and building layers (seams only if required).

## Deferred design decisions

- Exact crate split and internal module boundaries (consumer boundary is fixed; packaging shape is not).
- First delivery depth and sequence among generation, meshing, mutation, streaming, persistence, and richer matter sim.
- Voxel resolution, LOD/impostors, object-layer scaling, and meshing/representation technique choices.
- Performance budgets, benchmark scenes, and target hardware baselines (consumer/harness seeds do not define product identity).
- How far multiplayer-ready command authority is taken in early deliveries.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current repository delivery**, or only a **permitted adjacent artifact**?

- **Proposed safe answer:** Permitted adjacent artifact only—not part of Moria’s product identity; if present, it must consume solely public substrate interfaces.
- **If answered differently:** A “required delivery” answer keeps product identity as the substrate but adds a settled obligation to ship a harness beside it; a “not in repo” answer allows substrate-only delivery with validation left entirely outside this repository.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding boundary—substrate crate(s) only; game out of repo; harness if any is public-API-only validation; game/System/spell/gas/combat/AI/building layers out of scope.
- **`docs/seeds/product-one-seed.md`:** First consumer-shaped demo slice and validation motivations (natural mutable world, dig proof, performance curiosity); does not redefine product identity or import controller/content/platform gates into substrate scope.
- **`docs/seeds/voxel-world-substrate.md`:** Substrate purpose and engine-layer responsibilities (natural look, full mutability, deep Z, geology generation, reusable layering, GPU-resident matter world) and future-game enabling surface without making those games current product.
