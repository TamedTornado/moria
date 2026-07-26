# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for external games and tools—not a game, not an ARPG, and not a fortress or descent title.

## Purpose

Moria exists so multiple games can share one material world: a natural-looking surface over fully mutable voxel truth, including deep underground, with generation, matter, physics-facing queries, mutation, streaming, and persistence owned once in the substrate. Game rules, economy, combat, AI, LLM/System behavior, and building-as-gameplay stay above it. The substrate must stand alone with no LLM dependency.

## Product boundary

**This product owns:** the reusable world substrate and its public consumer surface—geological/natural generation, GPU-resident matter, mutation and query interfaces, meshing as a non-authoritative view of voxel truth, streaming and edit-delta persistence, and the physics-facing collision truth those interfaces expose.

**Not this product:** any playable game; game rules; the System/LLM; spells; gas policy; combat; AI; and building layers as game systems. Those are downstream consumers or excluded layers.

**Adjacent artifact:** a walkable-world executable may exist in this repository solely as a validation harness. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific substrate paths. Whether that harness is a required repository delivery is unresolved (see Q1). Its controllers, characters, authored demo content, presentation, routes, workloads, and performance gates are not product identity.

**Downstream consumers (not in this repository):** the actual game and later titles (e.g. System ARPG, fortress/colony, descent/roguelike, pure sandbox) that consume Moria as a crate.

## Required product outcomes

1. **Natural look, material truth** — A generated surface world (terrain, vegetation dressing, water bodies, cliffs, caves) that reads as ordinary nature while everything interactable is backed by mutable voxel matter, not decorative geometry outside that truth.
2. **Mutable everywhere, deep Z first-class** — Any matter in the world volume can be destroyed, moved, or placed; underground (strata, caves, ores, aquifers, depth) is content, not a painted floor under a heightmap.
3. **Geology-first generation** — Worlds are produced as geology and columns that materialize on demand so digs encounter true structure; generation is a reusable substrate asset, not a one-off demo map.
4. **Consumer-safe matter API** — External consumers mutate and query the world only through public verbs and queries; nothing above the matter boundary touches voxels directly; the render mesh is a regenerated view, never authoritative truth.
5. **GPU-resident scale** — Matter lives GPU-resident with sparsity-friendly residency so large regions stream and idle without treating the full volume as eagerly resident raw voxels; edit scars persist as generation-plus-deltas style truth.
6. **Multi-game reusability** — The same crate stack supports sandbox, adventure, fortress-style, and System-driven games by providing matter, physics-facing behavior, queries, and mutation while leaving pricing, rules, and presentation to each consumer.

## Future products and enabling implications

Future games (System ARPG, DF-style fortress/colony, Moria-style descent, pure sandbox) are **consumers**, not part of Moria’s identity. Enabling implications already implied by the substrate purpose: honest dig/build and deep play on one matter model; seamless geology and deep-Z for descent and underground work; shared mutation/query seams so spells, labor, or sandbox tools are policy above the same verbs. Their gameplay, UX, controllers, characters, authored content, and presentation remain theirs. Excluded building/game layers are not reintroduced as later substrate scope merely because a future game wants them.

## Non-goals

- Implementing a game, game rules, combat, stats, AI, or multiplayer session product
- System/LLM features, spells, gas metering, or intent/pricing policy inside the substrate
- Building-layer product features (blueprint/work-order gameplay, mechanism/entity game logic, room economy) as Moria deliverables
- Treating Minecraft-style cube look as the intended surface aesthetic (voxel grid is truth, not the look)
- Absorbing harness/demo ownership of character control, free-camera presentation, scripted routes, or benchmark theater into the product promise

## Confirmed vision constraints

- **Ecosystem:** product surface is a Rust crate or small family of tightly scoped Rust crates for integration by Rust game consumers.
- **Residency:** the world substrate is GPU-resident by product intent.
- **Standalone substrate:** zero LLM dependency; the world layer must function without the System.
- **Layer discipline:** game rules and System/LLM/spell/gas/combat/AI/building layers are not implemented here; compatibility seams may be designed where substrate needs demand them.
- **Harness access (if present):** validation uses only public interfaces; no privileged game-only paths through the substrate.
- **Consumer boundary:** the actual game is a separate downstream consumer and is not part of this repository’s product.

## Deferred design decisions

- Delivery depth and sequence for generation, matter simulation, vegetation/objects, fluids, integrity, ambient sim, and related substrate capabilities
- Exact meshing, storage, streaming, and persistence mechanisms; crate split within the family
- Voxel resolution, LOD strategy, object-layer capacity, and fluid-model fidelity
- Whether and how a validation harness is structured if Q1 requires one; its content and acceptance remain adjacent design
- Open technical tradeoffs left in the substrate seed (e.g. multiplayer readiness statements, distant terrain representation)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required repository delivery**, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted only—repository identity is the substrate; a harness may exist and, if it does, must exercise public interfaces only.
- **If answered differently:** A required harness becomes a mandatory adjacent delivery (still not the product identity), and planning must include shipping that executable without pulling its controls, content, presentation, or performance gates into substrate outcomes.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world substrate (Rust crate) and distinguishes the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binding product and repository boundary: Rust crate(s), game out of repo, optional harness only via public interfaces, and explicit exclusion of game/System/LLM/spell/gas/combat/AI/building layers.
- **docs/seeds/product-one-seed.md** — First-slice/demo seed for proving the substrate with a walkable region; supplies non-goals and mutability-proof intent, but its controller, content, milestones, platforms, and performance gates stay adjacent/deferred—not current product scope.
- **docs/seeds/voxel-world-substrate.md** — Long-form substrate purpose and responsibilities (natural look, full mutability, deep Z, geology-first gen, GPU-resident matter, consumer API layering, multi-game reuse, no LLM in the world layer); mechanisms and build order remain design material.
