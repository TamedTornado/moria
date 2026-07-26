# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates for external game consumers.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface and deep underground that are the same fully mutable voxel truth. The substrate provides matter, material physics, derived views, queries, and mutation. Game rules, content policy, and game-level presentation live above it. The stack must stand alone with no LLM dependency.

## Product boundary

**In product:** the reusable voxel-world substrate and its public consumer-facing surface (commands/verbs, queries against the coarse observation path, events, and material/world registries needed to use that surface). Substrate ownership includes derived terrain surface output, material-bound dressing, interactive voxel objects, and navigation data derived from matter.

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness. It is not the product and must use the same public interfaces an external game would use—no privileged or game-specific implementation paths. Whether that harness is a required delivery is open (Q1). If required, its first fused delivery slice is the Product One walkable-world proof described under Future products (still not substrate identity).

**Out of repository / out of product:** the actual game and game rules; System/LLM features; spells, gas policy, combat, AI; building as a game layer (UI, work orders, room economy, designation policy); character controllers and game-level presentation. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

- **Material truth under a natural look.** Consumers get a world that reads as ordinary terrain while remaining voxel-backed matter. Smooth extracted terrain surfaces are substrate-owned, regenerated, non-authoritative views. Noninteractive dressing is material-bound; anything that burns, breaks, or blocks is voxel-backed, including interactive voxel objects (placement and registration as world matter objects).
- **Mutable everywhere; deep Z first-class.** Any material cell can be destroyed, moved, or placed. Underground is content space (caves, strata, ore, buried-structure hooks), not a decorative floor under a heightmap.
- **Geology-first generation with sparse residency.** Worlds are produced as geology and related passes that materialize on demand; idle and homogeneous volume stays cheap so large regions remain practical.
- **GPU-resident brick world and CA simulation; public command/observation contract.** Brick-world state and cellular-automata simulation are GPU-resident. Consumers mutate and inspect only through public commands, coarse/stale mirror queries, and events—not live authoritative GPU cells, and never by touching voxels directly. Adjacent consumers have no privileged access. Exact API shape is deferred; residency and the stale/coarse observation constraint are not.
- **Environmental behavior and honest matter dynamics.** Thin-but-present time, weather, wetness, growth, and fire ecology keep the surface world behaving as a material place. Fluids, structural support, granular settle, and fire remain substrate matter outcomes; delivery depth and sequence are design decisions.
- **Mutation-safe 3D navigability, streaming, and persistence.** The substrate supplies navigation data derived from bricks that stays valid under mutation across continuous deep-Z space. Streaming serves active anchors. Persistence is generation function plus edit deltas for brick truth, plus journals for moved or changed substrate-owned objects and related entity state the substrate owns, with cross-run restoration and reuse of those persisted world changes. Collision and interaction resolve against material occupancy, not render mesh alone. Controllers, AI, and game policy stay downstream.

## Future products and enabling implications

Future consumers include an ARPG (with System/LLM as a game-layer client), a fortress/colony game, a descent-style adventure, and pure sandbox modes. Enabling implications only: the same substrate stays reusable across those modes via clean layering; gas or labor pricing is plug-in policy at the consumer API; the System authors and directs through the same registries and command surface as other clients. Gameplay, controllers, characters, authored demo routes, game-level presentation, and game-specific policy stay consumer-owned.

**Conditional adjacent delivery (Q1):** If the walkable-world executable is a required adjacent delivery, its first fused proof is a curated generated natural region; continuous third-person surface-to-deep-Z traversal; dig/place proof that visible terrain is mutable voxel truth; full generation with partial matter depth as that slice’s scope; and benchmark validation of the harness’s claims. That slice does not redefine substrate identity, and its controller, postcard content, platform gates, and presentation remain adjacent-consumer concerns.

## Non-goals

- Shipping a playable game, combat, stats, AI, or entity ecosystems beyond what a harness needs to exercise the substrate
- Implementing System/LLM, spells, gas metering, or building-game layers in this repository
- Treating the validation harness’s character, camera, seed-world postcard content, or benchmark gates as product identity
- Making the substrate depend on an LLM

## Confirmed vision constraints

- Identity is a **Rust** crate (or small crate family) for external consumption.
- The **actual game is not part of this repository**.
- The consumer boundary is mandatory: harness and games share the public surface; no privileged in-tree game paths.
- The substrate has **zero LLM dependency** and must stand alone as an engine layer.
- The brick world and CA simulation are **GPU-resident**; consumer observation is via a **stale/coarse mirror** plus events, with commands as the mutation path.

## Deferred design decisions

- Precise crate/workspace split and API shape of the public command/query/event surface
- Delivery depth and order for generation, meshing, matter simulation, ambient ecology, streaming, and persistence
- Voxel scale, LOD strategy, object-layer capacity, and fluid/integrity model fidelity
- Graphics stack, target environments, and quantitative performance budgets
- Packaging of any walkable-world harness relative to the crates (delivery obligation only: Q1)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required adjacent delivery** of this effort, or only a **permitted** validation artifact?

- **Proposed safe answer:** Permitted only—product success is the substrate crates and public boundary; a harness may exist to validate them but is not a mandatory ship item for product identity. The Product One walkable proof remains the fused first slice *if* delivery is later required.
- **If different:** Requiring it keeps substrate identity but adds a mandatory adjacent deliverable whose first fused slice is the Product One walkable proof (still not game scope). Forbidding it entirely would remove the in-repo validation consumer while leaving the substrate product unchanged.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/harness, not a game layer.
- **docs/seeds/project-boundary.md** — Fixes product identity as the substrate crate family, excludes the actual game and listed game layers from the repo, and mandates an equal public-interface consumer boundary for any harness.
- **docs/seeds/product-one-seed.md** — Defines the fused first walkable validation slice (generated region, continuous deep-Z traversal, dig/place proof, generation/matter depth, benchmarks) as adjacent-consumer delivery under Q1; does not import controller, content, or platform gates into substrate identity.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families: natural material world, deep-Z mutability, geology-first sparse generation, GPU brick/CA residency with command-in and stale-mirror/events-out, derived surface views and material-bound dressing/objects, environmental behavior, mutation-safe navigation, persistence with object journals and cross-run reuse, and future games as consumers above a standalone engine layer.
