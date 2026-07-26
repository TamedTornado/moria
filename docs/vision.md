# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates for external game consumers.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface and deep underground that are the same fully mutable voxel truth. The substrate provides matter, material physics, queries, and mutation. Game rules, content policy, and presentation ownership live above it. The stack must stand alone with no LLM dependency.

## Product boundary

**In product:** the reusable voxel-world substrate and its public consumer-facing surface (verbs, queries, events, and material/world registries needed to use that surface).

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness for terrain generation, streaming, meshing, editing, collision, persistence, and performance. It is not the product and must use the same public interfaces an external game would use—no privileged or game-specific implementation paths. Whether that harness is a required delivery is open (Q1).

**Out of repository / out of product:** the actual game and game rules; System/LLM features; spells, gas policy, combat, AI; and building as a game layer (UI, work orders, room economy, designation policy). Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

- **Material truth under a natural look.** Downstream consumers get a world that reads as ordinary terrain (hills, forests, water, cliffs, geology) while remaining voxel-backed matter. Extracted surface presentation is a non-authoritative view regenerated from material data; physics, queries, and gameplay-facing mutation operate on voxel truth.
- **Mutable everywhere, deep Z first-class.** Any material cell can be destroyed, moved, or placed. Underground is content space (caves, strata, ore, buried structure hooks), not a decorative floor under a heightmap.
- **Geology-first generation with sparse residency.** Worlds are produced as geology and related passes that materialize on demand; idle and homogeneous volume stays cheap so large regions remain practical.
- **GPU-resident world substrate.** Active material world state is organized for GPU residency as a defining product property, not a later port.
- **Consumer API boundary.** Mutation and inspection go through public verbs, queries, and events. Nothing above the matter surface touches voxels directly. Adjacent consumers have no privileged access.
- **Streaming, persistence, and honest interaction.** The product enables streaming around active anchors and persistence as generation function plus edit deltas; collision and interaction resolve against material occupancy, not against render mesh alone. Matter-level dynamics that make mutability physically honest (fluids, structural support, granular materials, and related state propagation) remain substrate outcomes; delivery depth and sequence are design decisions.

## Future products and enabling implications

Future consumers include an ARPG (with System/LLM as a game-layer client), a fortress/colony game, a descent-style adventure, and pure sandbox modes. Enabling implications only: the same substrate must remain reusable across those modes via clean layering; gas or labor pricing is a plug-in policy at the consumer API, not a baked game rule; the System authors and directs through the same registries and command surface as other clients. Gameplay, controllers, characters, authored demo routes, presentation, and game-specific policy stay consumer-owned.

## Non-goals

- Shipping a playable game, combat, stats, AI, or entity ecosystems beyond what a harness might need to exercise the substrate
- Implementing System/LLM, spells, gas metering, or building-game layers in this repository
- Treating the validation harness’s character, camera, seed-world postcard content, or benchmark gates as product identity
- Making the substrate depend on an LLM

## Confirmed vision constraints

- Identity is a **Rust** crate (or small crate family) for external consumption.
- The **actual game is not part of this repository**.
- The consumer boundary is mandatory: harness and games share the public surface; no privileged in-tree game paths.
- The substrate has **zero LLM dependency** and must stand alone as an engine layer.
- The world substrate is **GPU-resident** by product definition.

## Deferred design decisions

- Precise crate/workspace split and API shape of the public surface
- Delivery depth and order for generation, meshing, matter simulation, streaming, and persistence
- Voxel scale, LOD strategy, object-layer capacity, and fluid/integrity model fidelity
- Graphics stack, target environments, and quantitative performance budgets
- Whether and how a walkable-world harness is packaged relative to the crates (see Q1 for delivery obligation only)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required adjacent delivery** of this effort, or only a **permitted** validation artifact?

- **Proposed safe answer:** Permitted only—product success is the substrate crates and public boundary; a harness may exist to validate them but is not a mandatory ship item for product identity.
- **If different:** Requiring it keeps substrate identity but adds a mandatory adjacent deliverable (still not game scope). Forbidding it entirely would remove the in-repo validation consumer while leaving the substrate product unchanged.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/harness, not a game layer.
- **docs/seeds/project-boundary.md** — Fixes product identity as the substrate crate family, excludes the actual game and listed game layers from the repo, and mandates an equal public-interface consumer boundary for any harness.
- **docs/seeds/product-one-seed.md** — Describes a first walkable validation slice and demo claims that motivate substrate mutability, generation, meshing, streaming, and persistence; harness controls, content, platforms, and performance numbers stay adjacent.
- **docs/seeds/voxel-world-substrate.md** — Authorizes the substrate’s outcome families (natural material world, deep-Z mutability, geology-first sparse generation, GPU residency, matter physics, consumer layering) and positions future games as consumers above a standalone engine layer.
