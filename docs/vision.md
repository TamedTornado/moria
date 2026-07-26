# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or small family of tightly scoped Rust crates. It is an engine-layer foundation for natural, fully material worlds—not a game, not a demo product, and not an LLM-dependent system.

## Purpose

Downstream games need one shared world substrate where terrain, underground space, and player scars are the same mutable matter, presented as a normal-looking surface world rather than a cube aesthetic. Moria exists so adventure, fortress, sandbox, and similar consumers can stand on geology-first generation, deep-Z play space, matter mutation, and public world queries without each game re-owning the world engine. The substrate must stand alone with no LLM dependency.

## Product boundary

**In product:** the reusable voxel-world substrate and its public consumer-facing surface (generation of material worlds, sparse GPU-resident matter, smooth presentation of voxel truth, mutation and query verbs, collision against voxel truth, streaming, and persistence of edit deltas). Compatibility seams may be designed where substrate requirements demand them for later game layers, without implementing those layers here.

**Adjacent, not identity:** a walkable-world executable may exist in this repository solely as a validation harness. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific substrate paths. Whether shipping that harness is a required repository delivery remains open (see Q1). Its character, camera, controller, seed content, routes, presentation, workloads, and performance gates are not product scope.

**Out of product / downstream:** the actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and building layers (blueprints, mechanisms, room semantics, work orders, and similar game-facing construction systems). Gameplay, UX, authored campaign content, and game-specific policy remain consumer-owned.

## Required product outcomes

- **Normal-looking material world.** Consumers get rolling natural terrain (surface dressing and continuous smooth terrain presentation) where the voxel grid is authoritative truth and extracted geometry is a regenerated view, not the saved world.
- **Mutable everywhere, including deep Z.** Any material cell can be destroyed, altered, or placed; underground is first-class play space (caves, strata, ore, buried volumes), not a decorative floor under a heightmap.
- **Geology-first generation.** Worlds materialize as diggable geology (columns/strata, caves, ores, aquifers as world facts) with lazy materialization so large sparse regions remain tractable.
- **Matter and presentation substrate.** GPU-resident brick matter, incremental remeshing of changed regions, surface dressing driven by voxel state, and static fluid bodies as material volumes—sufficient for a fully material walkable world without implementing game rules.
- **Public mutation and query boundary.** Consumers change and inspect the world only through the substrate’s public verbs and queries; nothing above the matter core reaches voxels through privileged paths.
- **Streaming and scar persistence.** Active regions stream in and out; truth is worldgen plus edit deltas so defacement and reload preserve the same material world.

## Future products and enabling implications

Future consumers include a System-driven ARPG, DF-style fortress/colony play, a Moria-style descent experience, and pure sandbox modes. Those games are not this product. Enabling implications already motivated by the substrate seeds: reusable matter, mutation, queries, geology, deep-Z continuity, and (later, at consumer choice) physics and policy plug-ins such as pricing. Do not treat long-horizon matter features (for example full fluid flow, structural integrity, fire ecology, or tree felling) as a committed delivery sequence; design owns depth and order. Do not import consumer gameplay, characters, controllers, content, or presentation into this brief.

## Non-goals

- Implementing the game, game rules, System/LLM, spells, gas, combat, AI, or building layers in this repository.
- Treating the walkable demo’s specific seed region, third-person avatar, debug UX, or milestone catalog as the product identity.
- Making the substrate depend on an LLM or ship game-layer semantic systems (rooms, work orders, economy hooks) as current product.
- Privileged harness-only world paths that external games cannot use.

## Confirmed vision constraints

- **Rust library form:** exposed as a Rust crate or small family of tightly scoped Rust crates for game consumers.
- **GPU-resident substrate:** world matter and related heavy work are GPU-resident as part of product identity.
- **Strict consumer boundary:** any in-repo harness or external game is a peer consumer of public interfaces; no privileged game-specific implementation paths inside the substrate.
- **LLM-free engine layer:** the substrate stands alone with zero LLM dependency; the System is a future game-layer client, not a substrate feature.
- **Workspace separation intent:** substrate and any harness are separated so the consumer boundary is enforced; exact crate layout is design, not vision.

## Deferred design decisions

- Capability depth and build order for matter features beyond the outcome mandates (fluids beyond static bodies, integrity, granular settle, fire, object felling, ambient weather, and similar).
- Voxel resolution, LOD/impostors, object-layer scaling, and related fidelity tradeoffs.
- Exact public API shape, crate split, persistence encoding, streaming ring policy, and meshing approach.
- Whether and how multiplayer-ready command authorship is kept in scope statements.
- Platform, performance budgets, and benchmark harness details (including any machine-specific limits).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world validation executable a *required repository delivery* for the current product effort, or only *permitted* as an adjacent artifact?

- **Proposed safe answer:** Permitted only—product identity and required outcomes are the substrate; design may still add a public-API harness without making that executable part of what Moria *is*.
- **If answered differently:** Requiring the harness adds a repository delivery obligation (still adjacent to product identity) so planning must schedule and maintain a walkable consumer; it must not pull character, content, controls, or performance gates into substrate scope.

## Seed synthesis

- **README.md:** Names Moria as a reusable GPU-resident Rust voxel-world substrate and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md:** Binds current product identity to the substrate crates, forbids game/System/building-layer implementation here, and requires any harness to use public interfaces only.
- **docs/seeds/product-one-seed.md:** Describes a first walkable proof consumer (region, avatar, dig proof, milestones); used only to motivate substrate outcomes and the open harness-delivery question—not to redefine product scope.
- **docs/seeds/voxel-world-substrate.md:** Supplies design-goal altitude for normal-looking mutable worlds, deep Z, geology-first generation, GPU-resident matter, layering, and multi-game reuse without importing mechanism inventory or game layers.
