# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate for natural, fully material 3D worlds. It is delivered as a Rust crate or a small family of tightly scoped Rust crates. It is not a game, not a game layer, and not a content product. The repository’s first adjacent delivery is a walkable-world validation executable that consumes those crates through public interfaces, plus benchmarks and a public demo artifact—without those harness concerns becoming substrate identity.

## Purpose

Moria exists so multiple games can share one world foundation: a continuous voxel-true landscape that reads as ordinary outdoor and underground terrain, can be mutated anywhere, and exposes matter, generation, queries, mutation, and matter-derived services without embedding any particular game’s rules, economy, combat, or AI. Downstream products should start from a walkable material world rather than inventing their own geology, meshing, and edit model.

## Product boundary

**In product**
- The reusable substrate and its public consumer-facing interfaces (Rust crate or tightly scoped crate family).
- Substrate-owned world capabilities: geology-first generation and surface, sparse GPU-resident matter, smooth non-authoritative presentation of voxel truth, full mutability including deep Z, material behavior (objects, dressing, granular matter, fluids, ambient ecology, structural support, placement), matter-derived traversability, streaming and cross-run persistence of edits and substrate objects, and a commands-in / mirror-and-events-out consumer contract.
- Compatibility seams where substrate requirements demand them—without implementing game layers.

**Adjacent / not this product**
- The actual game(s) live outside this repository as separate consumers.
- A walkable-world executable is a required adjacent delivery: a public-API consumer and validation harness for generation, streaming, meshing, editing, collision, persistence, and performance. It is not product identity. Its controller, character, authored content, presentation, demo routes, benchmark workloads, machine targets, and numeric gates are harness-owned, not substrate scope.
- Game rules and future System, LLM, spell, gas, combat, AI, and building *layers* (blueprints-as-gameplay, mechanisms-as-game systems, labor/work orders, room designation UX) are out of scope here.

## Required product outcomes

1. **Reusable Rust substrate** — Consumers integrate Moria as crate(s) and obtain a shared world engine layer with no privileged internal paths reserved for in-repo demos.
2. **Voxel-true natural world, mutable through deep Z** — Fully material terrain (not heightmap-plus-props); presentation is a view of voxel truth. Any material can be destroyed, moved, or placed. Underground geology, voids, and depth are first-class. Worlds generate as geology and surface that materialize on demand so large continuous regions stay tractable under streaming.
3. **Material behavior and placement** — Natural objects that can burn, break, or block are voxel-backed and interactable; surface dressing stays consistent with digs, burns, and surface state. Granular materials settle and collapse honestly. Static fluid bodies and disturbed flow interact with materials. Thin-but-present weather, time, and fire ecology run at aggregate cost. Support and collapse are substrate responsibilities, with material-dependent span behavior available to consumers. Placing material (including stamped multi-cell shapes) is a peer of dig/mutate; building *gameplay* stays above the boundary.
4. **Matter-derived traversability and object-aware persistence** — Navigation and walkability derive from current voxel truth and remain valid under mutation, distinct from consumer AI. Generation plus edit deltas, and moved or changed substrate objects with their state, persist across streaming and runs.
5. **GPU-resident consumer contract, standalone engine** — Consumers issue commands and observe a mirror plus events; nothing above the matter layer touches voxels directly. Dig/place and queries alone do not replace event observability and mirror lifecycle. Core operation has zero LLM/System dependency; game policy lives above the substrate.
6. **First adjacent delivery** — A walkable-world validation executable, benchmarks, and public demo artifact that exercise the substrate through the same public interfaces an external game would use.

## Future products and enabling implications

Future consumers include a System-driven ARPG, fortress/colony play, descent/adventure modes, and pure sandboxes. They motivate a genre-agnostic, mutation-safe, deep-Z substrate. Enabling implications: public verb/query/event boundaries for later sandboxing or multiplayer-style authority; game-injected policy without forking matter; worlds whose scars and structures persist across modes as data. Gameplay, UX, controllers, authored content, presentation, and game-specific policy remain consumer-owned.

## Non-goals

- Implementing the actual game or shipping game rules, combat, stats, AI, spells, gas economies, or LLM/System behavior in this product.
- Owning building-layer gameplay (blueprints-as-gameplay, mechanisms-as-game systems, labor/work orders, room designation UX)—even when the substrate exposes matter verbs those layers would call.
- Treating the validation harness’s demo content, character, camera, routes, workloads, machine targets, or numeric gates as substrate product features.
- Making the substrate depend on or embed System/LLM world authorship for basic operation.

## Confirmed vision constraints

- Product identity is the reusable voxel-world substrate, not the game and not the harness.
- Exposure is a Rust crate or small family of tightly scoped Rust crates.
- The repository includes a walkable-world validation harness as a required adjacent delivery; it must use the same public interfaces available to an external game; adjacent consumers have no privileged access.
- The consumer/substrate boundary is required; exact package layout is not a vision decision.
- Game, System/LLM, spell, gas, combat, AI, and building layers must not be implemented here (seams only where substrate needs demand them); substrate stands alone without LLM dependency.
- Interaction contract: commands in, mirror and events out; voxels are not a direct consumer surface above matter.

## Deferred design decisions

- Crate split, APIs, algorithms, storage layouts, meshing strategy, voxel size, LOD, streaming rings, and persistence encoding.
- Delivery depth and sequencing of substrate outcome families across releases—without demoting those families from required product outcomes.
- Harness-owned details: seed region content, controller/camera, debug tools, benchmark workloads, platforms, and numeric performance gates.
- Multiplayer deployment and any non-substrate backend choices.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/harness, not game layer.
- **docs/seeds/project-boundary.md** — Locks identity to the substrate crate(s), places the real game outside the repo, requires public-API-only harness access when present, and excludes game/System/LLM/spell/gas/combat/AI/building layers.
- **docs/seeds/product-one-seed.md** — Establishes the first adjacent delivery (walkable demo, benchmarks, public artifact) and its harness-owned content/controls/acceptance; does not redefine substrate identity or import demo acceptance into substrate scope.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look over voxel truth, full mutability, deep-Z geology, generation/streaming, material behavior, traversability, object-aware persistence, commands/mirror/events contract, layered reuse without LLM dependency) at vision altitude without adopting its mechanism inventory as the product brief.
