# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate or a small family of tightly scoped Rust crates. It owns material world truth—generation, mutation, presentation-as-view, streaming, persistence of substrate-owned changes and objects, responsive matter behavior, and query/physics surfaces—that every consumer uses the same way. A **walkable-world executable** is a required adjacent first delivery: a validation harness and public-API consumer, not product identity and not a game layer.

## Purpose

Moria exists so multiple game styles—adventure, fortress/colony, descent, pure sandbox—can share one material world stack. The substrate must stand alone with **no LLM or System dependency**. Game policy lives above it; the substrate provides **matter, physics, queries, and mutation**, plus neutral seams so any client—including an optional future System—observes and drives the world through the same channels as hand-authored tools.

## Product boundary

**In product**

- The public Rust crate surface and the reusable world substrate behind it.
- Material world capabilities: geology-aware generation; fully mutable voxel matter; smooth natural-looking presentation as a regenerated view of voxel truth; deep continuous Z; streaming and persistence of substrate-owned changes and objects; collision and queries against that truth; responsive objects and dressing; active fluids; ambient fire, wetness, weather, time/seasons, and growth; granular response; structural integrity and collapse; mutation-aware 3D navigation derived from voxel truth; neutral registries, commands, queries, mirrors, and events.
- Equal integration: the walkable-world harness and any external game call only the same public interfaces.

**Out of product / adjacent**

- The actual game and all game rules; separate downstream consumers, not part of this repository.
- System / LLM runtime, spells, gas policy, combat, AI, agent labor policy, and building layers (blueprints-as-gameplay, work orders, mechanisms-as-gameplay, room/economy semantics). Compatibility seams may be designed where needed; those layers are not implemented here.
- Harness-owned concerns: controllers, cameras, demo routes and content catalogs, debug presentation, benchmarks, machine profiles, and performance gates—including those of the walkable-world executable.

## Required product outcomes

1. **Reusable crate consumption.** Downstream games, tools, and the walkable-world harness integrate only through the public Rust crate boundary; nothing above the substrate touches voxel storage by privileged back doors.
2. **Natural material world, deep continuous Z.** The world reads as ordinary terrain, vegetation, and water-bearing surface while interactive matter remains voxel truth—not a heightmap with non-material props. Underground is first-class continuous content. Worlds generate as geology: stages pure over coordinates and world seed, so any brick can materialize independently and lazily.
3. **Mutability and non-authoritative mesh.** Any material can be destroyed, moved, or placed through substrate verbs; dig and place are substrate responsibilities; the rendered mesh is a regenerated view only.
4. **Responsive matter behavior.** Substrate-owned objects and dressing participate as matter (things that can burn, break, or block are voxel-backed; other dressing stays anchored to voxel truth). Detached or failed substrate matter can enter physical motion, affect the world, and rejoin voxel truth. The substrate provides active fluids; ambient fire, wetness, weather, day/night and seasons, and growth; granular settle; and structural integrity with collapse. Simulation depth is deferred; these families are current product mandate.
5. **Stream, persist, query, and navigate.** Active regions stream around multiple anchors. Substrate-owned world changes and objects persist as generated truth plus edit deltas, with journals for moved substrate objects; cross-run reuse is supported. Physics, occupancy, and spatial queries run against voxel truth. Navigation is derived from that truth, invalidated by mutation, and usable for continuous three-dimensional movement.
6. **Neutral extension and observation.** Consumers and optional System clients use the same neutral registries, commands, queries, mirrors, and events as hand-authored clients. Truth is GPU-resident: commands in, stale mirror and events out.

## Future products and enabling implications

Future **consumers** (not current product) include a System-bearing ARPG, a fortress/colony mode, a Moria-style descent experience, and pure sandbox tools. They motivate a rules-free substrate accepting policy plug-ins through the same seams required above.

The walkable-world first delivery proves a narrower **first slice**: a natural-looking, smooth, deeply traversable generated region whose visible terrain and collision are voxel truth; reusable geology generation; dig/place/query proof; static lakes and river-channel water surfaces (no flow simulation or higher fluid tiers); voxel-object placement and rendering without felling or rigid conversion; and public-API validation of those surfaces. That slice excludes seasons and weather simulation, ambient CA, integrity, granular settle, and fluid flow—limits of the adjacent proof, not product identity. Its persistence proof is reload of the same seed plus deltas with exact restoration for that single-save path; that exactness does not generalize to every journaled substrate artifact.

Enabling implications only: keep mutation and queries symmetric for any agent; keep the mesh non-authoritative; keep worldgen and POI/metadata separable from LLM authorship. Do not import consumer gameplay, controllers, characters, animation, UI, or content.

## Non-goals

- Implementing the game, game rules, or repository ownership of a playable title.
- System / LLM runtime, spell systems, gas economies, combat, AI, and building gameplay layers.
- Treating the walkable demo’s character fantasy, content catalog, or trailer as product identity.
- Embedding consumer-chosen platforms, backends, or benchmark gates into the substrate promise.
- Narrowing the substrate mandate to Product One’s first-slice omissions (those bound the adjacent proof, not the product).

## Confirmed vision constraints

- **Ecosystem and residency:** Rust crate (or small crate family) for Rust consumers; GPU-resident world substrate as a product-defining property.
- **Deterministic generation:** generation stages pure over coordinates and world seed so bricks regenerate independently and lazily; persistent substrate truth is that generation plus edit deltas.
- **Consumer equality:** any harness uses the same public interfaces available to an external game; no privileged path inside the substrate.
- **Independence:** the substrate has zero LLM/System dependency and must be usable without those layers.
- **Layer exclusion:** game rules and the future System, LLM, spell, gas, combat, AI, and building layers are out of scope to implement here.
- **Adjacent first delivery:** a walkable-world executable is required as an adjacent public-API consumer and validation harness; it is not the product. Settled first-slice outcomes (natural walkable material world, geology, dig/place, static water bodies) are fixed; later sequencing of remaining outcomes remains design.

## Deferred design decisions

- Exact crate split and workspace layout (consumer boundary is fixed; package structure is design).
- Voxel resolution, brick layout, meshing/LOD, storage encodings, and simulation depth.
- Delivery order and depth for outcomes beyond the settled first-slice proof.
- Performance targets, supported hardware/OS matrices, and graphics stack choices.
- Harness-adjacent controls, content, presentation, workloads, and gates.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world Rust substrate and the walkable-world executable as adjacent harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Fixes product identity as the substrate crate(s), places the real game outside the repo, requires equal public interfaces, and excludes game/System/LLM/spell/gas/combat/AI/building layers.
- **`docs/seeds/product-one-seed.md`:** Pins the walkable-world first delivery—natural walkable region, geology, dig/place proof, static lakes and river channels, single-save seed-plus-deltas restoration—and that adjacent proof’s first-slice limits.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies the full substrate mandate—seed/coordinate-deterministic geology, mutability, deep Z, responsive objects with physical fall and re-voxelization, fluids, time/seasons/weather, granular and integrity outcomes, mutation-aware 3D navigation, multi-anchor streaming and persistence as generation plus deltas, GPU-resident neutral registries.
