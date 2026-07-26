# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate delivered as public crate interfaces. This repository ships that substrate for games and other independent consumers; it does not ship a game.

## Purpose

Moria exists so multiple downstream products can create, stream, inspect, edit, and persist the same kind of authoritative voxel world without reimplementing world truth or reaching into private engine internals. A minimal validation executable exists to exercise those public interfaces, not to define gameplay.

## Product boundary

- Moria owns deterministic seed-based generation, material storage, bounded streaming, bounded mutation, surface extraction, authoritative persistence, and read-only diagnostics as a multi-consumer substrate.
- The public surface lets consumers create and identify a world, request bounded regions, observe readiness, query material truth, submit bounded edits, and persist deltas—without access to storage, meshing, or scheduler internals.
- Validation is an adjacent delivery: headless fixtures and a small visual executable must use only that public surface (free-fly camera and exercise diagnostics allowed). They are not a game prototype and own no privileged world path.
- Game rules, combat, inventory, AI, narrative, characters, animation, authored levels, production content, and later Product One experiences remain outside Moria.

## Required product outcomes

- Consumers create and identify worlds from versioned generation parameters and a seed, and receive the same generated material result for the same inputs.
- Consumers request bounded regions, observe streaming readiness and lifecycle (including bounded residency and generation-tagged background results so stale work cannot overwrite newer truth), and read bounded authoritative material observations—optionally with non-game registered objects in queries that do not become game entities.
- Consumers submit bounded mutation commands with explicit admission failure or atomic commit and observable revision; failures stay typed and visible on the public surface.
- Consumers obtain surface extraction for rendering and persist or restore authoritative material deltas so restored material matches committed truth; derived meshes and diagnostics never become world truth.
- Read-only diagnostics report lifecycle and bounded work without exposing mutable internal handles.
- Headless fixtures cover generation, query, mutation, persistence, and lifecycle; a small visual fixture shows a relocated external consumer can render and edit through the public API only. Performance may be reported with machine identity; no machine-specific correctness threshold is promised.

## Future products and enabling implications

A separate later Product One may consume Moria as a third-person explorer in a generated region (hills, mixed forest, river, cave) with its own animation and curated traversal. That product is a future consumer in a different repository. It motivates keeping Moria multi-consumer and public-API-only; it does not put controllers, character meshes, animation, forest population, curated routes, or game assets into Moria’s current scope.

## Non-goals

- Implementing any game, game prototype, or player experience in this repository.
- Privileged or internal-only paths for validators or first-party games.
- Authoritative status for derived meshes or diagnostics.
- Machine-specific performance pass/fail as product correctness.
- Shipping Product One content, controls, or presentation inside Moria.

## Confirmed vision constraints

- Integration form is Rust public crate interfaces for external crates and a minimal validation executable.
- External consumers must not reach into storage, meshing, or scheduler internals.
- Generation is deterministic for the same versioned parameters and seed.
- Mutation commits are atomic through the bounded command surface; persistence restores the same authoritative material state.
- Streaming bounds resident work, exposes observable lifecycle states, and keeps failures typed and consumer-visible.
- Only current substrate commitments are repository deliverables; embedded later-product vision does not expand current scope.

## Deferred design decisions

- Internal sparse storage, streaming schedules, meshing approach, and persistence encoding.
- Crate and workspace layout used to enforce the public boundary.
- Exact public API shapes, command and identity schemas, and diagnostic payloads.
- Fixture and harness presentation beyond public-API-only use, free-fly exercise affordances, and non-game status.
- Performance budgets, benchmark workloads, or target environments beyond reporting with machine identity.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names Moria as the reusable voxel-world substrate, limits this repository to current substrate commitments, and treats the interface reference as non-expanding context.
- `docs/seeds/mixed-project-brief.md` — Binding current identity, public boundary, correctness, validation delivery, non-goals, and the Product One future-consumer relationship without transferring game scope into Moria.
- `docs/seeds/substrate-interface-reference.md` — Supporting detail on world identity, query, mutation, streaming, persistence, and diagnostics behavior that fleshes outcomes without adding deliverables.
