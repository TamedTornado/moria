# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer foundation for matter, world generation, mutation, and queries—not a game and not a presentation layer for one title.

## Purpose

Moria exists so multiple downstream games can share one trustworthy material world: terrain that reads as a natural outdoor environment, remains fully mutable voxel truth, and treats deep underground space as first-class content. Game rules, economy, combat, agents, and LLM-driven systems stay above the substrate so the same world stack can underpin different products without baking any one game into the engine.

## Product boundary

**In product:** the reusable voxel-world substrate and its public integration surface for Rust consumers—world/matter representation, geology-oriented generation, mutation and query capabilities, and GPU-resident operation as the substrate’s residency model.

**Out of product:** the actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and building-game layers. Those may later sit on Moria as separate products. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented in Moria.

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness for the substrate. If present, it is a separate consumer: it must use the same public interfaces available to an external game and must not own privileged or game-specific engine paths. Whether that harness is a required delivery is unresolved (see Q1). Its controllers, characters, content, presentation, routes, workloads, and acceptance gates are not product scope.

**Ownership rule:** gameplay, UX, authored content, presentation policy, and game-specific rules belong to consumers. Substrate-owned world and matter capabilities remain Moria’s responsibility even when a future game motivates them.

## Required product outcomes

1. **Rust-consumable substrate.** Downstream code integrates Moria through public Rust crate APIs and receives a reusable world foundation independent of any one game’s rules or content.
2. **Natural look over voxel truth.** The world reads as ordinary rolling terrain, forests, water, cliffs, and related surface features while remaining a fully material voxel world underneath—not decorative geometry over a non-material base.
3. **Mutable everywhere, deep Z included.** Any material volume can be destroyed, moved, or placed; underground space (caves, strata, depth) is real content, not a floor under a surface shell.
4. **Generation and residency for large worlds.** Worlds are produced as geology-first material volumes and stay practical at region scale through sparse, GPU-resident representation and demand-driven materialization rather than fully eager solid storage.
5. **Matter authority; mesh as view.** Simulation, collision-facing truth, queries, and mutation operate on voxel matter. Rendered surfaces are regenerated views of that matter, not a parallel authoritative world.
6. **Engine services without game policy.** The substrate provides matter, physics-facing world behavior, mutation verbs, and mirror-style queries suitable for multiple game types, with no dependency on an LLM and no requirement that consumers implement any particular game genre.

## Future products and enabling implications

Future consumers (not this product) include an actual game title, a System/LLM-backed ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. Enabling implications for the substrate—without importing consumer gameplay, content, controllers, or presentation—include standing alone without LLM dependency; exposing seams those games can attach to; and eventually supporting richer matter behaviors (for example multi-tier fluids, structural integrity, granular settle, fire ecology, and placement/stamp workflows) that fortress and adventure fantasies rely on. Depth and sequence of those capabilities are design decisions, not a committed roadmap in this brief.

## Non-goals

- Implementing the actual game, or any game-rules / System / LLM / spell / gas / combat / AI / building layer, in this product or repository.
- Treating a validation harness, demo route, or first walkable slice as the product identity.
- Making Minecraft-style cubic voxels the primary surface look (debug raw views may still exist).
- Coupling substrate correctness or operation to an LLM or to one game’s policy objects.

## Confirmed vision constraints

- Integration form is the Rust crate ecosystem (one crate or a small tightly scoped family).
- Consumers—including any validation harness—have no privileged access path; only public substrate interfaces.
- The substrate must stand alone with zero LLM dependency.
- World residency is GPU-resident at the product level.
- Game rules and excluded game layers live above Moria; they are not implemented here.

## Deferred design decisions

- Exact crate split within the allowed family, and how seams for future game layers are shaped without implementing those layers.
- Delivery depth and sequence among generation, meshing/view, mutation, streaming, persistence, vegetation/objects, fluids, integrity, and related matter behaviors.
- Voxel resolution, storage layout, meshing approach, streaming rings, and persistence encoding.
- Whether and when a walkable-world harness ships, and any harness-only content, controls, platforms, or performance gates (see Q1).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** beside the substrate crates, or only a **permitted adjacent artifact** that may be omitted without failing the product?

*Proposed safe answer:* Permitted adjacent artifact only—the current product promise is the reusable substrate crates; a harness may be added later to exercise public APIs but is not required to define or complete Moria.

*If answered differently:* Making the harness mandatory keeps substrate identity unchanged but expands delivery to include a separate executable that still must remain a non-privileged consumer; it does not pull demo content, controllers, or acceptance scenarios into substrate scope.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and frames the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Fixes product identity (Rust substrate crates), excludes the actual game and listed game layers from this repository, and requires any harness to share public interfaces only.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look over voxel truth, full mutability, deep Z, geology-first generation, GPU-resident matter, mesh-as-view, multi-game reuse, no LLM dependency) and names future consumer genres as downstream, not current product.
- **docs/seeds/product-one-seed.md** — Describes a first walkable demo slice that motivates proving material mutability and substrate APIs; its controller, content, presentation, platform, and performance details stay consumer/harness-owned and do not redefine Moria’s identity.
