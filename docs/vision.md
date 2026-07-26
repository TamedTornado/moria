# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for material worlds—not a game, not a character demo, and not an LLM system.

## Purpose

Moria exists so multiple downstream games can share one authoritative material world: a natural-looking surface over fully mutable voxel truth, with deep underground as first-class content. The substrate provides matter, world behavior, queries, and mutation; game rules, presentation policy, and authored experience live above it. It must stand alone with no LLM dependency.

## Product boundary

**Belongs to Moria**

- The reusable voxel-world substrate and its public consumer interfaces.
- World generation, material occupancy, mutation, derived presentation of voxel truth, streaming, persistence of material change, and world-facing queries consumers need to run games on top.
- Compatibility seams where substrate requirements demand them for future game layers—without implementing those layers.

**Does not belong to Moria**

- The actual game, and game rules, combat, AI, spell/gas policy, System/LLM behavior, and building-as-gameplay layers.
- Controllers, characters, cameras, HUD, authored routes, and game-specific content or presentation policy.
- Privileged or harness-only paths that bypass public substrate interfaces.

**Adjacent, not identity**

- A walkable-world executable may exist in this repository as a separate consumer and validation harness. It is not the product identity. Whether shipping it is a current delivery obligation is unresolved (see Q1). While open, this brief only records that such a harness may exist and must use the same public interfaces an external game would.

## Required product outcomes

1. **Material world truth** — The voxel grid is authoritative matter. Rendered terrain is a regenerated view of that truth, not a separate decorative heightmap with props. Consumers can treat what they see as diggable, placeable, and queryable matter.
2. **Natural surface, mutable everywhere** — Generated regions read as ordinary outdoor worlds (terrain, water bodies, vegetation and clutter consistent with matter) while remaining fully mutable: destroy, move, or place material anywhere, including deep underground.
3. **Deep Z and geology-first generation** — Underground is content, not a floor. Generation is geological (strata, caves, ores, aquifers, biomes) and materializes on demand so large sparse regions stay tractable.
4. **World behavior without game rules** — The substrate supplies matter-level world behavior consumers rely on—fluid bodies and flow, structural support and failure, granular settle, and ambient matter effects such as fire and wetness—without encoding game pricing, win conditions, or faction logic. Delivery depth and order of these behaviors are design choices, not a narrowing of product identity.
5. **Public mutation and query surface** — Consumers change and inspect the world only through public verbs, queries, and events. Nothing above the substrate touches voxels by privileged side channels; a validation harness has no special access.
6. **Streamable, persistent scars** — Active regions stream in and out; untouched world stays cheap; player and sim edits persist as deltas on generation so a changed world reloads as the same material state.

## Future products and enabling implications

Downstream consumers (not this product) include a System-driven ARPG, fortress/colony play, a Moria-style descent experience, and pure sandbox modes. They motivate a reusable stack that exposes the same matter operations and queries under different game policies (for example, different pricing of the same verbs). The System, if any, is a game-layer client of those interfaces—not a substrate feature. This brief does not commit their gameplay, content, controls, or release order.

## Non-goals

- Implementing game rules, combat, AI, spells, gas economy, System/LLM features, or building-as-gameplay layers in this product.
- Owning character control, camera grammar, combat feel, or other consumer presentation.
- Making the walkable demo, a specific seed region, or an audience clip the definition of the product.
- Requiring LLM or cloud services inside the substrate.

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates.
- Substrate is GPU-resident and intended for consumption by external Rust game code.
- Adjacent consumers, including any validation harness, use only public substrate interfaces.
- The actual game lives outside this repository’s product boundary.
- Substrate has zero LLM dependency; LLM/System behavior is not a substrate feature.

## Deferred design decisions

- How far each matter, generation, simulation, and query capability goes in a given delivery slice, and in what order.
- Internal crate split, storage layout, meshing approach, LOD, and exact generation pipeline stages.
- Whether richer semantic conveniences (for example room tags or blueprint formats) ship inside substrate crates or only as consumer libraries.
- Graphics/API stack choices, target machines, and numeric performance budgets.
- Concrete harness content, controls, route, and acceptance gates—if a harness is delivered (Q1).
- Tunables that differ by game genre (support spans, fluid fidelity, object scale limits).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current repository delivery**, or only a **permitted adjacent artifact** that may be added without being part of “done” for the substrate product?

- **Proposed safe answer:** Permitted only. Current product commitment is the reusable substrate crates and public interfaces; a harness may exist later or alongside design work but is not mandatory for product completeness.
- **If answered differently:** Required delivery means the repository must also ship a non-game walkable harness that exercises the substrate through public APIs. Product identity remains the substrate; scope gains a delivery obligation for that adjacent executable, still without absorbing game layers or harness-specific content into substrate identity.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/validation, not a game layer.
- **docs/seeds/project-boundary.md** — Binds current identity to the substrate crate(s), places the real game outside the repo, permits a public-API-only harness, and excludes game/System/LLM/spell/gas/combat/AI/building layers.
- **docs/seeds/product-one-seed.md** — Describes a first walkable proof consumer and substrate slice motivations; informs why mutability, generation, streaming, and public dig/place matter, without transferring demo controls, content, platforms, or performance gates into product identity.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (material truth, natural mutable worlds, deep Z, generation, matter behavior, queries, streaming/persistence, multi-game reuse without LLM) at vision altitude; mechanisms and build order remain design.
