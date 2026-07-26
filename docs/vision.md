# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is engine material for games, not a game. A walkable-world validation harness (Product One) is a required adjacent deliverable that exercises the substrate through public interfaces only.

## Purpose

Give downstream games a shared world foundation: a natural-looking continuous 3D volume whose truth is mutable voxel matter, with deep underground as first-class space, so adventure, fortress, sandbox, and related modes can share generation, matter response, construction and spatial services, queries, mutation, streaming, and persistence without each reinventing the world layer. The substrate stands alone with no LLM or game-rules dependency.

## Product boundary

**This product owns** the reusable voxel-world substrate and its public consumer interfaces: geology-first world generation; GPU-resident matter representation; meshing as a non-authoritative view; mutation and query surfaces; streaming and persistence of world truth; collision against voxel truth; responsive matter at substrate altitude (voxel-backed objects, granular response, active fluids and material interactions, ambient fire/wetness/growth, structural integrity and collapse); first-class placement and construction primitives, reusable structure representations, matter-participating mechanism objects, and derived room/structure semantics; mutation-safe 3D navigation and spatial queries; and consumer-control seams (commands/verbs, mirror queries, events, object and content registries, policy injection).

**Adjacent, required validation.** Product One is a walkable-world executable and benchmark deliverable in this repository. It pins what gets built first and validates generation, streaming, meshing, editing, collision, persistence, and performance through the same public interfaces available to an external game. It is not part of substrate identity. Its controller, character, authored scene and route, presentation, content palette, machine targets, and numeric performance gates are harness-owned, not substrate scope.

**Downstream / out of this repository.** The actual game (or games) are separate consumers. Game rules and the System, LLM, spell, gas, combat, AI, and building *game* layers (UI, labor policy, gameplay, authored content) are not implemented here. Substrate-required compatibility seams may be designed; those consumer layers must not be.

## Required product outcomes

- **Reusable Rust substrate.** Consumers integrate through public crate interfaces; no privileged in-repo path to world truth. Consumers and hand-authored tools share the same extension and mutation paths; the System, if any, is an ordinary game-layer client of those paths, not a substrate feature.
- **Natural look, voxel truth, full mutability.** Surface worlds read as ordinary natural terrain; the voxel grid is authoritative. Rendered geometry is a regenerated view. Matter can be destroyed, moved, or placed throughout the volume; deep Z is first-class; digging exposes coherent geology, not decorative void under a heightmap.
- **Sparse, lazy scale.** Untouched homogeneous space must not carry full per-cell cost; materialization of world truth is lazy on need (proximity, sim, mutation, or query). A validation region must not rely on fitting entirely as raw voxels.
- **Responsive matter.** Voxel-backed object behavior, granular response, active fluids and material interactions, ambient fire/wetness/growth, and structural integrity/collapse are reusable substrate world services—not game rules.
- **Construction and spatial services.** Placement/construction primitives, reusable structure representations, matter-participating mechanism objects, derived room/structure semantics, and mutation-safe 3D navigation and spatial queries are substrate outcomes. Building UI, labor policy, gameplay, and authored content remain consumer-owned.
- **Live world operations.** Streaming around active agents as well as cameras; mutation through commands/verbs with mirror queries and events (not an immediate-consistency promise across consumer views); collision against voxel truth; persistence as generated truth plus mutation deltas, journals for substrate-owned objects and state, and reuse of persisted changes across runs or modes.

## Future products and enabling implications

Future consumers (not this product) include a System/LLM-driven ARPG, a fortress or colony mode, a descent experience, and pure sandbox modes. They motivate a clean substrate API, full matter mutability and response, construction and spatial services, deep Z, and seams for pricing or content policy above the substrate. Their gameplay, controllers, characters, spells, gas policy, combat, AI, building UX, and authored content stay consumer-owned. Delivery depth and sequence are design concerns, not identity changes.

## Non-goals

- Shipping the actual game or its rules, progression, or modes in this repository.
- Implementing System/LLM integration, spells, gas policy, combat, AI agents, or building-game layers (UI, labor, content policy) here.
- Treating harness presentation, controllers, seed content, demo scenes, machine targets, or numeric gates as substrate product requirements.
- Making the product’s identity a single playable demo rather than the reusable crate substrate.

## Confirmed vision constraints

- **Rust integration boundary.** Delivered as a Rust crate or small family of tightly scoped Rust crates for game consumers in that ecosystem.
- **GPU-resident substrate.** World matter of load-bearing interest lives in a GPU-resident design, not a CPU-only voxel toy.
- **Strict consumer boundary.** The in-repo harness and every external game share the same public interfaces; privileged access is forbidden.
- **No game-layer implementation.** Game rules and the listed future game layers stay out; only seams required by the substrate may be designed.
- **Substrate, not game.** Reuse across multiple game styles is a product goal; game-specific policy lives above.
- **Required adjacent harness.** Product One is a required walkable validation and benchmark slice adjacent to the substrate, not optional and not identity-defining.

## Deferred design decisions

- Crate split, internal layering, APIs, storage layout, meshing approach, and sim algorithms.
- Voxel resolution, LOD, streaming rings, and persistence encoding details.
- Delivery depth and sequence within the matter, construction, and spatial outcome families; fidelity-vs-cost tradeoffs.
- Harness content, controls, platforms, and acceptance numbers (harness-owned).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is multiplayer / server-authoritative readiness a product-level compatibility constraint for the substrate (command/verb boundary and no privileged direct mutation), even though multiplayer itself is not a current delivery?

*Proposed answer:* Yes. Keep consumer mutation and observation on command/verb, mirror, and event paths that remain server-authoritative-ready; do not require shipping multiplayer, netcode, or hosted play.

*If different:* Treating multiplayer readiness as out of scope would allow single-player-only assumptions at the integration boundary and could drop “no direct voxel touch / ordinary client” force from product compatibility statements. Treating full multiplayer as current delivery would expand the product into networked play, which the seeds do not require.

## Seed synthesis

- **README.md** — Names Moria as a GPU-resident voxel-world substrate consumed as a Rust crate and identifies the walkable executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Locks product identity to the reusable Rust substrate, places the real game outside the repo, requires any harness to use public interfaces only, and excludes game rules and the listed future game layers.
- **docs/seeds/product-one-seed.md** — Defines the required first adjacent walkable demo and benchmark slice; motivates substrate mutability and public APIs without importing harness content, controls, platforms, or gates into substrate scope. Its partial matter slice does not shrink the substrate’s full responsive-matter responsibility.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families: natural look with voxel truth, full mutability and deep Z, geology generation with mandatory sparse lazy scale, responsive matter, construction and spatial services, consumer-control seams with stale mirror plus events, streaming and full persistence lifecycle, multi-game reuse, and the open multiplayer-readiness question.
