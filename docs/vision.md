# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world product: matter, queries, mutation, and substrate-owned world physics that games consume. It is not a game, not a game ruleset, and not a character-driven experience product.

This repository’s required first delivery also includes an **adjacent** walkable-world executable that validates generation, streaming, meshing, editing, collision, persistence, and performance through public substrate interfaces. That executable is not a game layer and is not Moria’s product identity.

## Purpose

Moria exists so multiple downstream games can share one material world foundation: a natural-looking surface over fully mutable voxel truth, with deep underground as first-class space, without each game reimplementing world matter. The substrate stands alone with no LLM or “System” dependency. Game identity, policy, and presentation live above it.

## Product boundary

**Belongs to Moria**

- The reusable voxel-world substrate and its public consumer-facing interfaces (Rust crate boundary).
- Outcome-level world capabilities: geology-backed generated material worlds; full mutability of matter; non-authoritative presentation from matter; voxel-backed interactive surface features; reactive matter and substrate-owned objects; command/mirror mutation and observation; durable restoration of the mutable world and substrate-owned object state; streaming of large regions.
- Compatibility seams only where substrate requirements demand them—not implementations of excluded game layers.

**Does not belong to Moria**

- The actual game (any ARPG, fortress, descent, or sandbox title) and its rules, content, UX, controllers, characters, combat, AI, economy, gas/pricing policy, spells, or LLM/System layer.
- Game-facing building, designation, work-order, and similar policy layers (matter placement as engine capability remains substrate; game building systems do not).
- Privileged or game-specific implementation paths that bypass the public substrate interfaces.

**Adjacent first delivery (not product identity)**

- A walkable-world validation executable is a **required** adjacent first delivery. It must use only public interfaces available to an external game. Its controller, character, camera, demo route, seed content, presentation, machine-specific gates, and performance thresholds are harness concerns—not Moria’s product identity. That slice proves a first usable depth of substrate outcomes; it does not narrow the substrate’s broader identity or required outcome families.

## Required product outcomes

Downstream design must make these product-level outcomes true:

1. **Material world truth.** Consumers operate on a fully mutable voxel material world: any matter can be destroyed, moved, or placed; dig and place are first-class substrate capabilities, not decorative geometry outside the simulation.
2. **Looks natural, stays voxel-true; deep Z is first-class.** The world can read as ordinary continuous terrain and structure while remaining voxel-authoritative; rendered surface is a non-authoritative view regenerated from matter, never save or physics truth. Underground volume (caves, strata, depth) is real space, not a painted floor under a heightmap.
3. **Geology-backed generation.** Worlds generate as layered geology and related structure so digging and exploration encounter honest material depth, materializable lazily so large regions remain tractable.
4. **Surface authority and reactive matter.** Everything that can burn, break, or block is voxel-backed; only non-interactive dressing may be derived from or anchored to voxels. Materials and substrate-owned objects react consistently through environmental state, flow, loss of support, motion, and granular behavior—including object failure and growth, granular settle, matter-coupled weather and fire ecology, and integrity-driven collapse. First-delivery depth of these families is design sequence; the product mandate is not static matter plus ambient cosmetics.
5. **Command/mirror contract and durable continuity.** Consumers mutate and observe through commands in and a stale mirror plus events out; upper layers do not touch voxels directly. World edits and substrate-owned object/entity transitions persist across streaming and runs so the mutable world and its object state can be restored.
6. **Reusable engine boundary.** The same substrate supports multiple game genres by exposing matter, physics-relevant behavior, queries, and mutation without embedding game rules, gas policy, or LLM dependency; adjacent consumers have no privileged access beyond public interfaces.

Enabling services implied above—multi-tier fluids, path-relevant derived data, and active-region streaming—are substrate responsibilities at outcome altitude. Delivery depth and sequence are design concerns.

## Future products and enabling implications

Future or separate products that consume Moria (not built here):

- System-driven ARPG and related spell/gas/combat experiences.
- Dwarf Fortress–style fortress/colony play.
- Moria-style descent / adventure and pure sandbox titles.

High-level enabling implications only: consumers need public mutation and query interfaces under the command/mirror contract, material and placement registries, and seams for game-injected policy (e.g. pricing of verbs) without the substrate owning those policies. Gameplay, content, presentation, controllers, characters, and game-specific systems remain consumer-owned.

## Non-goals

- Implementing the game, System/LLM, spells, gas/intent pricing, combat, AI, or game building layers.
- Treating the validation harness’s demo fantasy (specific character, route, postcard seed, clip goals, or machine gates) as Moria’s product definition.
- Making the substrate depend on an LLM or in-process game ruleset.
- Multiplayer product delivery as a current commitment (command-style boundaries may remain design-friendly; online service is not promised here).

## Confirmed vision constraints

- **Ecosystem:** Moria is a Rust crate (or small family of tightly scoped Rust crates) for integration by Rust consumers.
- **GPU residency and observability:** World matter and related sim are GPU-resident with a FleX-pattern consumer contract—commands in, stale mirror plus events out—so consumers never treat live voxel storage as a direct upper-layer surface.
- **Portable GPU path:** Load-bearing GPU work stays on a portable wgpu/WGSL path rather than a native Metal-only fork; portability across backends is part of the crate promise.
- **Consumer isolation:** The walkable-world validation executable and any other in-repo harness must use only public substrate interfaces available to an external game—no privileged harness-only world paths.
- **Standalone engine:** Zero LLM/System dependency inside the substrate; those remain optional game-layer clients.
- **Scope exclusion:** Game rules and the listed future game layers stay out of this repository’s product.

## Deferred design decisions

- Crate split, API surface shape, and internal layering within the substrate family.
- Representation, meshing, storage, generation pipeline, and sim scheme choices that realize the outcomes above.
- Capability depth and delivery sequence within the substrate (including how much of each reactive-matter family the first adjacent proof exercises versus later growth).
- Whether and how far multiplayer-authoritative deployment is pursued later.
- Harness-only detail: controls, content, presentation, platforms, benchmarks, and acceptance thresholds for the walkable-world executable.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world Rust substrate and identifies the walkable-world executable as the separate validation consumer for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **`docs/seeds/project-boundary.md`:** Binds product identity to the substrate crate(s), excludes the actual game and listed game layers, and requires any harness to consume only public interfaces while remaining adjacent.
- **`docs/seeds/product-one-seed.md`:** Settles the walkable-world executable as the required first adjacent delivery and proof slice; retains the command/mirror boundary and portable wgpu/WGSL path; harness controls, content, and machine gates stay out of substrate identity.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies substrate purpose and outcome families—mutable material world, natural look, deep Z, geology-first generation, voxel-backed interactive matter vs derived dressing, reactive matter (objects, granular, weather/fire, integrity), command/mirror GPU contract, durable world and object state, multi-game reuse without LLM dependency.
