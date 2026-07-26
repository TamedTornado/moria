# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for matter, world generation, presentation of that matter, mutation, queries, and related physics services—not a game and not a demo product.

## Purpose

Games need a shared material world that looks like continuous natural terrain, digs and builds as real matter all the way down, and stays free of any one game’s rules. Moria exists so multiple downstream games can consume the same substrate through public interfaces, with zero dependency on an LLM or any particular game layer.

## Product boundary

**In product:** the reusable substrate and its public integration surface for external Rust consumers.

**Adjacent, not the product:** a walkable-world executable, if present, is only a validation harness and external-style consumer of that surface. Its character controls, camera, authored demo region, presentation polish, debug keys, scripted routes, benchmarks, and machine-specific gates are harness concerns—not product identity. See Q1 for whether that harness is a required repository delivery.

**Out of repository / downstream:** the actual game (or games). Game rules and the System, LLM, spell, gas, combat, AI, and building *game* layers are not implemented here. Compatibility seams may be designed where substrate outcomes require them; those layers remain consumer-owned.

**Enforcement intent (not design):** adjacent consumers have no privileged access to internal voxel paths; they use the same public interfaces a third-party game would.

## Required product outcomes

- **Natural material world:** consumers can present a continuous, natural-looking surface world whose renderable geometry is a non-authoritative view of underlying voxel matter—not a heightmap with props.
- **Mutable everywhere, deep Z first-class:** any location can be destroyed, placed, or otherwise mutated; underground geology (strata, caves, ores, saturated bands, and similar material truth) is content, not a floor skybox.
- **Geology-first generation and scale:** worlds generate as geology and related surface structure, materialize lazily, and stay tractable at large extent through sparse representation so idle or homogeneous volume need not force full voxel residency.
- **GPU-resident matter services:** world matter is GPU-resident; mutation drives incremental remeshing and related updates so edited regions stay collidable and visually consistent with voxel truth, including matter-coupled surface dressing and voxel-backed interactable objects.
- **World physics and verbs without game policy:** the substrate provides matter-level fluids behavior, structural support and collapse, granular settle where materials require it, dig/place (including multi-voxel stamps) as mutation verbs, thin ambient time/weather hooks as matter services, and path/occupancy queries for 3D movement—not game UI, pricing, or building gameplay.
- **Public API, persistence, and autonomy:** consumers use public verbs, queries, and events only; nothing above the matter core touches voxels directly. Persistence is worldgen truth plus edit deltas; streaming serves active anchors. The substrate stands alone with no LLM/System requirement.

## Future products and enabling implications

Downstream consumers (separate products) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style deep-descent adventure, and pure sandbox modes. They motivate seams for content registries, priced policies, and semantic layers above the substrate. This vision does not commit their gameplay, content, controllers, characters, or presentation. Enabling implication only: the same crate stack should remain usable by those games without embedding their rules.

## Non-goals

- Shipping the actual game or any game-rules layer in this repository.
- Implementing System/LLM features, spells, gas economies, combat, or AI as product scope.
- Implementing fortress/ARPG building gameplay, blueprints-as-work-orders, or mechanism *gameplay*—beyond substrate matter verbs and seams those layers will need.
- Treating the walkable demo’s content, third-person controller, or acceptance route as the product definition.
- Making the substrate depend on an LLM or on one consumer’s policy model.

## Confirmed vision constraints

- Delivery form is the Rust crate ecosystem (one crate or a small tightly scoped family).
- Product identity is substrate/engine layer, not game layer.
- GPU-resident voxel world is part of the product promise.
- Consumer boundary is mandatory: harness and games share the public interface; no privileged game-specific implementation path inside the substrate.
- Substrate must stand alone with zero LLM dependency.
- Game rules and listed future game layers are not implemented in this product (seams only where required).

## Deferred design decisions

- Crate split, internal layering, algorithms, data layouts, and meshing/LOD strategy.
- Voxel size, region scale, material palette depth, and sim-tier depth or sequence for fluids, CA, integrity, and ambient systems.
- Whether, when, and how a validation harness is built out (character, camera, seed world, benchmarks)—subject to Q1 on delivery obligation only.
- Persistence encoding, streaming ring policy, and platform/backend implementation choices (including portable graphics stack details).
- Multiplayer readiness depth and any performance or hardware targets.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted only—not required for product completeness. When present, it must consume public substrate interfaces like any external game.
- **If answered “required”:** the repository must deliver a harness that exercises generation, streaming, meshing, editing, collision, persistence, and performance through public APIs; harness-specific content and controls still stay outside product identity.

## Seed synthesis

- **README.md:** Names Moria as the reusable GPU-resident voxel-world substrate consumed as a Rust crate, and situates the walkable-world executable as a separate validation consumer rather than a game layer.
- **docs/seeds/project-boundary.md:** Binds product identity to the Rust substrate crates, keeps the actual game out of the repository, permits a public-interface-only validation harness, and excludes game-rules and listed future game layers from implementation here.
- **docs/seeds/voxel-world-substrate.md:** Authorizes the substrate’s outcome families—natural look over voxel truth, full mutability, deep Z, geology-first generation, GPU-resident matter, dressing/objects, fluids and integrity, placement verbs, public API, persistence/streaming—and future multi-game reuse without LLM dependency.
- **docs/seeds/product-one-seed.md:** Describes a first walkable demo and harness slice that motivates and can validate the substrate; its demo content, controller, milestones, and machine/performance gates remain adjacent-consumer detail, not the current product boundary.
