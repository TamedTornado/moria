# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate or a small family of tightly scoped Rust crates. It owns material world truth—generation, mutation, presentation-as-view, streaming, persistence of substrate-owned changes and objects, responsive matter behavior, and query/physics surfaces—that every consumer uses the same way. A **walkable-world executable** is a required adjacent first delivery: a validation harness and public-API consumer, not product identity and not a game layer.

## Purpose

Moria exists so multiple game styles—adventure, fortress/colony, descent, pure sandbox—can share one material world stack instead of each inventing geology, mutability, and deep continuous space. The substrate must stand alone with **no LLM or System dependency**. Game policy lives above it; the substrate provides **matter, physics, queries, and mutation**, plus neutral seams so any client—including an optional future System—observes and drives the world through the same channels as hand-authored tools.

## Product boundary

**In product**

- The public Rust crate surface and the reusable world substrate behind it.
- Material world capabilities: geology-aware generation; fully mutable voxel matter; smooth natural-looking presentation as a regenerated view of voxel truth; deep continuous Z; streaming and persistence of substrate-owned changes and objects; collision and queries against that truth; responsive objects and dressing; active fluids; ambient fire, wetness, weather, and growth; granular response; structural integrity and collapse; neutral registries, commands, queries, mirrors, and events.
- Equal integration: the walkable-world harness and any external game call only the same public interfaces.

**Out of product / adjacent**

- The actual game and all game rules; separate downstream consumers, not part of this repository.
- System / LLM runtime, spells, gas policy, combat, AI, and building layers (blueprints-as-gameplay, work orders, mechanisms-as-gameplay, room/economy semantics). Compatibility seams may be designed where needed; those layers are not implemented here.
- Harness-owned concerns: controllers, cameras, demo routes and content catalogs, debug presentation, benchmarks, machine profiles, and performance gates. The walkable-world executable is a required adjacent first delivery that exercises the public product boundary; its controls, content, presentation, workloads, platforms, and numeric gates remain adjacent.

## Required product outcomes

1. **Reusable crate consumption.** Downstream games, tools, and the walkable-world harness integrate only through the public Rust crate boundary; nothing above the substrate touches voxel storage by privileged back doors.
2. **Natural material world, deep continuous Z.** The world reads as ordinary terrain, vegetation, and water-bearing surface while interactive matter remains voxel truth—not a heightmap with non-material props. Underground is first-class continuous content. Worlds generate as geology supporting lazy materialization.
3. **Mutability and non-authoritative mesh.** Any material can be destroyed, moved, or placed through substrate verbs; dig and place are substrate responsibilities; the rendered mesh is a regenerated view only.
4. **Responsive matter behavior.** Substrate-owned objects and dressing participate as matter (things that can burn, break, or block are voxel-backed; other dressing stays anchored to voxel truth). The substrate provides active fluids; ambient fire, wetness, weather, and growth; granular settle; and structural integrity with collapse. Simulation depth is deferred; these outcome families are current product mandate, not deferred to a future game.
5. **Stream, persist, and query.** Active regions stream around multiple anchors. Substrate-owned world changes and objects persist across streaming and runs via generated truth plus deltas and journals for moved substrate objects and their state; cross-run reuse is supported. Physics, occupancy, and spatial queries run against voxel truth.
6. **Neutral extension and observation.** Consumers and optional System clients use the same neutral registries, commands, queries, mirrors, and events as hand-authored clients. Truth is GPU-resident: commands in, stale mirror and events out. Authored content, policy, the System, and game semantics stay outside the product.

## Future products and enabling implications

Future **consumers** (not current product) include a System-bearing ARPG, a fortress/colony mode, a Moria-style descent experience, and pure sandbox tools. They motivate a rules-free substrate that accepts policy plug-ins and content authorship through the same registries and command/mirror seams required above.

The walkable-world first delivery proves a narrower **first slice** (generated walkable region, traversal, dig/place proof, and validation of generation, streaming, meshing, editing, collision, persistence, and performance via the public API). That slice’s exclusions—static fluid bodies without flow, no ambient CA yet, and similar demo non-goals—limit the adjacent proof, not the product’s identity or the full outcomes above.

Enabling implications only: keep matter mutation and queries symmetric for any agent; keep the mesh non-authoritative; keep worldgen and POI/metadata separable from LLM authorship. Do not import consumer gameplay, controllers, characters, animation, UI, or content into Moria.

## Non-goals

- Implementing the game, game rules, or repository ownership of a playable title.
- System / LLM runtime, spell systems, gas economies, combat, AI, and building gameplay layers.
- Treating the walkable demo’s character fantasy, content catalog, or trailer as product identity.
- Embedding consumer-chosen platforms, backends, or benchmark gates into the substrate promise.
- Narrowing the substrate mandate to Product One’s first-slice omissions (those bound the adjacent proof, not the product).

## Confirmed vision constraints

- **Ecosystem:** product is a Rust crate (or small crate family) for Rust consumers.
- **Residency model:** the world substrate is GPU-resident as a product-defining property.
- **Consumer equality:** the walkable-world harness and any other validation artifact use the same public interfaces available to an external game; no privileged path inside the substrate.
- **Independence:** the substrate has zero LLM/System dependency and must be usable without those layers.
- **Layer exclusion:** game rules and the future System, LLM, spell, gas, combat, AI, and building layers are out of scope to implement here.
- **Adjacent first delivery:** a walkable-world executable is required as an adjacent public-API consumer and validation harness for the first substrate proof; it is not the product.

## Deferred design decisions

- Exact crate split and workspace layout (consumer boundary is fixed; package structure is design).
- Voxel resolution, brick layout, meshing and LOD algorithms, storage encodings, and simulation depth for fluids, integrity, ambient, and granular systems.
- How much of the full substrate mandate ships in which delivery slice, and in what order—subject to the walkable-world first delivery’s proof purpose.
- Performance targets, supported hardware/OS matrices, and graphics stack choices.
- Harness-adjacent design of controls, content, presentation, workloads, and gates for the walkable-world executable.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world Rust substrate and states the walkable-world executable as a present adjacent consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Fixes product identity as the substrate crate(s), places the real game outside the repo, requires equal public interfaces for any harness, permits the walkable-world executable without canceling its selected delivery, and excludes game/System/LLM/spell/gas/combat/AI/building layers.
- **`docs/seeds/product-one-seed.md`:** Pins the required walkable-world first delivery—generated region, traversal, dig/place proof, validation responsibilities—and the first-slice limits of that adjacent proof; does not import demo controls, content catalogs, machine profiles, or numeric gates into substrate scope.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies the full substrate purpose and mandate—natural material world, mutability, deep Z, responsive objects/dressing, fluids, ambient behavior, granular and integrity outcomes, multi-anchor streaming and cross-run persistence of substrate-owned changes and objects, GPU-resident commands-in/mirror-and-events-out, neutral registries for all clients—without transferring mechanism inventories into design detail.
