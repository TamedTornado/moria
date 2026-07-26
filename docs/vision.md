# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** consumed as a Rust crate (or a small family of tightly scoped Rust crates). It is the world layer—not a game—that future games integrate through public interfaces.

## Purpose

Moria exists so adventure, fortress, sandbox, and related games can share one foundation: a natural-looking surface world over fully mutable voxel matter, with deep underground as first-class space. Game rules, economy, combat, AI, and authoring policy stay above the substrate. The substrate supplies matter, generation, presentation of voxel truth, queries, and mutation so each game need not re-implement the world.

## Product boundary

**Belongs to Moria**
- The reusable world substrate and its public consumer-facing surface (Rust crate(s)).
- World responsibilities at product altitude: geology-first generation; material voxel truth; smooth non-authoritative presentation of that truth; mutation and query surface; matter-backed interactable objects and matter-driven surface dressing; fluid and solid-matter behavior the world layer owns; persistence as worldgen plus edit deltas; streaming of large regions.
- Compatibility seams only where substrate requirements demand them—not implementations of game layers.

**Does not belong to Moria**
- The actual game product(s); they are separate downstream consumers, not this repository’s product.
- Game rules and the System / LLM, spell, gas, combat, AI, and building **gameplay** layers (player blueprints, work orders, room economy, game-specific pricing).
- Harness- or demo-owned character control, camera, authored routes, UI/debug presentation, fixture scenarios, and performance or acceptance gates of any adjacent walkable-world executable.

A walkable-world executable **may** exist in-repo as an adjacent validation harness that consumes the substrate only through the same public interfaces available to an external game. Whether that harness is a required repository delivery is open (see Q1). It is not part of product identity and does not import its controls, content, or gates into substrate scope.

## Required product outcomes

- **Material world truth.** Consumers get a fully material voxel world: any voxel can be destroyed, moved, or placed; dig and place are first-class; the world is not decorative geometry outside matter.
- **Looks normal, runs on voxels.** Surface worlds read as ordinary terrain (land, forest, water, cliffs, geology), not a cube aesthetic. Presentation is a regenerated view; interaction and queries use voxel truth.
- **Deep Z is content.** Underground volume is playable space—caves, strata, ore, aquifers, depth—not a flat floor under a skybox.
- **Geology-first generation.** Worlds generate as layered geology and natural features so digging reveals true structure; large regions can materialize lazily rather than as fully resident raw voxels.
- **Honest surface and matter behavior.** Interactable vegetation and micro-objects participate as matter-backed world objects where interaction matters; lighter clutter stays driven by voxel/surface data under dig and similar change. Static water bodies and further fluid/solid simulation the substrate owns remain world capabilities (delivery depth is design).
- **Reusable public engine surface.** Consumers mutate and inspect the world only through public verbs, queries, and events—no privileged paths. The same stack can underpin ARPG, fortress, descent, or sandbox games without those rules living in Moria. Saves are worldgen plus edit deltas; large worlds stream by activity.

## Future products and enabling implications

Future consumers (not current product): a System/LLM-driven ARPG, a fortress or colony game, a Moria-style descent experience, and pure sandbox modes. They own gameplay, content, controllers, presentation policy, and economy.

Supported enabling implications (not a committed roadmap of modules): mutability and structural honesty at surface and depth; fluid and integrity behavior sufficient for later hydrological and engineering play; object/matter coupling usable for felling, collapse, and similar; metadata and placement hooks so higher layers author content without owning geology. Weather, multiplayer command patterns, and semantic room/economy features stay consumer-side or later design unless a thin non-policy seam is required.

## Non-goals

- Implementing any full game, combat, stats, AI, or game entities.
- System / LLM integration, spells, gas metering, or intent/pricing policy as product features.
- Building-game UX, player blueprints, mechanisms-as-gameplay, or fortress labor/economy.
- Treating a walkable-world harness’s character, route, art direction, or benchmark scene as substrate requirements.

## Confirmed vision constraints

- **Ecosystem:** a Rust crate or small family of tightly scoped Rust crates for Rust consumers.
- **GPU-resident** world substrate as the intended execution model for the matter world.
- **Strict consumer boundary:** any in-repo harness and any external game use the same public interfaces; adjacent consumers have no privileged internal access.
- **Zero LLM dependency:** the substrate stands alone; the System is a future game-layer client, not a substrate feature.
- **Layering force:** game policy is not implemented in Moria; only substrate-demanded compatibility seams may be designed.

## Deferred design decisions

- Crate split, APIs, algorithms, storage, meshing, voxel size, LOD, streaming policy, and persistence encoding.
- How much generation detail, matter simulation (including fluids beyond static bodies, integrity, granular settle, fire), and object-physics coupling ships in which delivery slice.
- Performance budgets, target hardware, graphics backends, and validation workloads.
- Exact seams for future fortress/ARPG needs without pulling game layers in-repo.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world validation harness a **required repository delivery** for the current effort, or only **permitted** as an adjacent artifact?

- **Proposed safe answer:** Permitted only—identity stays the substrate; design may still add a harness that uses public APIs only.
- **If answered “required”:** the repository must ship a harness that exercises the public boundary, but harness-specific controls, content, presentation, routes, and performance gates remain adjacent—not substrate outcomes.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the substrate crate(s), excludes the actual game and named game layers from the repo, and requires any harness to use the same public interfaces as external games.
- **docs/seeds/product-one-seed.md** — Motivates a demo-shaped proof of a material walkable world and dig/place honesty; harness content, controls, and gates stay adjacent or design-time, not automatic substrate scope.
- **docs/seeds/voxel-world-substrate.md** — Supplies world-layer purpose and high-level substrate responsibilities (material truth, natural look, deep Z, generation, matter behavior, persistence/streaming, reusable layering) that ground required outcomes without importing game features.
