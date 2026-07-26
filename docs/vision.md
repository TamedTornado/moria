# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for matter, world generation, derived presentation of voxel truth, mutation, observation, collision, navigation support, streaming, and persistence—not a game and not a game-rules layer.

## Purpose

Moria exists so multiple downstream games can share one material world substrate: a natural-looking surface over continuous, fully mutable voxel truth, with deep underground as first-class space. Game rules, content policy, camera, UX, and authored presentation stay above the substrate. The substrate stands alone with no LLM or System dependency.

## Product boundary

**In product:** the reusable voxel-world substrate and its public consumer interfaces—including matter behavior, smooth derived representation and voxel-driven dressing coherence, mutation-safe navigation data, streaming, and persistence contracts.

**Out of product / repository:** the actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and building-as-gameplay layers. Compatibility seams may be designed where substrate outcomes require them; those layers are not implemented here.

**Presentation split:** the substrate owns that voxel truth can be presented as a smooth derived view and that dressing stays coherent with matter. Games own camera, UX, authored presentation, and content choices.

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness that consumes only public interfaces. Its controller, character, camera, authored route, scene content, workload, hardware targets, and metrics are consumer-owned. Whether that harness is a required current delivery is unresolved (see Q1). The harness’s already-defined first slice (when delivered) is a benchmarked walkable generated region proving a natural-looking, continuous deep voxel world with public dig/place, streaming, collision, and reload—not part of substrate identity.

## Required product outcomes

1. **GPU-resident matter truth with derived views.** Voxels are authoritative matter. Smooth extracted representation and voxel-driven dressing are non-authoritative views kept coherent with that truth, not a second world model.
2. **Natural-looking continuous mutable world.** Surface through deep underground is one diggable/placeable 3D volume. Worlds generate as diggable geology, materialize sparsely enough for large regions, and stream around active cameras or agents with distinct render, simulation, aggregate, and cold lifecycles.
3. **Matter and interactive behavior families.** The substrate is responsible for movable matter, voxel-backed interactive objects, cellular material behavior, fluids, granular response, fire and ambient behavior, and structural failure. First-slice depth and sequence are design concerns; these families are not optional later add-ons or consumer-owned gameplay.
4. **Isolated consumer contract.** Commands in; an intentionally coarse/stale mirror plus events out; collision and inspection only through public interfaces. Consumers do not get privileged voxel access or synchronous authoritative query state as the isolation model.
5. **Mutation-safe navigation support.** Navigation data is derived from matter and invalidated by mutation. This is substrate pathfinding support, not agents or game AI.
6. **Persistence and reuse.** Edit deltas plus object/entity journals retain change so worlds reload and can be reused across runs or modes—not merely “scars look intact.”

## Future products and enabling implications

Downstream consumers (not this product) include a System/LLM ARPG, fortress/colony and descent-style games, pure sandbox play, and the walkable-world harness when that adjacent artifact is delivered (see Q1). Those products motivate the substrate’s breadth; they do not import gameplay, controllers, characters, authored content, or game policy into this product. Long-horizon game modes and milestone catalogs are not a committed multi-game roadmap here.

## Non-goals

- Shipping the actual game or any game-rules layer in this repository.
- Implementing System/LLM, spell, gas, combat, AI, or building gameplay layers here.
- Owning harness/demo character control, camera policy, demo routes, hardware gates, or marketing milestone content as product scope.
- Treating a first consumer slice’s narrowed depth as a narrowing of substrate identity or required outcome families.

## Confirmed vision constraints

- Integration surface is Rust crates for external and in-repo consumers alike.
- Consumer isolation: harness and games share only public substrate interfaces (commands in; coarse/stale mirror and events out); privileged access is disallowed.
- Substrate independence: no LLM/System dependency in the product core.
- Load-bearing graphics paths stay on portable wgpu/WGSL; a native-Metal fork is rejected because Metal/Vulkan/DX12 portability is a crate objective. Machine-specific limits and performance gates are not product identity.
- Explicit product exclusions above are binding now, not deferred polish.

## Deferred design decisions

- Crate family split, internal layering, algorithms, data layouts, and API shape.
- Voxel scale, meshing strategy, LOD, streaming ring mechanics, and persistence encoding.
- Delivery depth and sequence within the required matter-behavior families.
- Performance budgets, target machines, and validation workloads (design and harness concerns).
- First vertical-slice contents and milestone order for proving the substrate.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** alongside the substrate crates, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted only—ship criteria for “current product” center on the reusable substrate; a harness may be built to validate public interfaces but is not mandatory identity.
- **If answered differently:** Substrate identity is unchanged. Requiring the harness adds a separate delivery obligation for the already-defined adjacent first slice (benchmarked walkable generated region proving continuous deep voxel world, public dig/place, streaming, collision, and reload). It still must not pull controller, character, camera, route, content, hardware, or metrics into product identity.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident Rust voxel substrate and positions the walkable-world executable as a separate non-game validation consumer.
- **`docs/seeds/project-boundary.md`:** Fixes repository identity on the substrate crates, excludes game and listed future layers (including AI), and requires public-interface-only consumer access if a harness exists.
- **`docs/seeds/product-one-seed.md`:** Defines the first adjacent proof slice and wgpu/WGSL portability motivation; narrows only that slice’s depth and keeps demo ownership and machine gates out of substrate identity.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes core substrate outcome families—natural look over voxel truth, full mutability, deep Z, generation, GPU-resident matter with commands/mirror/events, matter behaviors, navigation support, streaming, and persistence—while remaining a design-depth source, not a mechanism checklist for this brief.
