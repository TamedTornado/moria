# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is engine material for games, not a game.

## Purpose

Give downstream games a shared world foundation: a natural-looking continuous 3D volume whose truth is mutable voxel matter, with deep underground as first-class space, so adventure, fortress, sandbox, and related modes can share generation, matter, queries, mutation, and related physics without each reinventing the world layer. The substrate stands alone with no LLM or game-rules dependency.

## Product boundary

**This product owns** the reusable voxel-world substrate and its public consumer interfaces: world generation as geology, GPU-resident matter representation, meshing as a non-authoritative view, mutation and query surfaces, streaming and persistence of world truth, collision and related matter-backed interaction against voxel truth, and compatibility seams the substrate itself needs.

**Adjacent, not identity.** A walkable-world executable may exist in this repository as a validation harness. If present, it is an adjacent consumer: it must use the same public interfaces available to an external game and must not own privileged or game-specific substrate paths. Whether that harness is a current delivery obligation is open (see Q1). Its controller, character, authored route, presentation, content palette, platforms, and performance gates are not product scope unless a later boundary answer says otherwise.

**Downstream / out of this repository.** The actual game (or games) are separate consumers. Game rules and the System, LLM, spell, gas, combat, AI, and building *game* layers are not implemented here. Consumers may later use substrate seams; that does not pull their content, UX, policy, or gameplay into this product.

## Required product outcomes

- **Reusable Rust substrate.** Consumers integrate through public crate interfaces; no privileged in-repo path to world truth.
- **Natural look, voxel truth.** Surface worlds read as ordinary natural terrain and structures; the voxel grid remains authoritative for interaction. Rendered geometry is a regenerated view, not the saved or simulated authority.
- **Mutable continuous volume.** Matter can be destroyed, moved, or placed throughout the volume; deep Z (caves, strata, underground space) is first-class, not a decorative floor.
- **Geology-first generation.** Worlds are produced so digging and descent expose coherent materials and voids; materialization can be lazy so large regions stay tractable.
- **Matter services for games.** The substrate provides matter representation, mutation, queries, and related physics/sim responsibilities needed for material worlds (including surface dressing and interactable voxel-backed objects at the matter level), without embedding game rules.
- **Live world operations.** Streaming, editing/mutation with consistent views, collision against voxel truth, and persistence as generation plus edit history are product capabilities consumers and any harness can exercise through public APIs.

## Future products and enabling implications

Future consumers (not this product) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress or colony mode, a Moria-style descent experience, and pure sandbox modes. They motivate a clean substrate API, material mutability, deep Z, and seams for pricing or content policy above the substrate. Their gameplay, controllers, characters, spells, gas policy, combat, AI, building UX, and authored content stay consumer-owned. Long-horizon matter features that enable those modes remain substrate-enabling implications at outcome altitude; delivery depth and sequence are design concerns.

## Non-goals

- Shipping the actual game or its rules, progression, or modes in this repository.
- Implementing System/LLM integration, spells, gas policy, combat, AI agents, or building-game layers here.
- Treating validation-harness presentation, controllers, seed content, or demo acceptance scenes as substrate product requirements (see Q1 for harness delivery only).
- Making the product’s identity a single playable demo rather than the reusable crate substrate.

## Confirmed vision constraints

- **Rust integration boundary.** The product is delivered as a Rust crate or small family of tightly scoped Rust crates for game consumers in that ecosystem.
- **GPU-resident substrate.** World matter of load-bearing interest is intended to live in a GPU-resident design, not a CPU-only voxel toy.
- **Strict consumer boundary.** Any in-repo harness and every external game share the same public interfaces; privileged access is forbidden.
- **No game-layer implementation.** Game rules and the listed future game layers stay out of this product; only seams required by the substrate may be designed, not those layers themselves.
- **Substrate, not game.** Reuse across multiple game styles is a product goal; game-specific policy lives above.

## Deferred design decisions

- Crate split, internal layering, APIs, storage layout, meshing approach, and sim algorithms.
- Voxel resolution, LOD, streaming rings, and persistence encoding details.
- How much of the matter/sim outcome family ships in which delivery slice, and open substrate tradeoffs (e.g. fidelity vs cost).
- Whether, when, and how a walkable harness is built; its content, controls, platforms, and acceptance numbers (after Q1).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a mandatory current delivery of this repository, or only permitted as an adjacent consumer?

*Proposed answer:* Only permitted. Product identity and success center on the reusable substrate crates; a harness may exist later and must use public APIs if it does, but shipping a walkable demo is not required to define or complete the product.

*If different:* Making the harness mandatory keeps substrate identity unchanged but adds a required adjacent deliverable (still without importing its controller, content, or performance gates into substrate scope). Treating it as forbidden would remove even the permitted validation path the boundary currently allows.

## Seed synthesis

- **README.md** — Names Moria as a GPU-resident voxel-world substrate consumed as a Rust crate and describes a walkable executable as a separate harness for world operations, not a game layer.
- **docs/seeds/project-boundary.md** — Locks current product identity to the reusable Rust substrate, places the real game outside the repo, permits a public-API-only validation harness, and excludes game rules and the listed future game layers.
- **docs/seeds/product-one-seed.md** — Describes a first walkable demo consumer (region, character, proof dig/place, targets); used only to motivate substrate mutability and public mutation APIs, not to import demo content, controls, or gates into product scope.
- **docs/seeds/voxel-world-substrate.md** — Supplies outcome-level substrate responsibilities (natural look, full mutability, deep Z, geology generation, matter/query/mutation services, streaming and persistence) and multi-game reuse without making design inventory or future game modes part of current identity.
