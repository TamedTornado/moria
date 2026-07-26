# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer world of matter—generation, mutable material truth, dynamic matter and physics, persistence and streaming, command/mirror/event coupling, and mutation-aware spatial support—not a game. Its first delivery is that substrate validated by a required adjacent walkable generated-world harness using public interfaces only.

## Purpose

Moria exists so multiple games can share one credible material world: continuous terrain that reads as a natural surface world, fully mutable voxel truth all the way down, first-class deep underground space, and material behaviors games can exploit without reimplementing world truth. Game rules, presentation, controllers, and policy live above the substrate; the substrate provides the world they run on, with no dependency on an LLM or “System” layer.

## Product boundary

**In product:** the reusable substrate crates—world generation and materialization; matter representation and mutation; substrate-owned dynamic-matter and physics outcomes; persistence and streaming of world and object state; GPU-resident truth with commands in and a stale mirror plus events out; mutation-aware spatial and traversal support derived from that matter; and the public verb/query surface through which higher layers interact. Consumers integrate only through the same public interfaces an external game would use.

**Adjacent required first delivery, not product identity:** a walkable generated-world validation harness must ship with the first delivery. It proves natural and deep-Z voxel truth, dig/place mutation, streaming, persistence, collision against voxel truth, and performance through public interfaces only—no privileged substrate paths. Controllers, camera, demo route, content, presentation, workloads, platform targets, and performance gates remain harness-owned, not substrate scope.

**First-delivery substrate depth (Product One):** full generation outcome; a deliberately partial matter slice (static water bodies and interactive dig/place, without CA/fire, flowing fluids, integrity, granular simulation, or non-stretch object dynamics in that slice); and a small dig/place verb plus mirror-query API slice. Omissions apply only to that first slice—not to the substrate’s broader outcome mandate below.

**Out of this repository / product:** the actual game; game rules; and the System/LLM, spell, gas, combat, AI, and building *layers*. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here. Player and agent controllers, AI policy, and game-specific pathfinding policy remain consumer-owned even when the substrate supplies spatial data derived from mutable matter.

## Required product outcomes

1. **Material world truth with dressing boundary.** The world is a fully mutable voxel volume: any location can be destroyed, moved, or placed. Everything that can burn, break, or block is voxel-backed; non-voxel dressing is anchored to and responsive to voxel state—not independent decorative geometry outside the matter model.
2. **Natural surface and deep Z.** The world reads as continuous natural terrain (hills, forests, water, cliffs, and similar surface forms), not a cube aesthetic; the render is a view of voxel truth, not the authority. Underground space is first-class playable volume—caves, strata, and depth—not a thin floor under a skybox.
3. **Dynamic matter and physics families.** The substrate supports interactive voxel-backed objects, fluids, ambient/fire behavior, granular response, and structural failure as reusable world capabilities owned by this product—not as optional future-game work. First-slice delivery may omit CA/fire, flowing fluids, integrity, granular simulation, and non-stretch object dynamics without removing them from the substrate mandate.
4. **Persistence and streaming.** The substrate persists edited world state as generation seed/function plus edit deltas, journals object and entity state, streams active areas so large regions remain usable without keeping the whole volume hot, and enables cross-run delta reuse. Product One’s seed-plus-delta save/load restores that demo’s saved world changes exactly; exact restoration is not asserted for general object/entity journals.
5. **GPU consumer coupling and opaque matter access.** Consumers drive GPU-resident world truth through commands and observe it through a stale mirror plus events; all higher layers use verbs and queries rather than direct voxel access; adjacent tools and games share that public surface with no privileged in-tree path.
6. **Mutation-aware spatial support and validated first delivery.** The substrate derives navigation and traversal data from mutable matter and supports continuous-3D traversal reasoning; pathfinding policy, AI, and player controllers remain consumer-owned. The first delivery includes a public-interface walkable harness that validates the generation, dig/place, streaming, persistence, collision, and performance outcomes above. The substrate stands alone with zero LLM/System dependency.

## Future products and enabling implications

Downstream consumers (not current product) include a Moria-style descent/adventure game, a Dwarf Fortress–style fortress/colony game, a System-driven ARPG, and pure sandbox modes. The substrate’s enabling implication is a shared material world—mutable geology, deep volume, dynamic matter behaviors, persistence/streaming, and query/mutation surfaces—that those games can consume without reimplementing world truth. Their gameplay, UX, controllers, authored content, presentation, combat, AI, spells, gas policy, and building/game-rule layers remain consumer-owned and stay out of this product.

## Non-goals

- Shipping the actual game or game-rule layers in this repository
- Implementing System/LLM, spells, gas, combat, AI, or building layers here
- Treating the validation harness’s character, camera, demo seed, clip route, or benchmark suite as substrate product identity or promise
- Making the substrate depend on an LLM or embed game policy as engine truth
- Reassigning substrate-owned matter/physics, persistence, or spatial-data outcomes to downstream games
- Dropping broader substrate matter/physics families from product scope merely because the first delivery slice omits them

## Confirmed vision constraints

- Product form: Rust crate or small family of tightly scoped Rust crates
- World execution intent: GPU-resident voxel-world substrate
- Consumer observability: commands into GPU-resident truth; stale mirror plus events out; no direct voxel access above the public verb/query surface
- Independence: zero LLM/System dependency in the substrate
- Repository delivery: reusable substrate is the sole product identity; a non-privileged walkable validation harness is a required adjacent first delivery and must use the same public interfaces as an external game

## Deferred design decisions

- Implementation depth, algorithms, and delivery sequence for each matter/physics family beyond the settled first-slice omissions, meshing approach, API shape, and crate split
- Persistence encodings, streaming ring design, and numerical performance or memory gates
- Harness-owned controls, content, presentation, platforms, and acceptance thresholds (existence and proof obligations are settled; mechanism detail is not)
- Compatibility seams for future game layers without implementing those layers
- Voxel size, LOD, and other open technical questions left for measurement-backed design

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident Rust voxel-world substrate and identifies the walkable executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binding identity and boundary: substrate crates in-repo; game and listed game layers out; harness stays outside product identity and must use public interfaces.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (mutable material world, deep Z, dynamic matter/physics, edit-delta persistence, object/entity journals, streaming, cross-run delta reuse, GPU coupling, mutation-aware spatial support, multi-game reuse, no LLM dependency) without mechanism inventory.
- **docs/seeds/product-one-seed.md** — Defines first delivery and done: full generation, partial matter, dig/place query sliver, and a required walkable harness validating those outcomes; seed-plus-delta exact reload is Product One–scoped; first-slice omissions do not drop broader substrate families.
