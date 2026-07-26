# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for external games and tools—not a game, not an ARPG, and not a fortress or descent title.

## Purpose

Moria exists so multiple games can share one material world: a natural-looking surface over fully mutable voxel truth, including deep underground, with generation, matter, interactive objects, fluid and material behavior, ambient ecology, structural integrity, navigation, mutation, streaming, and persistence owned once in the substrate. Game rules, economy, combat, AI, LLM/System behavior, and building-as-gameplay stay above it. The substrate must stand alone with no LLM dependency.

## Product boundary

**This product owns:** the reusable world substrate and its public consumer surface—geological/natural generation; GPU-resident matter; voxel-backed interactive vegetation and objects as world truth; granular, fluid, and material interaction; thin ambient time/weather/growth/wetness/fire ecology; structural support and failure; mutation-safe derived navigation that supports differing 3D movement capabilities; commands in with a potentially stale mirror plus events out; consumer extension registries for materials, placements, and related world definitions; meshing as a non-authoritative view; streaming; edit-delta and journaled world-state continuity across runs, including a persistence seam for consumer entities and script state.

**Not this product:** any playable game; game rules; the System/LLM; spells; gas policy; combat; AI; building layers as game systems; multiplayer session product. Those are downstream consumers or excluded layers.

**Required adjacent first delivery:** a walkable-world executable in this repository as the validation harness and Product One demo vehicle. It is not the product. It must consume the substrate through the same public interfaces available to an external game and must not own privileged or game-specific substrate paths. Its controller, character, authored content, presentation, routes, workloads, platforms, and machine-specific performance gates are adjacent harness behavior, not product identity. Product One’s first-slice exclusions limit harness/slice depth only—not substrate outcomes below.

**Downstream consumers (not in this repository):** later titles (System ARPG, fortress/colony, descent/roguelike, pure sandbox) that consume Moria as a crate. The System, if present, is only another client of the same public surface.

## Required product outcomes

1. **Natural look, material truth** — Generated surface that reads as ordinary nature; interactable matter is mutable voxel truth, not decorative geometry. The render mesh is a regenerated view, never authoritative.
2. **Mutable everywhere, deep Z, geology-first** — Any matter can be destroyed, moved, or placed; underground is content. Worlds generate as geology that materializes on demand so digs encounter true structure.
3. **Interactive vegetation and objects** — Things that can burn, break, or block are voxel-backed; state and physical changes (including felling and re-integration) stay world truth. Non-interactive dressing is a pure function of voxel state.
4. **Granular, fluid, material, ambient, and structural behavior** — Granular settle; multi-tier fluid bodies and disturbed flow; material interactions; thin day/night, seasons, weather, growth, wetness, and fire ecology; structural support and failure with cascade—all substrate-owned world behavior.
5. **Movement-class-aware, mutation-safe navigation and consumer contract** — Derived navigation supports materially different 3D movement capabilities (walking, climbing, flying, burrowing, swimming) and stays valid under world mutation. Consumers send commands and observe via a potentially stale mirror plus events; extension registries let ordinary consumers or hand-authored content add materials, placements, and related definitions without privileged voxel access.
6. **GPU-resident scale and world-state continuity** — Sparse GPU residency for large regions. Altered matter persists via edit deltas; moved/felled objects, entities, and script state journal across runs for continuity. The substrate provides a persistence seam for consumer entities and script state; entity behavior and content remain consumer-owned. Exact restoration is required for Product One’s first-slice delta save/load proof (matter deltas in a slice without felling or non-player entities)—not as a force on every journaled artifact.

**Adjacent first-delivery mandate:** a generated curated natural region; public dig/place proof; traversal-facing collision against voxel truth; validation of generation, streaming, meshing, editing, collision, persistence, and performance at the public boundary, through a playable walkable-world executable. Product One depth limits do not make the substrate outcomes above optional.

## Future products and enabling implications

Future games are **consumers**, not Moria’s identity. Enabling implications: honest dig/build and deep play on one matter model; shared command/mirror/event seams so spells, labor, or sandbox tools are policy above the same surface; metadata registries for placement without touching geology code. Gameplay, UX, controllers, content, and presentation remain consumer-owned. Excluded building/game layers are not later substrate scope merely because a future game wants them.

## Non-goals

- A game, game rules, combat, stats, AI, or multiplayer session product
- System/LLM features, spells, gas metering, or pricing policy inside the substrate
- Building-layer gameplay (blueprints/work orders, mechanism/entity game logic, room economy) as Moria deliverables
- Minecraft-style cube look as the intended surface aesthetic
- Harness ownership of character control, free-camera presentation, scripted routes, or machine-specific benchmark theater as product promise

## Confirmed vision constraints

- **Ecosystem:** Rust crate or small family of tightly scoped Rust crates.
- **Residency:** GPU-resident world substrate by product intent.
- **Standalone:** zero LLM dependency; world layer functions without the System.
- **Layer discipline:** game/System/LLM/spell/gas/combat/AI/building layers not implemented here; compatibility seams only where substrate needs demand them.
- **Public-only access:** nothing above the matter boundary touches voxels directly; validation and external games share that surface.
- **Repository boundary:** the actual game is a separate downstream consumer, not this product; the walkable-world executable is a required adjacent delivery, not product identity.

## Deferred design decisions

- Algorithms, fidelity, and delivery sequence for vegetation/objects, fluids, granular, ambient ecology, integrity, and navigation
- Exact meshing, storage, streaming, and persistence encodings; crate split within the family
- Voxel resolution, LOD strategy, object-layer capacity, and fluid-model fidelity
- Adjacent harness structure, measurement protocol, and first-slice depth once design begins

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Should **server-authoritative multiplayer readiness** (architecture kept multiplayer-ready even though multiplayer is not built) be a **current product constraint**, or left out of scope statements?

- **Proposed safe answer:** Out of scope statements—keep the command/stale-mirror/events contract without binding multiplayer readiness as a product constraint.
- **If answered differently:** Readiness becomes a confirmed vision constraint on the public surface while multiplayer session product remains a non-goal.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world substrate (Rust crate) and the walkable-world executable as a separate validation consumer for generation, streaming, meshing, editing, collision, persistence, and performance.
- **docs/seeds/project-boundary.md** — Binding product/repository boundary: Rust crate(s), game out of repo, harness only via public interfaces, excluded game/System layers; “may” is repository permission, superseded for delivery force by Product One’s mandatory first slice.
- **docs/seeds/product-one-seed.md** — Pins first-slice “done” (curated region, runnable character, public dig/place, milestones through playable/downloadable demo); exact-restore applies to that slice’s delta load; slice exclusions limit depth only.
- **docs/seeds/voxel-world-substrate.md** — Substrate purpose and outcome families (natural look, mutability, deep Z, geology, voxel-backed objects, granular/fluids/materials, ambient ecology, integrity, movement-class navigation, command/mirror/event contract, extension registries, edit-delta and journaled continuity including entity/script-state seam, multi-game reuse); §14 multiplayer-readiness open as Q1.
