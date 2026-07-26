# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates for external games. It is an engine-layer world product, not a game and not a repository that ships the eventual title’s rules or content.

## Purpose

Moria exists so multiple game styles can share one material world foundation: generated, fully mutable voxel matter with substrate-owned physical behavior; a natural-looking, matter-derived world view; first-class deep vertical space; mutation-safe navigation and continuous 3D traversal support; and public command, observation, and mutation surfaces—without embedding game rules, LLM systems, or a particular title’s policies.

## Product boundary

**In product:** the reusable substrate and public consumer interfaces; substrate ownership of matter, physics and material behavior, matter-derived surface presentation, mutation-safe navigation and continuous-3D traversal support, public observation and mutation semantics, and persistence of substrate-owned mutable truth; compatibility seams only where substrate requirements demand them—not implementations of future game layers.

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness. Whether delivering it is part of the current commitment is unresolved (see Q1). If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific paths. Harness character control, demo route, camera and demo composition, workloads, machine profiles, and performance gates belong to that artifact, not substrate identity.

**Out of this product / repository:** the actual game; game rules and System, LLM, spell, gas, combat, AI, and building layers (seams only where required). Consumer-owned camera, demo composition, authored content, gameplay UX, controllers, and game-specific presentation policy remain outside Moria; the natural-looking matter-derived world view does not.

## Required product outcomes

1. **Material world with deep Z and generation** — Fully material voxel matter anywhere in scope can be destroyed, moved, or placed; truth runs on matter, not decorative geometry. Underground is first-class (caves, strata, descent). Geology-oriented generation materializes regions on demand so large extents need not be fully resident when untouched.
2. **Substrate physics and material behavior** — Interactive voxel-backed objects with responsive dressing; active fluids and material reactions; ambient and fire behavior; granular settling; structural failure and cave-ins; falling or reconstituted matter. Depth and sequence are design; these outcome families are product mandates, not optional stretch.
3. **Matter-derived world view** — Moria produces natural-looking surface presentation from voxel truth (terrain meshing and matter-driven dressing). Mesh and dressing are views regenerated from matter, not a second world model. Camera, demo framing, and game-specific presentation remain consumer-owned.
4. **Mutable-world navigation** — Mutation-safe navigation data and continuous-3D traversal support across relevant movement classes. Agent AI and game behavior remain consumer-owned.
5. **Public observation and mutation** — Single mediated mutation path (commands in); observation via a stale/coarse mirror plus events out, with freshness and event observability part of the contract—not raw voxel access. Dig/place and mirror-style queries sit on that surface.
6. **Persistence and multi-game substrate** — Truth is generation plus edit deltas; substrate-owned mutations and dynamic world-object state restore across runs (journals for moved or changed world objects and state). Game-owned saves stay consumer-owned. Game rules, pricing, and LLM/System live above Moria; the substrate has zero LLM dependency.

## Future products and enabling implications

Future consumers—not current Moria products—include a System-driven ARPG, a DF-style fortress/colony game, a Moria-style descent experience, and pure sandbox titles. They motivate reuse, not gameplay ownership.

Enabling implications already in the substrate vision (depth and sequence are design): continuous 3D mutable worlds; matter physics games can specialize; mutation-safe navigation; public commands, mirror, and events; cross-run reuse of substrate-owned mutations. Gameplay, UX, controllers, authored content, camera, demo composition, and game-specific policy remain consumer-owned. A first walkable demo may thin some physics for its own slice; that does not remove those outcome families from Moria’s identity.

## Non-goals

- Shipping the actual game, its rules, content, combat, AI, or building systems here.
- Implementing System/LLM features, spells, gas/pricing policy, or game-layer intent in the substrate.
- Treating a first walkable demo’s content, controller, camera, route, or benchmark gates as product identity.
- Expanding this brief into architecture, algorithms, asset catalogs, or acceptance thresholds.

## Confirmed vision constraints

- **Rust crate consumer surface** — A Rust crate or small family of tightly scoped Rust crates.
- **GPU-resident substrate** — Matter-heavy path intended to run GPU-resident.
- **Commands in; mirror and events out** — Mediated mutation; stale/coarse mirror plus events for observation.
- **Equal public access** — Any validation executable and external games share the same public interfaces.
- **No LLM in the substrate** — Standalone engine layer with zero LLM dependency.
- **Game layers excluded** — System, LLM, spell, gas, combat, AI, and building layers are not implemented here.

## Deferred design decisions

- Precise crate split, internal layering, and API shape within the public-consumer boundary.
- Delivery depth and sequence for each physics and material outcome family—not whether those families are product responsibilities.
- Representation/meshing detail, generation pipeline detail, streaming rings, and persistence encoding.
- Hardware/OS profiles, performance budgets, and validation workloads—including harness delivery after Q1.
- Voxel resolution and fidelity/cost tradeoffs.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** alongside the substrate crates, or only a **permitted adjacent validation artifact**?

- **Proposed safe answer:** Permitted adjacent validation artifact only. Current product commitment is the reusable substrate crates; a harness may exist later or in parallel but is not required to define “done” for Moria’s product identity.
- **If answered differently:** Requiring the harness adds a current delivery of a public-API walkable executable without moving controller, demo content, camera, demo composition, or performance gates into substrate identity; those remain harness-owned for design.

## Seed synthesis

- **`README.md`** — Names Moria as the reusable GPU-resident voxel-world Rust crate and frames the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binding boundary: substrate crates are the product; the real game is out of repo; harness if present is public-API-only; game/System/LLM/spell/gas/combat/AI/building layers are out of scope.
- **`docs/seeds/product-one-seed.md`** — Adjacent first-slice demo/harness intent; confirms dig/place, mirror queries, meshing/dressing, and generation in early validation without narrowing substrate physics, presentation, navigation, or persistence mandates to that slice alone.
- **`docs/seeds/voxel-world-substrate.md`** — Substrate purpose: natural-looking material worlds, physics outcome families, deep Z, matter-derived presentation, mutation-safe navigation, commands/mirror/events, substrate-owned persistence, multi-game reuse without LLM dependency; mechanisms deferred to design.
