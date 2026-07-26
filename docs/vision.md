# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer product for natural-looking, fully material 3D worlds—not a game, not a demo, and not a presentation shell.

## Purpose

Moria exists so multiple game styles can share one matter-world foundation: continuous terrain and underground depth that read as a normal world, remain voxel truth underneath, and can be queried and mutated without each game reimplementing geology, storage, meshing, or world physics. The substrate must stand alone with no LLM or game-rule dependency.

## Product boundary

**This product owns** the reusable world substrate: geology-oriented generation, sparse GPU-resident matter, non-authoritative visual reconstruction of matter, matter-side world behavior, and the public mutation/query surface through which every consumer interacts.

**Adjacent, not the product:** a walkable-world executable may exist in this repository as a validation harness. If present, it is a separate consumer of the substrate and must use the same public interfaces available to an external game. Its controller, character, camera, authored route, content palette, presentation, debug UX, workload, and performance gates are harness concerns, not substrate identity. Whether that harness is a required repository delivery remains open (see Q1).

**Downstream / out of this repository:** the actual game and all game layers—rules, System/LLM, spells, gas/pricing policy, combat, AI, and building layers (blueprints, mechanisms, rooms, work orders, designation UX). Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

**Consumer-owned even when they motivate the substrate:** gameplay, UX, controllers, authored content, presentation, and game-specific policy.

## Required product outcomes

1. **Natural world from voxel truth.** Surface terrain can read as ordinary landscape (hills, forest, water, cliffs, caves) while everything visible remains backed by mutable voxel matter; meshes and surface dressing are views of that matter, not authoritative geometry.
2. **Mutable everywhere, deep Z first-class.** Any region of matter can be destroyed, placed, or otherwise mutated through the public API; underground geology and voids are real content, not a flat floor under a skybox.
3. **Geology-first generation consumers can rely on.** Worlds are produced so digging reveals true structure (strata, caves, ores, aquifers, surface systems); materialization is lazy enough that large regions idle cheaply until touched.
4. **Matter-layer world behavior without game rules.** The substrate provides the world-physics responsibilities games will share—structural integrity, granular settle, multi-tier fluids, fire/wetness and related ambient matter behavior, and interactive voxel-backed objects where matter must burn, break, or block—without embedding game policy.
5. **Public-only integration boundary.** Consumers (including any harness) mutate and observe the world only through public verbs, queries, and events; nothing above the matter core may touch voxels directly; gas/pricing is a policy plug-in above the substrate, not a built-in game system.
6. **GPU-resident world with lasting scars.** Active world state lives on the GPU model intended by the substrate; persistence is worldgen function plus edit deltas; streaming keeps only needed neighborhoods live so the same crate stack can serve adventure and fortress-scale reuse of edited regions.

## Future products and enabling implications

Future **consumers**, not this product: a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, pure sandboxes, and any first walkable proof built on the crate. Those products own gameplay, content, presentation, and policy.

Enabling implications already implied by the substrate (not a committed consumer roadmap): continuous 3D play from canopy to deep caves; dig/build honesty that collapses decorative-geometry designs; column/Z-friendly structure so fortress-style views can sit on continuous matter; and a command/mirror-style boundary that keeps the substrate reusable and sandbox-safe. A first walkable slice may exercise a subset of these outcomes to prove the crate; that slice does not redefine product identity.

## Non-goals

- Shipping the actual game, its rules, combat, AI, spells, gas economy, or LLM/System integration in this repository.
- Implementing building-game layers (blueprint labor, mechanisms as gameplay, room economy, designation modes) here.
- Treating the walkable harness’s character, camera, route, seed postcard, or benchmark theater as substrate features.
- Making the substrate depend on an LLM, or requiring any one game genre’s UX to use the crate.

## Confirmed vision constraints

- Delivery form: Rust crate or small family of tightly scoped Rust crates.
- GPU-resident voxel-world substrate intended for reuse by external games.
- Any in-repo walkable executable is a consumer/harness only and must not own privileged or game-specific implementation paths into the substrate.
- Substrate stands alone with zero LLM dependency; game rules live above it.
- Future System, spell, gas, combat, AI, and building layers are out of implementation scope here (seams only where substrate requirements demand them).

## Deferred design decisions

- Depth and sequence of substrate capabilities versus any first validation slice (what ships when).
- Voxel resolution, LOD strategy, object-layer capacity, and fluid-model fidelity.
- Exact crate split and internal module boundaries (the consumer boundary is fixed; packaging layout is not).
- Performance targets, hardware baselines, and benchmark workloads for validation.
- Whether multiplayer-ready scope statements are carried now or later (architecture readiness is not the same as a multiplayer product).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world executable a **required current delivery** of this repository, or only a **permitted** adjacent validation artifact?

- **Proposed answer:** Permitted as the primary way to validate the substrate, but not part of product identity; the repository may ship it without treating harness content, controls, or acceptance numbers as substrate scope.
- **If different:** Making the harness mandatory adds a repository delivery obligation (still outside product identity); forbidding it confines proof to other external consumers and tests. Neither answer turns the harness’s character, route, or presentation into substrate features.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust crate and classifies the walkable executable as a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Binds current product identity to the substrate crate(s), forces a non-privileged consumer boundary for any harness, excludes the actual game and listed game layers from this repository, and allows compatibility seams without implementing those layers.
- **docs/seeds/product-one-seed.md** — Describes a first walkable proof (region, character, dig/place proof, performance theater) as a product-shaped demo that motivates and validates the substrate; its controller, content, and acceptance details stay consumer/harness-scoped and do not redefine the crate’s identity.
- **docs/seeds/voxel-world-substrate.md** — Supplies the substrate’s purpose and outcome family: natural look from voxel truth, everywhere-mutable deep-Z matter, geology-first generation, matter-layer world behavior, public verb/query boundary, GPU-resident sparsity, and persistence/streaming—while placing game genres and the System above the substrate.
