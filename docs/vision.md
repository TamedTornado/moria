# Project vision

## What we are building now

**Moria** is a reusable Rust voxel-world substrate, delivered as public crate interfaces. This repository ships that substrate for consumption by games and other downstream tools—not any particular game. Adjacent validation exercises the same public boundary; it is not the product’s identity.

## Purpose

Moria exists so multiple downstream consumers can share one material-world foundation: create and identify worlds, generate material from versioned parameters and seed, stream and observe bounded regions, query authoritative material truth, apply bounded edits, extract derived surfaces, persist and restore material state, and obtain read-only diagnostics—without privileged access to substrate internals. Determinism is a generation commitment (same versioned parameters and seed yield the same generation), not a claim that every consumer shares one world or that the mutated, streamed, and persisted world as a whole is deterministic.

## Product boundary

**In product:** world creation and identity; deterministic seed-based generation; sparse voxel material storage and query; bounded streaming with observable lifecycle; bounded mutation; surface extraction as derived (non-authoritative) output; persistence of authoritative material deltas; read-only diagnostics; the public consumer-facing interface contract (including registered objects that can participate in queries without becoming game entities).

**Adjacent, not product identity:** a minimal validation executable and fixtures that consume only public interfaces and hold no privileged world path. This repository includes an adjacent validation floor: headless coverage of generation, query, mutation, persistence, and lifecycle; a small visual fixture proving a relocated external consumer can render and edit through the public API; and performance reporting with machine identity. Harness controls, presentation, exact fixture protocols and counts, workloads, and machine-specific performance gates remain adjacent design, not product scope.

**Out of product:** any game, game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, production content, player controllers, or curated gameplay routes. Possible later games explain interface pressure only.

## Required product outcomes

- Consumers create and identify a world whose identity combines format version, generation parameters, and seed; request bounded regions; observe readiness; query bounded authoritative material observations with readiness; submit bounded edits; and persist deltas—through public crate interfaces only, without reaching into storage, meshing, or scheduler internals.
- Generation is deterministic for the same versioned parameters and seed; mutation is admitted through a bounded command API (with explicit bounds, admission failures, and commit revisions) and committed atomically; failures are typed and observable to public consumers.
- Material is held in sparse voxel storage that remains useful to multiple downstream consumers of the same substrate.
- Streaming bounds resident work, exposes observable lifecycle states (including requested, loading, resident, evicted, and failed), and keeps background results from replacing newer truth when generation identities show work is stale.
- Persistence restores the same authoritative material state from authoritative deltas; derived meshes and diagnostics never become authoritative world state.
- Registered objects can participate in queries without becoming game entities.
- Diagnostics report lifecycle and bounded work without exposing mutable internal handles.
- Adjacent validation proves the public boundary: headless coverage of generation, query, mutation, persistence, and lifecycle; a small visual fixture that a relocated external consumer can render and edit through the public API; performance reported with machine identity (no machine-specific correctness threshold from this vision).

## Future products and enabling implications

A separate Product One repository may later place a third-person explorer in a generated region (hills, dense mixed forest, river, cave), using skeletal animation and a curated cliff-to-cave traversal to communicate the world. That material is future-consumer context only.

**Enabling implication:** Moria’s public world, generation, streaming, mutation, surface, persistence, registered-object query participation, and diagnostics outcomes are intended to support such later games without embedding their controllers, characters, animation, content, presentation, or gameplay policy in this repository.

## Non-goals

- Implementing game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content.
- Treating the validation executable as a game prototype or granting it privileged substrate access.
- Establishing machine-specific performance correctness thresholds from this vision.
- Delivering Product One (or any particular game) in this repository.
- Promising whole-world determinism beyond generation, or that multiple consumers share a single world instance.

## Confirmed vision constraints

- Integration surface is public Rust crate interfaces; consumers have no privileged access to storage, meshing, or scheduler internals.
- Sparse voxel storage is a required product property of material holding (mechanism left to design).
- World identity combines format version, generation parameters, and seed; material queries provide bounded authoritative observations with readiness.
- Adjacent validation uses exactly those public interfaces, holds no privileged world path, and meets the delivery floor above.
- This vision does not set a machine-specific performance correctness bar.

## Deferred design decisions

- Delivery depth and sequence across substrate capabilities (first slice vs later depth).
- Algorithms, internal module structure, concrete API shapes, and the mechanism that implements sparse storage.
- Exact streaming residency policy, lifecycle presentation, and generation-identity mechanics beyond the outcome guarantees above.
- Exact validation fixture counts, protocols, visual harness UX, workloads, and performance measurement protocol beyond reporting with machine identity and the required proof kinds above.
- Shape and lifecycle of registered objects beyond query participation without game-entity status.

## Assumptions proposed for approval

None.

## Questions for human review

None. The authorized seeds agree that the current product is the reusable substrate; validation is an adjacent public-boundary consumer with the repository delivery floor above; Product One is a later separate consumer and does not expand Moria’s scope.

Other files under `docs/seeds/` (for example older substrate or Product One seeds) exist in the tree but are outside the seed-document manifest named by the repository README for this synthesis. They were not used to redefine product identity or scope.

## Seed synthesis

- **`README.md`** — Names Moria as the reusable voxel-world substrate defined by the mixed project brief; only current substrate commitments are repository deliverables; the interface reference is supporting and non-expanding.
- **`docs/seeds/mixed-project-brief.md`** — Binding current-product identity (including sparse voxel storage), public boundary, generation determinism, mutation/persistence/streaming correctness, validation delivery floor, non-goals, and explicit future-consumer (Product One) separation.
- **`docs/seeds/substrate-interface-reference.md`** — Supporting vocabulary for world identity (format version, parameters, seed), bounded query observations with readiness, mutation commands (bounds, admission failures, commit revisions), streaming lifecycle states, persistence of authoritative deltas, registered-object query participation without game entities, and diagnostics without mutable internal handles; does not add deliverables or expand scope.
