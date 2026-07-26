# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world **substrate** delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for natural-looking, fully material 3D worlds—not a game and not a shipped playable title.

## Purpose

Moria exists so multiple downstream games (and in-repo validation) can share one material world: continuous geology and deep underground, everywhere-mutable matter, and a public query/mutation boundary. Game identity, rules, and presentation live above the substrate; the substrate stands alone with no LLM or game-system dependency.

## Product boundary

**This product owns:** the reusable voxel-world substrate—matter representation and mutation, geology-oriented world generation, derived surface presentation from voxel truth, occupancy suitable for consumer collision and traversal, streaming and edit persistence, and the public engine interfaces those capabilities require.

**Adjacent, not product identity:** a walkable-world executable may exist in the repository as a validation harness. It must use the same public interfaces available to an external game. Whether that harness is a required repository delivery is open (see Q1). Its character, camera, demo route, curated seed content, controllers, debug presentation, scripted workloads, and numeric performance gates are harness- or consumer-owned, not substrate identity.

**Outside this repository and product:** the actual game; game rules; System/LLM, spell, gas, combat, AI, and building *layers*. Compatibility seams may be designed where substrate needs demand them; those layers are not implemented here.

## Required product outcomes

1. **Material natural world** — Consumers get a continuous 3D world that reads as ordinary outdoor/underground terrain (hills, forest, water, cliffs, caves) while remaining fully material voxel truth, not a heightmap with non-matter props.
2. **Mutable everywhere, deep Z first-class** — Matter can be destroyed, placed, or otherwise changed throughout the volume; underground geology and voids are content, not a false floor.
3. **Engine-visible world services** — Through a public API, consumers obtain generation of material worlds, mesh/dressing as a non-authoritative *view* of voxels, streaming of large regions, persistence of edits relative to generation, and occupancy/query support for movement and interaction against voxel truth.
4. **Reusable substrate, not a game stack** — The same crate boundary supports external games and any validation harness without privileged in-product paths; nothing above the matter contract touches voxels except via substrate verbs and queries.
5. **Standalone foundation** — The substrate does not depend on LLM/System features or other game layers to function as a world engine.

## Future products and enabling implications

Described future *consumers* (not current product): a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. They motivate a shared matter, physics, query, and mutation foundation and optional seams for higher policy (e.g. pricing of verbs), but their gameplay, content, UI, characters, combat, economy, and building systems remain consumer-owned. Long-horizon matter behaviors that only those games fully exercise are delivery-depth for design, not a separate committed roadmap in this brief.

## Non-goals

- Shipping a complete game or game rules in this repository
- Implementing System/LLM, spells, gas policy, combat, AI, or building layers here
- Treating demo/harness presentation, authored tour content, or character control as the product
- Making the substrate depend on LLM or game-layer services

## Confirmed vision constraints

- **Rust crate surface** — product is consumed as a Rust crate or small family of tightly scoped Rust crates
- **Public-interface consumer boundary** — any validation harness and external games use the same public substrate interfaces; no privileged game-specific implementation path inside the substrate
- **Repository scope** — the actual game is a separate downstream consumer, not part of this product repository
- **GPU-resident matter** — world matter is intended to live as a GPU-resident substrate (engine identity, not a harness benchmark list)
- **Zero LLM dependency** — the world layer must stand alone without the System/LLM stack

## Deferred design decisions

- How far each matter, fluid, integrity, vegetation-object, and ambient-sim behavior is taken in any given delivery slice
- Crate split, internal layering, APIs, storage/meshing/streaming mechanisms, and voxel scale/LOD choices
- Whether and how multiplayer-oriented command boundaries are realized beyond the public verb/query seam
- Harness-only acceptance numbers, platforms, and demo content (if a harness is delivered)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a *required* repository delivery for the current product effort, or only *permitted* as an adjacent artifact?

- **Proposed answer:** Permitted and adjacent only. Current product identity and outcomes stay the reusable substrate; a harness may exist to exercise public interfaces but is not itself the product promise.
- **If different:** Making the harness mandatory adds a repository delivery obligation (still not product identity) without importing its controller, content, or performance gates into substrate outcomes; forbidding it entirely removes even a permitted in-repo consumer and leaves validation fully external.

## Seed synthesis

- **`README.md`** — Names Moria as the GPU-resident Rust voxel-world substrate and positions the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binding product and repository boundary: substrate as Rust crate(s); game out of repo; harness may exist only via public interfaces; game/System/building layers out of scope.
- **`docs/seeds/product-one-seed.md`** — First walkable demo and harness-shaped proof of the substrate (slice depth, demo content, player, and numeric gates); used as consumer/harness motivation, not to redefine product identity or import demo-owned scope.
- **`docs/seeds/voxel-world-substrate.md`** — Substrate purpose and capability altitude: natural look over mutable voxels, deep Z, geology-first generation, matter/physics/query/mutation services, and multi-game reuse without LLM dependency; mechanisms deferred to design.
