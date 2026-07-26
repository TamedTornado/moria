# Project vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world product for external games—not a game, campaign, content pack, or character-driven experience.

This repository’s **required first delivery** also includes an adjacent **walkable-world executable** (Product One)—a **validation harness** that proves the substrate through the same public interfaces available to an external game, with no privileged paths. Product One narrows *what ships first*; it is not product identity.

Downstream games are **consumers outside this repository**.

## Purpose

Moria exists so multiple games can share one material world: continuous outdoor and underground space that **reads as a normal natural world** while remaining **fully mutable voxel truth**. Generation, matter, mutation, observation, streaming, and persistence live in the substrate; game rules live above it.

The substrate must stand alone with **zero LLM or “System” dependency**. Product One makes one claim undeniable: *this is not a heightmap with props—it is a fully material world, it looks good, and it is reusable as a public crate.*

## Product boundary

**Belongs to Moria (substrate)**

- The reusable substrate and its public consumer surface for external Rust consumers.
- World generation, sparse GPU-resident matter, derived presentation (mesh and dressing as views of matter), mutation and query interfaces, streaming, and persistence of generation identity plus edit deltas.
- Matter-facing engine services the substrate owns (including capabilities deferred past Product One), without shipping game systems that consume them.
- Compatibility *seams* only where substrate requirements demand them—not implementations of excluded layers.

**Required adjacent delivery (not product identity)**

- A walkable-world executable that uses only public interfaces and validates generation, streaming, meshing, editing, collision, persistence, and performance.
- Harness-owned controller, camera, demo seed/route, debug presentation, benchmarks, and machine-bound measurement—proof machinery, not product surface.

**Outside this repository**

- The actual game and game rules; System / LLM; spells; gas/pricing policy; combat; AI; and building *gameplay* layers (UI, work orders, mechanism entities as game logic, room/economy policy).
- Privileged paths that bypass the public crate boundary.

A Cargo workspace (or equivalent) must separate reusable crates from the harness; the exact crate graph is a design decision.

## Required product-level outcomes

### First delivery (Product One)

1. **Material world, not scenery** — Generated natural region that reads as ordinary landscape while material remains voxel truth. The mesh is a regenerated view, never authoritative for collision, queries, mutation, or saves.
2. **Mutable everywhere (first proof)** — Dig and place through public verbs change real matter with interactive remesh; cut faces read as cut earth. Mutability proof, not building gameplay.
3. **Deep Z and geology-backed generation** — Continuous 3D space; underground is content. Generation produces coherent geology; lazy materialization and sparse residency keep large regions tractable.
4. **Matter-coupled surface at first-slice depth** — Dressing stays synchronized with matter; interactive props are voxel-backed objects (place/register/render); static water bodies exist. Full reactive matter (flowing fluids, fire/CA, granular settle, structural failure, object felling) is **not** required in this first proof.
5. **Public consumer contract from day one** — Harness and external games share dig/place and mirror/query access; nothing above the matter core touches voxels directly.
6. **Streaming, exact restore, measurable performance** — Active neighborhoods stream; truth is generation seed plus edit deltas and **restores exactly** for the first-slice save model; interactive performance and residency are observable via public use and harness benchmarks (with machine profile—not portable correctness law here).
7. **Portable GPU path** — Load-bearing work stays on portable GPU abstraction (wgpu/WGSL); no native-Metal fork in load-bearing layers.

### Enduring substrate mandate (beyond the first slice)

High-level capabilities Moria must provide as a multi-game substrate. Product One may omit full depth; omission is staging, not permanent exclusion:

- Full mutability (destroy / move / place) throughout the volume.
- Matter-backed interactive objects with lifecycle beyond place-and-render.
- Reactive and ambient matter services (fluids beyond static bodies; fire/wetness-class behavior; granular settle; structural integrity; thin time/weather drive at the matter layer).
- Mutation-safe derived spatial services (e.g. navigation support for continuous 3D movement classes) without owning game AI or movement *policy*.
- Persistence/streaming that journals substrate-owned object change for cross-run reuse, beyond first-slice terrain deltas.
- Clean seams so consumer policy (pricing, registries, semantic layers) can attach later without becoming substrate features.

Future games motivate these outcomes; their gameplay, UX, characters, assets, and content are **not** imported into current scope.

## Non-goals

- Shipping a playable game, ARPG, fortress mode, descent campaign, or multiplayer *product* here.
- Implementing System/LLM, spells, gas/intent metering, combat, AI agents, or building-game layers here.
- Treating harness controller, camera, demo route, scenery, debug keys, or machine-specific benchmarks as substrate identity.
- Making the intended surface look a Minecraft-style cube aesthetic (voxel grid is truth, not the look).
- Depending on an LLM or in-process game-ruleset for core world behavior.
- Replacing design with a fixed algorithm catalog, crate map, milestone calendar, or numeric hardware matrix here.

## Unresolved human questions

None that change product identity, purpose, or repository boundary. The seeds agree:

- **Current product** = reusable GPU-resident voxel-world substrate as Rust crate(s).
- **Required adjacent first delivery** = walkable-world harness + Product One substrate slice.
- **Reference, not “now” mandate** = full reactive-matter and multi-game architecture depth in the substrate design seed, except high-level capability families retained above as enduring purpose.
- **Out of repo** = actual games and listed game/System layers.

Open engineering tradeoffs (voxel size, LOD, object scaling, fluid/integrity fidelity, APIs/encodings, deferred delivery sequence) are design problems after vision approval. Multiplayer implementation remains a non-goal; multiplayer-friendly layering is not a binding readiness promise.

## Seed contribution account

| Source | Contribution |
| --- | --- |
| **`README.md`** | Names Moria as the reusable GPU-resident voxel-world substrate (Rust crate); walkable executable is a separate validation consumer, not a game layer. |
| **`docs/seeds/project-boundary.md`** | Binding identity: substrate crate(s) are the product; game is external; harness uses public interfaces only; game/System/LLM/spell/gas/combat/AI/building layers out of scope (seams only). |
| **`docs/seeds/product-one-seed.md`** | Binding first delivery: walkable proof, dig/place mutability proof, generation/matter/API slice depth, first-slice non-goals, seed-plus-deltas exact restore, performance and portable-GPU pressure. Harness content and machine gates are not product identity. |
| **`docs/seeds/voxel-world-substrate.md`** | Long-horizon purpose and capability families as **architecture reference**: high-level enabling outcomes retained; mechanisms and future-game examples not imported as current deliverables. |
| **`docs/seeds/README.md`** | Manifest authority: Product One binds the milestone and harness; substrate doc is reference with only Product One–selected portions required now; Moria is substrate-only. |

**Omitted on purpose:** future game modes’ gameplay and content; System loops; building UI/work orders; Product One scenery, palettes, controls, and numeric gates as identity claims; bit layouts, algorithms, and crate graphs reserved for design.

**Where seeds differ in force:** project-boundary *permits* a harness; Product One and the seeds README *require* it. This vision takes the stronger reading: harness is required adjacent delivery, still outside product identity. Where Product One and the architecture reference disagree on depth, **first delivery follows Product One**; enduring purpose follows the reference without expanding the current milestone into a full matter stack.
