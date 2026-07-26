# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** for external games. It is exposed as a Rust crate or a small family of tightly scoped Rust crates. It is not a game, not a presentation product, and not an LLM system.

A walkable-world executable may exist in this repository only as an adjacent validation artifact that consumes the substrate through the same public interfaces available to an external game. Whether that artifact is a required current delivery is open (see Q1).

## Purpose

Moria exists so multiple game products can share one foundation: a natural-looking, fully material, deep-Z voxel world that supplies matter, physics, queries, and mutation without embedding game rules, content authorship systems, or consumer presentation. Downstream games own gameplay, policy, UX, controllers, and authored experiences; the substrate owns the reusable world underneath them.

## Product boundary

**This product owns** the reusable world substrate and its public consumer surface: geological generation into sparse, streamable material volumes; matter representation and mutation; non-authoritative visual extraction of that matter; physical interaction and queries against voxel truth; streaming and edit persistence; and integration as a Rust crate library.

**Adjacent, not the product:** a walkable-world validation harness (if delivered) that proves substrate capabilities through public APIs only—no privileged or game-specific implementation paths. Its character, camera, route, content set, presentation, controls, workloads, platforms, and performance gates are harness-owned, not substrate identity (see Q1 for delivery status).

**Out of this repository and product:** the actual game; game rules; System/LLM; spells; gas policy; combat; AI; and building layers as gameplay systems. Compatibility seams may be designed where substrate outcomes require them, but those layers are not implemented here.

## Required product outcomes

1. **Natural material worlds.** Generated terrain reads as an ordinary natural surface world (hills, forests, water, cliffs, caves) while remaining fully material voxel truth underneath—not a heightmap with props.
2. **Mutation everywhere.** Any material volume can be destroyed, altered, or placed; cuts and scars remain true matter, and the rendered surface is a regenerated view, never the authority.
3. **Deep Z as content.** Underground is first-class continuous 3D space—strata, caves, and buried material—not a thin floor under a skybox.
4. **Geology-first generation, sparse residency, and durable edits.** Worlds generate as geology, materialize on demand so large regions stay tractable, stream around activity, and persist changes as edits over the generative baseline rather than dense whole-world snapshots.
5. **Consumer API boundary.** Games and harnesses interact only through public verbs and queries; nothing above the substrate touches voxels directly, and in-repo consumers get no privileged path external games lack.
6. **Standalone reusable foundation.** The substrate runs with zero LLM dependency and provides matter, physics, queries, and mutation so genre-specific games can sit above it without forking the world core.

## Future products and enabling implications

Future consumers (not this product) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style descent experience, and pure sandboxes. Those games motivate a substrate that is mutable, deep-Z capable, queryable, and free of baked-in game policy. This vision does not commit their gameplay, content, controllers, presentation, or first-playable slices as substrate scope. Long-horizon matter behaviors that would further those games (extended fluid play, structural failure, ambient ecology, full construction toyboxes) are enabling implications for design to sequence—not a committed roadmap in this brief.

## Non-goals

- Shipping an actual game, game rules, combat, AI, spells, gas economy, or System/LLM features in this product
- Implementing building, semantic, or presentation layers that belong to games (blueprints-as-gameplay, mechanisms-as-game-systems, HUD, Diablo-style camera lock, authored campaign content)
- Treating the validation harness’s character, demo route, seed region décor, or benchmark theater as substrate requirements
- Making the substrate depend on an LLM or other game-layer client

## Confirmed vision constraints

- **Delivery form:** Rust crate or small family of tightly scoped Rust crates for external game consumption
- **Residency model:** GPU-resident voxel-world substrate
- **Consumer parity:** any in-repo validation executable must use the same public interfaces as an external game; privileged substrate paths for the harness are forbidden
- **Independence:** the substrate must stand alone with zero LLM dependency
- **Layering:** game rules and future System, spell, gas, combat, AI, and building layers stay above this product

## Deferred design decisions

- Depth and sequence of matter-simulation capabilities beyond the core outcomes above (e.g. fluid tiers, integrity, granular settle, fire ecology)
- Representation, meshing, streaming-ring, and persistence encodings that realize the outcomes
- Crate split and packaging inside the Rust ecosystem (boundary intent is fixed; structure is not)
- Voxel resolution, LOD, object-layer capacity, and other measurement-driven technical choices
- Whether and how multiplayer authority is later expressed on the public verb surface
- Harness content, controls, platforms, and acceptance thresholds if a walkable-world artifact is delivered

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** of this repository, or only **permitted** as an adjacent artifact?

- **Proposed safe answer:** Required as an adjacent validation harness that exercises generation, streaming, meshing, editing, collision, persistence, and performance through public substrate interfaces only—not as part of substrate product identity, and without importing its controller, character, content, presentation, route, platform, or numeric gates into that identity.
- **If answered differently:** “Permitted only” means current delivery can be crate-only until a later decision; “required” commits repository delivery of a harness executable without expanding what the substrate product *is*.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Fixes product identity (Rust crate substrate), repository exclusion of the actual game, public-interface harness rule, and out-of-scope game/System/building layers.
- **docs/seeds/product-one-seed.md** — Describes a product-shaped walkable demo that motivates proving material-world claims; its controller, seed content, platforms, milestones, and gates stay harness/demo-owned and do not redefine substrate identity.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate purpose and outcome families (natural look over voxel truth, full mutability, deep Z, geology-first sparse worlds, matter/physics/queries/mutation, LLM-free reuse) without transferring mechanism inventory or future-game features into current scope.
