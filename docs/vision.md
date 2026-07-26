# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for matter, world generation, presentation of voxel truth, mutation, queries, collision, streaming, and persistence—not a game and not a game rules layer.

## Purpose

Moria exists so multiple downstream games can share one material world substrate: a natural-looking surface over continuous, fully mutable voxel truth, with deep underground as first-class space. Game rules, content policy, and presentation ownership stay above the substrate. The substrate stands alone with no LLM or System dependency.

## Product boundary

**In product:** the reusable voxel-world substrate and its public consumer interfaces. Adjacent consumers, including any walkable-world validation harness, may exist only as separate artifacts that use those same public interfaces—no privileged or game-specific paths into the substrate.

**Out of product / repository:** the actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and building-as-gameplay layers. Compatibility seams may be designed where substrate outcomes require them; those layers are not implemented here.

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness for substrate capabilities (generation, streaming, meshing, editing, collision, persistence, performance). Its controller, character, camera, authored demo route, content inventory, presentation choices, and acceptance scene are consumer-owned. Whether that harness is a required current delivery is unresolved (see Q1); this brief does not treat it as settled delivery.

## Required product outcomes

1. **Matter truth on GPU.** The world is a GPU-resident material substrate: voxels are authoritative matter; extracted meshes and dressing are views derived from that truth, not a second world model.
2. **Natural-looking mutable world.** Rolling surface terrain can read as an ordinary outdoor world while remaining fully diggable/placeable matter; player cuts and edits remain honest material change, not decorative geometry outside the world.
3. **Deep continuous volume.** Surface through deep underground is one continuous 3D space—caves, strata, and buried structure are content the substrate can host, not a painted floor under a skybox.
4. **Generation, sparsity, and streaming.** Worlds are produced as diggable geology (not a heightmap shell with fake rock), materialize lazily enough that large regions are practical, and stream around active use without requiring the entire volume as dense resident data.
5. **Mutation, query, and collision APIs.** Consumers change and inspect matter, and resolve collision against voxel truth, only through public verbs and queries—nothing above the matter boundary touches voxels by side channel.
6. **Persistence of generation plus scars.** Truth is regenerable world state plus retained edit/object change so a world can be reloaded with prior defacement and structure intact at the substrate contract level.

## Future products and enabling implications

Downstream consumers (not this product) include a System/LLM ARPG, fortress/colony and descent-style games, pure sandbox play, and the walkable-world harness when that adjacent artifact exists as a non-game validator (see Q1). Broader matter behaviors described for those fantasies (richer fluid play, structural failure, fire and ambient ecology, building verbs and mechanisms, entity navigation on mutable nav) are enabling implications of a deep substrate, not a committed multi-game roadmap or first-release catalog in this brief. Gameplay, UX, controllers, characters, authored content, and game-specific policy remain consumer-owned.

## Non-goals

- Shipping the actual game or any game-rules layer in this repository.
- Implementing System/LLM, spell, gas, combat, AI, or building gameplay layers here.
- Owning harness/demo character control, camera policy, demo routes, or marketing milestone content as product scope.
- Treating the substrate’s first demo depth as a narrowing of product identity.

## Confirmed vision constraints

- Integration surface is Rust crates for external and in-repo consumers alike.
- Consumer isolation: harness and games share only public substrate interfaces; privileged access is disallowed.
- Substrate independence: no LLM/System dependency in the product core.
- Explicit product exclusions above are binding now, not deferred polish.

## Deferred design decisions

- Crate family split, internal layering, algorithms, data layouts, and API shape.
- Voxel scale, meshing strategy, LOD, sparsity/streaming ring policy, and persistence encoding.
- Depth and sequence of matter features beyond the outcome mandates (fluids, integrity, CA, objects, ambient sim).
- Performance budgets, target machines, and validation workloads (design and harness concerns).
- First vertical-slice contents and milestone order for proving the substrate.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** alongside the substrate crates, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted only—ship criteria for “current product” center on the reusable substrate; a harness may be built to validate public interfaces but is not mandatory identity.
- **If answered differently:** Requiring it keeps substrate identity unchanged but adds a separate delivery obligation (still not game content); it must not pull controller, character, demo content, or acceptance scenes into product identity.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident Rust voxel substrate and positions the walkable-world executable as a separate non-game validation consumer.
- **`docs/seeds/project-boundary.md`:** Fixes repository identity on the substrate crates, excludes game and listed future layers, and requires public-interface-only consumer access if a harness exists.
- **`docs/seeds/product-one-seed.md`:** Motivates an early walkable proof and first-slice non-goals; supplies validation motivation without transferring demo ownership or platform gates into product identity.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes core substrate outcomes—natural look over voxel truth, full mutability, deep Z, generation, GPU-resident matter, queries/mutation, streaming, and persistence—while remaining a design-depth source, not a mechanism checklist for this brief.
