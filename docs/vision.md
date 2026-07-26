# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for material worlds—not a game, not a character demo, and not an LLM system. This repository must also deliver a walkable-world validation executable as an adjacent public-API consumer; that harness is a required current delivery, not the product identity.

## Purpose

Moria exists so multiple downstream games can share one authoritative material world: a natural-looking surface over fully mutable voxel truth, with deep underground as first-class content. The substrate provides matter, world behavior, queries, and mutation; game rules, presentation policy, and authored experience live above it. It must stand alone with no LLM dependency.

## Product boundary

**Belongs to Moria (substrate identity)**

- The reusable voxel-world substrate and its public consumer interfaces.
- World generation, material occupancy, mutation, derived presentation of voxel truth, streaming, persistence of material and substrate-owned object change, matter-level world behavior, and world-facing queries (including collision and navigation data derived from voxels).
- Compatibility seams where substrate requirements demand them for future game layers—without implementing those layers.

**Does not belong to Moria**

- The actual game, and game rules, combat, AI, spell/gas policy, System/LLM behavior, and building-as-gameplay layers.
- Controllers, characters, cameras, HUD, authored routes, and game-specific content or presentation policy.
- Privileged or harness-only paths that bypass public substrate interfaces.

**Required adjacent delivery (not identity)**

- A walkable-world executable must ship in this repository as a separate consumer and validation harness. It proves generation, streaming, matter views, editing, collision against voxel truth, persistence, and performance through the same public interfaces available to an external game. Its character, controls, camera, seed region content, presentation, and numeric gates are harness concerns, not substrate identity.

## Required product outcomes

1. **Authoritative material world** — The voxel grid is authoritative matter. Rendered terrain is a regenerated view of that truth, not a separate decorative heightmap with props. Physics, collision, and other world-facing queries run against voxel truth, not the render mesh. Interactable vegetation and objects remain coupled to voxel matter; passive dressing (grass, flowers, ground clutter) has no individual voxel identity and is derived from underlying voxel state so it never desynchronizes from the matter world.
2. **Natural surface, mutable everywhere, deep Z** — Generated regions read as ordinary outdoor worlds while remaining fully mutable: destroy, move, or place material anywhere, including deep underground. Generation is geological (strata, caves, ores, aquifers, biomes) and materializes on demand so large sparse regions stay tractable.
3. **Matter-level world behavior** — The substrate supplies fluid bodies and flow, structural support and failure, granular settle, fire and wetness, and thin ambient ecology (day/night, season, weather, growth, fire ecology) without encoding game pricing, win conditions, or faction logic.
4. **Public mutation and query surface** — Consumers change and inspect the world only through public verbs, queries, and events. Nothing above the substrate touches voxels by privileged side channels; the validation harness has no special access.
5. **Mutation-safe navigation across continuous 3D** — The substrate maintains derived navigation and traversal data that stays consistent under mutation, supporting continuous three-dimensional movement rather than a 2D surface world with a painted floor.
6. **Streamable, persistent scars and object state** — Active regions stream in and out; untouched world stays cheap; voxel edit deltas and substrate-owned object state persist so a changed world reloads as the same material state.

**First substrate slice (binding delivery contour, not the full product ceiling)**

The first delivery includes generation at full intended depth for a curated proof region; a partial matter slice covering occupancy, regenerated matter views and dressing, static water bodies, and interactable object placement without felling or rigid-body conversion; dig/place and mirror-query boundary from day one; streaming; persistence of material change; and performance validation of that slice. That first slice excludes active cellular automata, fire simulation, flowing fluids, structural integrity, and granular settle. Outcome families above remain the eventual reusable substrate mandate; later depth and order beyond this contour are design choices.

## Future products and enabling implications

Downstream consumers (not this product) include a System-driven ARPG, fortress/colony play, a Moria-style descent experience, and pure sandbox modes. They motivate a reusable stack that exposes the same matter operations and queries under different game policies. The System, if any, is a game-layer client of those interfaces—not a substrate feature. This brief does not commit their gameplay, content, controls, or release order.

## Non-goals

- Implementing game rules, combat, AI, spells, gas economy, System/LLM features, or building-as-gameplay layers in this product.
- Owning character control, camera grammar, combat feel, or other consumer presentation.
- Defining the product as the walkable demo, a specific seed region, or an audience clip (the harness proves the substrate; it is not the substrate).
- Requiring LLM or cloud services inside the substrate.

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates.
- Substrate is GPU-resident and intended for consumption by external Rust game code.
- Adjacent consumers, including the validation harness, use only public substrate interfaces.
- The actual game lives outside this repository’s product boundary.
- Substrate has zero LLM dependency; LLM/System behavior is not a substrate feature.
- The walkable-world harness is a required current repository delivery and must remain a public-API consumer of the substrate.

## Deferred design decisions

- Capability depth and delivery order for substrate behaviors **beyond** the first-slice contour above.
- Internal crate split, storage layout, meshing approach, LOD, and exact generation pipeline stages.
- Whether richer semantic conveniences (for example room tags or blueprint formats) ship inside substrate crates or only as consumer libraries.
- Graphics/API stack choices, target machines, and numeric performance budgets for the product promise.
- Concrete harness character, controls, camera, seed-region content, presentation, and acceptance gates (harness design, not substrate identity).
- Tunables that differ by game genre (support spans, fluid fidelity, object scale limits).

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as a required consumer/validation harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md** — Binds current identity to the substrate crate(s), places the real game outside the repo, requires any harness to use public interfaces only, and excludes game/System/LLM/spell/gas/combat/AI/building layers.
- **docs/seeds/product-one-seed.md** — Pins the first substrate slice and what “done” means for the playable walkable-world proof, benchmarks, and downloadable demo; supplies inclusion/exclusion contours for that first delivery without redefining product identity as the harness.
- **docs/seeds/voxel-world-substrate.md** — Authorizes full substrate outcome families (material truth and mesh-as-view, natural mutable deep-Z worlds, generation, vegetation/objects vs dressing, matter and ambient behavior, mutation-safe nav, streaming/persistence of voxels and object state, multi-game reuse without LLM) at vision altitude; mechanisms remain design.
