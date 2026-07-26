# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is engine material for games, not a game. A walkable-world validation harness (Product One) is a required adjacent deliverable that exercises the substrate through public interfaces only.

## Purpose

Give downstream games a shared world foundation: a natural-looking continuous 3D volume whose truth is mutable voxel matter, with deep underground as first-class space, so adventure, fortress, sandbox, and related modes can share generation, matter response, construction and spatial services, queries, mutation, streaming, and persistence without reinventing the world layer. The substrate has no LLM dependency.

## Product boundary

**This product owns** the reusable voxel-world substrate and its public consumer interfaces: geology-first world generation; GPU-resident matter representation; meshing as a non-authoritative view; mutation and query surfaces; streaming and persistence of world truth; collision against voxel truth; responsive matter (including the burn/break/block vs dressing invariant); placement and construction primitives, reusable structure representations, matter-participating mechanism objects, derived room/structure semantics; mutation-safe 3D navigation and spatial queries; consumer-control seams (commands/verbs, mirror queries, events, registries, policy injection).

**Adjacent, required validation.** Product One is a walkable-world executable and benchmark deliverable in this repository, not substrate identity. Controller, character, scene inventory, presentation detail, content palette, machine targets, and numeric gates are harness-owned. Its fused adjacent-delivery outcome is one generated natural region that is end-to-end traversable (including a continuous surface-to-underground route), smoothly presented, with collision against voxel truth, dig/place proof of material mutability, and restoration from the same generation seed plus deltas. Generation for this slice ships full; matter response and API surface are deliberately partial (dig/place and mirror queries established; broader responsive-matter and scripting remain full substrate outcomes beyond the slice). The same-seed-plus-deltas restoration proof binds this harness acceptance only and does not redefine unrelated persistence artifacts.

**Downstream / out of this repository.** Actual games are separate consumers. Game rules and the System, LLM, spell, gas, combat, AI, and building *game* layers (UI, labor policy, gameplay, authored content) are not implemented here. Substrate-required compatibility seams may be designed; those layers must not be.

## Required product outcomes

- **Reusable Rust substrate.** Consumers integrate through public crate interfaces; no privileged path to world truth. Nothing above the matter layer touches voxels directly—only public commands/verbs and queries. The System, if any, is an ordinary game-layer client, not a substrate feature.
- **Natural look, voxel truth, full mutability.** Surface worlds read as ordinary natural terrain; the voxel grid is authoritative; rendered geometry is a regenerated view. Matter can be destroyed, moved, or placed throughout the volume; deep Z is first-class; digging exposes coherent geology, not decorative void under a heightmap.
- **Matter representation invariant.** Anything that can burn, break, or block is voxel-backed. Nonmaterial dressing is anchored to voxel matter and stays synchronized with it when that matter is mutated or its state changes.
- **Sparse, lazy scale.** Untouched homogeneous space must not carry full per-cell cost; materialization is lazy on need. A validation region must not fit entirely as raw voxels.
- **Responsive matter, construction, and spatial services.** Granular response, active fluids and material interactions, ambient fire/wetness/growth, structural integrity/collapse, placement/construction primitives, reusable structure representations, matter-participating mechanism objects, derived room/structure semantics, and mutation-safe 3D navigation and spatial queries are substrate services. Building UI, labor policy, gameplay, and authored content stay consumer-owned. Product One’s partial slice does not shrink these responsibilities.
- **Live world operations.** Streaming around active agents as well as cameras; mutation through commands/verbs with mirror queries and events; collision against voxel truth; persistence as generated truth plus mutation deltas; journaling surfaces for objects, entities, and script state—including consumer-owned entity and script state the substrate journals without owning the consumer logic—and reuse of persisted changes across runs or modes.

## Future products and enabling implications

Future consumers (not this product) include a System/LLM-driven ARPG, fortress or colony mode, descent experience, and pure sandbox. They motivate a clean substrate API, full matter mutability and response, construction and spatial services, deep Z, and seams for pricing or content policy above the substrate. Their gameplay, controllers, characters, content, and game-layer policy stay consumer-owned.

## Non-goals

- Shipping the actual game or its rules, progression, or modes in this repository.
- Implementing System/LLM integration, spells, gas policy, combat, AI agents, or building-game layers here.
- Treating harness presentation, controllers, content inventory, demo scenes, machine targets, or numeric gates as substrate requirements.
- Making product identity a single playable demo rather than the reusable crate substrate.
- Weakening the public-command/query and no-direct-voxel-touch boundary for any consumer.

## Confirmed vision constraints

- **Rust integration boundary.** A Rust crate or small family of tightly scoped Rust crates for game consumers in that ecosystem.
- **GPU-resident substrate.** Load-bearing world matter lives in a GPU-resident design, not a CPU-only voxel toy.
- **Strict consumer boundary.** In-repo harness and every external game share the same public command/query interfaces; privileged or direct voxel access is forbidden regardless of multiplayer.
- **No game-layer implementation.** Game rules and listed future game layers stay out; only substrate-required seams may be designed.
- **Substrate, not game.** Multi-game reuse is a product goal; game-specific policy lives above.
- **Required adjacent harness.** Product One is a required walkable validation and benchmark slice—not optional, not identity-defining—with the fused delivery outcome and partial-versus-full slice boundary above.

## Deferred design decisions

- Crate split, internal layering, APIs, storage layout, meshing approach, and sim algorithms.
- Voxel resolution, LOD, streaming rings, and persistence encoding details.
- Delivery depth and sequence within matter, construction, and spatial outcomes beyond Product One’s partial slice; fidelity-vs-cost tradeoffs.
- Harness content inventory, controls, platforms, and acceptance numbers.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Beyond the settled rule that all consumers mutate and observe only through public commands/queries with no privileged voxel access, is additional server-authoritative multiplayer readiness (keeping those paths compatible with a future authoritative server) a product-level compatibility constraint, even though multiplayer itself is not a current delivery?

*Proposed answer:* Yes. Shape public command/verb, mirror, and event paths to remain server-authoritative-ready; do not ship multiplayer, netcode, or hosted play. The settled no-privileged-access rule stands either way.

*If different:* Declining extra readiness allows single-player-only integration assumptions harder to host later, but must not authorize privileged or direct voxel access. Treating full multiplayer as current delivery expands the product into networked play.

## Seed synthesis

- **README.md** — Names Moria as a GPU-resident voxel-world substrate consumed as a Rust crate; walkable executable is a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Locks identity to the reusable Rust substrate, places the real game outside the repo, requires public interfaces only, excludes game rules and listed future game layers.
- **docs/seeds/product-one-seed.md** — Defines the required first adjacent walkable delivery: generated traversable natural region, smooth presentation, continuous surface-to-underground route, voxel-truth collision, dig/place mutability proof, full generation with partial matter/API, same-seed-plus-deltas restoration.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcomes (natural look with voxel truth, full mutability and deep Z, sparse lazy scale, burn/break/block vs dressing invariant, responsive matter, construction and spatial services, consumer-control seams, streaming, persistence including journals for objects/entities/script state and cross-run reuse) and the open server-authoritative readiness question distinct from the settled public-interface rule.
