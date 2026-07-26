# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it through public interfaces; this repository is not the game.

## Purpose

Give adventure, fortress, sandbox, and related games a shared world foundation: natural-looking terrain whose look is derived from full voxel matter; free mutation anywhere including deep underground; and generation, streaming, meshing, collision, persistence, and reusable physical and world simulation that treat matter as the authority. Game rules, content, and presentation stay above the substrate so the same crate stack can support multiple products without embedding any one of them.

## Product boundary

**In product:** the reusable voxel-world substrate and its public consumer API—matter, generation, mutation, queries, derived presentation of that matter, streaming, persistence, and the substrate-owned physical and world-simulation behavior those services require.

**Adjacent, not identity:** a walkable-world executable may exist in this repository as a validation harness. It must use the same public interfaces available to an external game—no privileged or game-specific substrate paths. Whether that harness is a mandatory current delivery is open (see Q1); do not treat it as part of product identity.

**Out of repository / downstream:** the actual game; game rules; System/LLM, spell, gas, combat, AI, and building *layers*. Compatibility seams may be designed where the substrate requires them; those layers are not implemented here.

**Consumer-owned even when motivated by the harness or future games:** character controllers, cameras, authored demo routes and content, presentation policy, gameplay, UX, and acceptance scenarios for any particular consumer.

## Required product outcomes

1. **Reusable engine layer.** Expose a substrate that stands alone with no game-rule or LLM dependency, consumable by external games through the same public API surface used by any in-repo harness.
2. **Matter-authoritative world.** Voxels (material and related state) are the truth for mutation, collision, queries, and physical world behavior; extracted or dressed visuals are regenerated views, not saved authority.
3. **Natural surface over voxel truth.** Generated worlds read as ordinary outdoor terrain (hills, forests, water, cliffs, caves) while remaining fully material and diggable—not a decorative heightmap with props.
4. **Mutable everywhere, deep Z included.** Consumers can destroy, move, and place matter throughout the volume; underground (caves, strata, ores, voids) is first-class content space, not a false floor.
5. **Reusable physical and world simulation.** The substrate owns dynamic-matter and physical world behavior for a material world—collision against voxel truth, and world-side matter simulation consumers reuse—without embedding game combat, pricing, or policy. Delivery depth of particular simulators is design; ownership of this outcome family is not deferred to games.
6. **Geology-first generation; edit, stream, and persist as scars.** Worlds are produced as geology evaluable so large regions stay cheap until touched; dig/place-style mutation updates the living world; streaming keeps active regions live; persistence is worldgen plus edit deltas (and related object/event state), not a full voxel dump of untouched space.

These outcomes define product responsibility at vision altitude. Delivery depth, algorithms, data layouts, and milestone order are design work—not a narrowing of this identity. A first consumer slice that proves only part of the simulation family does not shrink the substrate’s enduring responsibility.

## Future products and enabling implications

Future *consumers* (not this product) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox tools. The substrate enables them by providing matter, physics, queries, and mutation; it does not implement their rules, content, or UX.

Richer behaviors within the substrate-owned simulation family (multi-tier fluids, structural integrity and cave-ins, fire/ecology, voxel objects that couple to rigid fall, priced verb/policy hooks, multiplayer-ready command style) are **enabling depth** of that family and motivation for generality—not a committed roadmap, and not consumer-owned gameplay. First-slice exclusions of a walkable demo do not redefine the substrate’s purpose.

## Non-goals

- Shipping the playable commercial game, its systems, or its content in this repository.
- Implementing System/LLM, spells, gas policy, combat, AI, or building gameplay layers here.
- Treating the validation harness’s controller, route, art, or benchmark theater as product features.
- Making the mesh, dressing, or other views authoritative over matter.
- Privileged in-repo access paths that external games cannot use.
- Transferring game-layer ownership of physical world simulation to each downstream title.

## Confirmed vision constraints

- Product form is Rust crate(s); intended integration is as a library for Rust consumers.
- The consumer boundary is mandatory: no privileged harness-only substrate paths.
- Substrate must remain free of game-rule and LLM dependency.
- Game rules and named future game layers listed above stay out of this product’s implementation scope.
- GPU-resident world matter and simulation orientation is part of product identity (not a consumer-local choice).

## Deferred design decisions

- Crate split and workspace layout that enforce the consumer boundary.
- Voxel resolution, LOD, meshing strategy, storage packing, and streaming-ring policy.
- How far generation, matter simulation, and API surface go in each delivery slice (including sequencing within the physics/dynamic-matter family).
- Whether and how adjacent validation artifacts are structured, scripted, or benchmarked.
- Open technical tradeoffs left in design seeds (e.g. fluid fidelity, object-layer scaling, multiplayer scope statements).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **mandatory current delivery** of this repository, or only a **permitted adjacent artifact** that may be added later?

- **Proposed safe answer:** Permitted adjacent artifact only—the current product commitment is the substrate crates and public API; a harness may be added and, if present, must consume public interfaces, but is not required for product completeness.
- **If different:** Treating the harness as mandatory adds a required adjacent deliverable (still outside product identity) and makes “substrate without a shippable walkable demo” incomplete; it does not move controllers, content, or acceptance gates into the substrate itself.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel substrate consumed as a Rust crate and separates the walkable-world executable as harness, not game layer.
- **docs/seeds/project-boundary.md** — Fixes product identity (Rust substrate crates), excludes the game and named game layers, permits a public-API-only harness, and makes the consumer boundary non-optional.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look over matter truth, universal mutability, deep Z, geology-first generation, streaming/persistence, matter/physics/queries/mutation, reusable layering) and lists future game consumers without making them this product.
- **docs/seeds/product-one-seed.md** — Describes a first walkable demo/harness slice and proof points that motivate substrate outcomes; its controllers, seed content, platforms, performance gates, and first-slice depth limits remain consumer- or delivery-scoped and do not redefine enduring substrate identity or exclude substrate-owned physics/simulation.
