# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates for external game consumers. A required adjacent delivery is a walkable-world validation executable that consumes those crates through the same public interfaces an external game would use.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface and deep underground that are the same fully mutable voxel truth. The substrate provides matter, material physics, derived views, queries, and mutation. Game rules, content policy, and game-level presentation live above it. The stack must stand alone with no LLM dependency. The first adjacent validation delivery proves that foundation as a walkable generated world with dig/place mutability, continuous deep-Z traversal, and benchmarked interactive performance—not as a game layer.

## Product boundary

**In product:** the reusable voxel-world substrate and its public consumer-facing surface (commands/verbs, queries against the coarse observation path, events, and material/world registries needed to use that surface). Substrate ownership includes derived terrain surface output, material-bound dressing, interactive voxel objects, and navigation data derived from matter.

**Required adjacent delivery (not product identity):** a walkable-world executable that validates the substrate. It is not Moria’s product identity and must use the same public interfaces available to an external game—no privileged or game-specific implementation paths. Its first fused proof slice is defined under Required product outcomes. Controller, camera, postcard route content, presentation, platform gates, and schedule remain harness concerns, not substrate identity.

**Out of repository / out of product:** the actual game and game rules; System/LLM features; spells, gas policy, combat, AI; building as a game layer (UI, work orders, room economy, designation policy); character controllers and game-level presentation as product scope. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

- **Material truth under a natural look.** Consumers get a world that reads as ordinary terrain while remaining voxel-backed matter. Smooth extracted terrain surfaces are substrate-owned, regenerated, non-authoritative views. Noninteractive dressing is material-bound; anything that burns, breaks, or blocks is voxel-backed, including interactive voxel objects (placement and registration as world matter objects).
- **Mutable everywhere; deep Z first-class.** Any material cell can be destroyed, moved, or placed. Underground is content space (caves, strata, ore, buried-structure hooks), not a decorative floor under a heightmap.
- **Geology-first generation with sparse residency.** Worlds are produced as geology and related passes that materialize on demand; idle and homogeneous volume stays cheap so large regions remain practical.
- **GPU-resident brick world and CA simulation; public command/observation contract.** Brick-world state and cellular-automata simulation are GPU-resident. Consumers mutate and inspect only through public commands, coarse/stale mirror queries, and events—not live authoritative GPU cells, and never by touching voxels directly. Adjacent consumers have no privileged access. Exact API shape is deferred; residency and the stale/coarse observation constraint are not.
- **Matter dynamics, navigability, streaming, and persistence.** Thin-but-present time, weather, wetness, growth, and fire ecology keep the surface behaving as a material place. Fluids, structural support, granular settle, and fire remain substrate matter outcomes; depth and sequence are design decisions subject to the first-slice boundary below. The substrate supplies mutation-safe navigation data across continuous deep-Z space, streams for active anchors, and persists generation plus edit deltas (with journals for substrate-owned object/entity changes) for cross-run restoration. Collision and interaction resolve against material occupancy, not render mesh alone. Controllers, AI, and game policy stay downstream.
- **First adjacent validation slice (required delivery, not identity).** A generated—not authored—curated natural region traversable continuously from surface into deep Z with smooth terrain. Dig/place proves mutability with collision against voxel truth and responsive incremental remeshing. Reusable generation ships complete; sparse residency, material-bound dressing, registered voxel objects, static water bodies, streaming, seed-plus-delta restoration, and benchmarked interactive performance are in this slice. Active CA/fire, weather/seasons/growth, dynamic fluids, structural integrity, granular simulation, and rigid-body/felling are out of this slice (format may anticipate them; nothing runs them here except later stretch outside the required proof).

## Future products and enabling implications

Future consumers include an ARPG (with System/LLM as a game-layer client), a fortress/colony game, a descent-style adventure, and pure sandbox modes. Enabling implications only: the same substrate stays reusable across those modes via clean layering; gas or labor pricing is plug-in policy at the consumer API; the System authors and directs through the same registries and command surface as other clients. Gameplay, controllers, characters, authored demo routes, game-level presentation, and game-specific policy stay consumer-owned. Later mode choices start from a proven walkable world rather than a whiteboard; they are not current-product identity.

## Non-goals

- Shipping a playable game, combat, stats, AI, or entity ecosystems beyond what the validation harness needs to exercise the substrate
- Implementing System/LLM, spells, gas metering, or building-game layers in this repository
- Treating the harness’s character, camera, postcard content, platform gates, or benchmark workloads as substrate identity
- Making the substrate depend on an LLM
- Delivering active CA/fire, weather/seasons/growth, dynamic fluids, integrity, granular settle, or rigid/felling as part of the first required validation slice

## Confirmed vision constraints

- Identity is a **Rust** crate (or small crate family) for external consumption.
- The **actual game is not part of this repository**.
- The **walkable-world executable is a required adjacent delivery** for first product proof; it is not product identity.
- The consumer boundary is mandatory: harness and games share the public surface; no privileged in-tree game paths.
- The substrate has **zero LLM dependency** and must stand alone as an engine layer.
- The brick world and CA simulation are **GPU-resident**; consumer observation is via a **stale/coarse mirror** plus events, with commands as the mutation path.

## Deferred design decisions

- Precise crate/workspace split and API shape of the public command/query/event surface
- Delivery depth and order beyond the first validation slice’s included/excluded outcome boundary
- Voxel scale, LOD strategy, object-layer capacity, and later fluid/integrity model fidelity
- Graphics stack, target environments, quantitative performance budgets, and harness packaging relative to the crates
- Exact postcard composition, controller/camera presentation, and benchmark workload design (harness/design, not identity)

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/harness, not a game layer.
- **docs/seeds/project-boundary.md** — Fixes product identity as the substrate crate family, excludes the actual game and listed game layers, permits an in-repo walkable harness only under an equal public-interface consumer boundary, and does not erase Product One’s first-delivery mandate.
- **docs/seeds/product-one-seed.md** — Mandates the first adjacent validation delivery and its outcome boundary: generated curated region, continuous surface-to-deep-Z traversal, dig/place with voxel-truth collision and incremental remesh, complete generation, sparse residency, dressing, registered objects, static water, streaming, seed-plus-delta restore, and benchmarked interactive performance; excludes active CA/fire, weather/seasons/growth, dynamic fluids, integrity, granular, and rigid/felling from that slice.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families: natural material world, deep-Z mutability, geology-first sparse generation, GPU brick/CA residency with command-in and stale-mirror/events-out, derived surface views and material-bound dressing/objects, environmental behavior, mutation-safe navigation, persistence with object journals and cross-run reuse, and future games as consumers above a standalone engine layer.
