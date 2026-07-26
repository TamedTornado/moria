# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer product for natural-looking, fully material 3D worlds—not a game, not a demo, and not a presentation shell.

## Purpose

Moria exists so multiple game styles can share one matter-world foundation: continuous terrain and underground depth that read as a normal world, remain voxel truth underneath, and can be queried and mutated without each game reimplementing geology, storage, meshing, or world physics. The substrate must stand alone with no LLM or game-rule dependency.

## Product boundary

**This product owns** the reusable world substrate: geology-oriented generation, sparse GPU-resident matter, non-authoritative visual and derived views, matter-side world behavior (ambient environmental cycles and interactive voxel-object lifecycle), voxel-authoritative spatial queries and collision, mutation-safe navigation data, and the public mutation/query surface every consumer uses.

**Adjacent, not the product:** a walkable-world executable may exist in this repository as a validation harness. If present, it is a separate consumer and must use the same public interfaces available to an external game. Its controller, character, camera, route, content, presentation, debug UX, workload, and performance gates are harness concerns, not substrate identity. Whether that harness is a required repository delivery remains open (see Q1).

**Downstream / out of this repository:** the actual game and all game layers—rules, System/LLM, spells, gas/pricing, combat, AI, and building layers (blueprints, mechanisms, rooms, work orders, designation UX). Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

**Consumer-owned even when they motivate the substrate:** gameplay, UX, controllers, movement policy, authored content, presentation, and game-specific policy. Controllers and AI consume substrate collision, queries, and navigation; they do not own voxel truth.

## Required product outcomes

1. **Natural world from voxel truth.** Surface terrain can read as ordinary landscape while interactive matter stays mutable voxel truth; meshes and dressing are regenerated views, not authoritative geometry.
2. **Mutable everywhere, deep Z first-class.** Matter can be destroyed, moved, or placed through the public API; underground geology and voids are real content, not a skybox floor.
3. **Geology-first generation.** Digging reveals true structure (strata, caves, ores, aquifers, surface systems); materialization is lazy so large regions idle cheaply until touched.
4. **Matter-layer world behavior without game rules.** Shared world-physics includes structural integrity, granular settle, multi-tier fluids, fire and wetness, thin ambient environmental cycles (time, seasons, weather, growth, water-level and fire ecology), and interactive voxel-backed objects whose substrate lifecycle covers movement, falling, physical conversion, re-voxelization, and growth—without game policy. A first walkable slice may omit some behaviors; that does not demote them from the substrate.
5. **Voxel-authoritative space and mutation-safe navigation.** Collision and spatial queries use voxel occupancy/truth, not the render mesh; navigation data derives from mutable bricks so pathfinding remains valid after edits. Controllers, movement policy, and AI stay consumer-owned.
6. **Portable GPU residence, incremental views, durable scars.** Consumers use only public verbs, queries, and events; gas/pricing is a policy plug-in above the substrate. Load-bearing GPU work stays on wgpu/WGSL with no native Metal fork, preserving Vulkan/DX12-class portability. Generated voxel terrain persists as worldgen function plus edit deltas; substrate-owned object and entity lifecycle state persists separately; streaming supports cross-run reuse of edited regions. After mutation, visual and derived views update incrementally over dirty regions.

## Future products and enabling implications

Future **consumers**, not this product: a System/LLM-driven ARPG, a DF-style fortress/colony game, a Moria-style descent, pure sandboxes, and any first walkable proof on the crate. Those products own gameplay, content, presentation, and policy.

Enabling implications (not a committed consumer roadmap): continuous 3D play from canopy to deep caves; dig/build honesty that rules out decorative-geometry worlds; column/Z-friendly structure for fortress-style views on continuous matter; and a command/mirror-style boundary that keeps the substrate reusable and sandbox-safe. A first walkable slice may exercise a subset of outcomes to prove the crate; it does not redefine product identity or strip ambient cycles, object lifecycle, or spatial outcomes from the substrate.

## Non-goals

- Shipping the actual game, rules, combat, AI, spells, gas economy, or LLM/System integration here.
- Implementing building-game layers (blueprint labor, mechanisms as gameplay, room economy, designation modes) here.
- Treating the harness’s character, camera, route, seed postcard, or benchmark theater as substrate features.
- LLM dependence, or requiring any one genre’s UX to use the crate.
- Owning consumer controllers, movement policy, or AI while still providing voxel collision, query, and navigation those systems use.

## Confirmed vision constraints

- Delivery form: Rust crate or small family of tightly scoped Rust crates.
- GPU-resident voxel-world substrate for external games; load-bearing layers stay on wgpu/WGSL with no native Metal fork, preserving Vulkan/DX12-class portability.
- Any in-repo walkable executable is a consumer/harness only and must not own privileged or game-specific paths into the substrate.
- Substrate stands alone with zero LLM dependency; game rules live above it.
- Future System, spell, gas, combat, AI, and building layers are out of implementation scope here (seams only where required).
- Physics, spatial queries, collision, and navigation operate on voxel truth and brick-derived nav, not authoritative render meshes.

## Deferred design decisions

- Depth and sequence of substrate capabilities versus any first validation slice, including when ambient cycles and full object lifecycle land relative to an early walkable proof.
- Voxel resolution, LOD strategy, object-layer capacity, and fluid-model fidelity.
- Exact crate split and internal module boundaries (consumer boundary fixed; packaging is not).
- Performance targets, hardware baselines, and benchmark workloads (harness gates, not substrate identity).
- Whether multiplayer-ready scope statements are carried now or later.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world executable a **required current delivery** of this repository, or only a **permitted** adjacent validation artifact?

- **Proposed answer:** Permitted as the primary way to validate the substrate, but not part of product identity; the repository may ship it without treating harness content, controls, or acceptance numbers as substrate scope.
- **If different:** Making the harness mandatory adds a repository delivery obligation (still outside product identity); forbidding it confines proof to other external consumers and tests. Neither answer turns the harness’s character, route, or presentation into substrate features.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust crate and classifies the walkable executable as a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the substrate crate(s), forces a non-privileged consumer boundary for any harness, excludes the actual game and listed game layers, and allows compatibility seams without implementing those layers.
- **docs/seeds/product-one-seed.md** — Describes a first walkable proof as harness-shaped validation (controller against voxel occupancy, dig/place proof, incremental remesh, wgpu/WGSL portability); its content, acceptance numbers, and first-slice omissions do not redefine substrate identity or demote ambient cycles and object lifecycle from the crate.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate purpose and outcomes: natural look from voxel truth, everywhere-mutable deep-Z matter including movement, geology-first generation, matter physics and ambient cycles, voxel-object lifecycle, voxel-authoritative queries/navigation, incremental non-authoritative views, terrain deltas plus object journals, and public verb/query boundary—game genres and the System remain above the substrate.
