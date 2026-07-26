# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** for external games to consume as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer world of matter, physics, queries, and mutation—not a game, not a gameplay stack, and not the validation executable that proves it.

## Purpose

Give downstream games a shared foundation for a continuous, fully material 3D world: a natural-looking surface whose authoritative truth is a mutable voxel volume, including deep underground space. Game rules, content authorship, controllers, and presentation live above this layer so the same substrate can support multiple game styles without embedding any one of them.

## Product boundary

- **In product:** The substrate’s public capability surface for Rust consumers—generated natural matter worlds, mutation, view-from-truth meshing, streaming-scale residency, persistence of world deltas, and the reusable interaction, semantic, and registry surfaces under required outcomes. Integration only through public interfaces an external game could use; higher layers do not touch voxels directly.
- **Required adjacent delivery (not identity):** A **walkable-world executable** is a **required first validation delivery** in this repository. It is Moria’s validation consumer, not part of the crate product. It must exercise the same public API available to external games through a generated, traversable, mutable natural region that validates generation, streaming, meshing, editing, voxel-truth collision, persistence, and performance. Its controls, camera, authored route/content, exact workloads, machine targets, platform constraints, and milestone status are harness concerns—not product mandates.
- **Out of product / repository:** The actual game is a separate downstream consumer and is not part of this repository. Game rules and the System, LLM, spell, gas, combat, AI, and building **layers** (UX, work orders, economy, game policy, authored game content) are out of scope. Compatibility seams may be designed where substrate needs demand them; those layers must not be implemented here. Substrate-owned placement primitives and structure representation remain in product; game building layers do not.

## Required product outcomes

- **Rust-consumable, GPU-resident matter world.** External games integrate public crate API(s). The substrate provides matter, physics, queries, and mutation with no in-tree privilege and no LLM dependency for core operation. Consumers obtain a mesh (or equivalent) view for rendering; gameplay-relevant physics and queries run against voxel truth, not a frozen mesh as authority.
- **Natural world as matter truth.** Generate surface composition that reads as ordinary outdoor terrain—biome-linked vegetation, rocks, rivers/water, caves, strata, and other underground geology—while remaining fully mutable voxel truth through the volume (deep Z first-class; nothing decorative outside the material model). Exact species, palette, region dimensions, and generation mechanisms stay design or first-slice adjacent.
- **Simulation families (substrate-owned, long-horizon).** Interactive voxel-backed vegetation and matter-linked dressing; static and dynamic water; ambient weather/time/fire simulation; granular behavior and structural failure. Mechanisms and delivery sequence after the first slice are design; these families are product outcomes, not optional consumer features.
- **Reusable interaction and semantic surfaces.** Placement/building primitives; reusable stamps or equivalent structure representation; mechanism-capable objects; structure/room metadata; mutation-safe navigation and 3D traversal data; extensible material/object/content registries; and a command/query/event boundary so higher layers never touch voxels directly. Building UI, work orders, economy, AI, System behavior, and authored game content remain consumer-owned.
- **Persistence across runs.** Persist voxel scars and placements and relevant moved-object/entity state; saved world deltas are reusable across runs. Product One single-slot or exact-restore benchmark modifiers are harness constraints, not the general product mandate.
- **First delivery slice (Product One settles depth).** Initial cut: full generation for the proof region; GPU incremental meshing; dressing; placed/rendered voxel objects; static water; dig/place and mirror queries; streaming; persistence; benchmarking as part of the deliverable; and the required adjacent walkable harness exercising the public API. Out of that cut (still long-horizon product): active CA/fire, higher fluid tiers, integrity, granular settling, and non-stretch felling.

## Future products and enabling implications

Future **consumers** (not this product) include System/ARPG-style play, fortress/colony-style play, descent/adventure modes, and pure sandboxes. They own gameplay, UX, controllers, characters, authored content, pricing policy, and presentation. The substrate outcomes above enable those games; they are not deferred to “maybe later consumers.” Delivery sequence among long-horizon families remains design after the first slice.

## Non-goals

- Shipping a game, combat loop, stats, AI agents, or multiplayer service in this repository.
- Implementing System/LLM, spells, gas/pricing policy, or game building UX (blueprints-as-gameplay, work orders, economy).
- Importing the harness’s demo fantasy, controls, camera, route/content, or machine/platform performance gates into the product mandate.
- Making the substrate depend on an LLM to function.

## Confirmed vision constraints

- **Ecosystem:** consumption as Rust crate(s) in a Cargo ecosystem; the in-repo walkable harness is separated from reusable substrate code at the consumer boundary (exact package layout is design).
- **API symmetry:** the harness uses the same public interfaces available to an external game.
- **Standalone engine layer:** zero required LLM/System dependency for core world operation.
- **Layer ownership:** gas/pricing and game policy are consumer-injected; substrate verbs and queries stay game-agnostic.
- **First-slice vs product:** Product One pins first-delivery depth and “done” for the initial validation delivery; it does not shrink long-horizon substrate identity to that slice alone.

## Deferred design decisions

- Representation and runtime choices (resolution, storage, meshing approach, streaming layout, delta encoding, sim scheduling).
- Exact public API shape, crate family split, and compatibility seams for future game layers.
- How far multiplayer-oriented command/mirror patterns are taken early.
- Delivery sequence and depth among long-horizon simulation families after the first slice (not whether those families are product outcomes).

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident Rust-crate substrate and the walkable-world executable as its separate validation consumer for generation, streaming, meshing, editing, collision, persistence, and performance.
- **`docs/seeds/project-boundary.md`:** Binding boundary—Rust crate product, game out of repo, harness adjacent not crate identity, public-API-only access, exclusion of game/System/spell/gas/combat/AI/building layers (not exclusion of substrate placement/semantic primitives).
- **`docs/seeds/product-one-seed.md`:** Settles first-delivery depth and the required walkable demo/validation delivery with its fused proof obligation; harness controls, region script, and targets stay adjacent, not product identity.
- **`docs/seeds/voxel-world-substrate.md`:** Authoritative long-horizon substrate outcomes—natural matter world, simulation families, placement/semantic surfaces, registries, API boundary, and persistence force—without making mechanism inventories or build-order steps vision mandates.
