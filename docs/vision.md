# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for external games and tools—not a game, not an ARPG, and not a fortress or descent title.

## Purpose

Moria exists so multiple games can share one material world: a natural-looking surface over fully mutable voxel truth, including deep underground, with generation, matter, interactive objects, fluid and material behavior, ambient ecology, structural integrity, navigation, mutation, streaming, and persistence owned once in the substrate. Game rules, economy, combat, AI, LLM/System behavior, and building-as-gameplay stay above it. The substrate must stand alone with no LLM dependency.

## Product boundary

**This product owns:** the reusable world substrate and its public consumer surface—geological/natural generation; GPU-resident matter; voxel-backed interactive vegetation and objects as world truth; granular, fluid, and material interaction; thin ambient time/weather/growth/wetness/fire ecology; structural support and failure; mutation-safe derived navigation in continuous 3D; commands in with a potentially stale mirror plus events out; consumer extension registries for materials, placements, and related world definitions; meshing as a non-authoritative view; streaming; and cross-run persistence of altered matter plus moved or felled substrate objects and their state.

**Not this product:** any playable game; game rules; the System/LLM; spells; gas policy; combat; AI; building layers as game systems; multiplayer session product. Those are downstream consumers or excluded layers.

**Adjacent artifact:** a walkable-world executable may exist in this repository as a validation harness (see Q1). If present, it must use the same public interfaces available to an external game and must not own privileged substrate paths. Its controller, character, authored content, presentation, routes, workloads, platforms, and machine-specific performance gates are not product identity. Product One’s first-slice exclusions limit slice depth only—not substrate outcomes below.

**Downstream consumers (not in this repository):** later titles (System ARPG, fortress/colony, descent/roguelike, pure sandbox) that consume Moria as a crate. The System, if present, is only another client of the same public surface.

## Required product outcomes

1. **Natural look, material truth** — Generated surface that reads as ordinary nature; interactable matter is mutable voxel truth, not decorative geometry. The render mesh is a regenerated view, never authoritative.
2. **Mutable everywhere, deep Z, geology-first** — Any matter can be destroyed, moved, or placed; underground is content. Worlds generate as geology that materializes on demand so digs encounter true structure.
3. **Interactive vegetation and objects** — Things that can burn, break, or block are voxel-backed; state and physical changes (including felling and re-integration) stay world truth. Non-interactive dressing is a pure function of voxel state.
4. **Granular, fluid, material, ambient, and structural behavior** — Granular settle; multi-tier fluid bodies and disturbed flow; material interactions; thin day/night, seasons, weather, growth, wetness, and fire ecology; structural support and failure with cascade—all substrate-owned world behavior.
5. **Mutation-safe navigation and consumer contract** — Derived navigation across continuous 3D stays valid under mutation. Consumers send commands and observe via a potentially stale mirror plus events; extension registries let ordinary consumers or hand-authored content add materials, placements, and related definitions without privileged voxel access.
6. **GPU-resident scale and world-state persistence** — Sparse GPU residency for large regions; altered matter and moved/felled substrate objects and their state persist across runs with exact restoration.

**First-slice mandate (adjacent harness; delivery force per Q1):** a generated curated natural region; public dig/place proof; traversal-facing collision against voxel truth; validation of generation, streaming, meshing, editing, collision, persistence, and performance at the public boundary. Product One depth limits do not make the substrate outcomes above optional.

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
- **Repository boundary:** the actual game is a separate downstream consumer, not this product.

## Deferred design decisions

- Algorithms, fidelity, and delivery sequence for vegetation/objects, fluids, granular, ambient ecology, integrity, and navigation
- Exact meshing, storage, streaming, and persistence encodings; crate split within the family
- Voxel resolution, LOD strategy, object-layer capacity, and fluid-model fidelity
- Adjacent harness structure and measurement once Q1 settles delivery force

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required repository delivery**, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Required adjacent delivery—Product One and the README pin first-slice proof domains; project-boundary “may” is permission language. The harness stays outside product identity; its controller, content, presentation, workloads, and machine gates stay adjacent.
- **If answered differently:** Shipping the executable is optional; first-slice proof domains are no longer a mandatory adjacent delivery (substrate outcomes above still stand).

**Q2.** Should **server-authoritative multiplayer readiness** (architecture kept multiplayer-ready even though multiplayer is not built) be a **current product constraint**, or left out of scope statements?

- **Proposed safe answer:** Out of scope statements—keep the command/stale-mirror/events contract without binding multiplayer readiness as a product constraint.
- **If answered differently:** Readiness becomes a confirmed vision constraint on the public surface while multiplayer session product remains a non-goal.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world substrate (Rust crate) and the walkable-world executable as a separate validation consumer for generation, streaming, meshing, editing, collision, persistence, and performance.
- **docs/seeds/project-boundary.md** — Binding product/repository boundary: Rust crate(s), game out of repo, harness only via public interfaces, excluded game/System layers; “may” on the harness creates the Q1 delivery ambiguity.
- **docs/seeds/product-one-seed.md** — Pins first-slice “done” (curated region, traversal proof, public dig/place, validation domains); slice exclusions limit depth only; exact-restore informs restoration force without importing machine gates.
- **docs/seeds/voxel-world-substrate.md** — Substrate purpose and outcome families (natural look, mutability, deep Z, geology, voxel-backed objects, granular/fluids/materials, ambient ecology, integrity, navigation, command/mirror/event contract, extension registries, cross-run world-state persistence, multi-game reuse); §14 multiplayer-readiness open as Q2.
