# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for natural-looking, fully mutable continuous-3D voxel worlds providing matter, physics, queries, and mutation. It is not a game, not a demo product, and not an LLM-backed system.

## Purpose

Moria exists so multiple downstream games can share one standalone material world: geology-backed terrain, mutable matter all the way down, deep underground as first-class space, reactive physical matter, and a public consumer contract for inspection and change—without each game reimplementing that foundation, and with zero System/LLM dependency.

## Product boundary

**This product owns**

- The reusable voxel-world substrate: geology-first generation, GPU-resident matter and physical simulation outcomes, non-authoritative presentation of that matter, mutation-safe spatial support, persistence of world transformations, and the public command / mirror / event surface.
- The integration boundary that keeps adjacent consumers (including any validation harness) on the same public interfaces as an external game.

**Not this product**

- The actual game is a separate downstream consumer and is not part of this repository.
- Game rules and the System, LLM, spell, gas, combat, AI, and building layers are out of scope and must not be implemented here. Compatibility seams may exist only where the substrate itself requires them. Gameplay, UX, controllers, authored content, presentation, and game-specific policy remain consumer-owned.
- A walkable-world executable may exist as an adjacent validation harness; whether it is a current delivery is open (see Q1). While that is open, it is not recorded here as required or optional delivery. Its controller, character, authored route or content, presentation, workloads, platforms, and performance gates are not substrate scope.

**Adjacent first slice (harness-owned, delivery status open)**

If present, the walkable executable is a validation harness for generation, streaming, meshing, editing, collision, persistence, and performance. Its fused proof is traversal of a generated natural voxel world where dig and place show that what is seen is mutable voxel truth—not a heightmap with props. That purpose and proof are harness concerns; they do not narrow Moria’s identity or import harness controls, content, platforms, or numeric gates. A narrow first harness slice may exercise only part of the substrate; that does not optionalize unexercised substrate outcomes.

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
- Treating any validation harness as a game layer or allowing privileged substrate access for it.
- Primary “cube voxel” surface aesthetic; the grid is truth, not the intended look.
- Importing harness or future-game hardware targets, benchmark gates, or content inventories as product identity.

## Confirmed vision constraints

- Delivered for Rust consumers as a crate or small family of tightly scoped crates.
- GPU-resident voxel-world substrate with a commands-in / stale-mirror-and-events-out consumer contract.
- Operates with zero LLM/System dependency.
- Any validation harness must use the same public interfaces available to an external game.
- Clear consumer boundary between reusable substrate and adjacent artifacts is mandatory; exact packaging is a design detail.

## Deferred design decisions

- Mechanisms, fidelity tiers, and delivery sequence for meshing, fluids, integrity, CA, object-layer behavior, ambient sim, and navigation internals.
- Voxel resolution, LOD, and related fidelity tradeoffs.
- How the crate family is split while preserving the consumer boundary.
- API shapes for verbs, mirror, events, and persistence encodings.
- Harness content, controls, scenarios, platforms, and measurement methods (consumer/design concerns; delivery status open under Q1).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** of this repository, or only **permitted** as an adjacent artifact that may be added to exercise public interfaces?

- **Proposed answer:** Permitted only. Product completeness is defined by the substrate crates and public boundary; a harness is not required for the product to be itself.
- **If answered differently:** If required, record harness delivery as settled while keeping it outside product identity—and still exclude its controller, character, route, content, presentation, platform, and performance gates from substrate scope unless separately specified.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md** — Binds current identity to the substrate crate boundary, excludes the game and listed game layers from this repository, and requires any harness to use public interfaces only.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate purpose and current outcomes: natural look over mutable voxels, deep Z, geology-first lazy worlds, matter/physics/queries/mutation, reactive physical matter, GPU command/mirror/event lifecycle, mutation-safe navigation, and persistent generation-plus-edits with object/entity state—without LLM dependency.
- **docs/seeds/product-one-seed.md** — Describes an adjacent first walkable harness slice and its dig/place traversal proof; used only to describe harness validation purpose and proof condition, not to redefine substrate identity or import demo content, controls, platforms, or performance gates. Its narrow first-slice exclusions do not optionalize broader substrate outcomes.
