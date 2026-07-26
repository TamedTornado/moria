# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation: continuous three-dimensional matter that games and tools consume through public interfaces. It is not a game, not an ARPG shell, and not a fortress or descent product.

## Purpose

Moria exists so multiple games—and validation tools—can share one material world layer instead of reimplementing geology, mutability, presentation-from-matter, streaming, and matter services. The substrate makes one claim legible: the world is fully material voxel truth that can read as a natural surface landscape, not a heightmap dressed with props, while remaining diggable, placeable, and playable through deep underground volume.

## Product boundary

**In product:** the reusable substrate—world generation, sparse GPU-resident matter, smooth surface presentation driven by voxel truth, mutation and query surfaces, streaming and persistence of world state, and matter-level services that games build on (collision against voxel truth; structural, fluid, fire, and granular behavior as substrate responsibilities at product altitude).

**Adjacent, not identity:** a walkable-world executable may exist in this repository as a validation harness. If present, it must use only the same public interfaces available to an external game and must not own privileged or game-specific substrate paths. Whether that harness is a required repository delivery is open (see Q1). Character control, camera, authored demo content and routes, presentation polish, scripted workloads, device targets, and numeric performance gates are harness-owned, not substrate identity.

**Out of repository / downstream:** the actual game and all game rules. Combat, AI, the System/LLM, spells, gas policy, and building/gameplay layers (player building UX, blueprint labor, mechanism-as-game systems, work orders, room economy) are not this product. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

1. **Reusable crate boundary.** Downstream games and tools integrate Moria as Rust crate(s) through public APIs only; no consumer requires privileged access to internal matter storage.

2. **Natural-looking continuous world from voxel truth.** Generated terrain and materials present as a coherent natural landscape. The render surface is a view of matter, not the authority for physics or world queries.

3. **Everywhere-mutable material world.** Any region of the volumetric world can be destroyed, eroded, or filled through substrate mutation verbs. Dig and place are first-class capabilities; after edits, remeshed surfaces still read as real material.

4. **Deep continuous Z.** Underground space is first-class content volume—strata, caves, ores, aquifers—not a decorative floor under a heightmap. Queries and traversal address continuous 3D volume.

5. **Geology-first generation with sparse residency.** Worlds generate as geology that materializes on demand so large regions stay tractable: untouched volume stays cheap; scars and interesting shells occupy real resources. Generation is a reusable substrate asset, not a one-off demo map.

6. **Matter services for consumers.** Materials carry interaction-relevant properties; vegetation and clutter stay coupled to matter (interactable voxel-backed objects versus dressing derived from surface matter); static fluid bodies and broader fluid, fire, granular, and structural-integrity behavior remain substrate matter responsibilities (delivery depth is design).

7. **Query and mutation API.** Consumers observe world state and issue mutations through the substrate boundary rather than writing storage ad hoc—preserving reuse and a clean sandbox boundary.

8. **Streaming and persistence.** Active regions stream in; truth is regenerable worldgen plus edit deltas so change survives reload without saving an entire raw volume.

9. **Public-interface exercisability.** Generation, streaming, meshing, editing, collision against voxel truth, and persistence must be exercisable end-to-end through public interfaces (by a harness or external game). Harness-specific content and numeric gates are not product identity.

## Future products and enabling implications

Future consumers (not current Moria): a System/LLM-driven ARPG, a fortress/colony game, a descent-style adventure, and pure sandbox modes. They own gameplay, UX, controllers, authored content, presentation, pricing policy, and game-specific rules.

Enabling implications only: the substrate stands alone with no LLM dependency; pricing and game policy plug in above matter; the same stack should support dig/build fantasy, deep descent, and surface adventure over one world representation. Reuse of edit deltas across runs or modes is an implication of the persistence model, not a shipped game mode.

## Non-goals

- Implementing game rules, combat, AI, spells, gas economy, or System/LLM features
- Building/gameplay layers (building UX, blueprint labor systems, game mechanisms)
- Shipping a finished game as this product
- Committing harness demo content, controls, routes, platforms, or performance numbers as substrate requirements
- Treating first-slice demo limits as a narrowing of substrate product identity

## Confirmed vision constraints

- Product form: Rust crate or small family of tightly scoped Rust crates
- GPU-resident voxel-world substrate as product identity
- Strict consumer boundary: external games and any in-repo harness share the same public interface surface; privileged harness paths are excluded
- Standalone without LLM/System dependency
- Excluded game and building/gameplay layers are not implemented here; seams only where the substrate needs them

## Deferred design decisions

- Crate split and layout that enforce the consumer boundary
- Resolution, LOD, meshing approach, and storage encodings
- Depth and sequence of matter subsystems (multi-tier fluids, CA, integrity, granular settle, object–physics coupling)
- Streaming policy, persistence encoding, and command/mirror synchronization patterns
- How early multiplayer-oriented authority is pursued, if at all
- Harness demo content, controls, platforms, and performance targets—if a harness is delivered

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation executable a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

*Proposed safe answer:* Permitted only—the current product promise is the substrate crate(s). A harness may exist to exercise public APIs, but product identity does not depend on mandating that executable here.

*If answered differently:* Requiring the harness leaves substrate identity unchanged but adds a repository delivery obligation: an adjacent walkable-world executable that validates generation, streaming, meshing, editing, collision, persistence, and performance strictly through public interfaces—still excluding game layers and harness-owned content from substrate scope.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and distinguishes the walkable-world executable as a non-game validation consumer.
- **docs/seeds/project-boundary.md** — Locks product identity to the substrate crate boundary, places the real game outside the repository, permits a public-API-only harness, and excludes game/System/building layers.
- **docs/seeds/product-one-seed.md** — Motivates end-to-end proof of a material walkable world and a first validation slice; demo content, controls, platforms, and gates stay adjacent and do not redefine substrate identity.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look from voxel truth, full mutability, deep Z, geology generation, matter services, API boundary, streaming/persistence) and multi-game reuse without importing mechanism inventory or game layers into current scope.
