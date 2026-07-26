# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world **substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for matter, generation, mutation, queries, and related physics—not a game, demo title, or content product.

## Purpose

Moria exists so multiple future games can share one material world stack: a natural-looking surface and deep underground volume whose *truth* is mutable voxels, not decorative heightmap scenery. Game rules, economies, and presentation live above it; the substrate makes dig-anywhere geology, continuous 3D space, and clean consumer integration real without baking any one game into the world layer.

## Product boundary

**In product (Moria):**

- The reusable voxel-world substrate and its public Rust interfaces.
- World generation, matter representation, visual meshing of voxel truth, mutation (including dig/place), collision and related matter physics against that truth, streaming of large sparse regions, and persistence shaped as generation seed plus edit deltas.
- Compatibility seams the substrate itself needs so external games can attach later—without implementing those games here.

**Adjacent, not the product:**

- The actual game is a separate downstream consumer and is not part of this repository’s product identity.
- A walkable-world executable may exist as an adjacent validation harness. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific substrate paths. Whether that harness is a required current delivery is open (see Q1). Its character controls, camera, authored route, seed-world content, presentation, workloads, and performance gates are harness- or consumer-owned, not product scope.
- Future titles (System ARPG, fortress/colony, descent/roguelike, pure sandbox) are consumers, not Moria.

**Out of product:** game rules; System/LLM features; spells; gas policy; combat; AI; building/gameplay layers (UI, blueprints-as-gameplay, work orders, room economy, mechanisms-as-game entities).

## Required product outcomes

Downstream design must make these product-level outcomes true:

1. **Substrate, not game.** The same crate stack can underpin different game modes; game policy stays above the substrate. Matter mutation and queries go through public verbs and queries—nothing above the matter layer touches voxels as a private back door.
2. **Natural world over voxel truth.** Surface terrain reads as ordinary landscape (hills, forest, water, cliffs, caves), while the grid remains authoritative matter. The render mesh is a regenerated view, not saved authority.
3. **Mutable everywhere, deep Z first-class.** Any voxel can be destroyed, moved, or placed; underground volumes (strata, caves, ore, aquifers) are real playable space, not a painted floor under the sky.
4. **Geology-first generation.** Worlds are generated as layered geology and related features with lazy materialization so large regions stay tractable; generation is a reusable substrate capability, not a one-off demo scene bake.
5. **Interactable surface life as matter where it matters.** Interactable vegetation and clutter that can burn, break, or block is voxel-backed; pure dressing stays derived from voxel state so it cannot desync from the material world.
6. **Operable sparse world.** Streaming and persistence support large regions via sparsity and “seed + deltas” truth so consumers can load, edit, and return to scarred worlds without treating the full raw volume as resident.

## Future products and enabling implications

Described consumers—System-backed ARPG, DF-style fortress/colony play, Moria-style deep descent, pure sandbox—sit above Moria. Enabling implications only: expose matter, generation metadata/POI hooks, mutation/query surfaces, and seam points those modes would need. Do not import their gameplay, content, controllers, characters, UI, or policies into Moria.

A walkable third-person “product one” demo is an adjacent proof vehicle for the substrate claim (“fully material world, not heightmap with props”), not a second product identity. Its first-slice depth does not narrow Moria’s reusable purpose.

## Non-goals

- Shipping a game, combat loop, progression, or authored campaign in this product.
- Implementing System/LLM, spells, gas metering, AI agents, or fortress labor/economy.
- Implementing building *gameplay* layers (player building UI, work-order systems, mechanism puzzle rules) inside Moria.
- Treating harness-specific routes, characters, cameras, seed content inventories, machine-specific perf gates, or milestone marketing as product requirements.
- Making the substrate depend on any LLM.

## Confirmed vision constraints

- **Ecosystem:** exposed as a Rust crate or small family of tightly scoped Rust crates; intended for Rust consumers.
- **Consumer isolation:** any in-repo validation harness, if present, consumes only public interfaces—no privileged substrate paths reserved for the harness.
- **Layering:** game rules and named future game systems stay out of this product; seams may be designed, implementations of those layers must not land here.
- **GPU-resident world substrate:** the product promise includes a GPU-resident voxel world foundation (not a CPU-only toy grid).
- **Stand-alone substrate:** zero LLM dependency in the world layer.

## Deferred design decisions

- Crate split and workspace layout (boundary is fixed; packaging is design).
- Voxel resolution, meshing/LOD strategy, object-layer scaling, fluid-sim depth, integrity and ambient-sim extent, and related open substrate-design questions.
- Delivery sequence and depth of substrate capabilities for any first consumer slice.
- Harness-only choices: controllers, cameras, seed region content, benchmarks, target machines, and acceptance numbers.
- Multiplayer and long-horizon roadmap commitments.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** alongside the substrate crates, or only a **permitted** adjacent validation artifact?

- **Proposed answer:** Permitted adjacent validation harness only—not a committed required delivery of this vision pass. Product identity remains the substrate crates; if a harness exists, it still must use public APIs only.
- **If different:** Calling it required keeps product identity as the substrate but adds a current-delivery obligation to ship a walkable validation executable (still without pulling its controls, content, or performance gates into product scope).

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world substrate (Rust crate) and frames the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binds product identity to the substrate crates, places the real game outside the repo, permits a public-API-only harness, and excludes game/System/LLM/spell/gas/combat/AI/building layers.
- **`docs/seeds/product-one-seed.md`:** Motivates substrate proof outcomes (material world, dig/place, walkable continuous 3D) via an adjacent demo/harness; first-slice content, player, platforms, and metrics stay consumer-owned and do not redefine product identity.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes substrate outcome families—natural look over voxel truth, full mutability, deep Z, geology-first generation, matter/physics/query surfaces, streaming/persistence, reusable layering for future games—without making its mechanism inventory or future game features product scope.
