# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation: continuous three-dimensional matter consumed through public interfaces—not a game, ARPG shell, or fortress product.

## Purpose

Moria exists so games and validation tools can share one material world layer instead of reimplementing geology, mutability, matter-driven presentation, streaming, and matter services: fully material voxel truth that reads as a natural landscape—not a heightmap with props—while remaining diggable, placeable, movable as matter, and playable through deep underground volume.

## Product boundary

**In product:** the reusable substrate—world generation; sparse GPU-resident matter; smooth voxel-derived surface presentation, material transitions, and matter-coupled dressing; mutation, command, query, and event surfaces; streaming and persistence; collision against voxel truth; mutation-aware spatial and movement affordances; and matter services (structural, fluid, fire, granular, ambient world behavior, interactive object lifecycle).

**Adjacent, not identity:** a walkable-world executable may exist here as a validation harness. Whether it is a required repository delivery is open (see Q1). If present, it must use only public interfaces available to an external game—no privileged substrate paths. Character control, camera, authored demo content and routes, game-specific presentation polish, scripted workloads, device targets, and numeric performance gates are harness-owned. While Q1 is open, the harness is neither required nor optional elsewhere—only adjacent with unresolved delivery status.

**Out of repository / downstream:** the actual game and all game rules. Combat, AI, the System/LLM, spells, gas policy, and building/gameplay layers (player building UX, blueprint labor, mechanism-as-game systems, work orders, room economy) are not this product. Seams may be designed where needed; those layers are not implemented here. Future consumers own game-specific presentation, UX, controllers, and policy—not substrate matter-driven surface presentation.

## Required product outcomes

1. **Reusable crate boundary.** Games and tools integrate Moria as Rust crate(s) through public APIs only. Generation, streaming, meshing, editing, collision against voxel truth, persistence, and matter services must be exercisable end-to-end through that surface (harness or external game).

2. **Natural-looking continuous world from voxel truth.** Terrain presents as a coherent natural landscape via smooth voxel-derived surfaces, material transitions, and matter-coupled dressing. The render surface is a view of matter, not physics authority. Underground volume—strata, caves, ores, aquifers—is first-class continuous 3D content, not a heightmap floor.

3. **Everywhere-mutable matter and object lifecycle.** Any voxel can be destroyed, moved, or placed through substrate mutation verbs. Interactive voxel-backed objects break, fall, and re-voxelize where they land or shatter. After edits, remeshed surfaces still read as real material.

4. **Geology-first generation, streaming, and exact persistence.** Worlds generate as a pure function of coordinates and world seed so volume regenerates independently and sparsely. Active regions stream in. Truth is same-seed worldgen plus edit deltas, including moved or destroyed object and relevant entity state; reload restores edits and matter/object lifecycle state exactly.

5. **Binding matter and ambient families.** Materials carry interaction-relevant properties; vegetation and clutter stay matter-coupled (voxel-backed interactables versus surface dressing). Dynamic objects, fluids, fire, granular behavior, and structural integrity are required substrate families—not optional. A thin ambient family covers time, seasons, weather-driven material state, growth, and fire ecology. Fidelity, mechanism, and sequence within each family are design; existence is not. First-slice limits that omit weather, seasons, growth, or felling do not remove these families from substrate identity.

6. **Command boundary, lagged observation, and mutation-safe movement queries.** Consumers issue mutations through a GPU-resident command boundary and observe via lagged or coarse mirror queries plus events—not storage as synchronous truth. Mutation verbs and mirror queries exist from the first validation slice. The substrate derives walkability and related spatial/movement affordances from matter and invalidates them after edits; AI and character controllers remain consumer-owned.

## Future products and enabling implications

Future consumers (not current Moria): a System/LLM-driven ARPG, a fortress/colony game, a descent-style adventure, and pure sandbox modes. They own gameplay, UX, controllers, authored content, game-specific presentation, pricing policy, and game rules.

Enabling implications only: no LLM dependency; pricing and game policy plug in above matter; the same stack supports dig/build, deep descent, and surface adventure over one world. Cross-run delta reuse is a persistence implication, not a shipped mode.

**First validation slice (adjacent, conditional on Q1):** if the harness is required delivery, that obligation is a product-shaped proof—not substrate identity: one generated natural region traversed in third person; continuous surface-to-underground volume; debug dig/place as mutability proof; full generation with deliberately partial matter (static fluid bodies in; full flow, fire, integrity, growth, and object felling out of that slice only); and measured performance validation. Routes, content inventories, workloads, machine targets, and numeric gates remain harness design. Tree felling and richer matter depth stay substrate outcomes deferred only for that first slice.

## Non-goals

- Implementing game rules, combat, AI, spells, gas economy, or System/LLM features
- Building/gameplay layers (building UX, blueprint labor, game mechanisms)
- Shipping a finished game as this product
- Committing harness demo content, controls, routes, platforms, or performance numbers as substrate requirements
- Treating first-slice demo limits as narrowing substrate product identity

## Confirmed vision constraints

- Product form: Rust crate or small family of tightly scoped Rust crates
- GPU-resident substrate with command-in / lagged-mirror-and-events-out observation
- Strict consumer boundary: external games and any in-repo harness share the same public interface; no privileged harness paths
- Standalone without LLM/System dependency; excluded game and building layers not implemented here
- Required matter and ambient families are binding; only fidelity, mechanism, and sequence within them are design

## Deferred design decisions

- Crate split and layout that enforce the consumer boundary
- Resolution, LOD, meshing approach, and storage encodings
- Fidelity, mechanisms, and sequence within binding matter and ambient families
- Streaming policy, persistence encoding, and command/mirror synchronization implementation
- How early multiplayer-oriented authority is pursued, if at all
- Harness demo content, controls, platforms, and performance targets if delivered

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation executable a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

*Proposed safe answer:* Permitted only—the product promise is the substrate crate(s). A harness may exist to exercise public APIs, but identity does not require that executable here.

*If answered differently:* Requiring the harness leaves substrate identity unchanged but adds a repository delivery obligation: an adjacent walkable-world executable validating the fused first-slice proof (generated natural region, third-person traversal, continuous surface-to-underground volume, debug dig/place, full generation with partial matter, measured performance) strictly through public interfaces—still excluding game layers and harness-owned content, routes, workloads, platforms, and numeric gates from substrate scope.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and distinguishes the walkable-world executable as a non-game validation consumer.
- **docs/seeds/project-boundary.md** — Locks identity to the substrate crate boundary, places the real game outside the repository, permits a public-API-only harness, and excludes game/System/building layers.
- **docs/seeds/product-one-seed.md** — Motivates the adjacent first proof (walkable natural region, dig/place, generation vs partial matter, measured validation) and first-slice exclusions without redefining identity.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look, mutability and object lifecycle, deep Z, geology, binding matter and ambient services, mutation-safe movement queries, command/mirror/events, exact persistence) and multi-game reuse without importing mechanism inventory or game layers.
