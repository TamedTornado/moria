# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for downstream games—not a game, not a demo title, and not a gameplay stack.

## Purpose

Give future games a shared material world: continuous three-dimensional terrain and geology that reads as an ordinary natural landscape while remaining fully mutable voxel matter underneath. The substrate exists so adventure, fortress, sandbox, and related games can consume the same world foundation without baking game rules, economy, or presentation into the crate.

## Product boundary

**In product**

- Reusable substrate crates and the public interfaces through which consumers generate, stream, inspect, collide with, mutate, mesh-view, and persist a voxel world.
- World-as-matter responsibilities: sparse GPU-resident storage of a material volume, geology-oriented generation with lazy materialization, derived surface views, mutation and query surfaces, and edit-aware persistence suitable for large regions.

**Out of product / adjacent**

- The actual game is a separate downstream consumer and is not part of this product’s identity or repository product scope.
- A walkable-world executable may exist as an adjacent validation harness. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific implementation paths inside the substrate. Whether that harness is a required current delivery is unresolved (see Q1). Character control, camera, curated demo routes and scenery, debug presentation, scripted benchmark scenes, and harness-specific performance gates are not substrate scope.
- Game rules and the System, LLM, spell, gas, combat, AI, and building layers are out of scope. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

Downstream design must make these product-level outcomes true:

- **Ordinary look, voxel truth.** Consumers can present a natural-looking surface world (rolling terrain, vegetation, water bodies, cliffs, underground space) while all material interaction remains grounded in the voxel volume; render meshes are views of that volume, not a separate authoritative world.
- **Mutable everywhere, all the way down.** Through public mutation and query interfaces, matter can be destroyed, altered, or placed throughout the volume so digs, scars, and fills are real world state—not props outside the material model.
- **Deep Z as content.** Underground volume is first-class: continuous 3D space with coherent subsurface structure (strata, voids, materials at depth), not a heightmap skin over empty or fake fill.
- **Geology-first generation at scale.** Worlds are produced as material geology that digs honestly; large regions remain tractable via sparse residency and lazy materialization rather than requiring the full raw volume in memory.
- **Collision and interaction against matter.** Spatial queries and collision use voxel occupancy/truth so movement and interaction match what can be dug and seen after remeshing.
- **Reusable consumer boundary.** External games—and any validation harness—integrate only through public substrate interfaces. The substrate stands alone with no LLM dependency; game policy is injected above it, not hard-wired into it.

## Future products and enabling implications

Described future products (System-driven ARPG, fortress/colony play, Moria-style descent, pure sandbox) are **downstream consumers**, not this product. They own gameplay, UX, controllers, characters, authored content, presentation, economy, and game policy.

Supported enabling implications for those consumers (not a committed feature roadmap):

- Matter and simulation hooks rich enough that later games can attach fire, flow, structural failure, granular settle, and similar rules without re-owning world storage.
- Placement and object-matter patterns that let games treat vegetation and interactable props as material, not pure decoration—without implementing game building, labor, or mechanism layers here.
- Persistence and streaming shaped so long-lived player scars and multi-mode reuse of a region are possible at the world-data level.

## Non-goals

- Shipping a playable game, campaign, or genre ruleset in this product
- System/LLM features inside the substrate
- Spells, gas metering, combat, AI, and agent labor as product scope
- Building UI, blueprints, work orders, rooms-as-gameplay, and mechanism logic
- Treating harness controls, demo scenery, or marketing milestones as substrate requirements

## Confirmed vision constraints

- **Rust crate delivery.** The product is a Rust crate or tightly scoped crate family; the integration boundary for consumers is public crate interfaces.
- **GPU-resident world substrate.** Core world residency and heavy world work are GPU-resident by product intent.
- **Strict consumer isolation.** Adjacent consumers have no privileged access paths into voxel state; harness and games share the same public surface.
- **No game-layer implementation.** Game rules and listed game systems stay out of this product even when seams are anticipated.
- **Standalone engine layer.** The substrate must not depend on an LLM or System runtime to function.

## Deferred design decisions

- Exact crate split and internal module boundaries (workspace layout beyond the consumer isolation outcome)
- Voxel resolution, meshing approach, LOD, and material payload layout
- Which matter-simulation families ship in which delivery depth, and in what order
- Streaming ring policy, persistence encoding, and quantitative performance budgets
- Multiplayer authority design beyond keeping a command/query-shaped boundary open

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required current delivery** of this repository, or only a **permitted adjacent artifact**?

- **Proposed safe answer:** Permitted adjacent artifact only—it may exist to exercise public substrate interfaces, but it is not a required deliverable of the substrate product, and none of its controls, content, or acceptance scenarios enter product scope.
- **If answered differently:** Making the harness mandatory adds a repository delivery obligation for a separate executable while product identity remains the substrate; design must still keep harness-owned presentation and scenarios out of crate scope.

## Seed synthesis

- **`README.md`** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and frames the walkable-world executable as a separate consumer/validation concern rather than a game layer.
- **`docs/seeds/project-boundary.md`** — Binding identity and exclusion: product is the substrate crates; the real game is downstream; any harness must use public interfaces; game/System/LLM/spell/gas/combat/AI/building layers stay out.
- **`docs/seeds/product-one-seed.md`** — Motivates first-slice proof of a material walkable world and dig/place honesty, but its character, route, scenery, milestones, machine targets, and demo acceptance details remain harness/consumer material, not substrate identity.
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes substrate outcome families (normal look over voxel truth, universal mutability, deep Z, geology generation, matter/query/mutation foundation, multi-game reuse) without transferring game layers or mechanism inventory into this brief.
