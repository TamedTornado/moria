# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation—not a game—consumed by external games and by any in-repo validation artifact through the same public interfaces.

## Purpose

Games need a natural-looking surface world that is fully material underneath: continuous terrain and deep underground space, mutable anywhere, with generation, streaming, meshing, editing, collision against voxel truth, and persistence that stay reusable across titles. Moria exists so those outcomes live in one substrate with zero dependency on any particular game rules, LLM/System layer, or presentation stack. Future games (System ARPG, fortress/colony, descent, sandbox) sit above it; they do not redefine it.

## Product boundary

**In product (Moria substrate)**

- Public world substrate APIs and responsibilities: geological generation, lazy materialization and streaming, smooth visual meshing driven by voxel truth, matter mutation (including dig/place), collision-relevant voxel occupancy, matter-side physics and simulation hooks the substrate owns, queries/events, and edit-delta persistence.
- Clean layering so game policy (pricing, rules, content, UX) stays above the substrate; consumers have no privileged or game-specific paths into implementation.

**Out of product (adjacent or downstream)**

- The actual game and its repository; game rules; System/LLM, spell, gas, combat, AI, and building layers (compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here).
- A walkable-world executable may exist in this repository as an adjacent validation harness only. It is not the product identity. Whether it is a required near-term delivery is open (**Q1**). While open, this brief does not treat that executable as optional, required, planned, or part of current product delivery—only as a permitted adjacent artifact form. If present, it must use the same public interfaces as an external game.
- Harness- or demo-owned concerns stay outside product scope: character controller, camera, authored seed route, debug presentation, scripted workloads, machine-specific gates, and acceptance clips.

## Required product outcomes

1. **Normal look, voxel truth** — Surface worlds read as ordinary terrain (hills, forests, water, cliffs, caves), not cube aesthetics; the mesh is a regenerated view; gameplay-relevant queries and collision run on material occupancy, not on the mesh as authority.
2. **Mutable continuous 3D matter** — Any voxel can be destroyed, placed, or altered through public mutation verbs; deep underground is first-class content space, not a false floor.
3. **Geology-first generation with thrifty residency** — Worlds are generated as geology (strata, caves, ores, fluid bodies as material truth) so digging is honest; large regions stay tractable via lazy materialization, sparsity, and streaming around activity.
4. **GPU-resident substrate with a closed mutation boundary** — Matter lives in a GPU-resident design; nothing above the matter layer touches voxels directly—only public verbs, mirror queries, and events—so the same stack remains sandboxed and reusable.
5. **Persistence and interactive credibility** — Truth is worldgen plus edit deltas; consumers can reload scars and stream regions for interactive walkable use. Depth of simulation (e.g. multi-tier fluids, integrity, vegetation object lifecycle) is design-sequenced; the product responsibility for matter, physics, queries, and mutation remains with the substrate.
6. **Consumer-agnostic reuse** — The same crate stack can underwrite multiple game modes without embedding their rules; gas/pricing and similar policies are injectible above the substrate, not hard-wired into it.

## Future products and enabling implications

Described future consumers include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate substrate outcomes (deep Z, honest dig, material physics, public verbs/queries) but contribute none of their gameplay, content, controllers, characters, presentation, or acceptance scenarios to current scope. Enabling implications: expose seams for materials, stamps/POI metadata, and policy plugs so those games can author content without forking world truth.

## Non-goals

- Implementing any full game, game mode, or game rules in this product.
- System/LLM authorship, spells, gas economy, combat, AI agents, or building/UX layers (blueprints-as-gameplay, mechanisms-as-gameplay, room economy).
- Treating the walkable demo’s first slice (specific region, route, dressing set, controller, or milestone catalog) as the product identity or as a hard ceiling on substrate purpose.
- Shipping consumer presentation, controllers, or platform/performance gates as if they were substrate promises.

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates for intended consumers and integration.
- Any in-repo walkable-world executable, if present, consumes only public substrate interfaces—no privileged harness paths.
- Substrate stands alone with zero LLM/System dependency.
- Game rules and the System, LLM, spell, gas, combat, AI, and building layers are not implemented in this product; seams only where substrate needs demand them.

## Deferred design decisions

- Capability depth and delivery sequence for generation, matter simulation, meshing, objects/dressing, and API surface (including what a first vertical slice proves).
- Representation and algorithm choices (voxel scale, brick/layout details, meshing approach, LOD, object scaling, fluid/integrity fidelity).
- Exact crate split, persistence encoding, streaming topology, and benchmark methodology.
- Whether multiplayer-ready command authority is in scope statements or later work.
- Concrete validation scenarios, platforms, and performance thresholds for any harness.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required near-term repository delivery** (still outside product identity, public-API-only), or only a **permitted** adjacent validation artifact that need not ship with the current effort?

- **Proposed answer:** Required adjacent validation deliverable for proving generation, streaming, meshing, editing, collision, persistence, and interactive performance through public interfaces—not part of product identity and owning none of the substrate’s APIs.
- **If different:** “Permitted only” allows a crate-only current effort with no walkable executable; “required” commits the repository to deliver that adjacent harness without expanding Moria’s identity into a game or demo product.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/validation, not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the substrate crate(s), excludes the game and named game layers, and requires any harness to use public interfaces only.
- **docs/seeds/product-one-seed.md** — Motivates an undeniable walkable proof of material world + look, and a first demo/harness slice; its controllers, content, milestones, platforms, and numeric gates stay adjacent/design concerns, not product identity.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate purpose and responsibilities (normal look over voxel truth, mutability, deep Z, generation, matter/physics, closed verb boundary, persistence/streaming, multi-game reuse) at outcome altitude without transferring game-layer features into current scope.
