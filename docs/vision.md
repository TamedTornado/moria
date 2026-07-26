# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate for natural, fully material 3D worlds. It is delivered as a Rust crate or a small family of tightly scoped Rust crates. It is not a game, not a game layer, and not a content product. The repository’s first adjacent delivery is a walkable-world validation executable that consumes those crates through public interfaces, plus benchmarks and a public demo artifact—without those harness concerns becoming substrate identity.

## Purpose

Moria exists so multiple games can share one world foundation: a continuous voxel-true landscape that reads as ordinary outdoor and underground terrain, can be mutated anywhere, and exposes matter, generation, queries, mutation, and matter-derived services without embedding any particular game’s rules, economy, combat, or AI. Downstream products should start from a walkable material world rather than inventing their own geology, meshing, and edit model.

## Product boundary

**In product**
- The reusable substrate and its public consumer-facing interfaces (Rust crate or tightly scoped crate family).
- Substrate-owned capabilities: geology-first generation, sparse GPU-resident matter, smooth non-authoritative views of voxel truth, full mutability including deep Z, material behavior (voxel objects with dynamic fall/reintegration, dressing, granular matter, fluids, ambient ecology, structural support, placement), matter-derived traversability, streaming and cross-run persistence of generation-plus-edit truth plus objects, entities, and managed state, and a commands-in / stale-mirror-and-events-out contract.
- Compatibility seams where substrate requirements demand them—without implementing game layers.

**Adjacent / not this product**
- The actual game(s) live outside this repository as separate consumers.
- A walkable-world executable is a required adjacent delivery: a public-API consumer and validation harness, not product identity. Controller, character, content, presentation, routes, workloads, machine targets, and numeric gates are harness-owned. Its first settled slice is narrower than the full substrate (see Required product outcomes).
- Game rules and future System, LLM, spell, gas, combat, AI, and building *layers* (blueprints-as-gameplay, mechanisms-as-game systems, labor/work orders, room designation UX) are out of scope here.

## Required product outcomes

1. **Reusable Rust substrate** — Consumers integrate Moria as crate(s) and obtain a shared world engine layer with no privileged internal paths reserved for in-repo demos.
2. **Voxel-true natural world, mutable through deep Z** — Fully material terrain (not heightmap-plus-props); presentation is a view of voxel truth. Any material can be destroyed, moved, or placed. Underground geology, voids, and depth are first-class. Worlds generate as geology and surface that materialize on demand so large continuous regions stay tractable under streaming. Generation truth is deterministic from seed and pure functions; persisted edits are deltas on that truth.
3. **Material behavior, dynamic voxel objects, and placement** — Objects that can burn, break, or block are voxel-backed and interactable. Interactable objects convert to falling/dynamic bodies and reintegrate into matter where they land or break—this lifecycle is a full-product outcome, not optional scenery. Surface dressing stays consistent with digs, burns, and surface state. Granular materials settle honestly. Static fluid bodies and disturbed flow interact with materials. Thin weather, time, and fire ecology run at aggregate cost. Support and collapse are substrate responsibilities with material-dependent spans. Placing material (including stamps) is a peer of dig/mutate; building *gameplay* stays above the boundary.
4. **Matter-derived traversability and exact restoration** — Navigation and walkability derive from current voxel truth and remain valid under mutation, distinct from consumer AI. Persistence reconstructs the world exactly from deterministic generation plus edit deltas, with journals for substrate objects, entities, and their substrate-managed script state across streaming and runs.
5. **GPU-resident consumer contract, standalone engine** — Consumers issue commands and observe an asynchronously stale mirror plus events; nothing above the matter layer touches voxels directly. Dig/place and queries alone do not replace event observability or the stale-mirror contract. Core operation has zero LLM/System dependency; game policy lives above the substrate.
6. **First adjacent delivery (settled harness slice)** — A walkable-world validation executable, benchmarks, and public demo artifact through public interfaces. That first slice requires a curated generated natural region continuously traversable in third person that proves smooth voxel truth via dig/place; generation is complete while matter and API exposure are deliberately partial; benchmark evidence is part of the deliverable. Excluded from that first slice only (not from the full product): CA/fire, dynamic fluids, structural integrity, granular settling, and object felling/rigid conversion. Harness content, controls, presentation, and numeric gates stay outside substrate identity.

## Future products and enabling implications

Future consumers include a System-driven ARPG, fortress/colony play, descent/adventure modes, and pure sandboxes. They motivate a genre-agnostic, mutation-safe, deep-Z substrate. Enabling implications: public verb/query/event boundaries for later sandboxing or multiplayer-style authority; game-injected policy without forking matter; worlds whose scars persist across modes as data. Gameplay, UX, controllers, content, presentation, and game-specific policy remain consumer-owned.

## Non-goals

- Implementing the actual game or shipping game rules, combat, stats, AI, spells, gas economies, or LLM/System behavior in this product.
- Owning building-layer gameplay (blueprints-as-gameplay, mechanisms-as-game systems, labor/work orders, room designation UX)—even when the substrate exposes matter verbs those layers would call.
- Treating the harness’s content, character, camera, routes, workloads, machine targets, or numeric gates as substrate features.
- Making the substrate depend on or embed System/LLM world authorship for basic operation.
- Treating first-slice harness exclusions (CA/fire, dynamic fluids, integrity, granular settle, object felling) as full-product non-goals.

## Confirmed vision constraints

- Product identity is the reusable voxel-world substrate, not the game and not the harness.
- Exposure is a Rust crate or small family of tightly scoped Rust crates.
- The repository includes a walkable-world validation harness as a required adjacent delivery; it must use the same public interfaces available to an external game; adjacent consumers have no privileged access.
- The consumer/substrate boundary is required; exact package layout is not a vision decision.
- Game, System/LLM, spell, gas, combat, AI, and building layers must not be implemented here (seams only where substrate needs demand them); substrate stands alone without LLM dependency.
- Interaction contract: commands in, asynchronously stale mirror and events out; voxels are not a direct consumer surface above matter. Persistence reconstructs exactly from seed/function-derived truth plus deltas and object/entity/script-state journals.

## Deferred design decisions

- Crate split, APIs, algorithms, storage layouts, meshing strategy, voxel size, LOD, streaming rings, and persistence encoding.
- Depth and sequencing of substrate outcomes beyond the settled first harness slice (first-slice exclusions remain full-product outcomes for later design).
- Harness-owned details: seed-region inventory, controller/camera, debug tools, workloads, platforms, and numeric gates.
- Multiplayer deployment and any non-substrate backend choices.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/harness, not game layer.
- **docs/seeds/project-boundary.md** — Locks identity to the substrate crate(s), places the real game outside the repo, requires public-API-only harness access when present, and excludes game/System/LLM/spell/gas/combat/AI/building layers.
- **docs/seeds/product-one-seed.md** — Settles the first adjacent harness slice (curated natural region, third-person continuous traversal, dig/place proof, generation full with partial matter/API, benchmarks, exact restore for that slice) and its first-slice exclusions without redefining substrate identity.
- **docs/seeds/voxel-world-substrate.md** — Authorizes full-product substrate outcomes (natural look over voxel truth, mutability, deep-Z geology, dynamic voxel-object fall/reintegration, generation-plus-delta truth, object/entity/script journals, stale-mirror consumer contract, layered reuse without LLM dependency) at vision altitude without adopting its mechanism inventory as the product brief.
