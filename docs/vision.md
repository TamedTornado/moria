# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer world of matter: generation, storage, mutation, queries, and related world simulation—not a game.

## Purpose

Moria exists so multiple games can share one credible material world: continuous terrain that reads as a natural surface world, fully mutable voxel truth all the way down, and first-class deep underground space. Game rules, presentation, and policy live above the substrate; the substrate provides the world they run on, with no dependency on an LLM or “System” layer.

## Product boundary

**In product:** the reusable substrate crates—world generation and materialization of a voxel volume, matter representation and mutation, consumer-facing queries and change paths, and the world-level simulation outcomes the substrate owns (so games can dig, place, traverse, and reason about a living material world). Consumers integrate only through the same public interfaces an external game would use.

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness that exercises those public interfaces. It is not the product and does not own privileged or game-specific substrate paths. Whether shipping that harness is a required current delivery is unresolved (see Q1). Its controllers, camera, demo route, content set, presentation, workloads, platform targets, and performance gates are harness-owned and are not product scope.

**Out of this repository / product:** the actual game; game rules; and the System/LLM, spell, gas, combat, AI, and building *layers*. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

1. **Material world truth.** The world is a fully mutable voxel volume: any location can be destroyed, moved, or placed; nothing important is fixed decorative geometry outside that matter model.
2. **Natural surface reading.** The world reads as continuous natural terrain (hills, forests, water, cliffs, and similar surface forms), not as a cube aesthetic; the render is a view of voxel truth, not the authority.
3. **Deep Z as content.** Underground space is first-class playable volume—caves, strata, and depth—not a thin floor under a skybox.
4. **Reusable engine surface.** The substrate exposes matter, world simulation, queries, and mutation so multiple game styles can sit above it; game-specific rules and pricing policy stay above the crate boundary.
5. **Standalone substrate.** The substrate stands alone with zero LLM/System dependency; any future System is a client of the same public surface, not a substrate feature.
6. **Honest consumer boundary.** Adjacent tools and games, including any validation harness, share the public integration surface; no privileged in-tree path around that boundary.

## Future products and enabling implications

Downstream consumers (not current product) include a Moria-style descent/adventure game, a Dwarf Fortress–style fortress/colony game, a System-driven ARPG, and pure sandbox modes. The substrate’s enabling implication is a shared material world—mutable geology, deep volume, and query/mutation surfaces—that those games can consume without reimplementing world truth. Their gameplay, UX, controllers, authored content, presentation, combat, AI, spells, gas policy, and building/game-rule layers remain consumer-owned and stay out of this product.

## Non-goals

- Shipping the actual game or game-rule layers in this repository
- Implementing System/LLM, spells, gas, combat, AI, or building layers here
- Treating the validation harness’s character, camera, demo seed, clip route, or benchmark suite as substrate product scope
- Making the substrate depend on an LLM or embed game policy as engine truth

## Confirmed vision constraints

- Product form: Rust crate or small family of tightly scoped Rust crates
- World execution intent: GPU-resident voxel-world substrate
- Integration: external-style public interfaces only for consumers and any harness
- Independence: zero LLM/System dependency in the substrate
- Repository scope: reusable substrate (and only as permitted, a non-privileged validation executable)—not the game

## Deferred design decisions

- Depth and sequencing of substrate capabilities (meshing approach, fluid/integrity/vegetation fidelity, streaming and persistence design, API shape, crate split)
- How far any first vertical slice goes versus the full outcome set above
- Harness-only choices if a harness is built: controls, content, presentation, platforms, and acceptance thresholds
- Compatibility seams for future game layers without implementing those layers

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required current delivery** of this repository, or only a **permitted adjacent artifact**?

*Proposed safe answer:* Permitted adjacent artifact only—product identity and “done” for Moria are the substrate crates and their outcomes; a harness may exist to validate public interfaces but is not mandatory current delivery.

*If different:* Making the harness mandatory adds a second current deliverable (still outside product identity) whose existence the design plan must schedule; it still must not import harness-owned controls, content, or performance gates into the substrate promise.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident Rust voxel-world substrate and frames the walkable executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binding identity and boundary: substrate crates in-repo; game and listed game layers out; harness may exist only via the same public interfaces.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural mutable world, deep Z, matter/simulation/query/mutation, multi-game reuse, no LLM dependency) without making its mechanism inventory or build-order design into vision.
- **docs/seeds/product-one-seed.md** — Describes a first walkable demo/harness slice that motivates validation of public substrate capabilities; its scene, controller, and targets stay adjacent and do not redefine product identity.
