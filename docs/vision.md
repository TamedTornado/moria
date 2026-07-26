# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate, delivered in this repository as public crate interfaces for games and other tools. This repository ships that substrate—not a particular game or demo product.

## Purpose

Give multiple downstream consumers a shared, deterministic foundation for voxel worlds so each consumer does not reimplement generation, storage, streaming, mutation, surface extraction, persistence, or diagnostics. The substrate stays useful across independent games and tools while keeping world material truth under a single public contract.

## Product boundary

**In product scope.** World create and identity; deterministic seed-based generation; sparse voxel storage; bounded region request and streaming; material query and readiness; bounded mutation; surface extraction; persistence of authoritative material state; read-only diagnostics; and a public crate surface that admits no privileged path into storage, meshing, or scheduler internals.

**Adjacent program deliveries (not product identity).** Headless fixtures that exercise generation, query, mutation, persistence, and lifecycle through the public API. A small visual fixture that shows a relocated external consumer can render and edit through that same public API. A minimal validation executable that uses only those public interfaces; it is not a game prototype and owns no privileged world path. It may include a free-fly camera and diagnostics sufficient to exercise the crate.

**Out of product scope.** Any particular game, including a later Product One explorer demo; game rules, controllers, characters, animation, authored levels, production content, and game-specific presentation or policy.

## Required product outcomes

- **Public multi-consumer substrate.** Downstream crates and executables create and identify worlds backed by sparse voxel storage; request bounded regions; observe readiness and streaming lifecycle; query authoritative material; submit bounded edits; and persist deltas—without reaching into storage, meshing, or scheduler internals.
- **Deterministic generation and safe concurrency of truth.** The same versioned parameters and seed produce the same generation. Background results carry generation identities so stale work cannot replace newer truth. Failures are typed and observable to public consumers.
- **Bounded streaming with observable lifecycle.** Resident work stays bounded. Streaming exposes requested, loading, resident, evicted, and failed states to public consumers.
- **Authoritative material, non-authoritative derived views.** Mutation is admitted through a bounded command API and committed atomically, with explicit bounds, admission failures, and commit revisions. Persistence records and restores the same authoritative material state via deltas. Derived meshes and diagnostics never become authoritative world state.
- **Surface extraction and read-only diagnostics.** Consumers can obtain extracted surfaces for their own use. Diagnostics report lifecycle and bounded work without exposing mutable internal handles.
- **Queryable registered objects without game-entity status.** Objects may register to participate in queries without becoming game entities owned by the substrate.

## Future products and enabling implications

After the substrate ships, a separate Product One repository may host a third-person explorer in a generated region (hills, dense mixed forest, river, cave), possibly with skeletal animation and a curated cliff-to-cave traversal. That is future-consumer context only. It does not place a player controller, character mesh, animation clips, forest population workload, curated route, or game assets into Moria. Enabling implication: the substrate’s public world, material, streaming, mutation, extraction, and persistence outcomes must remain suitable for such independent game repositories.

## Non-goals

- Game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content inside Moria.
- Treating the validation executable or visual fixture as a game prototype or as privileged substrate surface.
- Machine-specific performance correctness thresholds as product pass/fail gates.
- Shipping Product One (or any other game) from this repository.

## Confirmed vision constraints

- Implementation ecosystem: Rust public crate interfaces for external consumers.
- World identity combines format version, generation parameters, and seed.
- Consumers—including validation—use only the public API; no privileged world path.
- Authoritative state is material truth; meshes and diagnostics are never authoritative.
- Persistence restores authoritative material state from deltas, not from derived meshes.
- Performance reporting includes machine identity; this vision does not set a machine-specific correctness threshold.

## Deferred design decisions

- Concrete API shapes, crate packaging, algorithms, data layouts, and streaming/eviction policy details.
- Surface-extraction and meshing design beyond the non-authority rule.
- Depth and presentation of the visual fixture (beyond free-fly and diagnostics permission) and any performance workloads or numeric gates.
- How registered objects are modeled beyond query participation without game-entity status.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names Moria as the reusable voxel-world substrate of the mixed brief, limits repository deliverables to current substrate commitments, and marks the interface reference as non-scoping support.
- `docs/seeds/mixed-project-brief.md` — Binding current identity, public boundary, correctness and validation commitments, non-goals, and explicitly demoted later Product One consumer vision.
- `docs/seeds/substrate-interface-reference.md` — Supporting outcome detail for world identity, query/mutation/streaming/persistence shape, registered objects, and diagnostics without adding deliverables or redefining product identity.
