# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for downstream games, not a game and not a gameplay product.

## Purpose

Moria exists so multiple future games can share one material world: a natural-looking continuous landscape whose truth is fully mutable voxels, with deep underground as first-class content. The substrate must stand alone—matter, physics, queries, and mutation without any LLM or game-rule dependency—so consumers can build ARPG, fortress, descent, or sandbox experiences on the same world stack.

## Product boundary

**This product owns:** the reusable voxel-world substrate and its public consumer surface (generation of material geology, GPU-resident matter, non-authoritative world presentation derived from that matter, mutation and query verbs, matter-coupled physics, and world persistence/streaming). Adjacent packages in-repo, if any, may only validate that surface.

**Adjacent, not product identity:** a walkable-world executable may exist as a validation harness. It is not a game layer. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific substrate paths. Whether shipping that harness is part of current delivery is open (see Q1). While open, this vision does not treat the harness as optional, required, or planned—only as an adjacent artifact that may exist.

**Not this product:** the actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and building layers as game systems. Compatibility seams may be designed where substrate responsibilities demand them; those layers are not implemented here. Character controllers, cameras, authored demo routes, UI, presentation polish, harness workloads, and acceptance scenes belong to consumers or the harness—not to substrate identity.

**Repository vs product:** the real game is a separate downstream consumer outside this repository’s product scope.

## Required product outcomes

Downstream design must make these consumer-visible guarantees true for the substrate (depth and sequence are design choices, not identity limits):

1. **Natural world, voxel truth.** Consumers can present a continuous natural landscape (terrain, cover, water bodies, cliffs, caves) that reads as an ordinary world while every visible solid remains backed by mutable voxel matter—not a heightmap with props.
2. **Mutation everywhere.** Through public verbs, consumers can dig, place, and otherwise alter matter anywhere in the volume; destroyed or carved faces remain honest material; the rendered surface is a regenerated view and is never authoritative or saved as truth.
3. **Deep Z is real.** Underground space, strata, voids, and buried materials are playable content in the same continuous 3D volume as the surface, not a decorative floor under the map.
4. **Geology-first, lazy worlds.** Worlds arise from seedable generation that builds geology (columns, strata, caves, ores, biomes/POI metadata as the substrate provides them) and materializes cost only where touched, so large regions stay tractable without eager full-volume residency.
5. **One public matter surface.** Matter, physics, queries, and mutation are reached only through the substrate’s public interfaces; in-repo validation and external games share that surface with no privileged back doors.
6. **Scar-cheap continuity.** World truth is generation plus edit deltas, with activity-centered residency so untouched bulk stays cheap and heavily edited regions remain reloadable as the same material world.

## Future products and enabling implications

Future **consumers** (not this product) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. They own gameplay, UX, controllers, content, economy, and policy (including how actions are priced).

Enabling implications the substrate should not foreclose: gas or labor pricing as a consumer policy on shared verbs; an LLM/System client that only reads mirrors and authors data through the same registries hand content uses; fortress- or adventure-scale use of the same deltas and material world; later matter behaviors (richer fluids, integrity, fire/ecology, granular settle, voxel objects such as trees) as substrate responsibilities when designed in—without importing those games’ content, controls, or acceptance scenarios into current scope.

A first walkable validation slice may exercise a curated region and dig/place proof for audiences and regression; that slice does not redefine Moria as a game demo product.

## Non-goals

- Shipping a commercial game, ARPG, fortress sim, or descent roguelike in this product
- Implementing System/LLM, spells, gas, combat, AI, or building layers here
- Treating harness character, camera, debug UX, seeded postcard content, or benchmark theater as substrate features
- Making the substrate depend on an LLM
- Replacing the public-API boundary with privileged in-repo engine paths for validation

## Confirmed vision constraints

- **Rust crate delivery:** the product is exposed as a Rust crate or small family of tightly scoped Rust crates; the consumer boundary between reusable substrate and any validation executable is mandatory (exact crate graph is design).
- **Public-only consumption:** any in-repo walkable-world harness, if present, consumes the substrate only through interfaces available to an external game.
- **GPU-resident world substrate:** the world substrate is intended to run as a GPU-resident matter foundation for its consumers.
- **No LLM in the substrate:** the world layer must function with zero LLM dependency.
- **Game systems stay out:** game rules and the future System, spell, gas, combat, AI, and building layers are not implemented in this product (seams only where substrate needs demand them).

## Deferred design decisions

- First-delivery depth versus full substrate capability (which matter behaviors ship when)
- Crate split, APIs, algorithms, data layouts, voxel scale, LOD, and meshing choices
- Streaming ring policy, persistence encoding, and physics coupling details
- Whether and how an in-repo harness is structured, what it shows, and on what machines it is measured
- Performance budgets, platforms, and backends (except where later design adopts an explicit product promise)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1. Is an in-repo walkable-world validation harness part of current delivery, or only permitted?**

- **Proposed safe answer:** Permitted only—the current product commitment is the reusable substrate and its public surface; a harness may be added to validate that surface but is not required for the product to be itself.
- **If answered differently:** Making the harness mandatory keeps substrate identity the same but expands current delivery to include an adjacent executable and whatever minimal public proof that commitment implies; it still must not pull character, content, or acceptance detail into substrate identity. Forbidding the harness entirely would remove even an in-repo validation consumer and force all proof out-of-repo.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`:** Fixes product identity (Rust substrate crates), forbids game/System/building-layer implementation here, and requires any harness to use the same public interfaces as an external game.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies the substrate’s purpose and outcome families—natural look over voxel truth, universal mutation, deep Z, geology-first lazy worlds, matter/physics/query/mutation layering, and persistence/streaming—without making future games or mechanism inventories part of identity.
- **`docs/seeds/product-one-seed.md`:** Describes a first walkable proof slice and demo non-goals that motivate dig/place honesty and substrate validation; its controller, region content, milestones, and hardware gates remain harness/consumer detail and do not redefine the current product as that demo.
