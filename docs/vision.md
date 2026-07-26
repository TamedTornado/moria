# Project vision

## What we are building now

**Moria** is a reusable **Rust voxel-world substrate** delivered as public crate interfaces. This repository ships that substrate—not a game. Games and a minimal validation executable consume it only through those public interfaces.

## Purpose

Moria exists so multiple independent consumers can create, stream, query, edit, and persist shared voxel worlds under the same correctness rules, without each reimplementing world identity, material truth, or privileged engine access. It supplies a stable, multi-consumer world foundation; presentation, control, and gameplay remain with each consumer.

## Product boundary

**In product:** the substrate’s public contract for world identity and lifecycle; deterministic seed-based generation; sparse voxel material truth; bounded region streaming with observable readiness; bounded mutation with atomic commit; surface extraction for consumer rendering; persistence of authoritative material deltas; and read-only diagnostics of lifecycle and bounded work. Registered non-game objects may participate in queries without becoming game entities.

**Adjacent, required delivery (not product identity):** headless fixtures covering generation, query, mutation, persistence, and lifecycle; and a small visual validation executable that exercises create/stream/query/edit/persist only through the public API, proving a relocated external consumer can do so. That executable is not a game prototype and has no privileged world path.

**Out of product:** any particular game, player control scheme, characters, animation, authored levels, production content, combat, inventory, AI, narrative, or game rules. Downstream consumers own presentation, UX, and policy.

## Required product outcomes

1. **Multi-consumer public surface.** Independent games and tools integrate via public crate APIs only: create and identify a world, request bounded regions, observe readiness, query authoritative material, submit bounded edits, and persist deltas—without access to storage, meshing, or scheduler internals.

2. **Deterministic, versioned worlds.** The same format version, generation parameters, and seed yield the same authoritative material state.

3. **Bounded, observable streaming.** Resident work stays bounded; consumers see requested, loading, resident, evicted, and failed states. Background work carries generation identity so stale results cannot overwrite newer truth. Failures are typed and visible on the public surface.

4. **Safe, atomic mutation.** Edits enter only as bounded commands; admission can fail explicitly; commits are atomic and revisioned.

5. **Authoritative persistence.** Restored worlds match persisted material truth. Derived meshes and diagnostics never become authoritative state. Persistence records material deltas, not derived geometry.

6. **Honest diagnostics and extractable surfaces.** Consumers can obtain surface data for rendering and read-only lifecycle/work diagnostics without mutable internal handles. Validation proves the contract without special substrate privileges.

## Future products and enabling implications

**Product One** (separate repository, after the substrate ships) is a future third-person explorer demo in a generated region—hills, mixed forest, river, cave—using skeletal animation and a curated traversal to showcase the world. It is a **future consumer**, not a Moria deliverable. It must not pull player controllers, character meshes, animation, forest population, curated routes, or game assets into Moria scope.

Enabling implication only: a stable multi-consumer world substrate with generation, streaming, mutation, surface data, and persistence that a later game can build on through public APIs.

## Non-goals

- Game rules, combat, inventory, AI, narrative, characters, animation, or production content
- Authored levels or a playable game prototype in this repository
- Privileged validation or consumer paths into substrate internals
- Machine-specific performance pass/fail gates as correctness criteria (performance may be reported with machine identity)

## Confirmed vision constraints

- **Ecosystem:** Rust public crate interfaces for external consumers
- **No privileged consumers:** validation and games use the same public boundary
- **Correctness:** deterministic generation; atomic bounded mutation; persistence restores material state; meshes and diagnostics are non-authoritative
- **Streaming discipline:** bounded residency, observable lifecycle, generation-tagged background results, typed public failures
- **Validation posture:** headless behavior coverage plus a small public-API visual fixture; not a game

## Deferred design decisions

- Concrete API shapes, data layouts, algorithms, crate packaging, and scheduler design
- Surface-extraction and streaming implementation strategy and depth
- Validation executable UX details (camera, diagnostics presentation) beyond “public API only, not a game”
- Performance measurement method and any non-correctness budgets
- How and when Product One or other games are scheduled relative to substrate slices

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds already fix current product identity (reusable Rust voxel substrate), the adjacent validation delivery without importing its controls or content, and Product One as a later consumer only.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable voxel-world substrate; only current substrate commitments are repository deliverables; later-product text in the brief is non-binding for scope; interface reference does not expand deliverables.
- **`docs/seeds/mixed-project-brief.md`:** Binding current identity, public boundary, correctness, validation commitments, non-goals, and explicit future-consumer (Product One) exclusion from Moria.
- **`docs/seeds/substrate-interface-reference.md`:** Supporting public-surface outcomes (identity, query, mutation, streaming states, delta persistence, non-entity registered objects, non-leaking diagnostics) without new product scope.
