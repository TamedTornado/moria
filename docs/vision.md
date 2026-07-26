# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for games and tools: it owns material world truth, generation, mutation, presentation-as-view, streaming, persistence, and query/physics surfaces that any external consumer uses the same way. It is not a game, not a character demo, and not a rules layer.

## Purpose

Moria exists so multiple game styles—adventure, fortress/colony, descent, pure sandbox—can share one material world stack instead of each inventing geology, mutability, and deep continuous space. The substrate must stand alone with **no LLM or System dependency**. Game policy (pricing, combat, AI, spells, gas, building gameplay) lives above it; the substrate provides **matter, physics, queries, and mutation**.

## Product boundary

**In product**

- The public Rust crate surface and the reusable world substrate behind it.
- Material world capabilities consumers need: geology-aware generation, fully mutable voxel matter, smooth natural-looking surface presentation whose mesh is a regenerated view of voxel truth, deep continuous Z, streaming and edit-preserving persistence, and collision/queries against that truth.
- A clear integration boundary so adjacent artifacts (including any validation harness) call only the same public interfaces an external game would use.

**Out of product / adjacent**

- The actual game and all game rules; they are separate downstream consumers and are not part of this repository.
- System / LLM, spells, gas policy, combat, AI, and building layers (blueprints, work orders, mechanisms-as-gameplay, room/economy semantics). Compatibility seams may be designed where substrate needs demand them; those layers are not implemented here.
- Harness- or demo-owned concerns: character controllers, cameras, authored demo routes and seed-world content catalogs, debug presentation choices, scripted benchmark scenes, machine profiles, and performance acceptance gates. A walkable-world executable **may** exist as an adjacent validation harness; whether it is a required repository delivery is open (**Q1**). Until resolved, this brief does not treat that executable as committed current delivery.

## Required product outcomes

1. **Reusable crate consumption.** Downstream games and tools integrate Moria through a public Rust crate boundary; nothing above the substrate touches voxel storage by privileged back doors.
2. **Natural world, material truth.** Consumers get a world that reads as ordinary terrain, vegetation, and water-bearing surface, while everything interactive remains material voxel truth—not a heightmap with non-material props.
3. **Mutability everywhere.** Any material can be destroyed, moved, or placed through substrate verbs; dig and place (and related matter mutation) are substrate responsibilities, and the rendered mesh stays non-authoritative.
4. **Deep continuous Z.** Underground space is first-class content volume (strata, caves, buried material structure), continuous with the surface, not a decorative floor.
5. **Generation that digs honestly.** Worlds are produced as geology and structure that support lazy materialization so large regions can exist without eagerly holding all voxels.
6. **Stream, persist, and query.** Active regions stream; material edits persist across load without discarding the generated base; physics, occupancy, and spatial queries run against voxel truth so consumers can traverse, dig, and reason without treating the mesh as ground truth.

## Future products and enabling implications

Future **consumers** (not current product) include a System-bearing ARPG, a Dwarf Fortress–style fortress/colony mode, a Moria-style descent experience, and pure sandbox tools. They motivate a substrate that stays free of game rules and that can later accept policy plug-ins (for example gas pricing) and content authorship (materials, stamps, scripts) at the consumer or semantic layer.

Enabling implications only: keep matter mutation and queries symmetric for any agent; keep the mesh non-authoritative; keep worldgen and POI/metadata separable from LLM authorship. Do not import consumer gameplay, controllers, characters, animation, UI, or content inventories into Moria.

## Non-goals

- Implementing the game, game rules, or repository ownership of a playable title.
- System / LLM runtime, spell systems, gas economies, combat, AI, and building layers.
- Treating a walkable demo, character fantasy, or X-audience trailer as the product identity.
- Embedding consumer-chosen platforms, backends, or benchmark gates into the substrate promise.

## Confirmed vision constraints

- **Ecosystem:** product is a Rust crate (or small crate family) for Rust consumers.
- **Residency model:** the world substrate is GPU-resident as a product-defining property.
- **Consumer equality:** any validation harness uses the same public interfaces available to an external game; no privileged or game-specific implementation path inside the substrate.
- **Independence:** the substrate has zero LLM/System dependency and must be usable without those layers.
- **Layer exclusion:** game rules and the future System, LLM, spell, gas, combat, AI, and building layers are out of scope to implement here.

## Deferred design decisions

- Exact crate split and workspace layout (consumer boundary is fixed; package structure is design).
- Voxel resolution, brick layout, meshing and LOD algorithms, storage encodings, and sim tiering depth.
- How much capability ships in which delivery slice, and in what order.
- Performance targets, supported hardware/OS matrices, and graphics stack choices.
- Whether and how a validation harness is structured if **Q1** affirms delivery—content, controls, and gates remain design of that adjacent artifact.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world executable a **required repository delivery** in this phase, or only **permitted** as an adjacent validation harness?

- **Proposed safe answer:** Permitted only—not required for product completeness. The product remains the substrate crates; a harness may be added later to exercise public interfaces without becoming part of product identity.
- **If answered differently:** Requiring the harness makes repository delivery include an adjacent executable that must still stay outside product identity and still consume only public APIs; it does not pull character, content, camera, route, or performance gates into substrate scope. Declaring it forbidden would remove even a permitted harness from the repository boundary.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world Rust substrate and positions a walkable-world executable as a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Fixes product identity as the substrate crate(s), places the real game outside the repo, requires equal public interfaces for any harness, and excludes game/System/LLM/spell/gas/combat/AI/building layers.
- **`docs/seeds/product-one-seed.md`:** Describes a first walkable demo slice and harness-facing proof points; used only to clarify validation-vs-product separation and first-slice non-goals, not to import demo content, controls, or performance gates into current product scope.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies substrate purpose and design goals—natural material world, full mutability, deep Z, reusable engine layer providing matter/physics/queries/mutation, GPU-resident—without transferring mechanism inventories or future game features into committed delivery detail.
