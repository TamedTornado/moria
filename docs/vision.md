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

- A walkable-world executable must ship here as a separate public-API consumer and validation harness. It proves generation, streaming, matter views, editing, collision against voxel truth, persistence, and performance. Character, controls, camera, seed region, presentation, and numeric gates are harness concerns, not substrate identity. Those details are established adjacent-delivery specification outside this vision’s approval boundary—not open substrate design questions.

## Required product outcomes

1. **Authoritative material world** — The voxel grid is authoritative matter. Rendered terrain is a regenerated view of that truth, not a decorative heightmap with props. Physics, collision, and other world-facing queries run against voxel truth, not the render mesh. **Categorical interactable-matter rule:** everything that can burn, break, or block is voxel-backed; only passive dressing (grass, flowers, ground clutter) may lack individual voxel identity, derived from underlying voxel state so it never desynchronizes. Interactable vegetation and objects are substrate voxel objects; the reusable mandate includes their material and physical lifecycle under mutation (including growth and falling/rigid conversion), even when the first slice places them without felling.
2. **Natural surface, mutable everywhere, deep Z** — Generated regions read as ordinary outdoor worlds while remaining fully mutable anywhere, including deep underground. Generation is geological (strata, caves, ores, aquifers, biomes) and materializes on demand so large sparse regions stay tractable.
3. **Matter-level world behavior** — Fluid bodies and flow, structural support and failure, granular settle, fire and wetness, and thin ambient ecology (day/night, season, weather, growth, fire ecology), without game pricing, win conditions, or faction logic.
4. **Public mutation and query surface** — Consumers change and inspect the world only through public verbs, queries, and events. Nothing above the substrate touches voxels by privileged side channels; the validation harness has no special access.
5. **Mutation-safe navigation across continuous 3D** — Derived navigation and traversal data stays consistent under mutation, supporting continuous three-dimensional movement rather than a 2D surface world with a painted floor.
6. **Streamable, persistent, responsive, evidenced** — Active regions stream; untouched world stays cheap; voxel edit deltas and substrate-owned object state persist so a changed world reloads as the same material state. First-slice promise also includes interactive frame behavior, bounded edit-to-remesh latency, cold-start time into a walkable world, sparse GPU residency under streaming, compact save with exact material restoration, and comparable benchmark evidence for regression checking. The adjacent harness validates these; exact thresholds, machines, and workloads remain adjacent-delivery detail.

**First substrate slice (binding delivery contour, not the full product ceiling)**

First delivery: generation at full intended depth for a curated proof region; partial matter (occupancy, regenerated matter views and dressing, static water bodies, interactable object placement without felling or rigid-body conversion); dig/place and mirror-query boundary from day one; streaming; persistence of material change; and the performance outcome family above. Excludes active cellular automata, fire simulation, flowing fluids, structural integrity, and granular settle. Outcome families above remain the eventual reusable mandate (including full object physical lifecycle); later depth and order are design choices.

## Future products and enabling implications

Downstream consumers (not this product) include a System-driven ARPG, fortress/colony play, a Moria-style descent experience, and pure sandbox modes. They motivate a reusable stack exposing the same matter operations and queries under different game policies. The System, if any, is a game-layer client—not a substrate feature. This brief does not commit their gameplay, content, controls, or release order.

## Non-goals

- Implementing game rules, combat, AI, spells, gas economy, System/LLM features, or building-as-gameplay layers in this product.
- Owning character control, camera grammar, combat feel, or other consumer presentation.
- Defining the product as the walkable demo, a specific seed region, or an audience clip.
- Any LLM dependency inside the substrate; LLM/System behavior is not a substrate feature.

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates.
- Substrate is GPU-resident for external Rust game code; load-bearing graphics stay on a portable, non-backend-specific path (wgpu/WGSL family)—not a single-API fork; cross-API portability is part of the crate’s purpose.
- Adjacent consumers, including the validation harness, use only public substrate interfaces.
- The actual game lives outside this repository’s product boundary.
- Substrate has zero LLM dependency.
- The walkable-world harness is a required current repository delivery and must remain a public-API consumer of the substrate.

## Deferred design decisions

- Capability depth and delivery order for substrate behaviors **beyond** the first-slice contour above.
- Internal crate split, storage layout, meshing approach, LOD, and exact generation pipeline stages.
- Whether richer semantic conveniences (room tags, blueprint formats) ship inside substrate crates or only as consumer libraries.
- Target machines and numeric budgets that refine the performance outcome family (the outcomes themselves are not deferred).
- Tunables that differ by game genre (support spans, fluid fidelity, object scale limits).

Harness controller, camera, curated region content, presentation contour, and acceptance gates are settled adjacent-delivery specification for that consumer. They remain outside substrate identity and are not deferred open questions for this vision.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as a required consumer/validation harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md** — Binds identity to substrate crate(s), places the real game outside the repo, requires public-API-only harness consumption, and excludes game/System/LLM/spell/gas/combat/AI/building layers.
- **docs/seeds/product-one-seed.md** — Pins the first substrate slice and walkable-world “done”; binds portable GPU path, first-slice performance outcomes, and established harness delivery details without redefining product identity as the harness.
- **docs/seeds/voxel-world-substrate.md** — Authorizes full substrate outcome families (material truth and mesh-as-view, categorical voxel-backed interactables vs passive dressing, object physical lifecycle, natural mutable deep-Z worlds, generation, matter and ambient behavior, mutation-safe nav, streaming/persistence, multi-game reuse without LLM) at vision altitude; mechanisms remain design.
