# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate for natural, fully material 3D worlds. It is delivered as a Rust crate or a small family of tightly scoped Rust crates. It is not a game, not a game layer, and not a content product.

## Purpose

Moria exists so multiple games can share one world foundation: a continuous voxel-true landscape that reads as ordinary outdoor and underground terrain, can be mutated anywhere, and exposes matter, generation, queries, and mutation without embedding any particular game’s rules, economy, combat, or AI. Downstream products should start from a walkable material world rather than inventing their own geology, meshing, and edit model.

## Product boundary

**In product**
- The reusable substrate and its public consumer-facing interfaces (Rust crate or tightly scoped crate family).
- Substrate-owned world capabilities: generation of natural geology and surface, sparse GPU-resident matter, smooth non-authoritative presentation of voxel truth, mutability, streaming-scale worlds, persistence of generation plus edits, and matter-level physics and query support that games sit on top of.
- Compatibility seams where substrate requirements demand them—without implementing game layers.

**Adjacent / not this product**
- The actual game(s) live outside this repository as separate consumers.
- A walkable-world executable may exist as an adjacent validation harness that exercises the substrate through the same public interfaces an external game would use. It is not the product identity; whether it is a required repository deliverable is open (see Q1). Its controllers, characters, authored demo content, presentation polish, routes, workloads, machine targets, and performance gates are not current-product scope.
- Game rules and future System, LLM, spell, gas, combat, AI, and building *layers* are out of scope here.

## Required product outcomes

1. **Reusable Rust substrate** — Consumers integrate Moria as crate(s) and obtain a shared world engine layer with no privileged internal paths reserved for in-repo demos.
2. **Voxel truth, ordinary look** — The world is fully material (not decorative heightmap-plus-props). What the player sees is a view of that matter; mutation and interaction remain honest to the voxel world.
3. **Mutable everywhere, including deep Z** — Any material can be destroyed, moved, or placed. Underground geology, voids, and depth are first-class content, not a painted floor under the surface.
4. **Geology-first generation and sparse scale** — Worlds are generated as geology and surface that materialize on demand so large continuous regions remain tractable; streaming and edit-friendly persistence (generation plus deltas) are product responsibilities.
5. **Matter services for games above** — The substrate provides the matter, physics, queries, and mutation surface games need (including dig/place-class verbs and the ability to support vegetation, fluids, integrity, and placement as substrate capabilities over time). Delivery depth and sequencing are design choices; the responsibilities are not reassigned to individual games.
6. **Standalone engine layer** — Core operation has zero dependency on an LLM or “System” client. Game policy (pricing, combat, agents, rooms, work orders, spells) lives above the substrate.

## Future products and enabling implications

Future consumers include a System-driven ARPG, fortress/colony play, descent/adventure modes, and pure sandboxes. They motivate a substrate that is genre-agnostic, mutation-safe, and deep-Z capable. Enabling implications for those consumers: public verb/query boundaries suitable for later sandboxing or multiplayer-style authority; room for game-injected policy without forking matter; and worlds whose scars and structures can persist across modes as data rather than one-off levels. Gameplay, UX, controllers, authored content, presentation, and game-specific policy remain consumer-owned.

## Non-goals

- Implementing the actual game or shipping game rules, combat, stats, AI, spells, gas economies, or LLM/System behavior in this product.
- Owning building-layer gameplay (blueprints-as-gameplay, mechanisms-as-game systems, labor/work orders, room designation UX)—even when the substrate later exposes matter verbs those layers would call.
- Treating the validation harness’s demo content, character, camera, or acceptance scenario as product features.
- Making the substrate depend on or embed System/LLM world authorship for basic operation.

## Confirmed vision constraints

- Product identity is the reusable voxel-world substrate, not the game and not the harness.
- Exposure is a Rust crate or small family of tightly scoped Rust crates.
- If a validation harness exists in-repo, it must use the same public interfaces available to an external game; adjacent consumers have no privileged access.
- The consumer/substrate boundary is required; exact package layout is not a vision decision.
- Game, System/LLM, spell, gas, combat, AI, and building layers must not be implemented in this product (seams only where substrate needs demand them).
- Substrate stands alone without LLM dependency.

## Deferred design decisions

- Crate split, APIs, algorithms, storage layouts, meshing strategy, voxel size, LOD, streaming rings, and persistence encoding.
- How deep the first release goes into fluids, integrity, vegetation objects, weather, or building-related matter verbs—versus later substrate depth.
- Harness-only concerns if a harness is built: seed region content, controller/camera, debug tools, benchmarks, platforms, and numeric performance gates.
- Multiplayer deployment and any non-substrate backend choices.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world validation harness a **mandatory deliverable** of this repository’s current work, or only **permitted** as an adjacent artifact?

- **Proposed answer:** Permitted only—product identity and required outcomes stay on the substrate; a harness may exist and, if present, must consume public interfaces, but it is not a required current delivery.
- **If different:** Making the harness mandatory adds a repository delivery obligation for a walkable validation executable without moving controllers, demo content, presentation, or performance gates into product identity; the brief’s delivery language would record that obligation and still keep those harness details out of substrate scope.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/harness, not game layer.
- **docs/seeds/project-boundary.md** — Locks identity to the substrate crate(s), places the real game outside the repo, permits a public-API-only harness, and excludes game/System/LLM/spell/gas/combat/AI/building layers.
- **docs/seeds/product-one-seed.md** — Describes a first walkable demo/validation consumer (content, controller, milestones, machine targets); used to clarify harness adjacency and proof needs, not to redefine product identity or import demo acceptance into substrate scope.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look over voxel truth, full mutability, deep-Z geology, generation/streaming/persistence, matter services, layered reuse without LLM dependency) at vision altitude without adopting its mechanism inventory as the product brief.
