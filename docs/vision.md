# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is the material world engine that downstream games consume: generated geology, fully mutable voxel matter with active material behavior, dynamic voxel-backed objects, smooth visual presentation of that matter, mutation-safe navigation, and world access through public commands, a deliberately stale mirror, and events. It is not a game, not a demo, and not an LLM-backed system.

## Purpose

Moria exists so multiple games can share one honest material world: a natural-looking surface and deep underground whose appearance is a view of voxel truth, not decorative geometry. The substrate stands alone with no LLM dependency. Game rules, economy, spells, combat, agents, and authored gameplay live in consumers above it. The same foundation should support adventure, fortress/colony, descent, and sandbox-style games without rewriting the world layer.

## Product boundary

**In product:** the reusable substrate and its public consumer-facing interfaces—geology-first generation, GPU-resident matter with the required material-behavior and object-lifecycle outcomes, meshing/presentation of voxel truth, mutation and observation through public commands/mirror/events, mutation-safe navigation support, and persistence/restoration of altered worlds.

**Adjacent, not identity:** a walkable-world executable may exist in this repository as a validation harness. If present, it is a separate consumer of the substrate and must use the same public interfaces available to an external game—no privileged or game-specific substrate paths. Whether shipping that harness is part of current delivery is open (see Q1).

**Out of repository / out of product:** the actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and building-game layers (blueprints, work orders, mechanism gameplay, room economies). Compatibility seams may be designed only where substrate requirements demand them; those layers are not implemented here.

**Consumer-owned (any harness or game):** character controllers, cameras, HUD, authored demo routes and seed-world content inventories, trailer presentation, and consumer-chosen performance gates or target machines. A first consumer may exercise only a slice of substrate outcomes; that does not demote the remaining required substrate outcomes to optional or future-only work.

## Required product outcomes

1. **Reusable public substrate.** Consumers integrate Moria as Rust crates and drive the world only through public interfaces. Authoritative mutation is command-driven; consumers observe via a deliberately stale mirror plus events—not by owning voxel storage or treating queries as a synchronous authoritative store.
2. **Voxel truth, normal look.** Terrain and structures read as a continuous natural world (smooth presentation with sharp cut/built features where matter demands), while physics, collision, and gameplay-relevant queries run against voxel occupancy and materials—not the render mesh. The mesh is a regenerated view, never authoritative truth.
3. **Full mutability and active matter.** Any voxel can be destroyed, moved, or placed. Dig and place are first-class. The substrate also provides these required consumer-visible matter behaviors: movable matter, responsive fluid flow, granular settling, fire and wetness interactions, ambient weather and time-driven world behavior, and structural failure including cave-ins. Algorithms, fidelity, and delivery order remain design choices; the outcome families do not.
4. **Deep-Z geology-first world.** Underground volume is first-class continuous 3D content (caves, strata, ore, dig-down discovery). Worlds are generated as layered geology with lazy materialization from seed and parameters so digging reveals true material structure, not a heightmap shell.
5. **Dynamic voxel-object lifecycle.** Interactable world objects are voxel-backed and participate in a required lifecycle: they can burn, break, block, grow, convert for physical motion, and return to voxel truth. Falling trees (cut support → physical motion → re-voxelization or breakup) are expressly required of the reusable substrate, even if a first consumer defers exercising them.
6. **Mutation-safe navigation and durable worlds.** Navigation support is derived from mutable voxel truth, invalidated after edits, continuous in Z, and usable by multiple movement classes—substrate responsibility, not consumer AI. World truth is reproducible generation plus edit deltas, including moved objects and entity state, with exact restoration on load and cross-run reuse of altered worlds.

## Future products and enabling implications

Downstream consumers (not this product) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style descent experience, pure sandbox tools, and any walkable validation harness. Enabling implications only: the required substrate outcomes above so those games can attach rules, agents, pricing, and content without forking the world engine. Their gameplay, controllers, characters, UI, and authored content are not Moria scope. A first-consumer slice that omits fluid flow, fire, integrity, object felling, or similar does not redefine substrate identity.

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

- Exact crate graph, APIs, algorithms, data layouts, meshing methods, and simulation implementations
- Fidelity, performance budgets, and delivery order among required matter, object, navigation, and persistence outcomes
- Voxel scale, LOD, and streaming policy details
- Harness/demo content, controls, platforms, and numeric acceptance thresholds (consumer/design concerns)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable **mandatory current delivery** alongside the substrate crates, or only a **permitted adjacent artifact**?

- **Proposed safe answer:** Permitted adjacent artifact only—the current product commitment is the reusable substrate and its public interfaces; a harness may live in-repo for validation but is not required to define or complete the product.
- **If answered differently:** Making the harness mandatory keeps substrate identity unchanged but expands current delivery to include a separate walkable consumer; treating harness-owned content, controls, or machine targets as substrate requirements would incorrectly widen product identity.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world substrate consumed as a Rust crate, with the walkable-world executable called out as a separate consumer/harness.
- **`docs/seeds/project-boundary.md`:** Fixes product identity on the substrate crates, places the real game outside the repo, permits a public-API-only validation harness, and excludes game/System/building layers while allowing necessary compatibility seams.
- **`docs/seeds/product-one-seed.md`:** Motivates a first-consumer proof of a diggable, walkable material world and a partial exercise of substrate outcomes; its controller, seed content, milestones, machines, and numeric gates stay consumer/harness concerns. Its first-slice omissions do not drop required substrate matter, object, navigation, or restoration outcomes.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes substrate outcome families at product altitude—normal look over voxel truth, full mutability including movement, active matter behaviors, dynamic object lifecycle, deep-Z geology-first generation, mutation-safe navigation, persistence with exact restoration and object/entity journals, and the GPU command/mirror/event observation model—while leaving mechanisms and build order to design.
