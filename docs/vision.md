# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for material worlds—not a game, and not an LLM-dependent system.

## Purpose

Moria exists so multiple downstream games can share one matter-truth world: a natural-looking surface over fully mutable voxels, with deep underground as first-class content. The substrate supplies generation, matter representation, physics-facing queries and mutation, streaming, and persistence. Game rules, UX, controllers, authored content, and presentation policy stay above it. The substrate must stand alone with zero LLM dependency.

## Product boundary

**In product:** the reusable substrate and its public Rust consumer surface.

**Adjacent, not product identity:** a walkable-world executable may exist as a validation harness for terrain generation, streaming, meshing, editing, collision, persistence, and performance. If present, it must consume the substrate through the same public interfaces available to an external game and must not own privileged or game-specific implementation paths. Whether that harness is a required repository delivery remains open (see Q1).

**Downstream / out of this repository:** the actual game; game rules; System and LLM features; spell, gas, combat, AI, and building *game layers*. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

**Consumer-owned even when used for validation:** character controllers, cameras, debug presentation, authored seed regions, demo routes, benchmark scenery, and acceptance workloads.

## Required product outcomes

1. **Reusable crate surface** — expose a GPU-resident voxel world that external games integrate without privileged internal access.
2. **Natural look, voxel truth** — continuous natural terrain derived from material voxels; the mesh is a regenerated, non-authoritative view; simulation, collision, and queries run against voxel truth.
3. **Everywhere-mutable matter** — destroy, erode, and place material through controlled verbs so the world is fully material, not decorative geometry with props.
4. **Deep-Z geology generation** — geology-first procedural generation (strata, caves, ore and aquifer structure, surface biomes) with sparse, lazy materialization so large regions stay tractable.
5. **Streaming and persistence** — stream active regions; durable truth is the worldgen function plus edit deltas (with matter-related object change continuity as needed).
6. **Living surface and matter behaviors** — voxel-backed interactable surface objects and matter-driven dressing; fluid volumes in the material model; substrate-level matter behaviors (including support/collapse and flow-capable fluid handling) that games may rely on and tune, without embedding game rules.

## Future products and enabling implications

Intended future consumers include a System/LLM-backed ARPG, a fortress/colony-style game, a descent/adventure game, and pure sandbox play. An early walkable-region slice can validate the stack; it does not redefine product identity or narrow reusable purpose to that slice’s depth.

Enabling implications (not a committed multi-title roadmap): verb, query, and event boundaries so pricing policy and agents remain consumer-side; material and placement registries so later content systems can author without owning geology; support, fluids, and ambient matter response so fortress- and adventure-style fantasies can share the same foundation.

## Non-goals

- Implementing the game, combat, stats, AI, System/LLM features, spells, or gas economy in this product
- Shipping game-layer building (player building UI, work orders, designations, mechanism gameplay) as substrate scope
- Making any validation harness a game layer or a privileged second API
- Treating demo controllers, seed content, machine-specific performance gates, or first-slice milestone depth as the product definition

## Confirmed vision constraints

- Integration surface is Rust crate(s)
- Matter representation is GPU-resident
- Substrate has zero LLM dependency
- Any adjacent consumer, including a harness if present, uses public interfaces only—no privileged game-specific paths
- Game policy and rules live above the substrate; the substrate remains reusable across the intended consumer styles

## Deferred design decisions

- Voxel resolution, storage layout, meshing approach, and distant-world presentation strategy
- Crate split within the family and concrete API shape
- Delivery depth and sequence among generation, meshing, objects, fluids, integrity, ambient simulation, and related matter systems
- Structure of any validation harness and which scenes or measurements it runs
- Persistence encoding, streaming policy, and platform/graphics backend choices

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required adjacent delivery** in this repository, or only **permitted** as a validation harness?

**Proposed answer:** Permitted adjacent artifact only. Current product identity and required product delivery remain the substrate crate(s); any harness work is adjacent and design-scoped after identity is fixed.

**If answered differently:** Requiring the harness adds a mandatory adjacent repository deliverable. It stays outside substrate identity and still must use public interfaces only; it does not import controller, content, presentation, or performance-gate details into product scope.

## Seed synthesis

- **README.md** — Establishes Moria as the reusable GPU-resident Rust voxel substrate and describes a walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binding identity and repository boundary: substrate crate(s); game out of repo; harness permitted under public interfaces; game/System/building layers excluded with seams allowed only where required.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families: natural look with voxel truth, full mutability, deep-Z geology generation, matter/query/mutation role, streaming and delta persistence, surface objects/dressing and matter behaviors, multi-game reuse, zero LLM dependency.
- **docs/seeds/product-one-seed.md** — Early consumer-shaped walkable-region slice that motivates mutability proof and continuous-world validation concerns without redefining product identity or importing demo controls, content, or hardware gates into current product scope.
