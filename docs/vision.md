# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate, consumed through public crate interfaces by games and by a minimal validation executable. This repository delivers that substrate, not any particular game.

## Purpose

Give multiple independent downstream consumers a shared foundation for deterministic voxel worlds—generation, sparse material storage, streaming, mutation, surface extraction, persistence, and read-only diagnostics—so each consumer integrates through the public crate boundary without reimplementing substrate responsibilities or depending on privileged internals.

## Product boundary

- **In product:** deterministic seed-based generation; sparse voxel material storage; bounded streaming; mutation; surface extraction; persistence of authoritative material state; read-only diagnostics; the public crate surface for create/identify world, request bounded regions, observe readiness, query material truth, submit bounded edits, and persist deltas.
- **In repository, outside product identity:** a minimal validation executable and fixtures that use exactly those public interfaces. Delivery of that adjacent harness is part of this repository’s current commitments; its camera, presentation, relocation, workloads, and fixture protocols are not substrate product scope. The harness is not a game prototype and owns no privileged world path.
- **Out of product:** game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, production content, player controllers, and later game-facing work such as Product One.
- **Consumer exclusion:** consumers must not reach into storage, meshing, or scheduler internals.

## Required product outcomes

- Worlds can be created and identified; generation is deterministic for the same versioned parameters and seed; capabilities remain useful to multiple downstream consumers.
- Consumers can request bounded regions, observe readiness, and query authoritative material truth. Streaming bounds resident work and exposes observable lifecycle states. Background results carry generation identities so stale work cannot replace newer truth.
- Bounded mutations are admitted through the public command surface and committed atomically. Failures remain typed and observable to public consumers.
- Surface extraction is provided by the substrate; derived meshes never become authoritative world state.
- Persistence records authoritative material deltas and restores the same authoritative material state. Read-only diagnostics never become authoritative world state.
- Adjacent validation in this repository exercises the public product boundary and does not use a privileged world path.

## Future products and enabling implications

After the reusable substrate ships, a separate Product One repository may place a third-person explorer in a generated region with hills, a dense mixed forest, a river, and a cave, and may use skeletal animation and a curated cliff-to-cave traversal. That is future-consumer context only. It pressures the substrate to remain a reusable public-API world foundation; it does not authorize controllers, character meshes, animation clips, forest population workloads, curated routes, or game assets in Moria.

## Non-goals

- Game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content inside Moria.
- Shipping Product One or any other game as this repository’s product identity.
- Treating the validation executable as a game prototype or granting it a privileged world path.
- Machine-specific performance correctness thresholds as product truth.

## Confirmed vision constraints

- Integration is through Rust public crate interfaces for games and the validation executable.
- Authoritative world state is material state; derived meshes and diagnostics are never authoritative.
- Validation and other external consumers share the public boundary; neither receives a privileged world path.
- No machine-specific performance correctness threshold is part of product truth.

## Deferred design decisions

- Public API shapes, command and identity encodings, and concrete lifecycle state catalogs.
- Storage layout, generation algorithms, streaming budgets and scheduling, surface-extraction approach, and persistence encoding.
- How validation fixtures are structured beyond the mandate to exercise the public boundary.
- Package or crate structure used to enforce the no-privileged-access boundary.
- Delivery depth and sequence within the approved substrate scope.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names Moria as the reusable voxel-world substrate, limits repository deliverables to current substrate commitments, and marks the interface reference as non-expanding support.
- `docs/seeds/mixed-project-brief.md` — Binding current-product identity, public boundary, correctness and validation commitments, non-goals, and explicit future-consumer framing for Product One.
- `docs/seeds/substrate-interface-reference.md` — Supporting semantics for world identity, queries, mutations, streaming, persistence, and diagnostics without adding deliverables or expanding scope.
