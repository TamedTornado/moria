# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for mutable natural worlds—not a game, and not a playable demo product.

## Purpose

Moria exists so multiple games can share one substrate for matter, world generation, mutation, queries, presentation of voxel truth, streaming, and persistence, while each game owns its rules, content, and presentation above that layer. The substrate must stand alone with no dependency on an LLM or game-specific policy.

## Product boundary

**This product owns:** the reusable substrate that produces and maintains a fully material voxel world—geology-backed generation, deep underground as first-class space, GPU-resident matter, derived smooth surface presentation, dig/place-style mutation, matter queries, collision against voxel truth, streaming, and edit-delta persistence—exposed only through public crate interfaces.

**Adjacent, not product identity:** a walkable-world executable may exist in this repository as a validation harness for generation, streaming, meshing, editing, collision, persistence, and performance. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific substrate paths. Whether that harness is a required current delivery is unresolved (see Q1); while that is open, it is not treated as settled product scope beyond this permission and interface rule.

**Downstream / out of this product:** the actual game (separate consumer, not this repository); game rules; System/LLM features; spells; gas policy; combat; AI; building layers (blueprints, mechanisms, room/economy semantics); and all consumer-owned gameplay, controllers, characters, cameras, authored demo routes, art direction beyond substrate presentation of matter, and acceptance scenarios.

## Required product outcomes

- External games can depend on Moria as a Rust library surface for a GPU-resident voxel world without forking substrate internals.
- Generated worlds read as natural surface environments (terrain, water bodies, vegetation-capable matter) while remaining fully material voxel truth that can be destroyed, moved, or placed anywhere, including deep underground geology (strata, caves, ores, aquifers as world content—not a flat floor under props).
- The mesh or other surface presentation is a non-authoritative view regenerated from matter; physics, collision, and queries run against voxel truth so digs and cuts are honest material changes.
- Consumers mutate and inspect the world only through public verbs and queries (including dig/place-class editing); nothing above the substrate reaches voxels by a private path.
- World residency streams around active interest; durable truth is the generation function plus edit deltas so untouched volume stays cheap and scars restore.
- The same substrate can underpin different game modes later without baking any one game’s rules into the crate.

## Future products and enabling implications

Future consumers (not current product) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox play. Those titles remain separate products.

Supported enabling implications only (not a committed delivery catalog):

- Richer matter behavior (multi-tier fluids, structural integrity, fire and granular settle, vegetation as interactive voxel objects) so those games inherit honest world physics from the substrate rather than reimplementing it.
- Compatibility seams for game-layer clients (including a future System that authors placement, materials, and structures through the same registries and command surface as any other consumer)—without implementing those game layers here.
- A first walkable validation or demo slice may motivate early substrate depth; its character, camera, route, and content stay consumer-owned (see Q1 for harness delivery status).

## Non-goals

- Shipping the actual game, its rules, or its content in this repository.
- Implementing System/LLM, spell, gas, combat, AI, or building layers here.
- Absorbing harness or demo ownership of controllers, characters, cameras, authored regions, or presentation polish into the substrate product identity.
- Treating a heightmap-with-props world, or a cube-only aesthetic as the primary surface look, as the product promise.

## Confirmed vision constraints

- Integration surface is Rust crates; intended consumers are Rust games and tools linking that library surface.
- Matter and related heavy work are GPU-resident by product intent.
- Adjacent validation code, if present, shares the public interface and has no privileged substrate path.
- Substrate requirements may motivate compatibility seams for future layers; those layers are not implemented in Moria.
- The substrate has zero LLM dependency and must function without the System.

## Deferred design decisions

- How many crates and how they are partitioned.
- Voxel resolution, brick layout, meshing method, LOD, and related storage choices.
- Delivery depth and order of matter-physics capabilities (e.g. multi-tier fluids, integrity, fire, granular settle, object felling).
- Form, content, and acceptance of any validation harness or first walkable slice (after Q1).
- Target platforms, graphics backends, and numeric performance gates.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required current repository delivery**, or only a **permitted adjacent artifact** when useful?

- **Proposed safe answer:** Permitted only—project boundary language allows a harness but does not mandate it; product identity remains the substrate crates. If a harness exists, it must use public interfaces only.
- **If answered “required”:** the repository must also deliver an adjacent walkable harness that validates generation, streaming, meshing, editing, collision, persistence, and performance through public APIs; identity stays the substrate, and character/content/controller details remain outside product scope and still need separate design.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable, GPU-resident voxel-world substrate consumed as a Rust crate, and positions a walkable-world executable as a separate consumer/validation harness for core world capabilities—not a game layer.
- **`docs/seeds/project-boundary.md`:** Fixes current identity as the reusable Rust substrate; keeps the real game out of the repository; permits a public-interface-only validation harness; excludes game rules and System/LLM/spell/gas/combat/AI/building layers while allowing future compatibility seams.
- **`docs/seeds/product-one-seed.md`:** Describes a first walkable consumer/demo slice (region, character, dig proof, targets) that motivates substrate editing and presentation outcomes; its controller, content, platform numbers, and milestone plan do not expand current product identity or import harness delivery without Q1.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes substrate outcome families—natural look over voxel truth, full mutability, deep-Z geology generation, GPU-resident matter, non-authoritative meshing, streaming and gen-plus-delta persistence, physics/queries/mutation as the engine layer, multi-game reuse, zero LLM dependency—without making mechanism inventory or future game features into current vision scope.
