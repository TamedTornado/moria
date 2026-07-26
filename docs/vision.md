# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** for external games to consume as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer world of matter, physics, queries, and mutation—not a game, not a gameplay stack, and not the validation executable that proves it.

## Purpose

Give downstream games a shared foundation for a continuous, fully material 3D world: a natural-looking surface whose authoritative truth is a mutable voxel volume, including deep underground space. Game rules, content authorship, controllers, and presentation live above this layer so the same substrate can support multiple game styles without embedding any one of them.

## Product boundary

- **In product:** The substrate’s public capability surface for Rust consumers—generated natural matter worlds, mutation, view-from-truth meshing, streaming-scale residency, persistence of world deltas, and the placement, observation, and registry surfaces under required outcomes. Integration only through public interfaces an external game could use; higher layers do not touch voxels directly.
- **Required adjacent delivery (not identity):** A **walkable-world executable** is a **required first validation delivery** in this repository. It is Moria’s validation consumer, not part of the crate product. It must exercise the same public API available to external games through a generated, traversable, mutable natural region that validates generation, streaming, meshing, editing, voxel-truth collision, persistence, and performance. Its controls, camera, authored route/content, exact workloads, machine targets, platform constraints, and milestone status are harness concerns—not product mandates.
- **Out of product / repository:** The actual game is a separate downstream consumer and is not part of this repository. Game rules and the System, LLM, spell, gas, combat, AI, and building **layers** (UX, work orders, economy, game policy, authored content, room assignment, mechanism entity logic) are out of scope. Compatibility seams may be designed where substrate needs demand them; those layers must not be implemented here. Substrate-owned placement primitives and reusable stamp/structure representation remain in product; game building layers, mechanism functionality, and room semantics do not.

## Required product outcomes

- **Rust-consumable, portable GPU matter world.** External games integrate public crate API(s). The substrate provides matter, physics, queries, and mutation with no in-tree privilege and no LLM dependency. Consumers obtain a mesh (or equivalent) view; gameplay-relevant physics and queries run against voxel truth, not a frozen mesh as authority. Load-bearing operation must stay portable across GPU backends and must not specialize to the validation machine; named devices, benchmark gates, and exact backend choices are not product identity.
- **Command / stale-mirror observation.** Consumers submit commands into the GPU-resident substrate and observe a **stale mirror** plus events—not live authoritative CPU-side voxel access. Higher layers never touch voxels directly.
- **Natural world as matter truth.** Generate surface composition that reads as ordinary outdoor terrain—biome-linked vegetation, rocks, rivers/water, caves, strata, and underground geology—while remaining fully mutable voxel truth through the volume (deep Z first-class; nothing decorative outside the material model). Exact species, palette, region size, and generation mechanisms stay design or first-slice adjacent.
- **Simulation families (substrate-owned, long-horizon).** Interactive voxel-backed vegetation with **growth over game time**; matter-linked dressing; static and dynamic water; ambient day/night, seasons, weather, and fire that affect the matter world; granular behavior and structural failure. Delivery sequence after the first slice is design; these families are product outcomes, not optional consumer features.
- **Reusable placement surfaces.** Voxel placement; reusable stamps or equivalent structure representation; mutation-safe navigation and 3D traversal data; extensible material/object/content registries. Mechanism-capable entity behavior and room detection/metadata are not current-product outcomes. Building UI, work orders, economy, AI, System behavior, and authored game content remain consumer-owned.
- **Persistence and first-slice depth.** Persist voxel scars, placements, and relevant moved-object/entity state; saved deltas are reusable across runs. Initial cut: generation for the proof region; GPU incremental meshing; dressing; placed/rendered voxel objects; static water; dig/place and mirror queries; streaming; persistence; benchmarking as deliverable; required adjacent walkable harness on the public API. Out of that cut (still long-horizon product): active CA/fire, higher fluid tiers, integrity, granular settling, vegetation growth and seasonal/ambient coupling, and non-stretch felling. Product One single-slot or exact-restore modifiers are harness constraints, not the general mandate.

## Future products and enabling implications

Future **consumers** (not this product) include System/ARPG-style play, fortress/colony-style play, descent/adventure modes, and pure sandboxes. They own gameplay, UX, controllers, characters, authored content, pricing policy, presentation, mechanism scripts, and room/economy semantics. The substrate outcomes above enable those games; they are not deferred to “maybe later consumers.” Delivery sequence among long-horizon families remains design after the first slice.

## Non-goals

- Shipping a game, combat loop, stats, AI agents, or multiplayer service in this repository.
- Implementing System/LLM, spells, gas/pricing policy, or game building UX (blueprints-as-gameplay, work orders, economy, room assignment).
- Requiring mechanism entity functionality or room detection as current substrate outcomes.
- Importing the harness’s demo fantasy, controls, camera, route/content, or machine/platform performance gates into the product mandate.
- Depending on an LLM for core operation, or forking load-bearing layers to a single GPU vendor/machine.

## Confirmed vision constraints

- **Ecosystem:** consumption as Rust crate(s) in a Cargo ecosystem; the in-repo walkable harness is separated from reusable substrate code at the consumer boundary (exact package layout is design).
- **API symmetry:** the harness uses the same public interfaces available to an external game.
- **Standalone engine layer:** zero required LLM/System dependency for core world operation; gas/pricing and game policy are consumer-injected.
- **Observation boundary:** commands enter the substrate; consumers read a stale mirror and events.
- **Portability over machine fit:** portable GPU operation is a crate-level requirement; validation-machine limits do not redefine the product.
- **First-slice vs product:** Product One pins first-delivery depth for the initial validation delivery; it does not shrink long-horizon substrate identity to that slice alone.

## Deferred design decisions

- Representation and runtime choices (resolution, storage, meshing approach, streaming layout, delta encoding, sim scheduling).
- Exact public API shape, crate family split, GPU backend selection, and compatibility seams for future game layers.
- How far multiplayer-oriented command/mirror patterns are taken early.
- Delivery sequence and depth among long-horizon simulation families after the first slice (not whether those families are product outcomes).

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident Rust-crate substrate and the walkable-world executable as its separate validation consumer for generation, streaming, meshing, editing, collision, persistence, and performance.
- **`docs/seeds/project-boundary.md`:** Binding boundary—Rust crate product, game out of repo, harness adjacent not crate identity, public-API-only access, exclusion of game/System/spell/gas/combat/AI/building layers (not exclusion of substrate placement and stamp/structure representation).
- **`docs/seeds/product-one-seed.md`:** Settles first-delivery depth and the required walkable validation delivery; pins dig/place plus mirror queries and crate-level backend portability; keeps growth, seasons, and weather out of the first cut without removing them from longer-horizon substrate identity.
- **`docs/seeds/voxel-world-substrate.md`:** Authoritative long-horizon substrate outcomes—natural matter world, stale-mirror observation, vegetation growth and ambient seasonal coupling, simulation families, placement/stamp surfaces, registries, and persistence—without promoting mechanism/room game layers into current product mandates.
