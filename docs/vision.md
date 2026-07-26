# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** for external games to consume as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer world of matter—not a game, demo product, or gameplay stack.

## Purpose

Give downstream games a shared foundation for a continuous, fully material 3D world: natural-looking terrain whose authoritative truth is a mutable voxel volume, including deep underground space. Game rules, content authorship, and presentation live above this layer so the same substrate can support multiple game styles without embedding any one of them.

## Product boundary

**In product**

- The substrate’s public capability surface: world representation, generation hooks, non-authoritative visual meshing, material mutation (including dig/place), occupancy useful for collision and queries, streaming-scale residency, and persistence of edits—expressed for Rust consumers.
- Integration only through public interfaces that an external game could use.

**Adjacent, not identity**

- A walkable-world executable **may** exist in this repository as a validation harness. It is not the product. It must not own privileged or game-specific paths. Whether shipping that executable is a required current delivery is **Q1**; until answered, treat it only as a permitted adjacent artifact, not as optional or mandatory product scope.
- Specific harness character controls, cameras, demo routes, seed-region content, presentation, and performance gates belong to that adjacent artifact, not to Moria’s identity.

**Out of product / repository**

- The actual game is a separate downstream consumer and is not part of this repository.
- Game rules and the System, LLM, spell, gas, combat, AI, and building **layers** are out of scope. Compatibility seams may be designed where substrate needs demand them; those layers must not be implemented here.

## Required product outcomes

- **Rust-consumable substrate.** External games integrate Moria as public crate API(s), with no need for in-tree privileges.
- **Material world truth.** The world is a fully mutable voxel volume: matter can be destroyed, moved, or placed through the substrate’s mutation surface; decorative geometry outside that truth is not the model.
- **Natural surface, continuous volume.** The product supports a surface that can read as ordinary outdoor terrain while remaining voxel-backed, and treats deep Z (caves, strata, underground volume) as first-class space rather than a flat floor under a skybox.
- **View vs truth.** Consumers can obtain a generated mesh (or equivalent view) for rendering; gameplay-relevant physics and queries run against voxel occupancy/truth, not against a frozen mesh as authority.
- **Generation and scale.** The substrate supports generating natural regions (terrain and geology-class structure) at scales where the full volume need not reside as dense raw voxels, so streaming-oriented use is meaningful.
- **Edits that last.** Touched world state can persist and reload relative to generation so material scars and placements survive sessions at the substrate level.

## Future products and enabling implications

Future **consumers** (not this product) include a System/ARPG-style game, fortress/colony-style play, descent/adventure modes, and pure sandboxes. They own gameplay, UX, controllers, characters, authored content, pricing policy, and presentation.

High-level enabling implications already sketched for the substrate (delivery depth and sequence are design): richer fluid and granular behavior, structural integrity and collapse, fire/ambient aggregate simulation, voxel-backed vegetation objects with physical felling, mechanism-like entities, and semantic helpers (nav aggregates, room-like regions). These motivate long-horizon substrate richness; they are not a committed feature roadmap in this brief.

## Non-goals

- Shipping a game, combat loop, stats, AI agents, or multiplayer service in this repository.
- Implementing System/LLM, spells, gas/pricing policy, or game building UX (blueprints-as-gameplay, work orders, economy).
- Treating the validation harness’s demo content, third-person fantasy, or benchmark numbers as product requirements.
- Making the substrate depend on an LLM to function.

## Confirmed vision constraints

- **Ecosystem:** intended consumption is as Rust crate(s) in a Cargo ecosystem; the in-repo harness, if present, is separated from reusable substrate code at the consumer boundary (exact package layout is design).
- **API symmetry:** any in-repo harness uses the same public interfaces available to an external game.
- **Standalone engine layer:** zero required LLM/System dependency for core world operation.
- **Layer ownership:** gas/pricing and game policy are consumer-injected concepts, not hard-wired substrate game modes.

## Deferred design decisions

- First delivery depth and milestone order (how much geology, matter simulation, vegetation, fluids, integrity, and persistence land in the initial cut).
- Representation and runtime choices (voxel resolution, brick/storage scheme, meshing approach, streaming rings, delta encoding).
- Exact public API shape, crate family split, and any compatibility seams for future game layers.
- Whether and how far multiplayer-oriented command/mirror patterns are taken in early design.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world executable a **required current repository delivery** beside the substrate crates, or only a **permitted** adjacent validation artifact?

- **Proposed safe answer:** Permitted only—the product’s identity is the public substrate crates; a same-interface harness may be added for validation but is not required for current-product completeness.
- **If different:** Making the executable mandatory adds a repository delivery obligation (design must plan a harness that exercises public APIs) without changing Moria’s identity as substrate-not-game; it still must not import harness controls, content, or performance gates into product outcomes.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident Rust-crate substrate and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding boundary—Rust crate product, game out of repo, harness-only executable if present, public-API-only access, and exclusion of game/System/spell/gas/combat/AI/building layers.
- **`docs/seeds/product-one-seed.md`:** First-slice / demo narrative that motivates substrate proof points and harness behavior; its controllers, region script, and targets inform adjacent validation and delivery planning, not product identity expansion.
- **`docs/seeds/voxel-world-substrate.md`:** Long-horizon world-layer purpose (material truth, natural surface, deep Z, multi-game substrate) and enabling richness; mechanism inventories and build order remain design inputs, not this brief’s mandate list.
