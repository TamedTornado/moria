# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for worlds of mutable matter—not a game, and not a game-content product.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface over continuous, diggable, buildable voxel truth, including deep underground geology. Game rules, economies, combat, agents, and presentation live in consumers. The substrate stands alone with no LLM or “System” dependency; those attach only as ordinary clients when a consumer chooses.

## Product boundary

**In product**

- The reusable world substrate: generation of geological worlds, GPU-resident matter, non-authoritative visual meshing of that matter, mutation and query surfaces, streaming of large regions, persistence of edits, and matter-side foundations (including interaction with fluids, structural support, and related material behavior) that games build on.
- A strict consumer boundary: anything that uses the substrate—including any validation harness—does so only through the same public interfaces an external game would use.

**Adjacent, not product identity**

- A walkable-world executable may exist in this repository as a validation harness and demo consumer of the substrate. Its character control, camera, authored demo route, presentation, debug tooling, and acceptance workloads are harness-owned. Whether that harness is a required current delivery is open (see Q1).

**Out of product / out of this repository**

- The actual game (or games) that ship on Moria.
- Game rules and the System, LLM, spell, gas, combat, AI, and building *game* layers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

1. **Integrable substrate.** Downstream games and tools consume Moria as Rust crate(s) and obtain a complete world foundation without privileged internal access paths.
2. **Natural look, material truth.** The world reads as continuous natural terrain (hills, forests, water, cliffs, caves) while remaining fully material voxel truth—not a heightmap decorated with non-material props. What you see can be reached and altered as matter.
3. **Mutable everywhere, deep Z first-class.** Any voxel can be destroyed, moved, or placed; underground geology, caves, and strata are real content space, not a floor under a skybox.
4. **GPU-resident matter world.** Authoritative matter and related simulation live in a GPU-resident design so large sparse worlds remain tractable.
5. **Geology-first generation with lazy cost.** Worlds are produced as layered geology (terrain, strata, caves, ores, and related structure) that can materialize on demand so idle or distant volume stays cheap.
6. **Public mutation, query, and lifecycle surfaces.** Consumers edit and inspect the world through public verbs and queries; the rendered mesh is a regenerated view of voxel truth, not the authority. Streaming and persistence preserve a large world as generation plus edit history so scars and structures survive reloads.

## Future products and enabling implications

Future *consumers* (not this product) include a System/LLM-backed ARPG, a Dwarf Fortress–style fortress or colony mode, a descent/adventure mode, and pure sandboxes. Those games own gameplay, UX, controllers, authored content, presentation, and policy (including how verbs are priced).

Enabling implications already assigned to the substrate (delivery depth is design, not a committed roadmap here): reusable generation and matter APIs; vegetation and clutter that stay consistent with matter; multi-tier fluid and ambient behavior as substrate capabilities over time; structural support and placement primitives that fortress- and sandbox-style consumers need; navigation data derived from matter so agents and players can move in continuous 3D. Multiplayer is not a current commitment; verb-and-query separation is noted only as architecture that can stay server-ready.

An early “walkable world” demo (one generated region, a player character, dig/place as proof) is a *consumer slice* that motivates and validates substrate outcomes. Its milestone list, seed-world contents, performance numbers, and platform choices do not redefine Moria’s product identity.

## Non-goals

- Shipping game systems: combat, stats, non-player AI, spells, gas metering, intent, building UI/blueprints-as-gameplay, or System/LLM authorship inside the substrate.
- Treating the walkable demo’s specific content, controls, or benchmark scene as the product itself.
- Embedding any single game’s rules so other intended consumers cannot share the crate stack.

## Confirmed vision constraints

- **Ecosystem:** Moria is a Rust crate or small family of tightly scoped Rust crates.
- **Residency:** The world substrate is GPU-resident by product intent.
- **Standalone:** The substrate has zero LLM dependency and must function without the System.
- **Interface equality:** Any in-repo harness or adjacent executable must use the same public interfaces as an external game; no privileged or game-specific implementation path inside the substrate.
- **Layer ownership:** Game rules and System, LLM, spell, gas, combat, AI, and building game layers are out of scope for this product.

## Deferred design decisions

- How deep each matter capability (fluids, integrity, granular settle, fire/ambient sim, voxel objects vs dressing, placement stamps) is in any given release, and in what order.
- Voxel scale, LOD strategy, object-layer capacity, and related fidelity/cost tradeoffs.
- Exact crate split within the family; internal data layouts, algorithms, and APIs.
- Whether and how multiplayer is pursued later.
- Harness-only choices if a walkable-world executable ships: controller, camera, demo route, content palette, debug presentation, and measured performance gates.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a *required current delivery* of this repository, or only *permitted* as an adjacent artifact?

- **Proposed answer:** Permitted only—it may exist to exercise public substrate interfaces, but it is not part of Moria’s product identity and is not mandated as a named current deliverable by this vision.
- **If different:** Requiring it adds a mandatory adjacent deliverable (still outside product identity; still without importing its controls, content, or performance gates into substrate scope). Forbidding it removes even the permitted in-repo harness and leaves all validation approach to design.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world Rust substrate and distinguishes the walkable-world executable as a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`:** Fixes product and repository boundary—substrate crates here; game elsewhere; harness only through public interfaces; game/System/spell/gas/combat/AI/building layers out of scope.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies the substrate’s purpose and outcome family—natural look over mutable voxel truth, deep Z, geology-first generation, GPU-resident matter, public verbs/queries, streaming/persistence, and matter foundations for multiple future games without embedding those games.
- **`docs/seeds/product-one-seed.md`:** Describes an early walkable-world consumer slice and proof points; used only to clarify harness-vs-product separation and motivating outcomes, not to import demo content, controls, platforms, or performance gates into current product scope.
