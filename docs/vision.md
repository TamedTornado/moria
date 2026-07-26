# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer foundation for natural, fully material worlds—not a game, demo app, or game-rules stack.

## Purpose

Downstream games need a shared world foundation that looks like ordinary continuous terrain while remaining fully mutable matter all the way underground, with generation, editing, queries, collision against voxel truth, meshing, streaming, and persistence available through public APIs. Moria exists so multiple games can share that foundation without embedding game policy, LLM features, or a privileged in-repo game path.

## Product boundary

**Belongs to Moria**

- The reusable substrate and its public consumer-facing interfaces (matter, generation, mutation/edit verbs, queries, meshing as a view of truth, streaming, persistence, collision against voxel truth).
- Integration as Rust crate(s) intended for external game consumers.
- Optional design of compatibility seams where substrate requirements demand them—without implementing game layers.

**Does not belong to Moria**

- The actual game (any title), game rules, and future System, LLM, spell, gas, combat, AI, and building layers.
- Controllers, cameras, characters, authored demo routes, presentation polish, and game-specific policy—these remain consumer-owned.
- A walkable-world executable, if present in the repository, is an **adjacent validation harness**, not the product identity. It must consume the substrate through the same public interfaces available to an external game (no privileged or game-specific implementation paths). Whether that harness is a required repository delivery is unresolved; see Q1. Until answered, this brief only records that such a harness **may** exist as an adjacent artifact.

## Required product outcomes

Downstream design must make these product-level outcomes true:

1. **Reusable GPU-resident substrate** — A Rust crate stack other games consume; game rules live above; no LLM dependency; core world residency is GPU-resident.
2. **Natural continuous look over full voxel truth** — Worlds read as ordinary terrain (hills, forest, water, cliffs, caves), not a primary cube aesthetic; the voxel field is the authoritative material world, mutable anywhere including deep underground.
3. **Geology-capable generation at scale** — Generation yields dig-honest geology (strata, caves, ores, related subsurface structure), not a heightmap with painted rock; large regions remain practical through sparse, demand-driven materialization.
4. **Public mutation, query, and collision** — Consumers dig, place, inspect, and collide against voxel truth only through public interfaces; no privileged direct voxel path. Dig/place prove material mutability; they are not a game building layer.
5. **Mesh as non-authoritative view; matter-backed surface life** — Rendered geometry regenerates from voxels (smooth terrain; cuts read as real material faces). Interactable vegetation/clutter is matter-backed or dressing driven by voxel state; physics and queries use voxel truth. Structural and fluid-capable matter behavior belong to the substrate at outcome altitude; delivery depth is design.
6. **Streaming and persistence** — Activity-centered streaming; truth is generation plus edit deltas so scars persist without storing untouched volume as full raw grids.

## Future products and enabling implications

Future **consumers** (not this product) include a System/LLM ARPG, fortress/colony-style play, Moria-style descent, pure sandbox, and any external game built on the crates. Those products own gameplay, content, controllers, UX, and policy.

Enabling implications only: keep public mutation/query seams and matter/physics outcomes general enough that such games can sit above without forking privileged world paths; do not import their systems, spells, economy, or building gameplay into Moria.

A walkable-world harness is an adjacent consumer used to exercise generation, streaming, meshing, editing, collision, persistence, and performance—not a future game. Its specific content and controls are not product scope; its delivery obligation is Q1.

## Non-goals

- Implementing the game, game rules, System/LLM, spells, gas policy, combat, AI, or building layers (blueprints, work orders, mechanisms-as-gameplay, room economies).
- Treating a demo character, seed postcard route, trailer presentation, or audience milestone plan as product identity.
- Making the validation harness’s platform, hardware profile, benchmark scene, or performance gates into the substrate’s portable product promise without a separate product-authority decision.

## Confirmed vision constraints

- Product identity is the reusable voxel-world substrate as Rust crate(s); the game lives outside this product/repository boundary.
- Any in-repo walkable harness, if present, uses only public substrate interfaces shared with external games—no privileged harness path.
- The substrate must stand alone with zero LLM dependency.
- GPU-resident substrate character is part of product identity.
- Compatibility seams for out-of-scope layers may be designed; those layers must not be implemented here.

## Deferred design decisions

- Crate/family split and packaging layout (workspace mechanics are design; the consumer boundary is not).
- Algorithms, voxel/brick sizing, LOD, data layouts, meshing method choice, and simulation tier depth/order.
- How far first deliveries go within each outcome family (e.g. fluid or integrity depth)—sequence is design, not identity.
- Harness-only choices: controller, camera, seed region content, debug presentation, target machines, and numeric performance gates.
- Persistence encodings, streaming ring policy, and multiplayer readiness beyond the public verb/command boundary already implied by reusable APIs.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed answer:** Permitted only—the product is the substrate crates; a harness may exist later or alongside to validate public APIs, but shipping it is not required to satisfy product identity.
- **If different:** Making it mandatory adds a required adjacent delivery (still not game-layer ownership) and would bind planning to produce some walkable harness that uses public interfaces; it would not move controllers, content, or performance gates into substrate identity unless separately decided.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident Rust voxel-world substrate and positions the walkable executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Settles current product identity (substrate crates), excludes the game and listed game layers, and binds any harness to public interfaces only.
- **docs/seeds/product-one-seed.md** — Motivates first validation concerns (natural walkable region, dig/place proof, generation/meshing/streaming/persistence) as consumer/harness detail; does not redefine product identity or import demo content into substrate scope.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural mutable worlds, deep Z, geology generation, matter/physics/query/mutation base, meshing as view, streaming/persistence, multi-game reuse) at design-detail altitude; mechanisms stay for later design.
