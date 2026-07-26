# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer product: matter, world generation, queries, and mutation for natural-looking, fully material worlds. It is not a game.

## Purpose

Games that need a diggable, deep, continuous material world should share one substrate instead of each rebuilding geology, mutable matter, and presentation-from-truth. Moria exists so multiple downstream titles—and pure sandboxes—can stand on the same world foundation with zero dependency on any particular game rules layer or LLM system.

## Product boundary

**Belongs to Moria**

- The reusable substrate and its public integration surface for external Rust consumers.
- Substrate responsibilities at outcome altitude: geological world generation, GPU-resident matter storage and simulation hooks the design phase scopes, non-authoritative visual extraction from matter, mutation and query APIs, vegetation and surface dressing that stay honest to matter, fluid and structural capabilities the substrate is responsible for enabling, and persistence/streaming of world truth.
- Compatibility seams where substrate requirements demand them so future game layers can attach without being implemented here.

**Does not belong to Moria**

- The actual game: rules, combat, stats, AI, System/LLM, spells, gas policy, economy, and building/gameplay layers.
- Authored game content, presentation policy, characters, controllers, cameras, and UX—those remain consumer-owned.
- A walkable-world executable, if present, is an adjacent validation consumer of public interfaces only; it is not the product identity and must not own privileged or game-specific substrate paths (see Q1 for whether delivering that harness is in current scope).

## Required product outcomes

- **Reads as a normal world, is material truth underneath.** Rolling terrain, forests, water, cliffs, and underground volumes are backed by mutable voxels; what the player sees is a view derived from matter, not decorative geometry outside the material world.
- **Mutable everywhere, including deep Z.** Any region of the material field can be destroyed, placed, or reshaped; the underground is first-class content (caves, strata, ore, voids), not a painted floor under a heightmap.
- **Geology-first generation, lazy and seed-driven.** Worlds are produced as layered geology and related surface structure so digging reveals true materials and spaces; unevaluated volume stays cheap until touched.
- **Substrate APIs mediate all matter access.** Consumers mutate and query only through the public verb/query surface; nothing above the matter core reaches voxels by private path. The same surface is what an external game would use.
- **Interactable matter vs derived dressing.** Things that burn, break, or block are material; lightweight surface clutter is driven by matter so it cannot desync from digs and state changes.
- **Standalone, multi-consumer engine layer.** The substrate runs without an LLM or any one game mode; it is fit for ARPG, fortress/colony, descent, and sandbox consumers that supply their own rules above it. Persistence is worldgen function plus edit deltas; activity-centered streaming keeps large regions tractable.

## Future products and enabling implications

Downstream of Moria (not this product): a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. Product-one-style walkable demos are adjacent proof vehicles, not alternate product identities.

Enabling implications the substrate must not foreclose: continuous dig/build play, deep underground traversal and revelation, natural surface reading with material honesty when cut, and clean attachment of future semantic/game layers (rooms, work orders, priced policies, agents) without embedding those layers here.

## Non-goals

- Implementing game rules, combat, AI, System/LLM features, spells, gas metering, or building/gameplay layers in this repository.
- Shipping a finished game, campaign, or authored adventure content as Moria itself.
- Treating heightmap-with-props or render-mesh-as-truth as the product model.
- Making the substrate depend on an LLM or on any single game mode’s policies.

## Confirmed vision constraints

- Delivered as a Rust crate or small family of tightly scoped Rust crates for Rust game consumers.
- GPU-resident world/matter substrate (design chooses concrete GPU stack and layouts).
- Adjacent validation consumers, if any, use only the same public interfaces available to an external game—no privileged harness paths.
- Future System, spell, gas, combat, AI, and building layers stay out of implementation scope; seams only where substrate requirements demand them.
- Substrate stands alone with zero LLM dependency.

## Deferred design decisions

- Capability depth and delivery sequence within the substrate (which simulation and generation facets ship in which increment).
- Concrete meshing, storage, simulation, streaming, and persistence mechanisms; voxel scale; LOD; object-layer capacity; fluid and integrity fidelity.
- Exact crate/package split inside the Rust workspace boundary.
- Harness-only concerns if a walkable-world executable is built: controller, demo route, content seed, presentation, platforms, and performance gates.
- Open technical tradeoffs left in the substrate seed (e.g. multipayer readiness statements, distant-terrain strategy).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** alongside the substrate crates, or only **permitted** as an adjacent artifact?

- **Proposed safe answer:** Permitted only—current product delivery is the reusable substrate; a harness may exist later or beside it but is not required to call the substrate done.
- **If answered differently:** Requiring the harness keeps product identity as substrate-only but adds a mandatory adjacent delivery (still without importing its controls, content, or acceptance scene into Moria’s identity). Treating it as out of repository entirely would drop even the permitted harness path described in the boundary seed.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident voxel-world substrate Rust crate and frames the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Binding repository boundary: substrate crates are the product; the real game is out of repo; harnesses must use public interfaces; game/System/building layers are not implemented here.
- **docs/seeds/product-one-seed.md** — First walkable demo and proof vehicle for substrate claims; supplies consumer-owned demo scope and validation motivations, not a second product identity or automatic transfer of demo content/performance into Moria.
- **docs/seeds/voxel-world-substrate.md** — Authoritative long-horizon substrate purpose and outcome families (natural look, full mutability, deep Z, generation, matter honesty, multi-consumer layering) without making every mechanism or future game feature a current delivery checklist.
