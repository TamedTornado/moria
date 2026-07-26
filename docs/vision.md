# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for natural-looking, fully mutable continuous-3D voxel worlds. It is not a game, not a demo product, and not an LLM-backed system.

## Purpose

Moria exists so multiple downstream games can share one standalone material world: geology-backed terrain, mutable matter all the way down, deep underground as first-class space, and consumer-facing queries and mutation—without each game reimplementing that foundation, and without any dependency on System/LLM layers to operate.

## Product boundary

**This product owns**

- The reusable voxel-world substrate: generation of diggable natural worlds, GPU-resident matter, presentation of that matter as a non-authoritative view, and the public mutation/query surface consumers use.
- The integration boundary that keeps adjacent consumers (including any validation harness) on the same public interfaces as an external game.

**Not this product**

- The actual game is a separate downstream consumer and is not part of this repository.
- Game rules and the System, LLM, spell, gas, combat, AI, and building layers are out of scope and must not be implemented here. Compatibility seams may exist only where the substrate itself requires them.
- A walkable-world executable may exist as an adjacent validation harness; whether it is a current delivery is open (see Q1). While that is open, it is not recorded here as required or optional delivery. Its controller, character, authored route, presentation, workloads, platforms, and performance gates are not substrate scope.

## Required product outcomes

1. **Natural world, voxel truth** — Surface regions read as ordinary outdoor environments (rolling terrain, vegetation cues, water bodies, cliffs, caves) while solids the player would treat as world are backed by mutable voxel matter, not decorative geometry outside that matter.
2. **Mutable everywhere; deep Z first-class** — Matter can be destroyed, moved, or placed throughout the volume. Underground space, strata, voids, and related subsurface content are continuous with the surface, not a flat floor under a skybox.
3. **Geology-first generation** — Worlds are produced as diggable geology (columns, material bands, caves, and related subsurface structure) that can materialize on demand, so mutation and descent stay honest rather than painted under a heightmap shell.
4. **View is not authority** — Physics-quality occupancy, queries, and mutation operate on voxel matter. Smooth surface presentation is derived from that matter and is never the saved or simulated source of truth.
5. **Equal public consumer boundary** — Consumers inspect and change the world only through the substrate’s public verbs and queries. No adjacent artifact owns privileged or game-specific paths into matter.
6. **Residency for large continuous regions** — The substrate supports GPU-resident operation with streaming of active space and persistence of generation-plus-edits so large continuous regions remain usable by downstream games.

## Future products and enabling implications

Future consumers (not Moria itself) include a System/LLM ARPG, a fortress/colony-style game, a Moria-style descent experience, and pure sandbox modes. At vision altitude this implies a substrate that stays policy-free above matter: games inject rules, pricing, content, and presentation. Extended matter behaviors that those games will want (richer fluids, structural integrity, ambient simulation, interactable voxel-backed objects, building-oriented seams) are enabling directions for the substrate’s matter and API surface—not licenses to pull gameplay, UX, controllers, characters, authored content, or game policy into this product.

## Non-goals

- Shipping or owning the game, its rules, or its presentation stack in this repository.
- Implementing System/LLM features, spells, gas policy, combat, AI, or building layers here.
- Treating any validation harness as a game layer or allowing privileged substrate access for it.
- Primary “cube voxel” surface aesthetic; the grid is truth, not the intended look.
- Importing harness or future-game hardware targets, benchmark gates, or content inventories as product identity.

## Confirmed vision constraints

- Delivered for Rust consumers as a crate or small family of tightly scoped crates.
- GPU-resident voxel-world substrate.
- Operates with zero LLM/System dependency.
- Any validation harness must use the same public interfaces available to an external game.
- Clear consumer boundary between reusable substrate and adjacent artifacts is mandatory; exact packaging is a design detail.

## Deferred design decisions

- Capability depth and delivery sequence within the substrate (meshing approach, fluid tiers, integrity, CA, object-layer behavior, ambient sim).
- Voxel resolution, LOD, and related fidelity tradeoffs.
- How the crate family is split while preserving the consumer boundary.
- Persistence, streaming, and multiplayer-readiness detail at the API level.
- Harness content, controls, scenarios, and measurement methods (consumer/design concerns; see Q1).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** of this repository, or only **permitted** as an adjacent artifact that may be added to exercise public interfaces?

- **Proposed answer:** Permitted only. Product completeness is defined by the substrate crates and public boundary; a harness is not required for the product to be itself.
- **If answered differently:** If required, record harness delivery as settled while keeping it outside product identity—and still exclude its controller, character, route, content, presentation, platform, and performance gates from substrate scope unless separately specified.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/harness, not game layer.
- **docs/seeds/project-boundary.md** — Binds current identity to the substrate crate boundary, excludes the game and listed game layers from this repository, and requires any harness to use public interfaces only.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate purpose and outcome altitude: natural look over mutable voxels, deep Z, geology-first worlds, GPU residency, matter/query/mutation reuse, and multi-game enablement without LLM dependency.
- **docs/seeds/product-one-seed.md** — Describes a first walkable demo/harness slice and substrate exercise points; used only to motivate validation adjacency and proof of mutability, not to redefine product identity or import demo content, controls, or performance gates.
