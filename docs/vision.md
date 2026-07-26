# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation: matter, world generation, queries, mutation, and related world services that external games consume through public interfaces. It is not a game.

## Purpose

Moria exists so multiple games can share one material world foundation instead of each reimplementing geology, mutability, deep underground space, and GPU-backed world services. The substrate must stand alone with no LLM or game-rule dependency, so a System ARPG, fortress/colony game, descent-style adventure, pure sandbox, or similar title can sit above the same stack.

## Product boundary

**Belongs to Moria**

- The reusable voxel-world substrate and its public consumer-facing surface (crate API).
- High-level world responsibilities: natural-looking presentation over voxel truth, full mutability of matter, first-class deep vertical play space, generation, streaming, meshing, editing, collision against voxel truth, and persistence of world change.
- Compatibility seams only where substrate requirements demand them—not implementations of upper layers.

**Does not belong to Moria**

- The actual game: rules, combat, stats, AI, economy, gas/pricing policy, spells, System/LLM behavior, and building layers.
- Controllers, characters, cameras, authored demo routes, presentation polish, and acceptance scenarios of any adjacent walkable-world or validation executable.
- Game-specific policy tables, content packs, and UX.

A walkable-world executable may exist as an adjacent validation harness that exercises the substrate only through the same public interfaces available to an external game (no privileged paths). Whether that harness is a required current delivery remains open—see Q1. Until resolved, this brief does not treat it as mandatory, optional, planned, or in-scope delivery; only as a permitted adjacent artifact form.

## Required product outcomes

1. **Material world truth.** Consumers get a fully mutable voxel world: any volume can be destroyed, moved, or placed; nothing important is decorative geometry outside the material model.
2. **Looks normal, behaves as voxels.** The world reads as continuous natural terrain and structure; the render surface is a view over voxel truth, not the authority for physics or queries.
3. **Deep Z is first-class.** Underground volume is real content space (strata, voids, descent), not a thin floor under a heightmap.
4. **World services for consumers.** The substrate provides generation, streaming, meshing, editing, collision against voxel truth, and persistence of edits so games build on a living world rather than static scenery.
5. **Reusable engine boundary.** Integration is through public Rust-crate interfaces; matter is reached via the substrate’s verbs and queries, not private voxel access. Gas, System, and game rules inject above this product or as policy outside it.
6. **GPU-resident substrate.** The product identity includes GPU-resident world operation suitable for reuse as a crate stack, independent of any one game’s presentation.

## Future products and enabling implications

Future *consumers* (not this product) include a System/LLM-driven ARPG, Dwarf Fortress–style fortress/colony play, a Moria-style descent adventure, and pure sandbox modes. They motivate breadth of matter, query, and mutation capability.

Enabling implications for Moria (not a committed consumer roadmap): keep the substrate free of LLM dependency; leave room for priced verbs and agent-driven mutation without encoding game pricing; support continuous 3D worlds that fortress-style tools may later *present* in slices; allow later games to author materials, placements, and structures through registries and stamps without those games living in this repository.

Gameplay, controllers, characters, animation, combat, building UX, and authored content remain consumer-owned.

## Non-goals

- Implementing the actual game or shipping game rules, combat, AI, spells, gas economy, or System/LLM features in this product.
- Owning building layers, blueprints-as-gameplay, mechanism gameplay, or room/economy policy.
- Treating the validation harness’s character, camera, debug UX, seed route, content set, hardware targets, or benchmark numbers as product identity.
- Making the substrate depend on an LLM or on any single game’s policy.

## Confirmed vision constraints

- Product form: Rust crate or small family of tightly scoped Rust crates.
- GPU-resident voxel-world substrate.
- Adjacent harnesses and external games share the same public interfaces; no privileged game-only substrate paths.
- Game rules and future System, LLM, spell, gas, combat, AI, and building layers are out of scope for implementation here.
- Substrate stands alone with zero LLM dependency.

## Deferred design decisions

- Exact crate split and packaging inside the Rust workspace boundary.
- Capability depth and delivery sequence for generation, matter simulation, vegetation/objects, fluids, integrity, ambient sim, and related services (outcome altitude is fixed; slice order and mechanism choice are design).
- Voxel resolution, storage layout, meshing strategy, LOD, streaming rings, and persistence encoding.
- Whether multiplayer/server-authoritative readiness is in early design scope.
- Validation harness content, controls, platforms, and performance gates—if a harness is delivered at all (Q1).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a *required current delivery* alongside the substrate crates, or only a *permitted* adjacent artifact?

- **Proposed safe answer:** Permitted only—current product delivery is the substrate crates; a harness may be added later without being part of product identity.
- **If different:** Requiring it keeps substrate identity but adds a mandatory adjacent deliverable (still not game content ownership); design must then plan a harness that uses only public APIs, without pulling controller/content/performance detail into the substrate’s own scope.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Binds repository product identity to the substrate crate(s), enforces public-API-only consumption for any harness, and excludes game/System/LLM/spell/gas/combat/AI/building layers from this product.
- **docs/seeds/product-one-seed.md** — Describes a first walkable-world demo and substrate slice (character, seed region, debug dig/place proof, milestones, machine-specific targets); used only as consumer/harness motivation and first-slice color, not to redefine product identity or import harness-owned scope.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate-level purpose and outcomes (natural look over voxel truth, full mutability, deep Z, matter/physics/queries/mutation, reusable layering, standalone engine) that motivate required outcomes; mechanisms and build-order detail stay deferred to design.
