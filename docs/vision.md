# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, exposed as a **Rust crate** or a small family of tightly scoped Rust crates for external games. It is an engine-layer world foundation—not a game, demo title, or LLM runtime.

## Purpose

Moria exists so multiple games can share one material world: a natural-looking continuous surface over fully mutable voxel truth, with deep underground geology, reactive matter, matter-coupled dressing and objects, mutation-safe navigation support, and a command/mirror/event integration surface. Game rules, pricing policy, and LLM systems sit above the substrate. The substrate must stand alone with **zero LLM dependency**.

## Product boundary

**In product**

- Geological generation, sparse GPU-resident matter, smooth visual reconstruction of voxel truth, dig/place-class mutation, reactive matter (fluids, ambient fire/wetness, granular settle, structural failure), surface dressing and voxel-backed natural objects with full object lifecycle, mutation-safe navigation data, streaming, full substrate persistence, and the public command / stale-mirror / event integration contract.
- Compatibility seams only where substrate reuse requires them (for later game or System attachment)—not those layers’ implementations.

**Out of product (adjacent or downstream)**

- The actual game and all game rules: System/LLM, spells, gas policy, combat, AI, and building-game layers (work orders, mechanism gameplay, room economy, fortress/ARPG policy).
- Authored demo content, character controllers, cameras, presentation polish, and acceptance scenarios for any walkable validation executable.
- Machine-specific benchmarks, device atomics limits, and provisional hardware performance gates (harness- or environment-owned, not product identity).

**Adjacent validation artifact**

A walkable-world executable **may** exist as a separate consumer that exercises the substrate. If present, it must use the **same public interfaces** available to an external game—no privileged or game-specific substrate paths. Whether that executable is a current delivery is **unresolved (Q1)**; it is not part of substrate identity. A first consumer slice may omit some substrate outcomes for its own demo scope; that does not make those outcomes optional for the reusable product.

## Required product outcomes

1. **Natural-looking continuous world, voxel-authoritative and mutable all the way down.** Terrain reads as a normal surface world via smooth extraction of material + density, while physics, queries, and gameplay always run against voxel truth. The mesh is a regenerated view—never authoritative, never saved. Consumers destroy, place, and reshape matter through public APIs; deep Z is first-class continuous geology (strata, caves, ore, aquifers/voids) produced geology-first and materialized lazily—not a heightmap with painted rock.
2. **Reactive matter as product outcomes.** Active fluid flow and material interactions; ambient fire and wetness behavior; granular settling; and structural support failure with collapse. These are substrate responsibilities, not format readiness or later-game hooks.
3. **Matter-coupled surface life and object lifecycle.** Grass and clutter dressing derive from voxel/surface state so they stay consistent under fire, dig, and other matter changes. Voxel-backed natural objects (trees, rocks, and similar) are breakable and blocking; trees support falling via rigid conversion and re-voxelization where they land, and growth as substrate-owned object behavior. Game movement, AI, and content authoring remain consumer-owned.
4. **Mutation-safe navigation for consumers.** The substrate supplies voxel-derived navigation data and continuous-3D movement support that stays consistent with world mutation; AI policy and game-specific movement rules stay downstream.
5. **Streaming and persistence that reconstructs exactly.** The world streams around active interest. Substrate-owned world state persists as worldgen plus edit deltas and journals for changed or moved objects, enabling cross-run reuse; load restores that state exactly. Harness size targets and save-slot policy are not product scope.
6. **Command / stale-mirror / event integration for reuse.** Consumers issue commands in and observe an explicitly stale aggregate mirror plus events out—the GPU-resident coupling contract. Nothing above the matter layer needs direct voxel access. The same crates support ARPG, fortress, descent, or sandbox games as pure consumers without embedding their policy.

## Future products and enabling implications

Future **consumers** (not this product) include a System/LLM ARPG, a DF-style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate mutability, deep Z, reactive matter, object lifecycle, navigation, persistence, and clean integration; they do **not** pull gameplay, controllers, content, or presentation into Moria.

A walkable “proof of world” consumer may demonstrate that the world is fully material and traversable; its specific region, character, route, and benchmark scene remain consumer-owned (see Q1 for delivery status). Omission of reactive matter or object felling from that first slice does not defer those outcomes out of the substrate product.

## Non-goals

- Implementing game rules, combat, AI, spells, gas, System/LLM, or building-game layers in this repository.
- Treating the validation harness’s controller, content, UI, save policy, or performance gates as substrate features.
- Making the substrate depend on an LLM or embed game policy (gas pricing, work orders, room assignment).
- A native Metal (or other vendor) fork in load-bearing substrate layers.

## Confirmed vision constraints

- **Form factor:** Rust crate or small family of tightly scoped Rust crates.
- **Runtime character:** GPU-resident voxel-world substrate.
- **Graphics portability:** substrate crates stay on wgpu/WGSL with no native Metal fork, so the same load-bearing path remains portable across Metal, Vulkan, and DX12.
- **Consumer equality:** any in-repo walkable executable, if present, is a non-privileged consumer of public interfaces.
- **Independence:** zero LLM dependency; game layers live outside the substrate.
- **Explicit exclusions:** System, LLM, spell, gas, combat, AI, and building layers are not implemented here (seams only if required).

## Deferred design decisions

- Crate split and internal module boundaries (consumer boundary is fixed; packaging is not).
- Voxel scale, meshing/LOD strategy, object-layer capacity, and how fluid, ambient, granular, and integrity sims are scheduled or resolved.
- Delivery sequence and first-slice depth for substrate outcomes (identity and outcome families are fixed; milestone order is design).
- Harness-specific content, controls, platforms, benchmarks, and performance gates—if a harness is delivered.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **mandatory current delivery** alongside the substrate crates, or only **permitted** as an adjacent artifact?

- **Proposed safe answer:** Permitted and encouraged for proving public APIs, but **not** required to define or ship the product; the product remains the substrate crates.
- **If different:** Making it mandatory keeps harness delivery in scope without importing its controller, content, or performance targets into product identity; treating it as required product surface would blur substrate vs. demo ownership.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident voxel-world substrate (Rust crate) and positions the walkable executable as a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binds product identity to the reusable substrate crates, excludes the actual game and named game layers, and requires non-privileged public-interface consumption for any harness.
- **`docs/seeds/product-one-seed.md`:** Describes a first walkable consumer slice and pins substrate crates to wgpu/WGSL portability; its demo omissions do not narrow reusable product outcomes.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes substrate outcome families—natural look over voxel truth, full mutability, deep-Z geology, reactive matter, object lifecycle, mutation-safe nav, full persistence, command/stale-mirror/events, and reuse across future games.
