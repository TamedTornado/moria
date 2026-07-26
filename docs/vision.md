# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. Downstream games consume it for generated geology, fully mutable voxel matter with active material behavior, dynamic voxel-backed objects, smooth presentation of that matter, mutation-safe navigation, scalable streaming of active versus cold world state, and world access through public commands, a deliberately stale mirror, and events. It is not a game, not a demo, and not an LLM-backed system.

## Purpose

Moria exists so multiple games can share one honest material world: a natural-looking surface and deep underground whose appearance is a view of voxel truth, not decorative geometry. The substrate stands alone with no LLM dependency. Game rules, economy, spells, combat, agents, and authored gameplay live in consumers above it. The same foundation should support adventure, fortress/colony, descent, and sandbox-style games without rewriting the world layer.

## Product boundary

**In product:** the reusable substrate and its public interfaces—geology-first generation, GPU-resident matter with required material-behavior and object-lifecycle outcomes, meshing/presentation of voxel truth, mutation and observation through public commands/mirror/events, mutation-safe navigation, streaming of active versus cold world state with bounded idle residency, and persistence of altered worlds.

**Adjacent, not identity:** a walkable-world executable may exist here as a validation harness. If present, it is a separate consumer and must use the same public interfaces available to an external game—no privileged or game-specific substrate paths. Whether shipping that harness is part of current delivery is open (see Q1).

**Out of repository / out of product:** the actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and building-game layers (blueprints, work orders, mechanism gameplay, room economies). Compatibility seams may be designed only where substrate requirements demand them; those layers are not implemented here.

**Consumer-owned:** character controllers, cameras, HUD, authored demo routes and seed-world content, trailer presentation, and consumer-chosen performance gates or target machines. A first consumer may exercise only a slice of substrate outcomes without demoting the rest to optional or future-only work.

## Required product outcomes

1. **Reusable public substrate.** Consumers integrate Moria as Rust crates and drive the world only through public interfaces. Authoritative mutation is command-driven; consumers observe via a deliberately stale mirror plus events—not by owning voxel storage or treating queries as a synchronous authoritative store.
2. **Voxel truth, normal look.** Terrain and structures read as a continuous natural world (smooth presentation with sharp cut/built features where matter demands), while physics, collision, and gameplay-relevant queries run against voxel occupancy and materials—not the render mesh. The mesh is a regenerated view, never authoritative truth.
3. **Full mutability and active matter.** Any voxel can be destroyed, moved, or placed. Dig and place are first-class. Required matter behaviors also include movable matter, responsive fluid flow, granular settling, fire and wetness interactions, ambient weather and time-driven world behavior, and structural failure including cave-ins. Algorithms, fidelity, and delivery order remain design choices; the outcome families do not.
4. **Deep-Z geology-first world.** Underground volume is first-class continuous 3D content (caves, strata, ore, dig-down discovery). Worlds generate as layered geology with lazy materialization from seed and parameters so digging reveals true material structure, not a heightmap shell. Idle or unfocused world stays cheap to hold until touched.
5. **Dynamic voxel-object lifecycle.** Things that can burn, break, or block are voxel-backed matter, not pure decoration. Object classes use the lifecycle capabilities that apply to them—trees fall under physical motion and re-voxelize or break up; boulders split or roll; flammable brush burns—without requiring every interactable object to exercise every lifecycle predicate. Falling trees (cut support → physical motion → re-voxelization or breakup) are expressly required of the reusable substrate, even if a first consumer defers them.
6. **Navigation, streaming, and durable worlds.** Navigation support is derived from mutable voxel truth, invalidated after edits, continuous in Z, and usable by multiple movement classes. The substrate streams active versus cold world state so large regions remain workable: runtime activation where needed, bounded residency when idle, and durable handling of cold state. World truth is reproducible generation plus edit deltas; seed-plus-delta saves restore exactly for that save form. Object and entity journals and cross-run reuse of altered worlds are required separately; exact restoration is not asserted for moved objects or entity/script state.

## Future products and enabling implications

Downstream consumers (not this product) include a System/LLM-driven ARPG, fortress/colony and descent experiences, sandbox tools, and any walkable validation harness. Enabling implications only: the required substrate outcomes above so those games can attach rules, agents, pricing, and content without forking the world engine. Their gameplay, controllers, UI, and authored content are not Moria scope.

## Non-goals

- Shipping a playable game or game systems in this product
- Implementing System/LLM, spells, gas, combat, AI, or building-game layers here
- Treating harness controls, demo seed content, cameras, or consumer benchmarks as substrate identity
- LLM dependency inside the substrate
- Heightmap-with-props worlds that cannot be dug, moved, and rebuilt as true matter

## Confirmed vision constraints

- **Ecosystem:** Rust crate (or small crate family) consumption boundary.
- **Consumer isolation:** adjacent consumers, including any validation harness, use only public interfaces; no privileged game paths in the substrate.
- **Standalone substrate:** zero LLM dependency; game policy injects above, not inside, the world layer.
- **GPU-resident world with observation model:** live matter is GPU-resident; consumers send commands in and receive a deliberately stale mirror plus events out.
- **Explicit exclusions:** game rules and future System, spell, gas, combat, AI, and building layers stay out of this product (seams only where substrate needs demand).

## Deferred design decisions

- Exact crate graph, APIs, algorithms, data layouts, meshing methods, and simulation
- Fidelity, performance budgets, and delivery order among required matter, object, navigation, streaming, and persistence
- Voxel scale, LOD, and how streaming policy is implemented
- Harness/demo content, controls, platforms, and numeric thresholds

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable **mandatory current delivery** alongside the substrate crates, or only a **permitted adjacent artifact**?

- **Proposed safe answer:** Permitted adjacent artifact only—current commitment is the reusable substrate and its public interfaces; a harness may live in-repo for validation but is not required to complete the product.
- **If answered differently:** Making the harness mandatory keeps substrate identity unchanged but expands current delivery to include a separate walkable consumer; treating harness-owned content or machine targets as substrate requirements would incorrectly widen product identity.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world substrate consumed as a Rust crate; walkable-world executable is a separate harness.
- **`docs/seeds/project-boundary.md`:** Fixes product identity on the substrate crates, places the real game outside the repo, permits a public-API-only validation harness, and excludes game/System/building layers.
- **`docs/seeds/product-one-seed.md`:** Motivates a first-consumer proof of a diggable, walkable material world and partial exercise of substrate outcomes, including streaming and seed-plus-delta persistence; harness controls, content, machines, and gates stay consumer concerns.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes substrate outcome families—normal look over voxel truth, full mutability, active matter, class-appropriate object lifecycles (falling trees required), deep-Z geology-first generation with lazy materialization, mutation-safe navigation, active-versus-cold streaming with bounded idle residency, edit deltas with object/entity journals and cross-run reuse, and the GPU command/mirror/event model.
