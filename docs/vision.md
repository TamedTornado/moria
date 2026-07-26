# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate for external games, exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world and matter foundation—not a game, not a demo product identity, and not an LLM-dependent stack. The repository also delivers a required adjacent walkable-world validation harness that uses public interfaces only.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface over continuous, fully mutable voxel truth with deep underground play, without reimplementing generation, matter, mutation, queries, or related world physics. The substrate must stand alone; game rules and any System/LLM client sit above it and are not required for product completeness.

## Product boundary

**This product owns**

- The reusable voxel-world substrate: geology-first generation, matter representation and mutation, visual meshing as a non-authoritative view, streaming, and persistence of generation truth, edit deltas, and journals for registered object, entity, and script state.
- A public command/query surface: consumers issue verbs/commands and observe via a mirror that may be stale/asynchronous, plus events; authoritative voxel truth stays behind the matter boundary.
- Substrate matter and world capabilities: full material mutability; continuous deep-Z; fluids and material interactions; granular behavior; weather/time/fire ecology; structural support and collapse; substrate-owned voxel objects with movement, breakage, re-voxelization after landing, and growth; matter-derived dressing; mutation-safe surface/Z and navigation query data.

**Adjacent, not product identity**

- A walkable-world executable is a **required repository delivery**: Moria’s first validation harness and Product One “done” artifact (playable demo through benchmark delivery, including a downloadable demo). It remains a separate public-interface consumer—not a game layer and not the product’s identity.
- The harness must use only the same public interfaces available to an external game. Its presentation, character control, cameras, authored routes, fixtures, platform/performance gates, and acceptance workloads stay harness-owned.

**Not this product**

- The actual game (downstream consumer, outside this repository’s product).
- Game rules and the System, LLM, spell, gas, combat, AI, and building *layers*; game entities and game-specific policy. Compatibility seams may be designed where substrate needs demand them; those layers are not implemented here. This does not remove substrate-owned voxel objects or their movement, breakage, re-voxelization, or growth.
- Harness-owned presentation, controls, content, fixtures, platform/performance gates, and acceptance workloads.

## Required product outcomes

- **Material world, not decorative terrain.** Consumers get a seed-derived natural world backed by voxel material truth; the render mesh is a regenerated view, not the authority for physics, queries, or mutation.
- **Geology-first natural generation.** Generation yields natural surface systems over real strata, caves, resources, water-bearing geology, and extensible placement/metadata so dig-down honesty and deep play come from the world.
- **Mutable everywhere; deep Z first-class.** Any material volume can be destroyed, changed, or placed; underground space is continuous playable volume (geology, voids, descent), not a thin floor under a heightmap.
- **Living matter substrate.** Fluids and material interactions, granular settle/collapse, weather/time and fire ecology, and structural support/collapse are required substrate outcome families. Substrate-owned voxel objects remain in the matter system (movement, breakage, re-voxelization after landing, growth); matter-derived dressing stays synchronized with voxel change.
- **Public boundary and mutation-safe queries.** Consumers integrate through verbs/commands, mirror observation that may be stale/asynchronous, and events—not private voxel paths. Surface/Z and navigation-related spatial data stay usable after edits. The required adjacent harness validates through the same public boundary. AI policy and game presentation stay consumer-owned.
- **Sparse residency and durable reuse.** GPU-resident worlds stay tractable at region scale via sparsity and streaming. Persistence recovers truth from generation, edit deltas, and journals for registered object/entity/script state so scarred worlds support cross-run reuse. Validation that exercises persistence must restore the seed-plus-deltas world exactly; actual game entities and their behavior remain consumer-owned.

## Future products and enabling implications

Future *consumers* (not current substrate scope) include a System/LLM-driven ARPG, a DF-style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate substrate generality without hard-wired game policy or LLM dependency. Their gameplay, UX, controllers, characters, content, and presentation remain consumer-owned.

**Required adjacent Product One delivery.** The walkable-world harness is the first repository delivery that proves the substrate: a curated generated natural region; dig/place proof through the public boundary; traversal/collision against voxel truth; a partial first-slice matter surface for that consumer (static fluid bodies, dressing, voxel-object placement/render—felling deferred from that first consumer slice only); streaming and persistence exercise with exact seed-plus-deltas restore; and benchmark validation. Those harness concerns stay outside substrate identity. First-slice harness depth does not narrow the substrate’s broader outcome responsibilities.

## Non-goals

- Shipping the actual game, combat, stats, AI, or game entities beyond consumer scope.
- Implementing System/LLM features, spells, gas economies, or building/game-designation layers here.
- Treating harness demo content, controls, presentation, or platform/performance gates as substrate product definition.
- A Minecraft-cube primary surface aesthetic, or a heightmap-with-props world that is not fully material underneath.
- Reading “no game entities” as exclusion of substrate-owned voxel objects or matter-derived dressing.

## Confirmed vision constraints

- **Rust crate delivery.** Consumed as a Rust crate or tightly scoped Rust crate family.
- **GPU-resident substrate.** Matter residency and related heavy work are GPU-oriented; integration is commands/verbs in and stale mirror plus events out.
- **Authoritative truth behind the matter boundary.** Consumers observe and mutate through the public surface; observation may be stale/asynchronous; they do not hold private authoritative voxel state.
- **Standalone substrate.** Zero LLM dependency; the System is a possible future client, not a substrate feature.
- **Strict consumer isolation.** Validation and games share the public interface only; privileged harness paths are disallowed.
- **Repository deliveries.** The reusable substrate is in-scope product work; the walkable-world harness is a required adjacent delivery that is not product identity; the actual game is not in-scope product.

## Deferred design decisions

- Exact crate split and internal module boundaries (beyond the required consumer/substrate separation).
- Voxel resolution, meshing approach, storage layout, streaming structure, and algorithms that implement the required matter and query outcomes.
- Implementation sequence and depth for required outcome families (fluids, granular, weather/time/fire ecology, integrity, object movement/breakage/re-voxelization/growth)—not whether they are substrate responsibilities.
- Performance budgets, target machines, graphics backends, and acceptance thresholds.
- Concrete harness controllers, content, fixtures, and platform/benchmark protocols (harness delivery is required; those details remain adjacent design).

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust crate and identifies the walkable-world executable as its separate validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Locks product identity to the substrate crate(s), excludes the game and named game/System layers, and requires any harness to use public interfaces only while remaining adjacent to the product.
- **docs/seeds/product-one-seed.md** — Pins the required first walkable validation/demo delivery and “done” shape (playable demo, benchmarks, downloadable demo); exact seed-plus-deltas restore for persistence validation; without importing demo controls, content, or gates into substrate identity.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate purpose and outcome families (geology-first generation, full mutability, deep Z, living matter including fluids/granular/weather-fire/integrity, voxel objects with movement/breakage/re-voxelization/growth, dressing, mutation-safe spatial queries, public command boundary with stale mirror plus events, persistence with object/entity/script journals and cross-run reuse) and future game consumers, without pulling mechanisms or game layers into design here.
