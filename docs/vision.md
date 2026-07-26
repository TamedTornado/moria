# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for natural-looking, fully mutable continuous-3D voxel worlds providing matter, physics, queries, and mutation. It is not a game, not a demo product, and not an LLM-backed system.

This repository also delivers a **walkable-world validation executable** as a required current adjacent delivery: a separate consumer of the substrate, not part of Moria’s product identity.

## Purpose

Moria exists so multiple downstream games can share one standalone material world: geology-backed terrain, mutable matter all the way down, deep underground as first-class space, reactive physical matter, and a public consumer contract for inspection and change—without each game reimplementing that foundation, and with zero System/LLM dependency.

The walkable executable exists to validate and prove the substrate through public interfaces—terrain generation, streaming, meshing, editing, collision, persistence, and performance—not as a game layer.

## Product boundary

**This product owns**

- The reusable voxel-world substrate: geology-first generation, GPU-resident matter and physical simulation outcomes, non-authoritative presentation of that matter, mutation-safe spatial support, persistence of world transformations, and the public command / mirror / event surface.
- The integration boundary that keeps adjacent consumers (including the validation harness) on the same public interfaces as an external game.

**Required adjacent delivery (not product identity)**

- A walkable-world executable is a **required current delivery** of this repository. It is an adjacent validation harness and consumer of the substrate, not part of Moria’s product identity.
- It must consume the substrate only through the same public interfaces available to an external game—no privileged or game-specific implementation paths.
- Its controller, character, authored route or content, presentation, platforms, machine-specific targets, and numerical performance gates are **not** substrate scope.

**Not this product**

- The actual game is a separate downstream consumer and is not part of this repository.
- Game rules and the System, LLM, spell, gas, combat, AI, and building layers are out of scope and must not be implemented here. Compatibility seams may exist only where the substrate itself requires them. Gameplay, UX, controllers, authored content, presentation, and game-specific policy remain consumer-owned.

**Harness validation purpose (harness-owned)**

The walkable executable’s fused proof is traversal of a generated natural voxel world where dig and place show that what is seen is mutable voxel truth—not a heightmap with props. That purpose and proof are harness concerns; they do not narrow Moria’s identity or import harness controls, content, platforms, or numeric gates. A narrow first harness slice may exercise only part of the substrate; that does not optionalize unexercised substrate outcomes.

## Required product outcomes

1. **Natural world, voxel truth** — Surface regions read as ordinary outdoor environments (rolling terrain, vegetation cues, water bodies, cliffs, caves) while solids are backed by mutable voxel matter, not decorative geometry outside that matter. Underground space, strata, voids, and related subsurface content are continuous with the surface. Worlds are diggable geology that materializes independently and lazily from seeded generation so mutation and descent stay honest.
2. **Physical, reactive matter** — The substrate provides matter, physics, queries, and mutation. Matter can be destroyed, moved, or placed throughout the volume. Required current outcomes include interactable voxel-backed objects and dressing, rigid and falling matter, active fluids, ambient material simulation, granular behavior, and structural failure/collapse—as world capabilities, not game policy.
3. **View is not authority** — Physics, queries, and mutation operate on voxel matter. Smooth surface presentation is derived from that matter and is never the saved or simulated source of truth.
4. **GPU consumer lifecycle and equal public boundary** — Consumers drive the world with commands in and observe through a stale mirror plus events out. Inspection and change use only the public surface (verbs, mirror queries, events, object model). No adjacent artifact owns privileged or game-specific paths into matter.
5. **Mutation-safe spatial support** — Derived navigation over mutable continuous-3D matter, invalidated when matter changes. This is substrate spatial support, not an AI or game-rule layer.
6. **Persistent large continuous worlds** — Reproducible lazy generation from seed plus authoritative restoration of persistent world transformations (edit deltas and moved objects/entity state for cross-run reuse). GPU-resident operation with streaming of active space keeps large continuous regions usable by downstream games.

## Future products and enabling implications

Future consumers (not Moria itself) include a System/LLM ARPG, a fortress/colony-style game, a Moria-style descent experience, and pure sandbox modes. At vision altitude this implies a substrate that stays policy-free above matter: games inject rules, pricing, content, and presentation. Building-oriented seams and multi-game reuse motivate substrate capabilities already in required outcomes; they do not license pulling gameplay, UX, controllers, characters, authored content, or game policy into this product.

## Non-goals

- Shipping or owning the game, its rules, or its presentation stack in this repository.
- Implementing System/LLM features, spells, gas policy, combat, AI, or building layers here.
- Treating the validation harness as a game layer or allowing privileged substrate access for it.
- Primary “cube voxel” surface aesthetic; the grid is truth, not the intended look.
- Importing harness or future-game hardware targets, benchmark gates, or content inventories as substrate product identity.

## Confirmed vision constraints

- Delivered for Rust consumers as a crate or small family of tightly scoped crates.
- GPU-resident voxel-world substrate with a commands-in / stale-mirror-and-events-out consumer contract.
- Operates with zero LLM/System dependency.
- A walkable-world validation executable is a required current adjacent delivery; it must use the same public interfaces available to an external game.
- Clear consumer boundary between reusable substrate and adjacent artifacts is mandatory; exact packaging is a design detail.

## Deferred design decisions

- Mechanisms, fidelity tiers, and delivery sequence for meshing, fluids, integrity, CA, object-layer behavior, ambient sim, and navigation internals.
- Voxel resolution, LOD, and related fidelity tradeoffs.
- How the crate family is split while preserving the consumer boundary.
- API shapes for verbs, mirror, events, and persistence encodings.
- Harness content, controls, scenarios, platforms, machine-specific targets, and numerical measurement methods (harness-owned; delivery of the executable itself is settled).

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and identifies the walkable-world executable as a separate consumer and validation harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md** — Binds current identity to the substrate crate boundary, excludes the game and listed game layers from this repository, and requires the harness to use public interfaces only without erasing its separate delivery commitment.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate purpose and current outcomes: natural look over mutable voxels, deep Z, geology-first lazy worlds, matter/physics/queries/mutation, reactive physical matter, GPU command/mirror/event lifecycle, mutation-safe navigation, and persistent generation-plus-edits with object/entity state—without LLM dependency.
- **docs/seeds/product-one-seed.md** — Pins first-build “done” through a playable walkable demo and benchmarks, establishing the downloadable walkable executable as a required adjacent delivery while remaining outside substrate identity; harness content, controls, platforms, and numeric gates stay harness-owned and do not redefine substrate outcomes.
