# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a Rust crate (or small family of tightly scoped crates). Downstream games and tools consume it; it is not itself a game.

## Purpose

Moria exists so multiple games—and validation artifacts—can share one material world foundation: a natural-looking surface world over fully mutable voxel truth, with deep underground play, generation, active matter behavior, queries, and mutation, and with **zero LLM/System dependency**. Game rules live above the substrate; the substrate provides the world they run on.

## Product boundary

**In product**

- The reusable voxel-world substrate and its public consumer-facing surface (crate API and related public interfaces).
- Substrate responsibilities: world generation; sparse GPU-resident matter; smooth visual reconstruction; mutation and query verbs; mutation-consistent traversal information; active matter behavior (granular settling, disturbed fluid flow and material interaction, thin ambient time/weather/ecology responses, material-dependent structural failure); and persistence/streaming for world and object truth that also accommodates downstream entity and script state.

**Adjacent, not identity**

- A walkable-world validation executable is an adjacent public-API consumer, not substrate identity. Product One pins it as the first delivery that proves the substrate: a generated natural region; continuous third-person traversal of surface and deep volume; dig/place as mutability proof; a bounded substrate slice (static water only—no flow sim—in that first slice); and benchmark plus persistence validation. It must use the same public interfaces available to an external game, without privileged substrate paths. First-slice bounds do not make full-product active matter optional for the reusable substrate.
- The actual game (or games) are separate downstream consumers and are **not** part of this repository.

**Out of product**

- Game rules; System/LLM features; spells; gas/pricing policy; combat; AI; building/gameplay layers (including building UX, fortress work orders, and game-specific presentation or controls); harness-owned content, characters, cameras, controllers, routes, and acceptance scenarios; and entity content or game script logic (persistence may journal them; the substrate does not implement them).

## Required product outcomes

A downstream design must make these product-level outcomes true:

1. **Material world as truth.** Any cell can be destroyed, moved, or placed; decorative-only geometry is not play or collision authority. The rendered surface is a regenerated view of voxel matter, not the authority.
2. **Reads as a continuous natural world.** Rolling terrain, vegetation, water bodies, cliffs, and similar surface reading without forcing a blocky cube aesthetic as the primary look; player-made cuts and sharp features remain legible as cuts and structures.
3. **Deep Z is first-class.** The underground is real continuous volume—strata, caves, ores, aquifers, deep space—not a painted floor under a heightmap.
4. **Geology-first generation with cheap idle cost.** Worlds are produced as geology and related volume (not heightmap-plus-paint), materializable on demand so large regions need not reside fully as dense voxels until touched.
5. **Public mutation/query boundary; shared objects and traversal.** Consumers change and inspect the world only through public verbs and queries; nothing above the matter core reaches voxels by privileged back doors. Gas/cost policy is consumer-injected. Substrate-owned voxel objects (trees, boulders, and similar—distinct from pure dressing that still tracks underlying matter) stay in the material model through state changes, breakage, and movement. Consumers obtain traversal information from voxel truth, invalidated by mutation, for continuous three-dimensional movement classes; controllers, AI, and pathfinding policy stay consumer-owned.
6. **Active matter behavior and complete persistence/streaming.** The substrate itself provides consumer-visible active matter: granular settling; disturbed fluid flow and material interaction; thin-but-present ambient time, weather, and ecology responses; and material-dependent structural failure. Product One’s first slice may bound fluids to static bodies and defer running these sims; that bound does not make active matter optional for the reusable product. Solver mechanisms and fidelity remain deferred. Authoritative truth is regenerable world definition plus edit deltas for touched voxels, with regional journals for voxel objects and for entities carrying script state so reload restores the same material and journaled situation and deltas remain reusable across runs or modes. Persistence accommodates that entity/script state alongside material and object truth without moving game logic or entity content into the substrate. Active regions stream so large editable worlds stay workable.

## Future products and enabling implications

Future **consumers** (not this product) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. Public mutation, query, and observation boundaries exist so those games can attach without forking world truth.

Enabling implications (not a committed multi-game roadmap): seams and registries so a later game or System can author materials, placements, and structures without embedding that game; column/Z conveniences for fortress-style views; and multiplayer-ready verb/query shape. Gameplay, content, UX, and policy remain consumer-owned.

The walkable-world first slice is the tech-proven demo path; harness character, camera, route, debug presentation, and numeric gates do not redefine product identity.

## Non-goals

- Implementing the System, LLM hooks as product features, spells, gas economies, combat, or agent AI.
- Implementing building/gameplay layers (blueprints-as-gameplay, work orders, mechanisms-as-game systems, room economy).
- Shipping the eventual commercial game inside this repository.
- Treating harness-specific content, controls, or benchmark theater as substrate identity.
- Owning entity content or game script logic (journals may record them; the substrate does not implement them).

## Confirmed vision constraints

- **Ecosystem:** delivered as Rust crate(s) for Rust consumers.
- **GPU-resident** voxel-world substrate (residency model promise, not a particular GPU vendor or API).
- **Standalone:** no LLM/System dependency in the substrate.
- **Consumer isolation:** adjacent executables and external games share one public interface class; no privileged harness path into matter.
- **Repository boundary:** the actual game is out of this repo; the walkable-world executable is an adjacent public-API validation consumer.
- **First slice ≠ full product:** Product One’s deferred matter sims and static-water-only fluid scope bound the adjacent first delivery, not the reusable substrate’s required active-matter and persistence outcomes.

## Deferred design decisions

- Exact crate family split and internal module boundaries (beyond the non-optional consumer boundary).
- Voxel resolution, LOD strategy, object-registry scaling, and fluid/integrity/ambient solver fidelity and mechanisms.
- Sequencing and depth within the first substrate slice versus later delivery of full active matter.
- Harness/demo content, controls, platforms, and numeric performance gates.
- Whether and when multiplayer is built (architecture may remain ready without committing delivery).
- Encoding and API shape for entity/script journals (substrate must accommodate them; design chooses form).

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and frames the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the substrate crate(s), excludes the game and listed game layers from the repo, and requires any harness to use public interfaces only.
- **docs/seeds/product-one-seed.md** — Specifies the adjacent first-slice delivery (generated natural region, continuous third-person traversal, dig/place proof, bounded slice with static water only, benchmark and persistence validation) without making full-product active matter optional or moving harness content into substrate identity.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate outcomes: natural look, full mutability, deep Z, generation, active matter (granular settle, fluid flow and interaction, ambient time/weather/ecology, structural failure), object lifecycle, mutation-consistent traversal, and persistence/streaming including regional object and entity/script journals with cross-run/mode-reusable deltas—plus future consumers without a multi-game delivery plan.
