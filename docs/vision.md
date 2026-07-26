# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, exposed as a **Rust crate** or a small family of tightly scoped Rust crates for external games. It is an engine-layer world foundation—not a game, demo title, or LLM runtime.

## Purpose

Moria exists so multiple games can share one material world: a natural-looking continuous surface over fully mutable voxel truth, with deep underground geology, matter-coupled dressing and objects, and public query/mutation interfaces. Game rules, pricing policy, and LLM systems sit above the substrate. The substrate must stand alone with **zero LLM dependency**.

## Product boundary

**In product**

- Geological generation, sparse GPU-resident matter, smooth visual reconstruction of voxel truth, dig/place-class mutation, surface dressing and voxel-backed natural objects, streaming, persistence, and public consumer APIs for verbs, queries, and events.
- Compatibility seams only where substrate reuse requires them (for later game or System attachment)—not those layers’ implementations.

**Out of product (adjacent or downstream)**

- The actual game and all game rules: System/LLM, spells, gas policy, combat, AI, and building-game layers (work orders, mechanism gameplay, room economy, fortress/ARPG policy).
- Authored demo content, character controllers, cameras, presentation polish, and acceptance scenarios for any walkable validation executable.

**Adjacent validation artifact**

A walkable-world executable **may** exist as a separate consumer that exercises the substrate. If present, it must use the **same public interfaces** available to an external game—no privileged or game-specific substrate paths. Whether that executable is a current delivery is **unresolved (Q1)**; it is not part of substrate identity.

## Required product outcomes

1. **Natural-looking continuous world, voxel-authoritative.** Terrain and structures read as a normal surface world (smooth extraction of material + density), while physics, queries, and gameplay consumers always run against voxel truth. The mesh is a regenerated view—never authoritative and never the save format.
2. **Mutable everywhere, all the way down.** Consumers can destroy, place, and reshape matter through public APIs; digs and builds change the material world, not decorative props outside it.
3. **Deep Z as first-class space.** Underground is real content: continuous 3D geology (strata, caves, ore, aquifers/voids) with lazy materialization so sparse regions stay cheap until touched.
4. **Geology-first generation.** Worlds are produced as geology and columns that materialize into bricks on demand—not a heightmap with rock painted underneath—so dig-down honesty and deep play are structural, not cosmetic.
5. **Matter-coupled life on the surface.** Interactable natural objects (trees, rocks, and similar) are voxel-backed; grass and clutter dressing derive from voxel/surface state so they stay consistent under fire, dig, and other matter changes.
6. **Reusable integration surface.** Streaming around active interest, persistence as worldgen function plus edit deltas, and a verb/query boundary so nothing above the matter layer needs direct voxel access—supporting ARPG, fortress, descent, or sandbox games as pure consumers of the same crates.

High-level substrate responsibilities also include fluid bodies and flow foundations, structural support/collapse readiness, and ambient material-rule hooks (for later games). **Delivery depth and sequence are design decisions**, not a narrowing of this identity.

## Future products and enabling implications

Future **consumers** (not this product) include a System/LLM ARPG, a DF-style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate mutability, deep Z, matter physics, and clean APIs; they do **not** pull gameplay, controllers, content, or presentation into Moria.

A walkable “proof of world” consumer may demonstrate that the world is fully material and traversable; its specific region, character, route, and benchmark scene remain consumer-owned (see Q1 for delivery status).

## Non-goals

- Implementing game rules, combat, AI, spells, gas, System/LLM, or building-game layers in this repository.
- Treating the validation harness’s controller, content, UI, or performance gates as substrate features.
- Making the substrate depend on an LLM or embed game policy (gas pricing, work orders, room assignment).

## Confirmed vision constraints

- **Form factor:** Rust crate or small family of tightly scoped Rust crates.
- **Runtime character:** GPU-resident voxel-world substrate.
- **Consumer equality:** any in-repo walkable executable, if present, is a non-privileged consumer of public interfaces.
- **Independence:** zero LLM dependency; game layers live outside the substrate.
- **Explicit exclusions:** System, LLM, spell, gas, combat, AI, and building layers are not implemented here (seams only if required).

## Deferred design decisions

- Crate split and internal module boundaries (consumer boundary is fixed; packaging is not).
- Voxel scale, meshing/LOD strategy, object-layer capacity, and fluid/integrity depth.
- How much of the full matter/sim surface ships in the first design slice versus later increments.
- Harness-specific content, controls, platforms, and performance gates—if a harness is delivered.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **mandatory current delivery** alongside the substrate crates, or only **permitted** as an adjacent artifact?

- **Proposed safe answer:** Permitted and encouraged for proving public APIs, but **not** required to define or ship the product; the product remains the substrate crates.
- **If different:** Making it mandatory keeps harness delivery in scope without importing its controller, content, or performance targets into product identity; treating it as required product surface would blur substrate vs. demo ownership.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident voxel-world substrate (Rust crate) and positions the walkable executable as a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binds product identity to the reusable substrate crates, excludes the actual game and named game layers, and requires non-privileged public-interface consumption for any harness.
- **`docs/seeds/product-one-seed.md`:** Describes a first walkable consumer slice (demo world, controller, proof dig/place, harness metrics) that motivates substrate outcomes without redefining product ownership.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes the substrate’s outcome families—natural look over voxel truth, full mutability, deep-Z geology, matter-coupled vegetation, streaming/persistence, and reusable verb/query layering for multiple future games.
