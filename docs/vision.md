# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates for external games. It is an engine-layer world product, not a game and not a repository that ships the eventual title’s rules or content.

## Purpose

Moria exists so multiple game styles can share one material world foundation: generated, fully mutable voxel matter with substrate-owned physical and environmental behavior; a natural-looking, matter-derived world view; first-class deep vertical space; mutation-safe navigation and continuous 3D traversal support; and public command, observation, and mutation surfaces—without embedding game rules, LLM systems, or a particular title’s policies.

## Product boundary

**In product:** the reusable substrate and public consumer interfaces; substrate ownership of matter, physics and material behavior, environmental lifecycle with material-world consequences, matter-derived surface presentation, mutation-safe navigation and continuous-3D traversal support, public observation and mutation semantics, and persistence of substrate-owned mutable truth; compatibility seams only where substrate requirements demand them—not implementations of future game layers.

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness and public-API consumer. Whether delivering it is part of the current commitment is unresolved (see Q1). If present, it remains outside substrate identity: harness controller, camera, authored route/content, workloads, machine profiles, numeric gates, and presentation choices belong to that artifact, not Moria. Conditional harness mandate if delivery is required: see Required product outcomes.

**Out of this product / repository:** the actual game; game rules and System, LLM, spell, gas, combat, AI, and building layers (seams only where required). Consumer-owned camera, demo composition, authored content, gameplay UX, controllers, and game-specific presentation policy remain outside Moria; the natural-looking matter-derived world view does not.

## Required product outcomes

1. **Material world with deep Z and generation** — Fully material voxel matter anywhere in scope can be destroyed, moved, or placed; truth runs on matter, not decorative geometry. Underground is first-class (caves, strata, descent). Geology-oriented generation materializes regions on demand so large extents need not be fully resident when untouched.
2. **Substrate physics and material behavior** — Interactive voxel-backed objects with responsive dressing; active fluids and material reactions; granular settling; structural failure and cave-ins; falling or reconstituted matter. Depth and sequence are design; these outcome families are product mandates.
3. **Environmental lifecycle with material consequences** — Growth behavior for voxel-backed living objects; thin-but-present day/night, seasons, weather, and fire ecology that produce material-world consequences (light, growth ticks, wetness/accumulation, ignition and fire propagation). Product One excludes weather, seasons, and growth only from that adjacent first slice—not the broader substrate. Depth and mechanisms are design; the consumer-visible environmental behavior is mandatory.
4. **Matter-derived world view** — Moria produces natural-looking surface presentation from voxel truth (terrain meshing and matter-driven dressing). Mesh and dressing are views regenerated from matter, not a second world model. Camera, demo framing, and game-specific presentation remain consumer-owned.
5. **Mutable-world navigation** — Mutation-safe navigation data and continuous-3D traversal support across relevant movement classes. Agent AI and game behavior remain consumer-owned.
6. **Public observation and mutation; persistence** — Single mediated mutation path (commands in); observation via a stale/coarse mirror plus events out, with freshness and event observability part of the contract—not raw voxel access. Dig/place and mirror-style queries sit on that surface. Truth is generation plus edit deltas; substrate-owned mutations and dynamic world-object state restore across runs. Game-owned saves stay consumer-owned. Game rules, pricing, and LLM/System live above Moria; the substrate has zero LLM dependency.

**Conditional (only if Q1 requires harness delivery):** An adjacent public-API walkable-world executable must prove generation, streaming, meshing, editing, collision, persistence, and performance. First-slice proof: a generated, traversable natural world whose voxel truth and mutability are shown through dig/place, on a deliberately partial substrate capability slice. That partial slice does not narrow substrate outcomes above. Controller, camera, authored route/content, workloads, platform profiles, numeric gates, and presentation remain harness-owned.

## Future products and enabling implications

Future consumers—not current Moria products—include a System-driven ARPG, a DF-style fortress/colony game, a Moria-style descent experience, and pure sandbox titles. They motivate reuse, not gameplay ownership.

Enabling implications already in the substrate vision (depth and sequence are design): continuous 3D mutable worlds; matter physics and environmental lifecycle games can specialize; mutation-safe navigation; public commands, mirror, and events; cross-run reuse of substrate-owned mutations. Gameplay, UX, controllers, authored content, camera, demo composition, and game-specific policy remain consumer-owned. A first walkable demo may thin some physics and environmental lifecycle for its own slice; that does not remove those outcome families from Moria’s identity.

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
- Delivery depth and sequence for each physics, material, and environmental outcome family—not whether those families are product responsibilities.
- Representation/meshing detail, generation pipeline detail, streaming rings, and persistence encoding.
- Hardware/OS profiles, performance budgets, and validation workloads—including harness delivery after Q1.
- Voxel resolution and fidelity/cost tradeoffs.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** alongside the substrate crates, or only a **permitted adjacent validation artifact**?

- **Proposed safe answer:** Permitted adjacent validation artifact only. Current product commitment is the reusable substrate crates; a harness may exist later or in parallel but is not required to define “done” for Moria’s product identity.
- **If answered differently:** Requiring the harness adds current delivery of the conditional validation outcome above. Controller, demo content, camera, route, workloads, platform profiles, numeric gates, and presentation stay harness-owned—not substrate identity.

## Seed synthesis

- **`README.md`** — Names Moria as the reusable GPU-resident voxel-world Rust crate; walkable-world executable is a separate harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **`docs/seeds/project-boundary.md`** — Binding boundary: substrate crates are the product; game out of repo; harness if present is public-API-only; game/System/LLM/spell/gas/combat/AI/building layers out of scope.
- **`docs/seeds/product-one-seed.md`** — Adjacent first-slice harness proof points; excludes weather/seasons/growth only from that slice; does not narrow broader substrate mandates.
- **`docs/seeds/voxel-world-substrate.md`** — Substrate purpose: material worlds; physics and environmental lifecycle (growth; thin day/night, seasons, weather, fire ecology); deep Z; matter-derived presentation; mutation-safe navigation; commands/mirror/events; substrate-owned persistence; multi-game reuse without LLM; mechanisms deferred.
