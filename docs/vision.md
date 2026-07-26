# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation—not a game, not a game ruleset, and not a presentation shell for a particular title.

## Purpose

Moria exists so multiple downstream games can stand on one shared, material world: a natural-looking surface and deep underground volume whose voxels are the authoritative truth for space, matter, mutation, and queries. Games supply rules, content, and presentation above it; the substrate provides the world they inhabit without depending on any one game’s policy, UI, or LLM features.

## Product boundary

**Belongs to Moria**
- The reusable world substrate: generation of natural geology and terrain, resident mutable matter, derived surface presentation of that matter, public mutation and query surfaces, streaming of active regions, and persistence of generated truth plus edits.
- Integration as a Rust library surface that external consumers link against.
- Compatibility seams only where substrate outcomes require them (hooks and extension points, not implementations of game systems).

**Does not belong to Moria**
- The actual game and its repository contents: rules, combat, AI, economy, spells, gas/pricing policy, the System/LLM layer, and building-as-gameplay (UI, work orders, fortress/ARPG policy).
- Authored game content, characters, controllers, cameras, routes, and demo presentation.
- A walkable-world executable, if present, is an adjacent consumer and validation harness—not part of product identity. It may exist in-repo solely to exercise the substrate; whether it is a required delivery is open (see Q1). If it exists, it must use only the same public interfaces available to an external game—no privileged or game-specific substrate paths.

## Required product outcomes

A downstream design must make these product-level outcomes true:

1. **Natural world from voxel truth** — Surface terrain reads as a continuous, natural world (hills, forests, water, cliffs, meadows). The voxel grid is authoritative matter, not the primary cube aesthetic; derived meshes and dressing are views of that truth.
2. **Mutable matter everywhere** — Any world volume in scope can be destroyed, placed, or otherwise changed through substrate verbs. Cuts and builds remain honest matter; presentation regenerates from truth and is never the authority for physics or gameplay queries.
3. **Deep Z as first-class content** — Underground space is real playable volume: strata, caves, ores, aquifers, and depth variation are generated and queryable as matter, not a decorative floor under a heightmap.
4. **Geology-first, sparse residency** — Worlds are generated as geology (not heightmap-plus-paint) so digging and descent remain coherent. Untouched volume stays cheap via lazy materialization and sparse residency so large regions are viable without loading everything as dense voxels.
5. **Public verb-and-query boundary** — Consumers mutate and inspect the world only through the substrate’s public interfaces. Nothing above the matter surface needs direct privileged voxel access; that boundary is the reuse and isolation seam shared by external games and any in-repo harness.
6. **Streaming, persistence, and matter-backed surface life** — Active regions stream in and out; persistence is worldgen identity plus edit deltas (and related object/entity change journals where the substrate owns them). Interactive surface features that can burn, break, or block are matter-backed; non-blocking clutter remains a pure function of matter so it cannot desync from the world.

Substrate-owned physical world services that enable multiple game modes—still and flowing fluids, structural support and collapse, granular materials, and thin ambient environment response—remain product outcome families at vision altitude. Delivery depth and order for those services are design choices, not a narrower product identity.

## Future products and enabling implications

Future consumers (separate products) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony mode, a Moria-style descent adventure, and pure sandbox titles. They motivate substrate outcomes (deep mutable geology, material honesty, clean APIs) but do not import their gameplay, content, controllers, characters, presentation, or acceptance scenarios into Moria. Enabling implication only: the substrate must remain game-agnostic and usable without LLM dependency so those titles can attach later.

## Non-goals

- Shipping a game, game rules, or game-layer systems (System/LLM, spells, gas policy, combat, AI, building gameplay).
- Owning harness- or demo-specific characters, third-person controls, cameras, authored demo routes, milestone marketing artifacts, or consumer performance gates.
- Implementing excluded layers “for later convenience” inside the substrate crates.

## Confirmed vision constraints

- Product form: Rust crate or small family of tightly scoped Rust crates for external consumption.
- World residency model: GPU-resident substrate as product character, not a later optional port.
- Isolation: substrate stands alone with zero LLM/System dependency.
- Consumer equality: any in-repo validation executable is a peer of external games at the public API; privileged internal paths for “the demo” are forbidden.
- Explicit exclusions: game rules and future System, LLM, spell, gas, combat, AI, and building layers are out of scope to implement here; seams only where substrate requirements demand them.

## Deferred design decisions

- Crate/package split and workspace layout that enforce the consumer boundary.
- Concrete generation pipeline stages, meshing/extraction strategy, voxel scale, LOD, object-layer scaling, fluid fidelity tiers, integrity granularity, and ambient-sim scope.
- Depth and sequence of substrate services in any given release (including how much of fluids, integrity, fire, or vegetation interactivity ships first).
- Harness-only choices if a walkable executable is built: controller, content set, benchmarks, target machines, and acceptance numbers.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation executable a **required current delivery** alongside the substrate crates, or only **permitted** as an adjacent in-repo artifact when useful?

- **Proposed safe answer:** Permitted only—not a required current delivery. Product identity and success criteria center on the reusable substrate; a harness may be added to validate public APIs but is not mandated by this vision.
- **If answered differently:** Making the harness mandatory adds a current-delivery obligation for a walkable consumer binary (still outside product identity, still public-API-only) without importing its controls, content, or performance gates into Moria itself.

## Seed synthesis

- **`README.md`** — Names Moria as the reusable GPU-resident voxel-world substrate consumed as a Rust crate, and positions the walkable-world executable as a separate validation consumer rather than a game layer.
- **`docs/seeds/project-boundary.md`** — Binding identity and boundary: substrate as Rust crate(s); game out of repo; harness may exist only as peer consumer of public APIs; game/System/building layers excluded with seams allowed only as needed.
- **`docs/seeds/product-one-seed.md`** — Adjacent first-slice/demo intent for a walkable proof harness (content, controller, performance, milestones). Motivates substrate mutability and material honesty; does not redefine product identity or transfer demo ownership into the substrate.
- **`docs/seeds/voxel-world-substrate.md`** — Authoritative substrate outcome families: natural look from voxel truth, full mutability, deep-Z geology, generation and sparse residency, matter-backed world services, public layering, streaming/persistence, and multi-game reuse without LLM dependency. Mechanisms and build order stay downstream.
