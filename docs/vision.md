# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a **Rust crate** (or a small family of tightly scoped Rust crates). It is engine-layer infrastructure for external games: a fully material 3D world consumers integrate through public interfaces—not a game, demo, or content product.

## Purpose

Games that need a natural-looking overworld and deep underground play on the same mutable matter model lack a standalone foundation that is substrate, not rules. Moria exists so those games can generate, stream, view, edit, collide with, and persist a dig-honest voxel world without embedding combat, economy, AI, LLM, or building-game policy in the world layer. The substrate must stand alone with zero dependency on any System or LLM.

## Product boundary

**In product**

- Reusable voxel-world substrate crates and the public integration surface they expose to external consumers.
- Substrate-owned world capabilities: geology-oriented generation with lazy materialization; GPU-resident matter; smooth surface presentation derived from voxel truth; mutation and query through that public surface; collision against voxel truth; streaming and edit-delta persistence so large regions remain tractable.
- Optional design of compatibility seams where substrate requirements demand them—without implementing the layers that use those seams.

**Out of product (adjacent or downstream)**

- The actual game and all game rules.
- System / LLM features, spells, gas/pricing policy, combat, AI, and building layers (blueprints, work orders, mechanisms-as-gameplay, designation UX, room economy, and similar).
- A walkable-world executable, if present: a **validation harness / adjacent consumer**, not a game layer and not privileged. Its controller, character, camera, authored seed content, presentation, routes, workloads, and performance gates are harness-owned, not substrate identity.
- Future games (System ARPG, fortress/colony, descent roguelike, pure sandbox) as products in this repository.

## Required product outcomes

1. **Substrate, not game.** Downstream games and any harness consume the same public verbs, queries, and events; nothing above the matter boundary touches voxels directly, and no consumer gets a privileged or game-specific implementation path.
2. **Reads as a normal world, is voxel truth.** Surface worlds can look continuous and natural (terrain, water bodies, vegetation dressing and voxel-backed objects as matter-backed presentation); the mesh is a regenerated view. Physics, queries, and gameplay-facing checks use material truth, not decorative geometry outside the matter model.
3. **Mutable everywhere, including deep Z.** Any voxel can be destroyed, modified, or placed; underground (caves, strata, ore, depth) is first-class content space, not a painted floor under a heightmap.
4. **Geology-first generation, lazy and sparse.** Worlds are produced so digging reveals true structure; untouched volume stays cheap via lazy materialization and sparse residency suitable for GPU-resident operation at region scale.
5. **Consumer-facing world lifecycle.** External code can drive generation, streaming, meshing/view update after edits, collision against matter, and persistence of scars as deltas over the generative truth—enough that a separate harness or game can validate and ship play on top without forking the substrate.
6. **Multi-game foundation without multi-game rules.** The same crate stack is fit for adventure, fortress-style, descent, or sandbox consumers; gas and other policies remain injectable or above the substrate, not hard-wired here.

## Future products and enabling implications

Future **consumers** (not this product): a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox tools. They own gameplay, UX, controllers, content, presentation, and policy.

**Enabling implications** (substrate responsibilities those consumers motivate, not a committed multi-title roadmap): a dig-honest deep-Z material world; public mutation/query boundaries suitable for priced verbs and agents later; matter-backed vegetation and static water bodies; and seams so later games can attach integrity, flow, fire, or building policy without reimplementing core matter. Those extensions are not smuggled in as current game or building-layer scope.

## Non-goals

- Implementing the game, its rules, or shippable game content in this repository.
- System/LLM runtime, spells, gas metering policy, combat, AI agents, or building-game layers.
- Treating the walkable demo’s character, camera, seed postcard, debug keys, or benchmark scene as product features.
- Fluids beyond what the substrate must support for static bodies as a matter/presentation concern in early proof; full flow/weather/growth/season sims as game-facing systems.
- Multiplayer product delivery (architecture may stay consumer-agnostic; shipping netcode is not in scope).

## Confirmed vision constraints

- Delivery form is the **Rust** crate ecosystem (one crate or a small tightly scoped family).
- The world substrate is **GPU-resident** as part of product identity.
- Substrate has **zero LLM / System dependency**; those are optional game-layer clients.
- Harness and external games are equal consumers of **public interfaces only**.
- Game rules and the future System, LLM, spell, gas, combat, AI, and building layers are **not implemented** here; seams only where substrate needs demand them.

## Deferred design decisions

- Exact crate split and internal module boundaries within the substrate family.
- Voxel resolution, LOD strategy, object-layer capacity, and related fidelity/cost tradeoffs.
- Which matter-adjacent capabilities (e.g. integrity, granular settle, active fluid flow, fire CA, felling/rigid coupling) ship in the first vertical slice versus later substrate increments.
- Persistence encoding, streaming-ring policy, and harness-only acceptance workloads or machine targets.
- Whether and how a walkable-world harness is packaged relative to the crates (see Q1).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world executable a **mandatory current delivery** beside the substrate crates, or only a **permitted** adjacent validation artifact?

- **Proposed safe answer:** Permitted, not mandatory. Product identity and success center on the reusable substrate crates and public API; a harness may exist to exercise generation, streaming, meshing, editing, collision, persistence, and performance through those interfaces, but shipping the harness is not required to define the product.
- **If answered differently:** Making the harness mandatory expands the current delivery set to include a shippable walkable executable without turning it into the game—and still without importing its controller, content, presentation, or performance gates into substrate scope.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binding identity and exclusion: substrate crates only; game out of repo; harness if any is non-privileged; game/System/LLM/spell/gas/combat/AI/building layers out of scope.
- **docs/seeds/product-one-seed.md** — Motivates a first proof-oriented walkable consumer and a narrowed early substrate slice; supplies demo non-goals and validation pressure without transferring harness content, controls, or gates into product identity.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate purpose and outcome families (normal look vs voxel truth, mutability, deep Z, geology generation, matter-backed dressing, layering vs game) at design-spec depth; mechanisms and open tradeoffs remain deferred design.
