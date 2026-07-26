# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer product for external games, not a game and not a game layer.

## Purpose

Games need a material world that looks continuous and natural while remaining fully diggable, placeable, and queryable as voxel truth—including deep underground—without each title rebuilding a private world core. Moria exists so downstream games share one substrate for matter, physics, queries, and mutation, with game rules living above it and with no LLM dependency in the substrate itself.

## Product boundary

**Belongs to Moria**

- The reusable voxel-world substrate and its public consumer interfaces (Rust crate surface).
- World generation, matter representation, presentation of voxel truth as a natural-looking world, mutation and query services, streaming/sparse residency, and edit persistence as substrate responsibilities.
- Compatibility seams where substrate requirements need them—without implementing the game layers those seams face.

**Does not belong to Moria**

- The actual game and its rules; those are separate downstream consumers outside this product’s identity.
- System/LLM, spell, gas, combat, AI, and building layers: out of scope to implement here.
- A walkable-world executable may exist only as an adjacent validation harness. It is not the product. While its delivery status is open (Q1), this brief does not treat that harness as required, optional, or part of current delivery—only as a possible adjacent artifact that, if present, must use the same public interfaces as an external game and must not own privileged or game-specific paths.

## Required product outcomes

1. **Reusable Rust substrate** — Ship Moria as a Rust crate (or small family of tightly scoped crates) that external games can consume without privileged in-repo access.
2. **Natural look, voxel authority** — Present a continuous, natural-looking world while voxel matter remains the authority for physics, queries, and mutation; rendered geometry is a regenerated view, not the truth store.
3. **Mutable material world** — Support destroy, place, and move of matter anywhere in the volume, including deep underground—not decorative shells over a non-material core.
4. **Deep Z and dig-honest geology** — Treat underground as first-class content: generation and matter must support continuous vertical play and dig-honest material structure (strata, voids, materials players can actually hit).
5. **Matter, physics, queries, mutation for consumers** — Provide the engine-layer responsibilities games build on—matter services, physics coupling against voxel truth, mirror-style queries, and mutation verbs—so higher layers never touch voxels directly.
6. **Large-world residency and edit persistence** — Keep large regions practical through sparse residency, lazy materialization, and streaming; persist edits relative to generated baseline (worldgen function plus deltas) so scars and player change survive without storing untouched volume.

## Future products and enabling implications

Future consumers—not this product—include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox titles. They own gameplay, UX, controllers, authored content, presentation policy, and game-specific rules (including gas pricing, combat, AI, and building-as-gameplay).

Enabling implications already implied by the substrate purpose: the same public matter/query/mutation surface should support those modes without embedding their rules; any validation harness exercises generation, streaming, meshing/view, editing, collision, persistence, and performance through that surface rather than by becoming a second product.

## Non-goals

- Implementing the actual game or game rules in this product.
- Implementing System/LLM, spell, gas, combat, AI, or building layers.
- Treating the walkable demo’s character, camera, route, seed content, or acceptance scene as substrate scope.
- Making LLM features a dependency of the world substrate.

## Confirmed vision constraints

- Integration form: Rust crate or small family of tightly scoped Rust crates.
- World residency: GPU-resident substrate.
- Access rule: consumers—including any validation harness—use the same public interfaces; no privileged or game-specific implementation paths for adjacent artifacts in this repo.
- Independence: substrate stands alone with zero LLM dependency.
- Scope fence: game rules and the listed future layers are not implemented here; seams only where substrate requirements demand them.

## Deferred design decisions

- Precise crate split and package layout inside the Rust surface.
- Delivery depth and sequence for generation, meshing/view, matter-physics breadth, streaming, and persistence.
- Voxel resolution, meshing approach, storage layout, algorithms, and similar mechanism choices.
- Whether and how a validation harness is built (content, controls, presentation, workloads, platforms, performance gates)—after Q1 settles delivery status.
- Open technical calls left in the substrate seed (e.g. resolution tradeoffs, distant LOD, object-layer scaling) as design/measurement work, not vision identity.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current delivery** of this effort, or only a **permitted adjacent artifact** that may be omitted without failing the product?

- **Proposed safe answer:** Permitted adjacent artifact only—the current product promise is the substrate; a harness may be added later to validate public interfaces without defining product success.
- **If answered differently:** Treating the harness as mandatory current delivery expands the effort to include an adjacent executable as a delivery commitment (still outside product identity) and forces planning for some consumer-facing walkable proof; it does not move game content, controllers, or presentation into the substrate itself.

## Seed synthesis

- **README.md** — Names Moria as a GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Settles product identity (substrate crate family), excludes the game and listed future layers, and binds the public-interface consumer rule for any harness.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look over voxel truth, full mutability, deep Z, matter/physics/queries/mutation, large-world residency and edit persistence) without making mechanism inventory into vision scope.
- **docs/seeds/product-one-seed.md** — Describes a first walkable proof and harness-oriented slice that motivates substrate capabilities and validation themes; its content, controller, platform, and performance specifics remain adjacent-consumer detail and do not redefine product identity.
