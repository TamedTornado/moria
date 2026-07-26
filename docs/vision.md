# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a **Rust crate** (or a small family of tightly scoped Rust crates). Downstream games and tools consume it as an engine layer for matter, world generation, queries, and mutation—not as a finished game.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface over authoritative voxel truth, fully mutable underground depth, and clean public interfaces—without embedding game rules, LLM systems, or a particular title’s presentation. It must stand alone as an engine substrate with zero LLM dependency.

## Product boundary

**In product:** the reusable substrate and its public consumer-facing interfaces (generation, matter representation, meshing/views derived from matter, mutation and query APIs, streaming and persistence of world truth, and compatibility seams the substrate itself needs).

**Adjacent, not product identity:** a walkable-world executable may exist in this repository as a **validation harness** and consumer of the substrate. It is not a game layer. Whether shipping that harness is a required repository delivery remains open (see Q1). If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific implementation paths.

**Out of this product and repository:** the actual game(s); game rules; and the System, LLM, spell, gas, combat, AI, and building **layers**. Those are downstream consumers or future products. Gameplay, UX, character controllers, cameras, authored demo content, and presentation belong to consumers or harnesses, not to Moria’s product identity.

## Required product outcomes

Downstream design must make these true for the substrate:

1. **Reusable engine boundary.** Consumers integrate through public APIs only. Nothing above the matter layer reaches voxels directly; mutation and inspection go through verbs and queries. Gas pricing and similar policies are injectable game concerns, not hard-wired substrate policy. The System/LLM is never a substrate dependency or feature.

2. **Voxel truth, normal look.** The world reads as continuous natural terrain (hills, forests, rivers, cliffs, meadows), not a block aesthetic as the primary surface look. Rendered geometry is a view regenerated from matter; physics, queries, and gameplay-facing truth run against voxels, not the mesh.

3. **Mutable material world, deep Z first-class.** Any region of matter can be destroyed, altered, or placed. Underground depth is content (strata, caves, ore, aquifers, and similar geological structure), not a false floor under a heightmap skin.

4. **Geology-first generation with sparse residency.** Worlds are generated as geology and related structure so digging reveals honest material, not painted rock under a height surface. Large regions stay practical via lazy materialization and sparse residency of uninteresting volume (including homogeneous empty or solid regions).

5. **Surface life as matter or matter-coupled dressing.** Interactable vegetation and clutter that can burn, break, or block is voxel-backed (including object-style trees and similar). Lightweight surface dressing (for example grass) stays driven by underlying voxel state so it does not desync from the material world.

6. **Engine-level matter dynamics and world continuity.** The substrate provides the reusable world capabilities games need: material mutation; fluid bodies and coarser fluid behavior as substrate responsibility; structural support and failure; granular settle where materials require it; ambient world behavior thin enough for a living surface; derived navigation affordances from matter; and persistence as generation parameters plus edit deltas, with streaming so large worlds need not be fully resident. Delivery depth and sequence are design choices; these outcome families are not demoted to optional future products.

## Future products and enabling implications

Future **consumers** (not current Moria scope) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style descent experience, pure sandbox tools, and any validation harness that exercises the crate.

Enabling implications only: those consumers need a shared mutable voxel world, deep underground play, geology and materials they can author against, matter-coupled presentation, and public mutation/query boundaries so game rules, labor, spells, and content stay above the substrate. Their gameplay, controllers, content, and policies are not imported into Moria.

## Non-goals

- Implementing the actual game or any title-specific loop in this product
- System/LLM, spells, gas metering policy, combat, AI agents, or building/gameplay layers in this repository
- Treating harness or demo presentation, character control, cameras, or authored seed-world content as substrate product scope
- Making the primary product a finished walkable game rather than a reusable crate substrate

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates intended for consumption by external games and tools.
- Product is GPU-resident voxel-world substrate; it is not the game layer.
- Adjacent consumers (including any validation harness) get no privileged access paths; the consumer boundary is mandatory even when package layout is left to design.
- Substrate must stand alone with zero LLM dependency; LLM/System attaches only as a game-layer client of the same public surfaces.
- Game rules and the excluded layers above are not implemented here; compatibility seams only where substrate requirements demand them.

## Deferred design decisions

- Exact crate split, internal layering, algorithms, data layouts, and API surface shape
- First delivery depth and sequence (which matter, fluid, integrity, vegetation, and sim capabilities ship in which slice)
- Voxel resolution, LOD, object-layer scaling, fluid-model fidelity, and related open engineering choices
- Platforms, backends, performance budgets, and acceptance workloads (including any harness benchmarks)
- Whether multiplayer readiness is in scope statements for early design

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world **validation harness** executable a **required current delivery** of this repository, or only **permitted** as an adjacent artifact?

- **Proposed safe answer:** Permitted only. Product identity stays the substrate crates; a harness may exist and, if present, must consume public interfaces, but it is not a required ship item of the product vision.
- **If answered differently:** Making the harness required adds a repository delivery obligation (an executable consumer that validates terrain, streaming, meshing, editing, collision, persistence, and performance through public APIs) without moving game content, controllers, or presentation into substrate identity. Treating it as forbidden would remove that adjacent artifact entirely.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and frames the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Locks product identity to the substrate crate(s), excludes the actual game and listed game layers from the repository, and requires any harness to use public interfaces without privileged paths.
- **docs/seeds/product-one-seed.md** — Describes a first walkable demo consumer and validation slice (region, controller, proof edit, performance narrative); motivates substrate capabilities and dig/place proof without transferring demo ownership into product identity.
- **docs/seeds/voxel-world-substrate.md** — Authorizes the substrate’s outcome families (normal look over voxel truth, full mutability, deep Z, geology generation, matter dynamics, persistence/streaming, reusable layering) at engine altitude for downstream design.
