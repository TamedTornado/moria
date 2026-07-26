# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for matter, generation, presentation of that matter, mutation, active material simulation, queries, generic semantic services, and related physics—not a game and not a demo product.

## Purpose

Games need a shared material world that looks like continuous natural terrain, digs and builds as real matter all the way down, actively simulates fluids, fire, weather-driven wetness, structure, and ambient response, and stays free of any one game’s rules. Moria exists so downstream games consume the same substrate through a public command-and-observation interface, with zero LLM dependency.

## Product boundary

**In product:** the reusable substrate and its public integration surface for external Rust consumers—including matter and generation, active material simulation, voxel-object lifecycle, generic structure/room semantics, mutation-safe navigation, generic object/mechanism integration, and shared authoring representations.

**Adjacent, not the product:** a walkable-world executable, if present, is only a validation harness and external-style consumer of that surface. Character controls, camera, authored demo region, presentation polish, debug keys, scripted routes, benchmarks, exact restore checks, and machine-specific gates are harness concerns—not product identity. See Q1 for whether that harness is a required repository delivery. A first harness slice may omit some full-substrate outcomes (for example active fluid flow, fire, or tree felling); that omission does not remove those outcomes from the product.

**Out of repository / downstream:** the actual game (or games). Game rules and the System, LLM, spell, gas, combat, AI, and building *gameplay* layers—including work-order labor rules and pricing policy—are not implemented here. Compatibility seams may be designed where substrate outcomes require them; those layers remain consumer-owned.

**Enforcement intent (not design):** adjacent consumers have no privileged access to internal voxel paths; they use the same public interfaces a third-party game would.

## Required product outcomes

- **Natural material world over voxel truth:** a continuous, natural-looking surface world whose geometry is a non-authoritative view of underlying voxel matter—not a heightmap with props. Any location can be destroyed, placed, or otherwise mutated; underground geology (strata, caves, ores, saturated bands) is first-class content. Worlds generate as geology and related surface structure, materialize lazily, and stay tractable at large extent through sparse representation so idle or homogeneous volume need not force full voxel residency.
- **GPU-resident matter services:** world matter is GPU-resident; mutation drives incremental remeshing and related updates so edited regions stay collidable and visually consistent with voxel truth, including matter-coupled surface dressing and voxel-backed natural objects.
- **Active material simulation:** active fluid flow and material interactions, weather-driven wetness, fire propagation, structural support and collapse, granular settle where materials require it, and ambient world behavior driven by time and weather. Static fluid bodies or passive time/weather hooks alone do not satisfy this. Mechanisms and delivery sequence are deferred; the outcome families are not.
- **Voxel-object lifecycle:** objects such as trees and boulders remain matter through breaking, movement, falling, growth, and related state changes. Tree falling—with dynamic-body conversion, impact, and re-voxelization or breakup—is a full-substrate outcome. First-slice harnesses may omit felling without demoting that lifecycle from the product.
- **Verbs and generic semantic services:** dig/place (including multi-voxel stamps); reusable structure/room semantics; mutation-safe navigation and path/occupancy queries for 3D movement; generic object/mechanism integration; and shared authoring representations (for example blueprint stamps). These are substrate services, not consumer building gameplay, work-order rules, or gas/labor pricing.
- **Public command-and-observation contract, persistence, and autonomy:** consumers drive GPU-resident mutation through commands/verbs and observe via coarse or stale mirrors and events; nothing above the matter core touches voxels directly. Persistence is worldgen truth plus edit deltas plus object/entity journals so changed worlds restore both voxel edits and persistent object/entity lifecycle state across runs and support cross-run reuse; streaming serves active anchors. Exact restore targets, if any, belong to adjacent harness validation. No LLM/System requirement.

## Future products and enabling implications

Downstream consumers (separate products) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style deep-descent adventure, and pure sandbox modes. They motivate priced-policy seams and game-layer rules above the substrate. This vision does not commit their gameplay, content, controllers, characters, or presentation. Enabling implication only: the same crate stack remains usable without embedding their rules.

## Non-goals

- Shipping the actual game or any game-rules layer in this repository.
- Implementing System/LLM features, spells, gas economies, combat, or AI as product scope.
- Implementing fortress/ARPG building *gameplay*, work-order labor rules, or pricing policy—beyond substrate verbs, generic mechanisms, room/nav services, and shared authoring representations.
- Treating the walkable demo’s content, controller, first-slice omissions, or acceptance route as the product definition.
- Making the substrate depend on an LLM or one consumer’s policy model.
- A load-bearing native Metal-only graphics path that abandons portable GPU backends.

## Confirmed vision constraints

- Delivery form is the Rust crate ecosystem (one crate or a small tightly scoped family).
- Product identity is substrate/engine layer, not game layer.
- GPU-resident voxel world is part of the product promise.
- Portable GPU delivery: load-bearing layers retain portable GPU backends and must not depend on a native Metal-only path; machine-specific targets and harness performance gates are not product constraints.
- Consumer boundary is mandatory: harness and games share the public command-and-observation interface; no privileged game-specific path inside the substrate.
- Substrate stands alone with zero LLM dependency; game rules and listed future game layers are not implemented here (seams only where required).

## Deferred design decisions

- Crate split, internal layering, algorithms, data layouts, and meshing/LOD strategy.
- Voxel size, region scale, material palette depth, and delivery sequence or depth for active-sim families already required as outcomes.
- Whether, when, and how a validation harness is built out—subject to Q1 on delivery obligation only.
- Persistence encoding details and streaming ring policy.
- Multiplayer readiness depth and performance targets.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted only—not required for product completeness. When present, it must consume public substrate interfaces like any external game.
- **If answered “required”:** the repository must deliver a harness that exercises generation, streaming, meshing, editing, collision, persistence, and performance through public APIs; harness-specific content and controls still stay outside product identity.

## Seed synthesis

- **README.md:** Names Moria as the reusable GPU-resident voxel-world substrate consumed as a Rust crate, and situates the walkable-world executable as a separate validation consumer rather than a game layer.
- **docs/seeds/project-boundary.md:** Binds product identity to the Rust substrate crates, keeps the actual game out of the repository, permits a public-interface-only validation harness, and excludes game-rules and listed future game layers here.
- **docs/seeds/voxel-world-substrate.md:** Authorizes full-substrate outcomes—natural look over voxel truth, mutability, deep Z, geology-first generation, GPU-resident matter, dressing, voxel-object lifecycle including tree falling, active fluids/fire/wetness/integrity/ambient sim, placement verbs, generic room/nav/mechanism/authoring services, command/mirror/event contract, and edit-delta plus object/entity persistence—without LLM dependency.
- **docs/seeds/product-one-seed.md:** Describes a first walkable harness slice that may omit active sim and felling; pins portable GPU backends as a crate constraint; demo content, controller, milestones, exact restore check, and machine gates remain adjacent-consumer detail, not product boundary.

