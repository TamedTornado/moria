# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer foundation for natural, fully material worlds—not a game, not a gameplay demo, and not the future ARPG or fortress products that will sit above it.

## Purpose

Moria exists so multiple games can share one world stack: a surface that reads as ordinary terrain (hills, forest, water, cliffs) whose truth is continuous mutable matter, including deep underground volume. Downstream titles—System ARPG, fortress/colony, descent-style adventure, or pure sandbox—should compose on this substrate rather than each reimplementing geology, matter, mutation, and world services. The substrate must stand alone with no LLM or System dependency.

## Product boundary

**In product:** the reusable voxel-world substrate and its public consumer interfaces. Adjacent games live outside this repository and consume those interfaces like any external client.

**Adjacent, not identity:** a walkable-world executable may exist in-repo as a validation harness only. It must not own privileged or game-specific substrate paths; it uses the same public interfaces an external game would. Whether shipping that harness is a current delivery obligation is unresolved—see Q1. Until answered, treat it only as a permitted adjacent artifact, not as part of product identity or as a settled required deliverable.

**Out of product:** the actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and building/gameplay layers. Compatibility seams may be left where substrate outcomes require them; those consumer layers are not implemented here.

**Consumer-owned (even when a harness or future game needs them):** character controllers, cameras, authored demo routes, presentation polish, gameplay UX, content packs, and game-specific policy. A harness may motivate that the substrate expose generation, streaming, meshing, editing, collision-relevant occupancy, persistence, and performance-relevant world services through public APIs; it does not pull harness controls, characters, scenes, platforms, or acceptance gates into substrate scope.

## Required product outcomes

Design must make these consumer-visible guarantees true of the substrate (delivery depth and sequence are design’s problem; these are the product’s outcome families):

1. **Natural world, voxel truth.** Generated surface worlds that read as normal terrain and dressing, while the voxel volume—not the mesh—is the authoritative material world. Rendered geometry is a view regenerated from matter, never the source of truth.

2. **Mutable volume, full depth.** Matter can be destroyed, placed, and reshaped throughout the playable volume, including deep underground. Dig and place are first-class substrate capabilities, not decorative surface tricks.

3. **Geology-first generation.** Worlds are produced as diggable geology (columns, strata, caves, ores, water bodies, lazy materialization of volume), not as a heightmap with rock painted underneath. Deep Z is content: continuous vertical play through surface and subterranean space.

4. **Matter and world services for games.** The substrate owns reusable world capability for material representation and mutation, surface dressing and voxel-backed objects, static and flowing fluid behavior at the tiers the stack supports, structural support and failure, ambient world behavior at aggregate scale where required, derived navigation-friendly occupancy, persistence of generation-plus-edits, and streaming of large regions—exposed so games can query and command the world without touching internal storage.

5. **Clean integration boundary.** All consumer access goes through public verbs, queries, and events. Nothing above the matter surface reaches voxels by privileged path. Gas/pricing and game rules are policy injected above the substrate, not baked into it. Multiple game types can share the same crate stack.

6. **GPU-resident engine layer.** Core world residency and heavy world work are designed for GPU-resident operation as part of the product’s identity as an engine substrate for Rust consumers.

## Future products and enabling implications

Future **consumers** (not this product): a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent adventure, and sandbox modes. They own gameplay, content, presentation, and policy.

Enabling implications already owned at substrate altitude: a material world deep enough for mines and caves; mutability and placement suitable for later building games; seams for game-authored materials, stamps/prefabs, and agents that only speak public commands; persistence and multi-anchor streaming so abandoned sites and large maps can return as data, not special cases. Long-horizon modules (rich fluid engineering, full fire ecology, multiplayer authority) remain design/roadmap choices unless later vision feedback promotes them—they are not a committed delivery catalog here.

A first walkable “product one” region and character demo is a **consumer/harness slice** that can prove the substrate; its milestone depth does not shrink Moria’s identity to that slice.

## Non-goals

- Shipping an actual game or game rules in this repository.
- Implementing System/LLM, spells, gas economy, combat, AI agents, or building/gameplay layers.
- Treating the walkable demo’s controller, camera, curated postcard route, material list, or benchmark scene as substrate requirements.
- Making the mesh, props, or heightmap the authoritative world.
- Coupling the substrate to an LLM or to one game’s policy model.

## Confirmed vision constraints

- Product form: Rust crate or small family of tightly scoped Rust crates.
- GPU-resident voxel-world substrate for external game consumption.
- Zero LLM/System dependency inside the substrate; it stands alone as an engine layer.
- Strict public-interface boundary: validation harness and external games share the same access path; no privileged in-repo game path.
- Game and listed future gameplay layers are out of repository scope; seams only where substrate needs allow.

## Deferred design decisions

- Crate split, APIs, algorithms, storage layouts, and meshing/LOD strategy.
- Capability depth and sequence within each outcome family (what ships in the first vertical slice vs later).
- Whether and how a walkable-world harness is structured, what it shows, and on what machines it is validated—if it is in scope at all (Q1).
- Voxel resolution, region sizing, persistence encoding, streaming policy, and performance budgets.
- How far ambient sim, fluids, integrity, objects, and building-related substrate verbs go in early deliveries.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **mandatory current delivery** of this repository, or only a **permitted adjacent artifact**?

- **Proposed safe answer:** Permitted only—the product promise is the substrate crates and public interfaces; a harness may be added to exercise them but is not required for the product to be complete.
- **If answered differently:** Making the harness mandatory keeps substrate identity unchanged but adds a required adjacent deliverable (still not game content); design must then plan harness existence and public-API-only consumption without importing demo gameplay into the crate’s identity.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate for Rust game consumption and positions the walkable-world executable as a separate validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the substrate crate(s), excludes the actual game and listed gameplay layers from the repo, permits a public-API-only validation harness, and makes the consumer boundary non-optional.
- **docs/seeds/product-one-seed.md** — Describes a first walkable consumer/demo slice (region, character, dig-proof, milestones) that motivates substrate proof points; its controls, content, platforms, and gates stay consumer-owned and do not redefine product identity or mandatory harness delivery.
- **docs/seeds/voxel-world-substrate.md** — Supplies the substrate’s purpose and outcome families (natural look over voxel truth, full mutability, deep Z, geology-first generation, matter/world services, clean layering, GPU-resident engine) for multiple future game consumers without importing those games into current scope.
