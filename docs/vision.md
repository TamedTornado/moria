# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped Rust crates). Games and other tools consume it through public interfaces. It is an engine-layer product for material worlds—not a game, not a demo experience, and not an LLM or rules stack.

## Purpose

Moria exists so multiple future games can share one trustworthy world foundation: a natural-looking surface over fully mutable voxel matter, continuous three-dimensional space including deep underground, and clean mutation/query seams that keep game policy above the substrate. The substrate must stand alone with zero dependency on any System, LLM, or game-specific rules layer.

## Product boundary

**In product**

- Reusable world substrate: generation, material storage and mutation, presentation of voxel truth as a non-authoritative view, collision against matter (not against a frozen mesh), streaming, and persistence of world truth as generation plus edit deltas.
- Public consumer-facing verbs and queries so nothing above the matter layer touches voxels directly.
- Optional adjacent **walkable-world validation harness** that exercises the substrate only through those same public interfaces (see Q1).

**Out of product (adjacent or downstream)**

- The actual game(s), game rules, combat, stats, AI, economy, and authored campaign content.
- System / LLM integration, spells, gas/pricing policy, and intent layers.
- Building as a game layer (blueprints-as-gameplay, work orders, mechanisms-as-gameplay, room assignment, fortress UX). Compatibility seams may be designed later where substrate contracts require them; those layers are not implemented here.
- Harness-owned concerns unless later bound as product: character controller, camera, demo route, authored seed scenery package, debug presentation chrome, platform-specific acceptance gates, and marketing milestones.

**Repository posture**

- External games are separate consumers; this product must not grant the harness privileged or game-specific implementation paths.
- Ecosystem fact: Rust crates over a portable GPU stack (wgpu/WGSL)—not a proprietary native-Metal-only core.

## Required product outcomes

A downstream design must make these true:

1. **Voxel truth, material everywhere** — Any solid of the world is mutable matter. Digging and placing change the world; the visible mesh is a regenerated view, never the authority for physics or queries.
2. **Reads as a normal world** — Rolling terrain, forests, water, cliffs, and underground spaces present as a continuous natural environment; the grid is the truth, not a cube aesthetic as the primary look.
3. **Deep Z is first-class** — Underground volume is real playable content (caves, strata, buried materials), not a painted floor under a heightmap.
4. **Geology-first generation with cheap idle cost** — Worlds are produced as layered geology and related structure, materializing interest lazily so untouched volume stays sparse and cheap.
5. **GPU-resident substrate usable as a library** — The world lives as a consumable Rust engine layer with command/query coupling suitable for later discrete-GPU and multiplayer-ready boundaries; consumers integrate through public APIs only.
6. **Streaming and scar-cheap persistence** — Active regions stream in; truth is worldgen plus compact edit deltas so heavily altered places remain practical to save and reload.

## Future products and enabling implications

Future **consumers** (not this product): a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style deep descent, and pure sandbox modes. A later “product two” game starts from a walkable material world, not from a whiteboard.

**Enabling implications** (substrate may eventually support; not a committed current roadmap): richer matter simulation (fire, wetness, granular settle, multi-tier fluids, structural integrity), vegetation and clutter as matter-backed or matter-driven dressing, object-layer interactables, priced verb policies injected by games, and nav/mirror aggregates games can read. Gameplay, UX, controllers, characters, animation, and content for those modes remain consumer-owned.

## Non-goals

- Shipping a playable game, ARPG loop, fortress mode, or combat/AI stack in this repository.
- Implementing System/LLM, spells, gas metering, or building/fortress gameplay layers here.
- Treating the validation harness’s character, camera, demo itinerary, or performance scorecard as the product’s identity.
- Making the LLM generate geology or requiring any LLM for the substrate to function.
- Expanding this brief into algorithms, crate graphs, voxel sizes, benchmark numbers, or milestone catalogs (design phase).

## Confirmed vision constraints

- Product identity and repo boundary: substrate yes; full game no; harness is consumer of public APIs only.
- Substrate stands alone with zero LLM dependency.
- Intended integration surface: Rust crate(s) for external game consumers.
- Portable GPU path via wgpu/WGSL in load-bearing layers; no native-Metal fork of those layers.
- Dev environment called out by seeds: Apple Silicon (M4-class) constraints include no reliance on 64-bit buffer atomics; bandwidth-conscious sparsity is load-bearing, not a later polish pass.
- Game rules, System/LLM, spell, gas, combat, AI, and building layers are out of implementation scope here.

## Deferred design decisions

- How much of the long-horizon matter, fluid, integrity, vegetation-object, and ambient-sim suite ships in the first vertical slice versus later substrate increments.
- Exact public API shape, crate family split, meshing approach, storage layout, streaming ring policy, and persistence encoding.
- Voxel scale, LOD strategy, and object-layer capacity—decision bed for measurement, not vision identity.
- Whether multiplayer is stated in scope language early or only architectural readiness is preserved.
- Concrete validation scenarios, demo content, controllers, and numeric performance gates for any harness.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an adjacent walkable-world validation executable a **mandatory current delivery**, or only **permitted** beside the substrate crates?

- **Proposed safe answer:** Mandatory as a thin adjacent harness that proves substrate outcomes through public interfaces only—not a game-shaped product and not a carrier of harness-specific content, controls, or acceptance gates into substrate identity.
- **If different:** “Permitted only” means current delivery can be crates and automated/API tests alone; “mandatory with Product One’s full demo package” would expand current identity toward a character-driven walkable demo with authored region and performance gates, which this brief currently keeps out of product scope.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and frames the walkable executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binding boundary: substrate crates in-repo; real game out; harness public-API-only; game/System/building layers out of scope.
- **docs/seeds/product-one-seed.md** — First-slice and demo-shaped validation pressure (walkable region, edit proof, harness concerns); used to surface the harness delivery question, not to redefine product identity.
- **docs/seeds/voxel-world-substrate.md** — Long-horizon world-substrate outcomes (material mutability, natural look, deep Z, geology, library layering, streaming/persistence) fused into product outcomes and enabling implications without importing game layers or mechanism inventories.
