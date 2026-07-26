# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer world of matter—generation, mutable material truth, dynamic matter and physics behavior, persistence and streaming, command/mirror/event consumer coupling, and mutation-aware spatial support—not a game.

## Purpose

Moria exists so multiple games can share one credible material world: continuous terrain that reads as a natural surface world, fully mutable voxel truth all the way down, first-class deep underground space, and material behaviors games can exploit without reimplementing world truth. Game rules, presentation, controllers, and policy live above the substrate; the substrate provides the world they run on, with no dependency on an LLM or “System” layer.

## Product boundary

**In product:** the reusable substrate crates—world generation and materialization; matter representation and mutation; substrate-owned dynamic-matter and physics outcomes; persistence and streaming of world and object state; GPU-resident truth with commands in and a stale mirror plus events out; mutation-aware spatial and traversal support derived from that matter; and the public verb/query surface through which higher layers interact. Consumers integrate only through the same public interfaces an external game would use.

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness that exercises those public interfaces. It is not the product and does not own privileged or game-specific substrate paths. Whether shipping that harness is a required current delivery is unresolved (see Q1). Its controllers, camera, demo route, content set, presentation, workloads, platform targets, and performance gates are harness-owned and are not product scope.

**Out of this repository / product:** the actual game; game rules; and the System/LLM, spell, gas, combat, AI, and building *layers*. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here. Player and agent controllers, AI policy, and game-specific pathfinding policy remain consumer-owned even when the substrate supplies spatial data derived from mutable matter.

## Required product outcomes

1. **Material world truth with dressing boundary.** The world is a fully mutable voxel volume: any location can be destroyed, moved, or placed. Everything that can burn, break, or block is voxel-backed; non-voxel dressing is anchored to and responsive to voxel state—not independent decorative geometry outside the matter model.
2. **Natural surface and deep Z.** The world reads as continuous natural terrain (hills, forests, water, cliffs, and similar surface forms), not a cube aesthetic; the render is a view of voxel truth, not the authority. Underground space is first-class playable volume—caves, strata, and depth—not a thin floor under a skybox.
3. **Dynamic matter and physics families.** The substrate supports interactive voxel-backed objects, fluids, ambient/fire behavior, granular response, and structural failure as reusable world capabilities owned by this product—not as optional future-game work.
4. **Persistence and streaming.** The substrate restores edited world and object state across sessions (generation seed/function plus edit deltas, with exact restoration of saved change), streams active areas so large regions remain usable without keeping the whole volume hot, and enables cross-run reuse of those deltas.
5. **GPU consumer coupling and opaque matter access.** Consumers drive GPU-resident world truth through commands and observe it through a stale mirror plus events; all higher layers use verbs and queries rather than direct voxel access; adjacent tools and games share that public surface with no privileged in-tree path.
6. **Mutation-aware spatial support.** The substrate derives navigation and traversal data from mutable matter and supports continuous-3D traversal reasoning. Pathfinding policy, AI, and player controllers remain consumer-owned. The substrate stands alone with zero LLM/System dependency.

## Future products and enabling implications

Downstream consumers (not current product) include a Moria-style descent/adventure game, a Dwarf Fortress–style fortress/colony game, a System-driven ARPG, and pure sandbox modes. The substrate’s enabling implication is a shared material world—mutable geology, deep volume, dynamic matter behaviors, persistence/streaming, and query/mutation surfaces—that those games can consume without reimplementing world truth. Their gameplay, UX, controllers, authored content, presentation, combat, AI, spells, gas policy, and building/game-rule layers remain consumer-owned and stay out of this product.

## Non-goals

- Shipping the actual game or game-rule layers in this repository
- Implementing System/LLM, spells, gas, combat, AI, or building layers here
- Treating the validation harness’s character, camera, demo seed, clip route, or benchmark suite as substrate product scope
- Making the substrate depend on an LLM or embed game policy as engine truth
- Reassigning substrate-owned matter/physics, persistence, or spatial-data outcomes to downstream games

## Confirmed vision constraints

- Product form: Rust crate or small family of tightly scoped Rust crates
- World execution intent: GPU-resident voxel-world substrate
- Consumer observability: commands into GPU-resident truth; stale mirror plus events out; no direct voxel access above the public verb/query surface
- Independence: zero LLM/System dependency in the substrate
- Repository scope: reusable substrate (and only as permitted, a non-privileged validation executable)—not the game

## Deferred design decisions

- Implementation depth, algorithms, and delivery sequence for each matter/physics family, meshing approach, API shape, and crate split
- Persistence encodings, streaming ring design, and numerical performance or memory gates
- How far any first vertical slice goes versus the full outcome set above (a first-slice omission is not a product-scope omission)
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
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (mutable material world with dressing boundary, deep Z, dynamic matter/physics, persistence/streaming, GPU command/mirror/event coupling, mutation-aware spatial support, multi-game reuse, no LLM dependency) without making mechanism inventory or build order into vision.
- **docs/seeds/product-one-seed.md** — Describes a first walkable demo/harness slice and seed-plus-delta exact reload; its scene, controller, platforms, and targets stay adjacent; first-slice omissions do not drop broader substrate outcome families.
