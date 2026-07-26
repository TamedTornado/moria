# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation: matter, generation, presentation of voxel truth, queries, mutation, and related physics-facing world services—not a game.

## Purpose

Moria exists so downstream games can share one material world stack: a natural-looking surface over fully mutable voxel truth, with deep underground treated as first-class space. The substrate must stand alone with no dependency on game rules, LLM systems, or a particular title. Multiple game styles (adventure, fortress/colony, sandbox, descent) are intended consumers of the same crate boundary.

## Product boundary

**In product**

- The reusable voxel-world substrate and its public consumer-facing interfaces (Rust crate API).
- World and matter capabilities games need without owning representation: generation of natural geology and surface, sparse GPU-resident storage, smooth meshed views of voxel truth, dig/place and related matter mutation, collision and queries against voxels, streaming residency, and persistence of generated world plus edits.
- Compatibility seams the substrate needs so game layers can attach later without embedding those layers here.

**Adjacent, not the product**

- A walkable-world executable, if present, is only a validation harness and separate consumer. It must use the same public interfaces available to an external game and must not own privileged or game-specific paths. Whether that harness is a required delivery of this effort remains open (see Q1).
- The actual game lives outside this repository as a downstream consumer.

**Out of this product**

- Game rules and gameplay systems: combat, AI, economy, spells, gas/pricing policy, LLM/System behavior, and building/gameplay layers.
- Consumer-owned presentation, controllers, characters, authored demo content, UI, and title-specific acceptance scenarios.

## Required product outcomes

Downstream design must make these true of the substrate:

1. **Reusable Rust world substrate.** Consumers integrate Moria as crate(s) and obtain a material-world foundation without pulling in a game.
2. **Natural look, voxel truth.** Generated regions read as ordinary outdoor worlds (terrain, water bodies, vegetation dressing, cliffs, underground spaces) while the voxel field remains the authority for matter, simulation, and interaction; meshes and dressing are views derived from that truth.
3. **Mutable everywhere, including deep Z.** Any solid volume in the playable field can be destroyed, placed, or otherwise edited; underground depth is content space (strata, caves, buried material), not a decorative floor.
4. **Geology-first generation with lazy cost.** Worlds are produced as geology and columns that materialize on demand so large regions stay tractable when mostly untouched.
5. **Matter services for games.** The substrate exposes mutation and query surfaces (including dig/place) and provides the world-side physics and matter behavior games rely on—support for collision against voxel occupancy, structural and granular honesty where matter requires it, fluid bodies and related matter rules as substrate capabilities—without encoding game policy.
6. **Streaming and persistence.** Active regions stream in and out; truth is the generation function plus edit deltas so scars and player change remain cheap to store and restore.
7. **Hard consumer boundary.** Nothing above the matter surface touches voxels directly; harnesses and games share the public verb/query path. Adjacent consumers have no privileged access.

## Future products and enabling implications

- **Downstream games** (ARPG with a System/LLM layer, fortress/colony play, descent/roguelike, pure sandbox) are future or external consumers. Moria enables them by owning world, matter, mutation, and query capability; it does not own their rules, content, controllers, or presentation.
- **Walkable-world / “product one” style demos** may exercise generation, meshing, editing, collision, streaming, persistence, and performance through public APIs. Their routes, characters, debug controls, region art direction, and hardware score targets remain consumer concerns (delivery status of a harness: Q1).
- Long-horizon matter richness described for later game fantasies (rich fluid toyboxes, full fire ecology, fortress machinery, multiplayer) motivates substrate generality; it is not a committed game roadmap inside this product.

## Non-goals

- Shipping a game, combat loop, AI, spells, gas economy, or LLM/System.
- Implementing building, semantic, or game-policy layers in this repository.
- Treating a demo character, free camera, authored postcard region, or social-post milestones as the product identity.
- Binding product identity to a single demo machine, GPU class, or consumer benchmark scene.

## Confirmed vision constraints

- **Rust crate delivery** for the substrate; intended integration is as a library consumers depend on.
- **GPU-resident** world/matter substrate.
- **Substrate stands alone**—no LLM/System dependency for core world operation.
- **Strict layering:** game rules live above; consumers use public interfaces only.
- **Validation harnesses**, when they exist, exercise the substrate only through those public interfaces.

## Deferred design decisions

- Crate split and workspace layout (boundary is required; packaging is design).
- Voxel scale, LOD, meshing strategy, storage layout, and sim scheduling.
- Depth and sequence of matter subsystems (fluids tiers, integrity, CA, object layer, ambient weather) within the substrate outcome families above.
- Persistence encoding, streaming ring policy, and performance budgets.
- Whether and how a validation executable is structured, what scene it loads, and on which machines it is measured—after Q1.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required deliverable** of the current Moria effort, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted adjacent artifact—encouraged for proving public APIs, but not part of product identity; substrate crates can ship and be validated without mandating a specific walkable demo binary in this vision.
- **If answered differently:** Making the harness mandatory adds a required adjacent delivery (still not game scope) and obligates design to plan a public-API walkable consumer; it does not move controllers, content, or performance gates into the substrate product itself.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident voxel-world Rust substrate and separates the walkable-world executable as harness, not game layer.
- **docs/seeds/project-boundary.md** — Fixes current product identity, Rust crate boundary, out-of-repo game, harness interface rules, and exclusion of game/System/building layers.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate purpose and outcome families (natural look over voxel truth, full mutability, deep Z, generation, matter/physics/query/mutation, streaming, persistence, reusable layering).
- **docs/seeds/product-one-seed.md** — Describes a first walkable consumer/demo slice and its non-goals; motivates validation concerns without redefining the substrate as that demo or importing its content, controls, or machine targets into product scope.
