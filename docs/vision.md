# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer material world for external consumers—not a game, not a character demo, and not game rules.

## Purpose

Give multiple future games and tools one shared foundation: a natural-looking surface world over continuous, fully mutable voxel matter that stays honest underground. Game rules, economy policy, AI, and presentation live above this product; the substrate supplies matter, generation, queries, mutation, and the seams those consumers need.

## Product boundary

**In product:** geological world generation; GPU-resident voxel matter and mutation; non-authoritative visual presentation of that matter; public verbs and queries so nothing above the matter layer touches voxels directly; streaming and persistence of world truth as generation plus edit deltas; reusable crate integration for external games and tools.

**Out of product:** the actual game (a separate downstream consumer, not part of this repository); game rules; System/LLM features; spells; gas policy; combat; AI; building layers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

**Adjacent artifact:** a walkable-world executable may exist in the repository as a validation harness. Whether it is a required delivery is open (Q1). If present, it must consume the substrate through the same public interfaces available to an external game and must not own privileged or game-specific implementation paths. Harness character control, camera, authored demo route, fixture content, presentation, workloads, and performance gates are harness concerns, not substrate identity.

## Required product outcomes

1. **Reusable Rust substrate.** Downstream games integrate a GPU-resident voxel world as crate consumers without embedding their rules into Moria.
2. **Material natural world.** Generated terrain reads as continuous natural landscape (surface and underground) while remaining fully material—walkable, diggable, and placeable matter, not a heightmap dressed with non-matter props.
3. **Deep Z as first-class space.** Underground volume is continuous content with the surface: strata, voids, and material truth at depth participate in the same world model.
4. **Voxel truth, view mesh.** Physics- and gameplay-facing queries and mutations operate on voxel matter; meshes and surface dressing are regenerated views and are never the saved or authoritative world state.
5. **Stream and scar cheaply.** Large regions idle until touched; persistence is worldgen function plus edit deltas for touched matter and related object state.
6. **Public mutation and query boundary.** Consumers (and any in-repo harness) inspect and change the world only through public substrate interfaces; policy such as pricing is injectable above the matter core, not hard-wired as one game’s economy.

## Future products and enabling implications

Described future consumers—not current Moria scope—include a System-driven ARPG, a Dwarf Fortress–style fortress or colony game, a descent-style adventure, and a pure sandbox. Moria enables them by owning the material world and the verb/query boundary; each consumer owns gameplay, UX, controllers, content, and policy.

High-level enabling implications already in the seeds: the same substrate can serve those modes without LLM dependency in the engine layer; an optional System attaches as a game-layer client on public mirrors, commands, and content registries; gas or labor pricing is a consumer-injected policy over shared verbs, not a substrate game mode.

## Non-goals

- Shipping the actual game, combat, AI, spells, gas economy, System/LLM runtime, or building layers in this repository
- Treating the walkable demo’s character, camera, seed route, or benchmark theater as product features
- Making the visual mesh authoritative for collision, queries, or saves
- Expanding current scope with consumer-owned presentation, controllers, or acceptance scenarios

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates for intended consumers in that ecosystem
- World matter is GPU-resident as part of product identity
- Substrate stands alone with zero LLM dependency
- Any in-repo harness uses only public interfaces; no privileged game-specific paths through the substrate
- Game and building layers are not implemented here; seams only where substrate requirements demand them

## Deferred design decisions

- Internal crate split and workspace layout that enforce the consumer boundary
- Depth and sequence of generation, meshing, matter simulation, object, streaming, and persistence capabilities
- Representation and resolution choices for voxels, storage, meshing, and related runtime structure
- Target environments, performance budgets, and validation workloads (including any harness content once Q1 is settled)
- Open substrate engineering questions left for measurement-driven design (for example voxel size and distant presentation strategy)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required repository delivery** or **only a permitted** adjacent validation artifact?

- **Proposed answer:** Permitted only—identity stays the reusable substrate; a harness may exist and, if it does, must use public interfaces only (per the project boundary’s “may include”).
- **If answered “required”:** the repository must also deliver a walkable-world harness that exercises the substrate through public APIs for generation, streaming, meshing, editing, collision, persistence, and performance—without absorbing that harness’s controls, content, or gates into substrate identity.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and frames the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the Rust crate substrate, places the real game outside the repo, constrains any harness to public interfaces, and excludes game/System/building layers from implementation here.
- **docs/seeds/product-one-seed.md** — Motivates first-slice proof of a material walkable world and dig/place honesty; its character, route, content, platforms, and performance theater stay harness/demo concerns and do not redefine the reusable product.
- **docs/seeds/voxel-world-substrate.md** — Supplies the substrate’s outcome-level purpose: natural look over mutable voxel truth, deep Z, generation-plus-deltas, GPU-resident matter, and multi-game reuse without embedding game rules.
