# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate for external games, exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world and matter foundation—not a game, not a demo product identity, and not an LLM-dependent stack.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface over continuous, fully mutable voxel truth with deep underground play, without reimplementing generation, matter, mutation, queries, or related world physics. The substrate must stand alone; game rules and any System/LLM client sit above it and are not required for product completeness.

## Product boundary

**This product owns**

- The reusable voxel-world substrate: geology-first generation, matter representation and mutation, visual meshing as a non-authoritative view, streaming, and persistence of matter plus substrate-owned object/lifecycle state.
- A public command/query surface: consumers issue verbs/commands and receive mirror queries plus events; authoritative voxel truth stays behind the matter boundary.
- Substrate matter and world capabilities: full material mutability; continuous deep-Z; fluids and material interactions; granular behavior; weather/time/fire ecology; structural support and collapse; substrate-owned voxel objects and matter lifecycles; matter-derived dressing; mutation-safe surface/Z and navigation query data.

**Adjacent, not product identity**

- A walkable-world executable may exist here as a validation harness—a separate consumer, not a game layer and not the product. Whether that already-defined adjacent harness is a required repository delivery is open (Q1). While open, this brief does not treat it as required, optional, or planned delivery—only as a permitted adjacent artifact (Product One shape under Future products).
- If present, the harness must use only the same public interfaces available to an external game.

**Not this product**

- The actual game (downstream consumer, outside this repository’s product).
- Game rules and the System, LLM, spell, gas, combat, AI, and building *layers*; game entities and game-specific policy. Compatibility seams may be designed where substrate needs demand them; those layers are not implemented here. This does not remove substrate-owned voxel objects or their movement, breakage, growth, or restoration lifecycles.
- Harness-owned presentation, character control, cameras, authored demo routes, fixtures, platform/performance gates, and acceptance workloads.

## Required product outcomes

- **Material world, not decorative terrain.** Consumers get a seed-derived natural world backed by voxel material truth; the render mesh is a regenerated view, not the authority for physics, queries, or mutation.
- **Geology-first natural generation.** Generation yields natural surface systems over real strata, caves, resources, water-bearing geology, and extensible placement/metadata so dig-down honesty and deep play come from the world.
- **Mutable everywhere; deep Z first-class.** Any material volume can be destroyed, changed, or placed; underground space is continuous playable volume (geology, voids, descent), not a thin floor under a heightmap.
- **Living matter substrate.** Fluids and material interactions, granular settle/collapse, weather/time and fire ecology, and structural support/collapse are required substrate outcome families. Substrate-owned voxel objects remain in the matter system (movement, breakage, growth, restoration); matter-derived dressing stays synchronized with voxel change.
- **Public boundary and mutation-safe queries.** Consumers integrate through verbs/commands, mirror queries, and events—not private voxel paths. Surface/Z and navigation-related spatial data stay usable after edits. Adjacent validation uses the same public boundary. AI policy and game presentation stay consumer-owned.
- **Sparse residency and durable reuse.** GPU-resident worlds stay tractable at region scale via sparsity and streaming. Persistence recovers truth from generation, edit deltas, and substrate-owned object/lifecycle journals so scarred worlds support cross-run reuse.

## Future products and enabling implications

Future *consumers* (not current substrate scope) include a System/LLM-driven ARPG, a DF-style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate substrate generality without hard-wired game policy or LLM dependency. Their gameplay, UX, controllers, characters, content, and presentation remain consumer-owned.

**Adjacent Product One slice (delivery status: Q1).** Seeds already define the walkable-world harness shape if delivered: a curated generated natural region; dig/place proof through the public boundary; traversal/collision against voxel truth; a partial matter slice (static fluid bodies, dressing, voxel-object placement/render—felling deferred from that first consumer slice only); streaming and persistence exercise; and benchmark validation. Those harness concerns stay outside substrate identity. While Q1 is open, do not treat this slice as required, optional, or planned delivery—only as the already-specified adjacent consumer shape.

## Non-goals

- Shipping the actual game, combat, stats, AI, or game entities beyond consumer scope.
- Implementing System/LLM features, spells, gas economies, or building/game-designation layers here.
- Treating harness demo content, controls, presentation, or platform/performance gates as substrate product definition.
- A Minecraft-cube primary surface aesthetic, or a heightmap-with-props world that is not fully material underneath.
- Reading “no game entities” as exclusion of substrate-owned voxel objects or matter-derived dressing.

## Confirmed vision constraints

- **Rust crate delivery.** Consumed as a Rust crate or tightly scoped Rust crate family.
- **GPU-resident substrate.** Matter residency and related heavy work are GPU-oriented; integration is commands/verbs in and mirror queries plus events out.
- **Authoritative truth behind the matter boundary.** Consumers observe and mutate through the public surface; they do not hold private authoritative voxel state.
- **Standalone substrate.** Zero LLM dependency; the System is a possible future client, not a substrate feature.
- **Strict consumer isolation.** Validation and games share the public interface only; privileged harness paths are disallowed.
- **Repository product focus.** The reusable substrate is in-scope product work; the game is not.

## Deferred design decisions

- Exact crate split and internal module boundaries (beyond the required consumer/substrate separation).
- Voxel resolution, meshing approach, storage layout, streaming structure, and algorithms that implement the required matter and query outcomes.
- Implementation sequence and depth for required outcome families (fluids, granular, weather/time/fire ecology, integrity, object lifecycles)—not whether they are substrate responsibilities.
- Performance budgets, target machines, graphics backends, and acceptance thresholds.
- Concrete harness controllers, content, fixtures, and platform/benchmark protocols if a walkable-world executable is delivered (Q1).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the already-defined walkable-world / Product One validation harness a **required repository delivery** alongside the substrate crates, or only a **permitted** adjacent artifact?

- **Proposed answer:** Permitted only—product identity and success are defined by the substrate crates and public boundary; a harness may exercise that boundary but is not required for product completeness.
- **If different:** Making the executable mandatory adds a repository delivery commitment for that adjacent consumer without changing substrate identity; its demo content, controls, presentation, workloads, and platform gates still stay outside product scope.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust crate and situates the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Locks product identity to the substrate crate(s), excludes the game and named game/System layers, and requires any harness to use public interfaces only.
- **docs/seeds/product-one-seed.md** — Defines the adjacent first walkable validation/demo consumer slice and “done” shape; records that conditional harness handoff and public dig/place proof obligations without importing demo controls, content, or gates into substrate scope.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate purpose and outcome families (geology-first generation, full mutability, deep Z, living matter including fluids/granular/weather-fire/integrity, voxel objects and dressing, mutation-safe spatial queries, public verb/mirror boundary, persistence with object journals and cross-run reuse) and future game consumers, without pulling mechanisms or game layers into design here.
