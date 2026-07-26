# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it as an engine layer for matter, world generation, mutation, queries, and related world physics—not as a finished game.

## Purpose

Moria exists so adventure, fortress, sandbox, and related games can share one material world foundation: a landscape that reads as a normal natural surface world while remaining fully mutable voxel truth all the way down, including deep underground play. Game rules, economy, combat, and AI live above it. The substrate must stand alone with no dependency on an LLM or “System” layer.

## Product boundary

**This product owns** the reusable world substrate and its public integration surface: generation of geology-backed terrain, GPU-resident matter storage and views of that matter, mutation and query APIs, matter-coupled dressing and interactable world objects, streaming and persistence of world state, and world-side physics/query support against voxel truth.

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness for the substrate. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific paths through the substrate. Harness character control, camera, authored demo route, presentation, debug chrome, scripted workloads, and machine-specific acceptance numbers are harness concerns—not product scope. Whether that harness is a required delivery remains open (see Q1).

**Not this product:** the actual game (separate downstream consumer, not this repository); game rules; System/LLM features; spells; gas/pricing policy; combat; AI; and building gameplay layers (UI, blueprints, mechanisms, room/economy systems). Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

- **Reusable Rust substrate.** External games integrate a GPU-resident voxel world through public crate APIs. Consumers have no privileged access path; validation harnesses, if any, share that same contract.
- **Natural look, material truth.** The world reads as continuous natural terrain (surface landscape, water bodies, vegetation and clutter) while voxels remain authoritative. Rendered mesh and dressing are regenerated views—never saved truth—and gameplay-facing physics and queries run against voxel matter, not the mesh.
- **Mutable everywhere, deep Z first-class.** Any region of matter can be destroyed, eroded, or placed; dig/place-style mutation stays consistent with remeshing and queries. Underground is content: geology-first generation (strata, caves, ores, aquifers and related volume features), not a painted underside of a heightmap.
- **Scale without eager full residency.** Large regions stay practical via sparse residency and lazy materialization from generation; untouched volume does not force full voxel cost. Active areas stream; persistence is generation plus edit deltas so scars and edits reload faithfully without saving the whole world as raw voxels.
- **Matter behavior without game rules.** The substrate provides world-level matter outcomes games rely on—structural support and failure of solid matter, granular settle where materials need it, multi-tier fluids (static bodies through coarser active flow and fine splash coupling), and thin ambient time/weather/fire ecology that keeps the landscape legible—while pricing, win conditions, and mode policy stay out of the crate.
- **Standalone engine layer.** The substrate has zero LLM dependency. Multiple game shapes (ARPG, fortress/colony, descent, pure sandbox) can sit above the same stack; gas or labor pricing, if any, is consumer policy injected above matter—not hardwired game mode logic inside the substrate.

## Future products and enabling implications

Future **consumers** (not current product) include the actual Moria/System ARPG, a DF-style fortress or colony game, a descent roguelike, and pure sandbox tools. A walkable “product one” demo is a consumer-shaped validation and communication artifact, not a second product identity.

**Enabling implications only:** the same public matter, generation, mutation, streaming, and persistence surface should be sufficient for those games to author content, attach agents, and apply their own rules without forking privileged world paths. Multiplayer-ready command/mirror separation is a design-ready posture of that boundary, not a committed shipping milestone here. Gameplay, UX, controllers, characters, animation, authored content, and presentation remain consumer-owned.

## Non-goals

- Shipping the actual game, its modes, or game-layer systems in this repository
- Implementing System/LLM, spells, gas metering/pricing, combat, AI, or agent labor policy
- Building UI, blueprints, work orders, mechanisms, or room/economy layers as product features
- Treating harness demo content (seed postcard route, third-person controller, camera, debug keys, trailer beats) as substrate requirements
- Making the mesh or dressing authoritative for simulation or saves

## Confirmed vision constraints

- Integration form is a **Rust crate or small family of tightly scoped Rust crates** for external game consumption
- World representation is **GPU-resident**
- **No privileged substrate paths** for any consumer, including a validation harness
- **Zero LLM/System dependency** in the substrate
- **Game rules and named game layers** (System, LLM, spell, gas, combat, AI, building) stay out of implementation scope here; seams only where the substrate itself needs them
- **Voxel matter is truth**; mesh and dressing are views

## Deferred design decisions

- Precise crate split and workspace layout (boundary intent is fixed; packaging is design)
- Voxel resolution, LOD strategy, object-layer capacity, and related fidelity/cost tradeoffs
- Delivery depth and sequence of matter-sim capabilities (fluid tiers, CA/fire, integrity, granular, ambient)
- Graphics/backend and platform performance targets, benchmark harnesses, and acceptance thresholds
- How far generation, meshing, and sim features land in any first vertical slice
- Whether a walkable validation executable is part of current delivery (Q1)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** alongside the substrate crates, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted—not required. Moria’s identity and success criteria are the reusable substrate crates; a harness may exist and must use public APIs if built, but shipping it is not mandatory for the product to be complete.
- **If answered differently:** Making the harness mandatory adds a required adjacent deliverable (a walkable executable that exercises the public API) without moving harness-owned controls, content, presentation, or acceptance numbers into substrate identity.

## Seed synthesis

- **`README.md`** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/harness, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binds product identity to the substrate crate(s), excludes the game and named game layers from this repository, and fixes the public-API consumer boundary for any validation harness.
- **`docs/seeds/product-one-seed.md`** — Describes a first walkable demo consumer and proof points that motivate substrate outcomes (mutability, geology, meshing-as-view, streaming/persistence); its controller, seed content, milestones, and machine gates stay out of product scope.
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes the substrate’s outcome families (natural look over voxel truth, deep-Z geology, matter sim, generation, streaming/persistence, layered reuse without LLM) without redefining identity or importing game-layer features.
