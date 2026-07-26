# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it through public interfaces; this repository is not the game.

## Purpose

Give adventure, fortress, sandbox, and related games a shared world foundation: natural-looking terrain whose look is derived from full voxel matter; free mutation anywhere including deep underground; and generation, streaming, meshing, collision, and persistence that treat matter as the authority. Game rules, content, and presentation stay above the substrate so the same crate stack can support multiple products without embedding any one of them.

## Product boundary

**In product:** the reusable voxel-world substrate and its public consumer API (matter, generation, mutation, queries, derived presentation of that matter, streaming, and persistence as substrate services).

**Adjacent, not identity:** a walkable-world executable may exist in this repository as a validation harness. It must use the same public interfaces available to an external game—no privileged or game-specific substrate paths. Whether that harness is a mandatory current delivery is open (see Q1); do not treat it as part of product identity.

**Out of repository / downstream:** the actual game; game rules; System/LLM, spell, gas, combat, AI, and building *layers*. Compatibility seams may be designed where the substrate requires them; those layers are not implemented here.

**Consumer-owned even when motivated by the harness or future games:** character controllers, cameras, authored demo routes and content, presentation policy, gameplay, UX, and acceptance scenarios for any particular consumer.

## Required product outcomes

1. **Reusable engine layer.** Expose a substrate that stands alone with no game-rule or LLM dependency, consumable by external games through the same public API surface used by any in-repo harness.
2. **Matter-authoritative world.** Voxels (material and related state) are the truth for mutation, collision, and queries; extracted or dressed visuals are regenerated views, not saved authority.
3. **Natural surface over voxel truth.** Generated worlds read as ordinary outdoor terrain (hills, forests, water, cliffs, caves) while remaining fully material and diggable—not a decorative heightmap with props.
4. **Mutable everywhere, deep Z included.** Consumers can destroy, move, and place matter throughout the volume; underground (caves, strata, ores, voids) is first-class content space, not a false floor.
5. **Geology-first generation and lazy world cost.** Worlds are produced as geology (columns, strata, caves, materials) evaluable so large regions stay cheap until touched; sparse storage keeps idle volume inexpensive.
6. **Edit, stream, and persist as scars on generation.** Dig/place-style mutation updates the living world; streaming keeps active regions live; persistence is worldgen function plus edit deltas (and related object/event state), not a full voxel dump of untouched space.

These outcomes define product responsibility at vision altitude. Delivery depth, algorithms, data layouts, and milestone order are design work—not a narrowing of this identity.

## Future products and enabling implications

Future *consumers* (not this product) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox tools. The substrate enables them by providing matter, physics-oriented coupling, queries, and mutation; it does not implement their rules, content, or UX.

Long-horizon substrate capabilities described in design seeds (richer fluid tiers, structural integrity and cave-ins, fire/ecology CA, voxel objects that fall as rigid bodies, priced verb/policy hooks, multiplayer-ready command style) are **enabling implications** for those consumers. They motivate substrate generality; they are not a committed roadmap or first-release checklist in this brief.

## Non-goals

- Shipping the playable commercial game, its systems, or its content in this repository.
- Implementing System/LLM, spells, gas policy, combat, AI, or building gameplay layers here.
- Treating the validation harness’s controller, route, art, or benchmark theater as product features.
- Making the mesh, dressing, or other views authoritative over matter.
- Privileged in-repo access paths that external games cannot use.

## Confirmed vision constraints

- Product form is Rust crate(s); intended integration is as a library for Rust consumers.
- The consumer boundary is mandatory: no privileged harness-only substrate paths.
- Substrate must remain free of game-rule and LLM dependency.
- Game rules and named future game layers listed above stay out of this product’s implementation scope.
- GPU-resident world matter and simulation orientation is part of product identity (not a consumer-local choice).

## Deferred design decisions

- Crate split and workspace layout that enforce the consumer boundary.
- Voxel resolution, LOD, meshing strategy, storage packing, and streaming-ring policy.
- How far generation, matter simulation, and API surface go in each delivery slice.
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
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look over matter truth, universal mutability, deep Z, geology-first generation, streaming/persistence, reusable layering) and lists future game consumers without making them this product.
- **docs/seeds/product-one-seed.md** — Describes a first walkable demo/harness slice and proof points that motivate substrate outcomes; its controllers, seed content, platforms, and performance gates remain consumer-owned and do not redefine product identity.
