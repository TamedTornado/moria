# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer product for natural-looking, fully material 3D worlds—not a game or presentation shell.

## Purpose

Moria exists so multiple game styles can share one matter-world foundation: continuous terrain and underground depth that read as a normal world, remain voxel truth, and can be queried and mutated without each game reimplementing geology, storage, meshing, or physics. It stands alone with no LLM or game-rule dependency.

## Product boundary

**This product owns** the reusable world substrate: geology-oriented generation, sparse GPU-resident matter, non-authoritative views, matter-side world behavior (ambient cycles and interactive voxel-object lifecycle), voxel-authoritative spatial queries and collision, mutation-safe navigation data, public registries for consumer extension, and the public mutation/query surface with a commands-in, stale-mirror-and-events-out contract.

**Adjacent, not the product:** a walkable-world executable may exist as a validation harness. If present, it is a separate consumer using the same public interfaces as an external game. Controller, character, camera, route, content, presentation, debug UX, workload, and machine-specific acceptance numbers are harness concerns, not substrate identity. Whether it is a required delivery remains open (see Q1).

**Downstream / out of this repository:** the actual game and all game layers—rules, System/LLM, spells, gas/pricing, combat, AI, and building layers (blueprints, mechanisms, rooms, work orders, designation UX). Compatibility seams may be designed where required; those layers are not implemented here.

**Consumer-owned:** gameplay, UX, controllers, movement policy, authored content, presentation, and game-specific policy. Controllers and AI consume substrate collision, queries, and navigation; they do not own voxel truth. System and hand-authored clients extend only through the same public paths.

## Required product outcomes

1. **Natural world from voxel truth; mutable everywhere; deep Z first-class.** Surface can read as ordinary landscape while interactive matter stays mutable voxel truth. Meshes and dressing are regenerated views, not authoritative geometry. Matter can be destroyed, moved, or placed through the public API; underground geology and voids are real content, not a skybox floor.
2. **Geology-first generation and exact terrain reload.** Digging reveals true structure (strata, caves, ores, aquifers, surface systems). Generation is a pure function of coordinates and world seed; the same seed plus terrain edit deltas restores the edited voxel terrain exactly. Object/entity journals do not inherit that terrain-exactness claim. Materialization is lazy so large regions idle cheaply until touched.
3. **Matter-layer world behavior without game rules.** Shared world-physics includes structural integrity, granular settle, multi-tier fluids, fire and wetness, thin ambient cycles (time, seasons, weather, growth, water-level and fire ecology), and interactive voxel-backed objects (movement, falling, physical conversion, re-voxelization, growth)—without game policy. A first walkable slice may omit some behaviors without demoting them from the substrate.
4. **Voxel-authoritative space, mutation-safe navigation, and shared extensibility.** Collision and spatial queries use voxel occupancy/truth, not the render mesh; navigation data derives from mutable bricks so pathfinding remains valid after edits. Controllers, movement policy, and AI stay consumer-owned. The substrate owns public registries for materials, palettes, placement metadata, rules, and kernels; System and hand-authored clients use those same paths; authored content and building-game functions remain consumer-owned.
5. **Portable GPU residence with stale-mirror observation.** Consumers use only public verbs, queries, and events under a GPU interaction contract: commands in, a potentially stale mirror plus events out—observation is not assumed live. Gas/pricing is a policy plug-in above the substrate. Load-bearing GPU work stays on wgpu/WGSL with no native Metal fork (Vulkan/DX12-class portability).
6. **Bounded runtime streaming, interactive remesh, durable scars.** Runtime streaming keeps render, simulation, aggregate, and cold residency bounded per active anchor so large regions need not reside as raw voxels—independent of cross-run persistence. Cross-run reuse comes from saved terrain deltas (plus separate object/entity journals). Dirty regions remesh within interactive frame-scale latency. Untouched sparse regions idle cheaply at effectively boundless scale. Substrate-level responsiveness and sparse working-set behavior are product outcomes; machine-specific thresholds remain harness-owned.

## Future products and enabling implications

Future **consumers**, not this product: a System/LLM-driven ARPG, a DF-style fortress/colony game, a Moria-style descent, pure sandboxes, and any first walkable proof. Those own gameplay, content, presentation, and policy.

Enabling implications (not a committed consumer roadmap): continuous 3D play from canopy to deep caves; dig/build honesty; column/Z-friendly fortress-style views; deterministic seed-and-delta reuse; and a commands-in / stale-mirror-out boundary for reuse and sandbox safety. A first walkable slice may exercise a subset of outcomes without redefining product identity.

## Non-goals

- Shipping the actual game, rules, combat, AI, spells, gas economy, or LLM/System integration here.
- Implementing building-game layers (blueprint labor, mechanisms as gameplay, room economy, designation modes) here.
- Treating the harness’s character, camera, route, seed postcard, or benchmarks as substrate features.
- LLM dependence, or requiring any one genre’s UX to use the crate.
- Owning consumer controllers, movement policy, or AI while providing the voxel collision, query, and navigation they use.

## Confirmed vision constraints

- Delivery form: Rust crate or small family of tightly scoped Rust crates.
- GPU-resident substrate; load-bearing layers stay on wgpu/WGSL with no native Metal fork (Vulkan/DX12-class portability).
- Public interaction is commands in and a potentially stale mirror plus events out; consumers do not assume live reads.
- Any in-repo walkable executable is a consumer/harness only and must not own privileged or game-specific paths.
- Zero LLM dependency; game rules live above it; future System, spell, gas, combat, AI, and building layers are out of implementation scope here (seams only where required).
- Physics, queries, collision, and navigation operate on voxel truth, not authoritative render meshes; substrate quality includes interactive remesh responsiveness and cheap idle of sparse untouched regions (hardware gates are not identity).

## Deferred design decisions

- Depth and sequence of substrate capabilities versus any first validation slice.
- Voxel resolution, LOD strategy, object-layer capacity, and fluid-model fidelity.
- Exact crate split (consumer boundary fixed; packaging is not).
- Machine-specific performance numbers and benchmark workloads (harness gates); interactive remesh and sparse idle remain product outcomes.
- Whether multiplayer-ready scope statements are carried now or later.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world executable a **required current delivery** of this repository, or only a **permitted** adjacent validation artifact?

- **Proposed answer:** Permitted as the primary validation path, not product identity; harness content, controls, and acceptance numbers are not substrate scope.
- **If different:** Making the harness mandatory adds a repository delivery obligation (still outside product identity); forbidding it confines proof to other consumers and tests.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust crate; walkable executable is a separate consumer/validation harness.
- **docs/seeds/project-boundary.md** — Binds identity to the substrate crate(s), forces a non-privileged consumer boundary for any harness, and excludes the actual game and listed game layers.
- **docs/seeds/product-one-seed.md** — First walkable proof as harness-shaped validation: streaming without raw-voxel residency, exact seed-plus-delta reload, interactive remesh as product proof, retained command/mirror architecture, wgpu/WGSL portability.
- **docs/seeds/voxel-world-substrate.md** — Substrate outcomes: natural look from voxel truth, pure seed-and-coordinate generation, exact terrain-delta restoration, matter physics, ambient cycles, object lifecycle, voxel-authoritative queries/navigation, interactive remesh, sparse cheap idle, bounded per-anchor streaming separate from delta persistence, stale mirror plus events, public registries—game genres and System remain above.
