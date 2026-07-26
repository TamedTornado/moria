# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for material worlds—not a game, and not an LLM-dependent system.

## Purpose

Moria exists so multiple downstream games can share one matter-truth world: a natural-looking surface over fully mutable voxels, with deep underground as first-class content. The substrate supplies generation, matter representation and living-matter behavior, physics-facing queries and mutation, spatial and construction support, streaming, and persistence. Game rules, UX, controllers, authored content, pricing policy, agents, and presentation stay above it. The substrate must stand alone with zero LLM dependency.

## Product boundary

**In product:** the reusable substrate and its public Rust consumer surface—including mutation verbs; observation via mirrors, queries, and events; object and registry seams; and substrate-owned matter, spatial, and construction-support capabilities.

**Adjacent, not product identity:** a walkable-world executable may exist as a validation harness for terrain generation, streaming, meshing, editing, collision, persistence, and performance. If present, it must consume the substrate through the same public interfaces available to an external game and must not own privileged or game-specific implementation paths. Whether that harness is a required repository delivery remains open (see Q1).

**Downstream / out of this repository:** the actual game; game rules; System and LLM features; spell, gas, combat, AI, and building *game layers*. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

**Consumer-owned even when used for validation:** character controllers, cameras, debug presentation, authored seed regions, demo routes, benchmark scenery, acceptance workloads, pricing policy, agents, and game-specific building UX (work orders, designations, mechanism gameplay).

## Required product outcomes

1. **Reusable public control surface** — expose a GPU-resident voxel world that external games integrate without privileged internal access. All mutations go through verbs; observation goes through mirrors, queries, and events. Provide object and registry seams so materials, structures, and related definitions can be authored without owning geology. Pricing, agents, the System, and game policy remain consumer-owned.
2. **Natural look, voxel truth** — continuous natural terrain derived from material voxels; the mesh is a regenerated, non-authoritative view; simulation, collision, and queries run against voxel truth.
3. **Everywhere-mutable matter** — any voxel may be destroyed, moved, or placed through controlled verbs. Voxel-backed objects participate in dynamic matter lifecycles so interactable surface matter is the same material system, not decorative props.
4. **Deep-Z geology generation** — geology-first procedural generation (strata, caves, ore and aquifer structure, surface biomes) with sparse, lazy materialization so large regions stay tractable.
5. **Living matter and ambient world** — granular behavior; fluid behavior and material reactions; structural integrity and collapse; material-state responses; and thin-but-present ambient fire, weather, time, season, and growth behavior that games may rely on and tune, without embedding game rules.
6. **Spatial support, construction support, streaming, and continuity** — derived Z and spatial data; mutation-aware navigation; first-class material and structure placement; reusable structure descriptions; structure and room metadata. Stream active regions. Durable truth is the seed-driven generation function plus edit deltas with object and entity journals; reconstruction is deterministic, and substrate-owned edits and object state continue across runs.

## Future products and enabling implications

Intended future consumers include a System/LLM-backed ARPG, a fortress/colony-style game, a descent/adventure game, and pure sandbox play. An early walkable-region slice can validate the stack; it does not redefine product identity or narrow reusable purpose to that slice’s delivery depth.

Enabling implications (not a committed multi-title roadmap): different games inject different pricing and agent policies over the same verbs; the System, if used, is a game-layer client of the same public surface, not a substrate feature.

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
- Durable world truth is reconstructible from seed-driven generation plus deltas (including substrate-owned object state), not from authoritative mesh or one-off world dumps

## Deferred design decisions

- Voxel resolution, storage layout, meshing approach, and distant-world presentation strategy
- Crate split within the family and concrete API shape
- Delivery depth and sequence among generation, meshing, objects, fluids, integrity, ambient simulation, navigation, structure metadata, and related systems
- Structure of any validation harness and which scenes or measurements it runs
- Persistence encoding details, streaming policy, and platform/graphics backend choices
- Simulation depth and tuning tables within each required living-matter outcome family

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required adjacent delivery** in this repository, or only **permitted** as a validation harness?

**Proposed answer:** Permitted adjacent artifact only. Current product identity and required product delivery remain the substrate crate(s); any harness work is adjacent and design-scoped after identity is fixed.

**If answered differently:** Requiring the harness adds a mandatory adjacent repository deliverable. It stays outside substrate identity and still must use public interfaces only; it does not import controller, content, presentation, or performance-gate details into product scope.

## Seed synthesis

- **README.md** — Establishes Moria as the reusable GPU-resident Rust voxel substrate and describes a walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binding identity and repository boundary: substrate crate(s); game out of repo; harness permitted under public interfaces; game/System/building layers excluded with seams allowed only where required.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families: natural look with voxel truth; full mutability including move; deep-Z geology; living matter and thin ambient simulation; verb/mirror/query/event control surface with object and registry seams; spatial and construction support; streaming and deterministic delta persistence; multi-game reuse; zero LLM dependency.
- **docs/seeds/product-one-seed.md** — Early consumer-shaped walkable-region slice that motivates mutability proof, public control-surface discipline from first use, and continuous-world validation concerns without redefining product identity or importing demo controls, content, or hardware gates into current product scope.
