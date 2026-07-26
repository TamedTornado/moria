# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, exposed as a **Rust crate** or a small family of tightly scoped Rust crates for external games. It is an engine-layer world foundation—not a game, demo title, or LLM runtime.

A **required adjacent first delivery** is a walkable generated-world validation executable that exercises the substrate through public interfaces. That executable is not part of substrate identity.

## Purpose

Moria exists so multiple games can share one material world: a natural-looking continuous surface over fully mutable voxel truth, with deep underground geology, reactive matter and ambient world behavior, matter-coupled dressing and type-appropriate natural objects, mutation-safe navigation support, and a command/mirror/event integration surface. Game rules, pricing policy, and LLM systems sit above the substrate. The substrate must stand alone with **zero LLM dependency**.

## Product boundary

**In product**

- Geological generation, sparse GPU-resident matter, smooth visual reconstruction of voxel truth, dig/place-class mutation, reactive matter, ambient time/weather/ecology behavior, surface dressing and voxel-backed natural objects with type-appropriate lifecycles, mutation-safe navigation data, streaming, substrate persistence (worldgen + edit deltas and journals), and the public command / stale-mirror / event integration contract.
- Compatibility seams only where substrate reuse requires them—not those layers’ implementations.

**Out of product (adjacent or downstream)**

- The actual game and all game rules: System/LLM, spells, gas policy, combat, AI, and building-game layers (work orders, mechanism gameplay, room economy, fortress/ARPG policy).
- Authored demo content, character controllers, cameras, presentation polish, benchmark workloads, platforms, and machine-specific acceptance targets for the walkable validation executable.
- Machine-specific benchmarks, device atomics limits, and provisional hardware performance gates (harness- or environment-owned, not product identity).

**Required adjacent delivery**

A walkable-world validation executable **must** ship as a first delivery alongside the substrate crates. It is a separate, non-privileged consumer of the same public interfaces available to an external game—not a game layer and not substrate identity. Its controller, authored region, presentation, workloads, platforms, and machine-specific targets remain harness-owned. A first consumer slice may omit some substrate outcomes for its own demo scope; that does not make those outcomes optional for the reusable product.

## Required product outcomes

1. **Natural-looking continuous world, voxel-authoritative and mutable all the way down.** Terrain reads as a normal surface world via smooth extraction of material + density, while physics, queries, and gameplay always run against voxel truth. The mesh is a regenerated view—never authoritative, never saved. Consumers destroy, place, and reshape matter through public APIs; deep Z is first-class continuous geology (strata, caves, ore, aquifers/voids) produced geology-first and materialized lazily—not a heightmap with painted rock.
2. **Reactive matter and ambient natural-world behavior.** Active fluid flow and material interactions; fire and wetness; granular settling; structural support failure with collapse; and substrate-owned day/night and seasons, weather fronts that affect water and wetness (including storms and drought), and fire ecology at landscape scale. Simulation mechanisms and scheduling are design.
3. **Matter-coupled surface life and type-appropriate object behavior.** Grass and clutter dressing derive from voxel/surface state so they stay consistent under fire, dig, and other matter changes. Things that can burn, break, or block are voxel-backed rather than pure decoration. Object types carry type-appropriate lifecycles—for example trees grow and can fall, then re-voxelize where they land or break into log items; boulders and bushes have different interaction behaviors—not one universal full lifecycle on every object. Game movement, AI, and content authoring remain consumer-owned.
4. **Mutation-safe navigation for consumers.** The substrate supplies voxel-derived navigation data and continuous-3D movement support that stays consistent with world mutation; AI policy and game-specific movement rules stay downstream.
5. **Streaming and persistence for reuse.** The world streams around active interest. Substrate-owned state persists as worldgen plus edit deltas, with object/entity journals and cross-run reuse. Exact restoration applies to the Product One same-seed-plus-edit-deltas save/load path; general journals support persistence without extending that exact-restoration modifier to all journaled state. Harness size targets and save-slot policy are not product scope.
6. **Command / stale-mirror / event integration for reuse.** Consumers issue commands in and observe an explicitly stale aggregate mirror plus events out—the GPU-resident coupling contract. Nothing above the matter layer needs direct voxel access. The same crates support ARPG, fortress, descent, or sandbox games as pure consumers without embedding their policy.

## Future products and enabling implications

Future **consumers** (not this product) include a System/LLM ARPG, a DF-style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate mutability, deep Z, reactive matter, ambient world behavior, object lifecycles, navigation, persistence, and clean integration; they do **not** pull gameplay, controllers, content, or presentation into Moria.

The required walkable validation executable demonstrates that the world is fully material and traversable and proves dig/place, streaming, and persistence through public interfaces; its specific region, character, route, presentation, workloads, and platforms remain consumer-owned. Omission of reactive matter, ambient sim, or object felling from that first slice does not defer those outcomes out of the substrate product.

## Non-goals

- Implementing game rules, combat, AI, spells, gas, System/LLM, or building-game layers in this repository.
- Treating the validation harness’s controller, content, UI, save policy, or performance gates as substrate features or product identity.
- Making the substrate depend on an LLM or embed game policy (gas pricing, work orders, room assignment).
- A native Metal (or other vendor) fork in load-bearing substrate layers.

## Confirmed vision constraints

- **Form factor:** Rust crate or small family of tightly scoped Rust crates.
- **Runtime character:** GPU-resident voxel-world substrate.
- **Graphics portability:** substrate crates stay on wgpu/WGSL with no native Metal fork, so the same load-bearing path remains portable across Metal, Vulkan, and DX12.
- **Consumer equality:** the walkable validation executable is a non-privileged consumer of public interfaces.
- **First delivery:** that adjacent walkable validation executable is a required first delivery (public-interface dig/place proof, streaming/persistence validation, and a downloadable demo artifact), without importing its harness-owned details into substrate identity.
- **Independence:** zero LLM dependency; game layers live outside the substrate; System, LLM, spell, gas, combat, AI, and building layers are not implemented here (seams only if required).

## Deferred design decisions

- Crate split and internal module boundaries (consumer boundary is fixed; packaging is not).
- Voxel scale, meshing/LOD strategy, object-layer capacity, and how fluid, ambient, granular, and integrity sims are scheduled or resolved.
- Delivery sequence and first-slice depth for substrate outcomes (identity and outcome families are fixed; milestone order is design).
- Harness-specific content, controls, platforms, benchmarks, and performance gates for the required adjacent executable.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident voxel-world substrate (Rust crate) and positions the walkable executable as a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binds product identity to the reusable substrate crates, excludes the actual game and named game layers, and requires non-privileged public-interface consumption for any harness.
- **`docs/seeds/product-one-seed.md`:** Pins first-delivery “done” for the adjacent walkable consumer (public dig/place proof, streaming/persistence validation, downloadable demo) and the wgpu/WGSL portability constraint; its demo omissions do not narrow reusable product outcomes.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes substrate outcome families—natural look over voxel truth, full mutability, deep-Z geology, reactive matter, ambient time/weather/ecology, type-appropriate object behavior, mutation-safe nav, journals and cross-run persistence, command/stale-mirror/events, and reuse across future games.
