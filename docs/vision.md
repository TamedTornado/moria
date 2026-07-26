# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer library for natural-looking, fully material, mutable 3D worlds—not a game, not a content pack, and not a character-driven demo product.

## Purpose

Moria exists so multiple games and tools can share one world foundation: a surface that reads as ordinary terrain while remaining voxel truth underneath; free mutation anywhere in continuous 3D, including deep underground; and a clean separation so game rules, presentation, and policy stay above the substrate. The substrate must stand alone with **no LLM or System dependency**.

## Product boundary

**Belongs to Moria**

- The reusable voxel-world substrate and its public consumer interfaces (matter, generation-backed world state, queries, mutation, and related physics/sim capabilities expressed at the substrate boundary).
- Integration as a Rust library (crate or tightly scoped crate family) for external games and tools.

**Adjacent, not product identity**

- A walkable-world executable **may** exist in this repository as a **validation harness** only. If present, it must consume the substrate through the same public interfaces available to an external game—no privileged or game-specific implementation paths.
- The actual game, and any “product one” walkable demo’s controller, camera, authored seed route, debug presentation, scripted workloads, and performance gates, are adjacent consumers of the substrate—not definitions of the substrate product.

**Outside this product and repository**

- The shipping game and its rules.
- System / LLM, spells, gas policy, combat, AI, and building layers (game and semantic building systems). Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

Downstream design must make these true for substrate consumers:

- **Natural world, material truth.** Consumers can present a world that reads as normal terrain (hills, forests, water, cliffs, underground) while the voxel field remains the authoritative matter representation—not decorative geometry beside a heightmap.
- **Mutable everywhere.** Consumers can destroy, place, and otherwise alter matter throughout the volume; cuts and scars remain real material change, not mesh-only edits.
- **Deep Z is first-class.** Underground space is playable content volume (caves, strata, buried material), not a thin floor under a skybox.
- **Substrate services, not game rules.** The product provides matter, physics, queries, and mutation so multiple game styles can sit above it; pricing, combat, agents, spells, and game policy remain consumer-owned.
- **GPU-resident world substrate.** Authoritative world matter is intended to live and update in a GPU-resident model suitable for large sparse regions.
- **Generation that supports dig honesty.** Worlds are produced so geology and voids exist as real structure for traversal and mutation, not as painted under-heightmap filler.
- **Derived views stay non-authoritative.** Presentation (e.g. smoothed terrain) is regenerated from matter; interaction and simulation against the world use voxel truth rather than treating the render mesh as source of truth.
- **Equal public boundary.** Every consumer—including any in-repo harness—uses the same public substrate interfaces; nothing above the matter boundary reaches voxels through private paths.

## Future products and enabling implications

Described games (System ARPG, fortress/colony, Moria-style descent, pure sandbox) are **future or external consumers**, not current Moria scope. They motivate a substrate that stays free of game rules and can later support richer matter behavior, semantic overlays, and authored content packages without embedding those games.

A walkable “product one” demo may exist as an adjacent harness or showcase consumer; its specific region, character, UI, and benchmark protocol do not transfer into substrate identity or required outcomes. Long-horizon matter and world behaviors that would enable fortress- or adventure-style play remain **enabling implications** for design prioritization—not a committed multi-game roadmap in this brief.

## Non-goals

- Implementing a full game, ARPG, fortress mode, or descent roguelike in this repository.
- System / LLM features, spells, gas metering, combat, AI, or building-game layers (blueprints-as-gameplay, work orders, mechanisms-as-game systems).
- Treating the validation harness’s character controls, camera, content route, or presentation as substrate features.
- Making the substrate depend on an LLM to generate or run the world.

## Confirmed vision constraints

- **Rust crate consumer surface** — intended integration is as a Rust crate or small family of tightly scoped crates.
- **No LLM dependency** — the world substrate must function as a standalone engine layer without the System.
- **Harness privilege ban** — any walkable-world executable in-repo is validation-only and must use public interfaces only.
- **Game layers stay out** — game rules and the listed future game layers are not implemented in Moria; seams only where substrate needs require them.

## Deferred design decisions

- Delivery depth and sequence of matter/sim capabilities (how much fluid, integrity, vegetation, ambient sim, and related behavior ships when).
- Representation, meshing, storage, streaming, and persistence mechanisms.
- Precise crate split and workspace layout (beyond the non-optional consumer boundary).
- Voxel resolution, LOD, object-layer capacity, and other open substrate parameters.
- Whether and how multiplayer-oriented command authority is pursued.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and frames the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Fixes product identity on the substrate crate(s), places the real game outside the repo, constrains any walkable executable to a public-API harness, and excludes System/LLM/spell/gas/combat/AI/building layers from implementation here.
- **docs/seeds/product-one-seed.md** — Describes an adjacent first walkable demo/harness slice (region, character, proof dig/place, milestones); used only to clarify validation-consumer relationship and material-world proof intent, not to import demo content, controls, or performance gates into product scope.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate purpose and outcome altitude: natural look with voxel truth, full mutability, deep Z, reusable matter/physics/query/mutation services, GPU-resident foundation, and geology-oriented generation—without adopting its mechanism inventory or game-layer designs as current mandates.
