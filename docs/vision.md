# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world **substrate**, exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for material worlds—not a game, not a demo product, and not a gameplay stack.

## Purpose

Moria exists so multiple downstream experiences can share one stand-alone world foundation: a natural-looking surface world whose truth is fully mutable voxel matter, with deep underground as first-class space, and with clean mutation and query boundaries above which game rules live. The substrate must stand alone with **zero LLM dependency**.

## Product boundary

**This product owns** the reusable substrate crates: geology-first world generation sufficient for a diggable natural world; GPU-resident matter representation; smooth surface presentation derived from that matter (the mesh is a view, not authority); mutation and query interfaces; streaming and persistence of world truth as generation function plus edit deltas; collision and related physics against voxel truth; and matter-system concerns such as surface dressing and voxel-backed interactive objects—at outcome altitude, not as a fixed feature inventory.

**Adjacent, not identity.** A walkable-world executable **may exist** in this repository only as a separate consumer and validation harness. Whether that harness is a required repository delivery is **unresolved** (see Q1). If present, it must use the same public interfaces available to an external game—no privileged or game-specific substrate paths.

**Not this product.** The actual game is a separate downstream consumer and is **not** part of this repository. Game rules and the future System, LLM, spell, gas, combat, AI, and building **layers** are out of scope here. Compatibility seams may be designed where substrate requirements demand them; those layers must not be implemented in Moria.

**Not transferred from consumers.** Character control, cameras, authored demo routes and seed content, presentation polish, benchmark scenes, platform-specific harness gates, and game-specific policy remain consumer- or harness-owned unless a later approved boundary moves them.

## Required product outcomes

A downstream design must make these product-level outcomes true:

1. **Rust-consumable substrate.** Intended integration is as a Rust crate (or small crate family) that external and in-repo consumers use through public interfaces only.
2. **Voxel truth, normal look.** The world reads as continuous natural terrain (hills, forest, water, cliffs, caves) while remaining fully material: what you see is backed by mutable voxels, not a heightmap with non-matter props as the real world.
3. **Mutable everywhere.** Any voxel material can be destroyed, moved, or placed; dig and place are first-class substrate capabilities exercised through substrate interfaces—not direct voxel poking from above the matter boundary.
4. **Deep Z is first-class.** Underground volume (strata, caves, buried material truth) is content space, not a decorative floor under a surface shell.
5. **Matter, physics, queries, mutation.** The substrate provides the shared matter world, physics against that truth, queries, and mutation so games and harnesses do not each reimplement the material world.
6. **Generation, streaming, persistence.** Worlds are generated (geology-first, lazy where needed), streamable in active regions, and persistable as generation plus edit deltas so scars and progress are cheap and reloadable.
7. **Standalone and reusable.** No LLM or game-rules dependency inside the substrate; the same stack can underpin distinct future games without embedding their policies.

## Future products and enabling implications

Described **future consumers** (not current Moria scope) include a System/LLM-backed ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style descent experience, and pure sandbox play. They motivate a clean substrate-versus-game split and policy injection above the matter layer (for example gas or labor pricing as consumer policy, not substrate features).

Enabling implications only: shared mutable matter, deep-Z geology, mutation/query boundaries, and optional seams for higher layers. Gameplay, UX, controllers, authored content, presentation, combat, AI, spells, and building-game systems stay with those consumers.

## Non-goals

- Shipping the actual game, game rules, or game-layer systems in this repository
- Implementing System/LLM, spell, gas economy, combat, AI, or building layers here
- Treating any validation harness, demo character, or trailer route as the product itself
- Importing harness- or game-owned content, controls, presentation, or acceptance scenarios into substrate identity

## Confirmed vision constraints

- **Ecosystem:** Moria is a Rust library crate (or small family of crates) for integration by Rust consumers.
- **Residency:** The voxel world substrate is GPU-resident as a product property of Moria.
- **Consumer equality:** Any in-repo validation executable, if present, is a consumer of public substrate interfaces—not a privileged co-owner of substrate internals.
- **Boundary force:** The substrate-versus-game (and substrate-versus-harness) consumer boundary is required; excluded game layers must not be implemented here.
- **Independence:** The substrate must operate with zero LLM dependency.

## Deferred design decisions

- Internal crate split and packaging within the Rust family
- Voxel resolution, meshing strategy, LOD, streaming ring layout, and persistence encoding
- Depth and sequence of matter-physics capabilities (fluids, integrity, fire, granular settle, object felling, and similar) beyond the outcome mandates above
- Worldgen pipeline staging, material registries, and object-layer capacity strategy
- Harness design (if delivered): controllers, content, presentation, workloads, and performance gates
- Multiplayer readiness beyond keeping command/query style boundaries available for later design

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current repository delivery**, or only a **permitted** adjacent artifact?

- **Proposed answer:** Permitted only—the required product is the substrate crates; a harness may exist later without being part of product identity.
- **If different:** Making the harness mandatory adds a repository delivery obligation beside the crates, still without moving harness controls, content, or acceptance details into substrate scope.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world substrate (Rust crate) and frames the walkable-world executable as a separate consumer/validation harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **`docs/seeds/project-boundary.md`:** Locks current product identity to the substrate crate family, places the real game outside the repository, permits a public-interface-only validation executable, and excludes game-rule and future System/LLM/spell/gas/combat/AI/building layers.
- **`docs/seeds/product-one-seed.md`:** Describes a first walkable-world demo/harness slice (content, controller, milestones, platform gates) that motivates mutability proof and substrate enablement without transferring demo ownership into Moria’s product identity; delivery status of that harness is the open Q1 conflict with the boundary seed’s “may include.”
- **`docs/seeds/voxel-world-substrate.md`:** Supplies substrate design goals and reusable-engine purpose—normal look over voxel truth, full mutability, deep Z, GPU-resident matter/physics/queries/mutation, generation and persistence model, and multi-game layering with zero LLM dependency—while game-layer systems remain future consumers.
