# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** for natural, fully material 3D worlds. It is delivered as a **Rust crate** (or a small family of tightly scoped Rust crates) that downstream games and tools consume. It is an engine-layer world foundation—not a game, not a rules stack, and not a character demo.

## Purpose

Moria exists so multiple future games—adventure, fortress/colony, descent, sandbox, and similar—can share one substrate for matter, world generation, spatial queries, and mutation, without each re-implementing a diggable, deep, natural-looking voxel world. The substrate must stand alone with **zero dependency on LLM or “System” gameplay**. Game policy, content, and presentation live above it.

## Product boundary

**In product**
- The reusable voxel-world substrate and its public consumer-facing interfaces (mutation verbs, mirror/queries, events, and related engine APIs).
- Substrate-owned world capabilities: material representation, geology-oriented generation, sparse residency, meshing as a non-authoritative view, streaming, persistence of edits, and matter/physics foundations games build on.

**Adjacent / not this product**
- The actual game (or games) are separate downstream consumers and are **not** part of this repository’s product identity.
- A **walkable-world executable** may exist as an adjacent validation harness. It is not the game layer and, if present, must exercise the substrate only through the same public interfaces an external game would use. Whether that harness is a mandatory current delivery is unresolved—see Q1. Do not treat harness controllers, cameras, authored demo routes, seed scenery, presentation, or performance gates as substrate scope.
- Game rules and the System, LLM, spell, gas, combat, AI, and building **game layers** are out of scope. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

Downstream design must make these consumer-visible guarantees true of the substrate:

1. **Natural material world** — Generated terrain and structures read as a normal world (hills, forests, water, cliffs, caves), not as a cube aesthetic. The render mesh is a view; **voxel data is the authority** for matter, queries, collision, and mutation.
2. **Mutable everywhere** — Any material cell can be destroyed, placed, or otherwise changed. Dig and place (and related mutation) are first-class substrate verbs so “everything you see is matter” is true in depth, not only on a height surface.
3. **Deep Z is first-class** — Underground volume (strata, caves, ores, aquifers, and continuous descent) is real world content, not a thin floor under a skybox.
4. **Geology-first generation with sparse residency** — Worlds are produced as geology (columns, strata, voids, materials) evaluable without eagerly materializing everything. Untouched volume stays cheap (homogeneous or unmaterialized); interesting shells pay the cost. Large regions remain practical under sparsity and lazy materialization.
5. **GPU-resident world substrate** — Core world representation and update paths are GPU-resident so the crate is a real-time world engine layer, not a CPU-only offline map format.
6. **Public verb/query boundary** — Nothing above the matter core touches voxels by privileged path. Consumers mutate and inspect the world through the same public interfaces; this is the sandbox, multiplayer-readiness, and reuse boundary.
7. **Streaming and durable edits** — Active regions stream in rings of responsibility; truth is worldgen plus edit deltas so scarred worlds reload faithfully without storing untouched volume.
8. **Matter-consistent surface and interactables** — Interactable surface content (for example voxel-backed objects and dressing driven by voxel state) stays consistent with material truth so fire, dig, and similar rules cannot desync “looks” from “is.”
9. **Reusable without game rules** — The same crate stack supports multiple game styles by providing matter, physics foundations, queries, and mutation—not combat, economy, AI policy, or LLM features.

## Future products and enabling implications

Future **consumers** (not this product) include a System/LLM ARPG, a fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate substrate generality: continuous 3D, honest underground play, dig/build matter, and clean layering.

Enabling implications at vision altitude only: expose seams so games can price verbs differently, author materials and stamps, and attach semantic or agent layers later—without embedding those layers in Moria. Gameplay, UX, controllers, characters, authored content, and presentation remain consumer-owned.

## Non-goals

- Shipping a playable game, campaign, or combat/ARPG loop in this product.
- Implementing System/LLM, spells, gas policy, combat, AI agents, or building **game** layers here.
- Treating the walkable-world harness’s character, camera, demo route, seed set dressing, or benchmark scene as substrate requirements.
- Defining supported consumer hardware, frame-time gates, or backend graphics stacks as product identity (those belong to consumers or later design unless explicitly productized later).

## Confirmed vision constraints

- **Ecosystem:** Moria is a **Rust** crate (or small family of crates) for integration by Rust consumers.
- **Layering:** Substrate stands alone; **no LLM/System dependency** in the product core.
- **Consumer isolation:** Adjacent tools and games—including any validation harness—use **public interfaces only**; no privileged in-tree game paths.
- **Identity quality:** The world is **GPU-resident** and **fully material** (mutability and deep volume are product promises, not optional skins).
- **Repository scope:** The actual game is outside this product; game-layer features listed under Non-goals are excluded even when future consumers need them.

## Deferred design decisions

- Capability **depth and sequence** within substrate subsystems (for example how far fluids, structural integrity, ambient weather, or object dynamics go in any given release).
- Concrete algorithms, voxel size, brick layout, LOD strategy, mesh extraction method, and storage encodings.
- Crate split within the workspace and enforcement mechanics for the consumer boundary.
- Harness content, controls, platforms, and acceptance thresholds—if a harness is delivered.
- Open technical calls already flagged in substrate design (object-layer scaling, distant LOD, fluid fidelity, multiplayer timing).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world **validation executable** a **mandatory current delivery** of this effort, or only a **permitted adjacent artifact** that may exist beside the substrate crates?

- **Proposed safe answer:** **Permitted, not mandatory.** Product identity and success center on the reusable substrate crates and public APIs; a harness may exist to validate generation, streaming, meshing, editing, collision, persistence, and performance through those APIs, but shipping the executable is not required to call the product complete.
- **If answered differently:** Making the harness mandatory expands current delivery to include an adjacent runnable consumer (still not a game layer) without moving controllers, demo content, or performance gates into substrate identity. Treating it as forbidden would drop validation-via-walkable-world from the effort entirely.

## Seed synthesis

- **`README.md`** — Names Moria as the GPU-resident voxel-world substrate consumed as a Rust crate, and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`** — Fixes product identity on the reusable substrate, keeps the real game out of repo scope, permits a public-API-only harness, and excludes game/System/LLM/spell/gas/combat/AI/building layers.
- **`docs/seeds/product-one-seed.md`** — Describes a first walkable-world consumer slice (region, character proof, dig-as-proof, demo non-goals). Motivates material-world and dig/place outcomes; does not redefine the product as the demo or import harness content, controls, or platform gates into substrate scope.
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes substrate outcome families: natural look over voxel truth, full mutability, deep Z, geology-first sparse generation, GPU residency, streaming and delta persistence, matter-consistent interactables, and multi-game reuse without embedding game rules or LLM dependence.
