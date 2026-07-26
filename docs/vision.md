# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate or a small family of tightly scoped Rust crates. It is the world-and-matter foundation later games consume—not a game, campaign, or playable title in its own right.

## Purpose

Give future games and tools one shared material world: natural-looking continuous terrain and deep underground volume, fully mutable voxel truth, and equal public interfaces for generation, query, mutation, streaming, collision, and persistence—without baking any one game’s rules into the engine layer.

## Product boundary

- **This product:** the reusable substrate—GPU-resident voxel world representation, geology-oriented generation, meshing/view of voxel truth, public mutation and query surfaces (including dig and place as matter verbs), streaming, collision against voxel authority, and edit persistence.
- **Adjacent consumer:** a walkable-world executable may exist only as a validation harness. It must call the same public interfaces an external game would use and must not own privileged or game-specific substrate paths.
- **Not this product / not this repository:** the actual game; game rules; System/LLM features; spells; gas; combat; AI; and building layers as game systems (blueprints, work orders, mechanisms-as-gameplay, designation UX).
- Compatibility seams may be designed where substrate needs require them; those higher layers are not implemented here.
- Character control, camera, demo routes, authored set dressing, presentation, acceptance scenes, and game-specific policy stay with consumers or the harness—not with substrate identity.
- Workspace packaging may separate substrate from harness; that enforcement detail is design. The consumer-equality boundary is product law.

## Required product outcomes

- A downstream team can depend on Moria as Rust crate(s) and integrate a GPU-resident voxel world without pulling in game logic or an LLM runtime.
- Generated regions read as natural surface worlds (terrain, water bodies, vegetation-capable surface) over continuous deep-Z volume—geology and caves as content, not a skybox floor or heightmap with disconnected props.
- The voxel grid is the authority for matter, collision, queries, and mutation; any mesh or dressing is a regenerated view, not saved truth.
- Consumers can destroy, alter, or place material through public surfaces so the world is demonstrably mutable matter everywhere in scope.
- Worlds support streaming and persistence of edits so touched volume can be reloaded as the same material truth.
- Adjacent validation may exercise generation, streaming, meshing, editing, collision, persistence, and performance through those public interfaces without expanding harness content into product scope.

## Future products and enabling implications

Future consumers include a System-driven ARPG, fortress/colony play, a Moria-style descent adventure, and pure sandboxes. They own rules, content, UX, controllers, and policy.

Substrate-facing enabling implications (not a committed delivery roadmap): deeper fluid behavior, structural integrity and collapse, fire and ambient ecology, interactable vegetation and rigid-matter coupling, and optional semantic hooks (for example navigation or room metadata) when games need them. An excluded game building layer is not reintroduced as substrate work merely because a fortress consumer would want it later.

## Non-goals

- Shipping the playable game or its combat, stats, AI, spells, gas economy, or System/LLM layer in this repository.
- Implementing game building systems (blueprint/work-order/mechanism gameplay) inside the substrate product.
- Absorbing harness-owned character, camera, seed-world postcard content, demo routes, or device-specific performance gates into the product promise.
- Requiring LLM or System authoring for the substrate to generate, simulate, or serve a world.

## Confirmed vision constraints

- Delivery form: Rust crate or small family of tightly scoped Rust crates for intended consumers in that ecosystem.
- Consumer equality: any in-repo harness uses only public interfaces available to external games—no privileged internal path.
- Substrate autonomy: core operation has zero LLM/System dependency.
- Offering quality: the world substrate is GPU-resident as part of product identity.

## Deferred design decisions

- Exact crate split and workspace layout (beyond the substrate-vs-harness consumer boundary).
- Voxel resolution, storage layout, meshing approach, LOD, and streaming-ring policy.
- How much matter simulation (fluids beyond static bodies, integrity, fire, granular settle, vegetation dynamics) belongs in the first design slice versus later substrate depth.
- Persistence encoding, benchmark harness design, and platform-specific engineering limits.
- Open technical tradeoffs left by the seeds (for example fidelity vs. cost, distant terrain representation).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required current delivery** of this repository, or only **permitted** as an adjacent consumer?

- **Proposed safe answer:** Required as an adjacent validation artifact that exercises the substrate only through public APIs, but outside product identity—so harness controller, content, presentation, and performance gates do not become substrate requirements.
- **If answered differently:** “Only permitted” drops mandatory harness delivery from the current mandate; treating the walkable demo’s content, controls, or acceptance targets as product scope would re-center identity on a vertical demo instead of a reusable crate substrate.

## Seed synthesis

- **README.md** — Establishes Moria as a GPU-resident voxel-world substrate consumed as a Rust crate, with the walkable executable cast as a separate validation harness for core world operations.
- **docs/seeds/project-boundary.md** — Binding boundary correction: substrate is the product; the game and listed game layers are out of repo; harness must share public interfaces; building/System/spell/gas/combat/AI layers stay unimplemented here.
- **docs/seeds/product-one-seed.md** — Shapes an early validation-facing cut of substrate outcomes and non-goals; supplies adjacent walkable-demo detail kept out of product identity while motivating dig/place proof and public mutation surfaces.
- **docs/seeds/voxel-world-substrate.md** — Defines substrate-level world/matter responsibilities (natural mutable volume, deep-Z, generation, physics-facing matter) for reuse by multiple future games without transferring those games’ rules into this product.
