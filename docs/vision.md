# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine layer for natural, fully material 3D worlds—not a game.

## Purpose

Moria exists so future games can share one matter world: terrain that reads as a normal outdoor surface, remains voxel truth underneath, and stays diggable, placeable, and continuous through deep underground space. Games supply rules, presentation, and content above the substrate; the substrate provides generation, matter, mutation, queries, and the public integration surface those games need.

## Product boundary

**In product:** the substrate’s public interfaces and the world capabilities they expose—seeded geology and region generation, sparse GPU-resident voxel matter, smooth mesh views derived from that matter, dig and place style mutation, collision and queries against voxel truth, streaming and lazy materialization of large regions, and persistence of generation plus edit deltas. Consumers integrate only through those public interfaces; they receive no privileged or game-specific substrate paths.

**Adjacent, not the product identity:** a walkable-world executable may exist in this repository as a validation harness that exercises the substrate through the same public APIs an external game would use. Whether that harness is a required delivery of this repository is unresolved (see Q1). Its character controls, camera, authored demo route, presentation, content palette, hardware targets, and performance gates are not substrate scope.

**Out of repository / out of product:** the actual game (or games); game rules; the System / LLM layer; spells, gas policy, combat, AI, and building-as-gameplay layers. Compatibility seams may be designed where substrate requirements demand them; those layers must not be implemented here.

## Required product outcomes

1. **Reusable, standalone Rust substrate.** Consumers integrate through a public crate surface that keeps voxel access behind substrate verbs and queries. The same stack can support multiple games without privileged paths. The substrate has no LLM or game-rule dependency; gas/pricing and similar policies stay consumer concerns.

2. **Natural look, material truth.** Surface worlds read as ordinary terrain (ground, water, vegetation, rock) while the voxel grid remains the authoritative world—not decorative geometry beside a fake heightfield. What can block, break, or burn is matter-backed (including voxel-backed objects where interaction requires it); dressing stays driven by voxel state. Simulation and collision use voxel truth; the mesh is never the authority.

3. **Mutable everywhere.** Any material volume in play can be destroyed, altered, or placed; visible cuts and fills remesh as views of updated matter, not as one-off props.

4. **Deep continuous Z.** Underground volume is first-class world space (strata, caves, buried materials and voids), continuous with the surface rather than a thin floor under a skybox.

5. **Geology-first generation.** Worlds come from seedable geological pipeline logic so digging reveals true materials and structure; untouched volume stays cheap until touched (lazy materialization and sparse representation).

6. **Scale and durable scars.** Large regions stream and idle without holding the full volume as dense voxels; persistence is worldgen plus edit deltas so material change reloads as the same world.

## Future products and enabling implications

Downstream consumers (not this product) include a System-driven ARPG, DF-style fortress/colony play, Moria-style descent, and pure sandbox modes. The substrate enables them by owning reusable matter, generation, mutation, queries, and related world physics at the engine layer. Their gameplay, UX, controllers, authored content, presentation, and policy remain consumer-owned. Long-horizon matter features described in substrate design (multi-tier fluids, fire ecology, structural integrity, stamp/blueprint placement, ambient weather) are enabling implications of that engine role; delivery depth and sequence are design decisions, not a committed consumer roadmap here.

## Non-goals

- Shipping a complete game, combat, stats, AI, or multiplayer service in this product
- Implementing the System / LLM, spells, gas metering, or game pricing policy
- Building UI, blueprints-as-gameplay, work orders, or fortress designation layers
- Treating the validation harness’s demo content, controller, or performance scene as product identity
- Making decorative-only geometry the authority for physics or traversal

## Confirmed vision constraints

- Product form is Rust crate(s) in a Cargo-consumable shape suitable for external games
- Substrate must stand alone with zero LLM dependency
- Nothing above the matter/API boundary may treat voxels as a private side channel; public verbs and queries are the integration surface for all consumers
- Adjacent harness code, if present, must use those same public interfaces
- Game layers listed under Non-goals stay out of this repository even when future consumers need them

## Deferred design decisions

- Voxel resolution, LOD, meshing strategy details, and storage layout
- How far matter simulation goes in each release (fluids tiers, CA, integrity, granular settle, object felling)
- Streaming ring policy, persistence encoding, and API surface shape
- Crate split within the workspace and any concrete validation scenarios or benchmarks
- Whether and how multiplayer-oriented command authority is exposed

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current delivery** of this repository, or only a **permitted adjacent artifact** that may exist to exercise public APIs?

- **Proposed safe answer:** Permitted adjacent artifact—strongly motivated for validation, but not part of product identity; do not treat its demo slice, content, or performance gates as substrate acceptance criteria until design defines a harness plan.
- **If answered “required”:** the repository must ship a harness that consumes public APIs and validates generation, streaming, meshing, editing, collision, persistence, and performance at a high level, still without folding harness-owned controls, content, or device targets into the substrate product.

## Seed synthesis

- **`README.md`** — Names Moria as a reusable GPU-resident voxel substrate (Rust crate) and separates the walkable executable as consumer/validation, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binding identity and boundary: substrate crates in-repo; game out; harness only as validation through public APIs; game/System/building layers excluded.
- **`docs/seeds/product-one-seed.md`** — Motivates a first walkable proof of material world, dig/place honesty, and continuous Z; supplies adjacent-harness detail that must not redefine product identity (feeds Q1).
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes substrate outcome families (natural material worlds, universal mutability, deep Z, geology-first generation, sparsity/streaming, persistence, matter-backed interaction, reusable engine layering) without importing mechanism inventory or game layers.
