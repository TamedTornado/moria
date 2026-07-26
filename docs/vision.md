# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate. Downstream games and a minimal validation executable consume it only through public crate interfaces. This repository’s product is the substrate itself, not a game.

## Purpose

Moria exists so multiple independent consumers can share one authoritative voxel world: create and identify worlds, stream and observe bounded regions, read material truth, apply bounded edits, extract surfaces, persist and restore authoritative material state, and inspect lifecycle and work through read-only diagnostics—without each consumer reinventing world machinery or depending on another game’s internals.

## Product boundary

- **In product:** deterministic seed-based generation, sparse voxel storage, bounded streaming, mutation, surface extraction, persistence of authoritative material state, and read-only diagnostics—exposed so consumers can create and identify a world, request bounded regions, observe readiness and lifecycle, query material truth, submit bounded edits, persist deltas, and obtain diagnostics without mutable internal handles.
- **Authoritative vs derived:** only material state is authoritative; derived meshes and diagnostics never become world truth.
- **Out of product:** game rules, combat, inventory, AI, narrative, characters, animation, authored levels, production content, player controllers, curated routes, and game-facing presentation policy.
- **No privileged consumer path:** adjacent code must not reach storage, meshing, or scheduler internals.
- **Validation is adjacent:** the validation executable and fixtures consume the public interface; they are not product identity. Their controllers, camera mode, content, presentation, routes, workloads, platforms, and performance gates are not substrate scope.
- **Later games stay separate:** Product One or any later game repository’s gameplay, assets, or demo scenario are not Moria.

## Required product outcomes

- **Shared reusable substrate.** Capabilities remain useful to multiple downstream consumers through the same public crate interfaces.
- **Authoritative, deterministic material world.** For the same versioned parameters and seed, generation yields the same material truth. World identity combines format version, generation parameters, and seed. Queries return readiness and bounded authoritative material observations. Persistence records authoritative deltas (not derived meshes) and restores that material state.
- **Bounded, observable mutation.** Edits enter through a bounded command API with admission failures and commit revisions; commits are atomic. Background results carry generation identities so stale work cannot replace newer truth.
- **Bounded streaming with visible lifecycle.** Resident work is bounded. Streaming exposes requested, loading, resident, evicted, and failed states. Failures are typed and observable on the public surface.
- **Non-authoritative derivation and inspection.** Surface extraction and diagnostics never become world truth. Registered objects may participate in queries without becoming game entities. Diagnostics report lifecycle and bounded work without mutable internal handles.
- **Validation through the public surface.** Headless fixtures cover generation, query, mutation, persistence, and lifecycle behavior. A small visual fixture shows that a relocated external consumer can render and edit only through the public API. Performance may be reported with machine identity; this vision sets no machine-specific correctness threshold.

## Future products and enabling implications

After the substrate ships, a separate Product One repository may place a third-person explorer in a generated region (hills, dense mixed forest, river, cave) and may use skeletal animation and a curated cliff-to-cave traversal to communicate that world. That material is future-consumer context only.

Enabling implications already inside current Moria responsibility: deterministic seed-based generation and authoritative material state a later explorer can consume; public query, edit, stream, and persist paths a game may build on. Nothing in Product One authorizes player controllers, character meshes, animation clips, forest population workloads, curated routes, or game assets inside Moria.

## Non-goals

- Implementing any game, game prototype, or production content pack in this repository.
- Game systems: rules, combat, inventory, AI, narrative, characters, animation, authored levels.
- Giving consumers privileged access to storage, meshing, or scheduler internals.
- Treating derived meshes or diagnostics as authoritative world state.
- Establishing machine-specific performance pass/fail thresholds as product correctness.
- Absorbing Product One’s demo scenario, presentation, or controls into the substrate.

## Confirmed vision constraints

- Delivery form is a Rust crate (public crate interfaces) consumed by external code, not a standalone game binary as the product.
- Consumers, including validation, use only the public interface; no privileged world path.
- Generation determinism is tied to versioned parameters and seed.
- Mutation is admitted and committed with explicit bounds; persistence restores material authority.
- Streaming bounds resident work and exposes lifecycle; failures remain typed and consumer-visible.
- Validation obligations above are program commitments; they do not enlarge substrate identity into a game or harness product.

## Deferred design decisions

- Concrete public API shapes, type layouts, and package structure (beyond the outcome-level surface above).
- Generation algorithms, sparse storage representation, streaming policies and numeric bounds, surface-extraction method, and persistence encoding.
- How validation fixtures are packaged and which non-privileged camera or diagnostic UX they use.
- Depth and sequencing of substrate capability delivery within the current product identity.
- Any performance targets or benchmark workloads beyond reporting with machine identity.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names Moria as the reusable voxel-world substrate in this repository and limits deliverables to current substrate commitments, with the mixed brief as binding program authority and the interface reference as non-expanding technical context.
- `docs/seeds/mixed-project-brief.md` — Defines current product identity, public boundary, correctness and validation commitments, non-goals, and Product One as later-consumer context that must not import game systems into Moria.
- `docs/seeds/substrate-interface-reference.md` — Clarifies the consumer-facing surface (identity, query, mutation, streaming states, persistence of deltas, registered objects, diagnostics) without adding deliverables or widening scope.
