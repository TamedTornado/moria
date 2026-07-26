# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer foundation for matter, world generation, mutation, queries, and world services—not a game and not a presentation layer for one title.

## Purpose

Moria exists so multiple downstream games can share one trustworthy material world: terrain that reads as a natural outdoor environment, remains fully mutable voxel truth, and treats deep underground space as first-class content. Game rules, economy, combat, agents, labor policy, and LLM-driven systems stay above the substrate so the same world stack can underpin different products without baking any one game into the engine.

## Product boundary

**In product:** the reusable voxel-world substrate and its public integration surface for Rust consumers—world and matter representation, geology-oriented generation, mutation and observation control, reactive matter behavior, hydrology, structural response, construction-enabling substrate operations, mutation-safe derived world services, durable world continuity, and GPU-resident operation as the substrate’s residency model.

**Out of product:** the actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and consumer building-game layers (building UI, labor, economy, work orders, and authored game content and policy). Those may later sit on Moria as separate products. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented in Moria.

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness for the substrate (see Q1). If present, it is a separate consumer: it must use the same public interfaces available to an external game and must not own privileged or game-specific engine paths. Its controllers, characters, content, presentation, routes, workloads, platforms, and acceptance gates are not product scope.

**Ownership rule:** gameplay, UX, authored content, presentation policy, and game-specific rules belong to consumers. Substrate-owned world, matter, placement, construction primitives, derived services, and continuity remain Moria’s responsibility even when a future game motivates them. A first delivery slice that omits some substrate behaviors does not reclassify those behaviors as non-product.

## Required product outcomes

1. **Rust-consumable, controlled integration.** Downstream code integrates through public Rust crate APIs. Mutations go through verbs/commands; observation goes through mirrors/events; higher layers have no direct voxel access.
2. **Natural look over fully material truth.** The world reads as ordinary rolling terrain, forests, water, cliffs, and related surface features while remaining voxel matter underneath—including deep Z—not decorative geometry over a non-material base. Rendered surfaces are regenerated views, not a parallel authority.
3. **Generation, residency, and continuity.** Worlds are produced as geology-first material volumes and stay practical at region scale through sparse, GPU-resident representation and demand-driven materialization. Edit-delta persistence, object/entity state continuity, active-area streaming, and cross-run reuse keep worlds durable and scalable across sessions.
4. **Reactive world matter.** Voxel-backed interactive objects, matter-anchored dressing, material-state simulation, granular behavior, fire, weather and time, and ambient ecology are substrate-owned world behavior.
5. **Dynamic hydrology and structural response.** The substrate provides dynamic fluids—including standing bodies, flow, fine effects, aquifer breaches, and material interactions—and material-dependent support, failure, cave-ins, and consumer-queryable integrity.
6. **Construction primitives and derived world services.** First-class placement, reusable stamps and object primitives, and world-derived structure metadata enable construction without owning consumer labor or economy policy. Navigation derived from mutable matter, continuous-3D traversal support, and multiple movement-query classes stay mutation-safe as the world changes.

## Future products and enabling implications

Future consumers (not this product) include an actual game title, a System/LLM-backed ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. Enabling implications—without importing consumer gameplay, content, controllers, or presentation—include standing alone without LLM dependency; exposing seams those games can attach to; and treating gas pricing and the System as game-layer clients or policies, not substrate features. Delivery depth and sequence among substrate outcome families are design decisions, not a committed roadmap in this brief.

## Non-goals

- Implementing the actual game, or any game-rules / System / LLM / spell / gas / combat / AI layer, or consumer building gameplay (UI, labor, economy, work orders, authored content policy), in this product or repository.
- Treating a validation harness, demo route, or first walkable slice as the product identity or as the limit of substrate outcome scope.
- Making Minecraft-style cubic voxels the primary surface look (debug raw views may still exist).
- Coupling substrate correctness or operation to an LLM or to one game’s policy objects.
- Giving higher layers direct voxel access outside the verb/command and mirror/event integration model.

## Confirmed vision constraints

- Integration form is the Rust crate ecosystem (one crate or a small tightly scoped family).
- Consumers—including any validation harness—have no privileged access path; only public substrate interfaces.
- The substrate must stand alone with zero LLM dependency.
- World residency is GPU-resident at the product level.
- Game rules and excluded consumer layers live above Moria; they are not implemented here.
- Higher layers mutate through verbs/commands and observe through mirrors/events, not by direct voxel access.

## Deferred design decisions

- Exact crate split within the allowed family, and how seams for future game layers are shaped without implementing those layers.
- Delivery depth and sequence among generation, views, mutation, reactive matter, hydrology, integrity, construction primitives, derived services, streaming, and persistence.
- Mechanisms: voxel resolution, storage layout, meshing approach, fluid/integrity/nav implementations, streaming rings, and persistence encoding.
- Whether and when a walkable-world harness ships, and any harness-only content, controls, platforms, or performance gates (see Q1).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** beside the substrate crates, or only a **permitted adjacent artifact** that may be omitted without failing the product?

*Proposed safe answer:* Permitted adjacent artifact only—the current product promise is the reusable substrate crates; a harness may be added later to exercise public APIs but is not required to define or complete Moria.

*If answered differently:* Making the harness mandatory keeps substrate identity unchanged but expands delivery to include a separate executable that still must remain a non-privileged consumer; it does not pull demo content, controllers, or acceptance scenarios into substrate scope.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and frames the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Fixes product identity (Rust substrate crates), excludes the actual game and listed consumer layers from this repository, and requires any harness to share public interfaces only.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look over voxel truth, full mutability, deep Z, geology-first generation, reactive matter, hydrology, structural response, construction primitives, derived world services, continuity, GPU-resident matter, controlled integration, multi-game reuse, no LLM dependency) and names future consumer genres as downstream, not current product.
- **docs/seeds/product-one-seed.md** — Describes a narrow first walkable demo slice that motivates proving material mutability and substrate APIs; its omissions, controller, content, presentation, platform, and performance details scope that slice only and do not redefine Moria’s product mandate or demote substrate outcome families.
