# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world **substrate**, delivered as a Rust crate or small family of tightly scoped Rust crates. It is an engine-layer world foundation for matter, generation, mutation, queries, material physics, navigation from voxel truth, and persistence—not a game, demo, or content product.

## Purpose

Moria exists so multiple future games can share one material world stack: a natural-looking surface and deep underground whose *truth* is mutable voxels, not decorative heightmap scenery. Game rules and presentation live above it; the substrate provides dig-anywhere geology, continuous 3D space, material and environmental behavior, and clean consumer integration without baking any one game in.

## Product boundary

**In product (Moria):**

- The reusable voxel-world substrate and its public Rust interfaces (commands in, queries against a stale/coarse mirror, events out).
- World generation; matter; visual meshing of voxel truth; mutation (including dig/place); collision and substrate-owned matter physics (fluids; ambient day/night, seasons, weather, and fire ecology; vegetation growth; structural integrity/collapse; granular behavior; particle/dynamic-matter coupling); navigation derived from voxels and kept valid under mutation; sparse streaming; persistence of seed, edit scars, and substrate-managed object/entity state for cross-run reuse.
- Compatibility seams external games need—without implementing those games here.

**Adjacent, not the product:**

- The actual game is a separate downstream consumer, outside this repository's product identity.
- A walkable-world executable may exist as an adjacent validation harness. If present, it must use the same public interfaces available to an external game and must not own privileged substrate paths. Whether it is a required current delivery is open (see Q1). Its controls, camera, route, seed content, presentation, workloads, and performance gates are harness- or consumer-owned, not product scope.
- Future titles (System ARPG, fortress/colony, descent/roguelike, pure sandbox) are consumers, not Moria.

**Out of product:** game rules; System/LLM features; spells; gas policy; combat; AI agents and game labor; building/gameplay layers (UI, blueprints-as-gameplay, work orders, room economy, mechanisms-as-game entities).

## Required product outcomes

Downstream design must make these product-level outcomes true:

1. **Substrate, not game; public integration contract.** The same crate stack can underpin different game modes; game policy stays above. Mutation and inspection use public commands, queries against a stale/coarse mirror, and outbound events—nothing above the matter layer touches voxels as a private back door. Simulation is GPU-resident as part of that contract.
2. **Natural world over fully mutable voxel truth; deep Z first-class.** Surface terrain reads as ordinary landscape while the grid remains authoritative matter; the render mesh is a regenerated view, not saved authority. Any voxel can be destroyed, moved, or placed; underground volumes (strata, caves, ore, aquifers) are real continuous-3D space.
3. **Geology-first generation and interactable surface life.** Worlds generate as layered geology with lazy materialization so large regions stay tractable. Interactable vegetation and clutter that can burn, break, or block is voxel-backed; pure dressing stays derived from voxel state. Growing vegetation (trees and similar voxel objects) advances through growth stages over world time, including canopy expansion.
4. **Material and environmental simulation as substrate behavior.** Consumers can rely on fluid bodies and flow; day/night and seasonal progression that drive light, growth ticks, and snow-line change (plus weather and fire ecology); structural integrity with collapse when support fails; granular settle of unsupported granular matter; and coupling between voxel matter and dynamic particle/rigid proxies. Depth and algorithms are design; presence of these behaviors is not.
5. **Mutable-world navigation.** Navigation is derived from voxel truth, invalidated when that truth mutates, and usable for continuous-3D movement classes (walk, climb, fly, burrow, swim). Agents, pathing policy, and game AI remain consumer-owned.
6. **Operable sparse world with full substrate restoration.** Streaming and sparsity keep large regions tractable. Persistence restores seed plus edit scars *and* substrate-managed object/entity lifecycle state so worlds and substrate-owned objects can be reused across runs. Consumer game-state policy is outside this guarantee.

## Future products and enabling implications

Described consumers—System-backed ARPG, DF-style fortress/colony, Moria-style deep descent, pure sandbox—sit above Moria. Enabling implications only: expose matter, generation metadata/POI hooks, mutation/query/event surfaces, navigation, material physics, and seam points those modes need—not their gameplay, content, controllers, characters, UI, or policies.

A walkable third-person “product one” demo is an adjacent proof vehicle for the substrate claim (“fully material world, not heightmap with props”), not a second product identity. Its first slice may postpone seasons, vegetation growth, weather sim, and other substrate behaviors without demoting those outcomes from Moria.

## Non-goals

- Shipping a game, combat loop, progression, or authored campaign in this product.
- Implementing System/LLM, spells, gas metering, AI agents, or fortress labor/economy.
- Implementing building *gameplay* layers (player building UI, work-order systems, mechanism puzzle rules) inside Moria.
- Treating harness routes, characters, cameras, seed content, machine-specific perf gates, or milestone marketing as product requirements.
- Making the substrate depend on any LLM.

## Confirmed vision constraints

- **Ecosystem:** Rust crate or small family of tightly scoped Rust crates; intended for Rust consumers.
- **Consumer isolation:** any in-repo validation harness, if present, consumes only public interfaces—no privileged paths reserved for the harness.
- **Layering:** game rules and named future game systems stay out; seams may be designed, those layers must not be implemented here.
- **GPU-resident integration:** GPU-resident simulation with commands-in / stale-or-coarse-mirror-queries / events-out.
- **GPU backend portability:** load-bearing work stays on portable GPU abstraction (wgpu/WGSL); no load-bearing native Metal fork. Hardware-specific acceptance numbers stay out of product scope.
- **Stand-alone substrate:** zero LLM dependency in the world layer.

## Deferred design decisions

- Crate split and workspace layout (boundary is fixed; packaging is design).
- Voxel resolution, meshing/LOD strategy, object-layer scaling, and *depth/algorithms* for fluids, integrity, ambient weather/time/seasons, vegetation growth, granular, and particle coupling (presence required above).
- Delivery sequence and first-consumer-slice depth.
- Harness-only choices: controllers, cameras, seed content, benchmarks, target machines, acceptance numbers.
- Multiplayer and long-horizon roadmap.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** alongside the substrate crates, or only a **permitted** adjacent validation artifact?

- **Proposed answer:** Permitted adjacent harness only—not a committed required delivery of this vision pass. Product identity remains the substrate crates; if a harness exists, it still must use public APIs only.
- **If different:** Calling it required keeps identity as the substrate but adds a current-delivery obligation to ship a walkable validation executable (still without its controls, content, or performance gates as product scope).

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world substrate (Rust crate) and frames the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binds product identity to the substrate crates, places the real game outside the repo, permits a public-API-only harness, and excludes game/System/LLM/spell/gas/combat/AI/building layers without excluding substrate navigation or material physics.
- **`docs/seeds/product-one-seed.md`:** Motivates substrate proof via an adjacent demo; first-slice exclusions (seasons, growth, etc.) limit that harness only and do not demote those substrate outcomes. Supplies crate-level wgpu/WGSL portability (no load-bearing native Metal fork); harness metrics stay consumer-owned.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes substrate outcome families—natural look over voxel truth, full mutability, deep Z, geology-first generation, vegetation growth, day/night and seasons, material/environmental simulation, GPU commands/mirror/events, mutation-safe navigation, streaming, and persistence including object/entity state—without mechanism inventory or future game features as product scope.
