# Project vision

## What we are building now

**Moria** is a reusable voxel-world substrate. Downstream games and tools consume it only through public interfaces. This repository delivers that substrate—not a playable game, controller, character, or authored world experience.

An adjacent minimal validation executable is a required delivery that exercises the same public interfaces. It is not the product and owns no privileged world path.

## Purpose

Moria exists so multiple independent consumers can share one correct, inspectable material world foundation: create and identify worlds, stream and observe bounded regions, query material truth, apply bounded edits, extract renderable surfaces, persist authoritative deltas, and read diagnostics—without each game reimplementing world authority or reaching into substrate internals.

## Product boundary

**In product:** the reusable substrate and its public consumer surface for world identity, generation, bounded residency/streaming, material query and mutation, surface extraction, persistence of authoritative material state, and read-only diagnostics useful to many consumers.

**Adjacent, not identity:** the minimal validation executable and headless/visual fixtures that prove public-API use (including that a relocated external consumer can render and edit through it). Their specific camera, presentation, workloads, and performance gates are not product scope.

**Out of product:** any particular game; player control, characters, animation, combat, inventory, AI, narrative, authored levels, production content, and game-specific policy.

## Future products and enabling implications

**Product One** is a later, separate game-facing demo/repository: a third-person explorer in a generated region (hills, mixed forest, river, cave) that may use skeletal animation and a curated traversal to show the world. It is a future consumer of Moria, not a current deliverable.

Enabling implication only: the substrate must remain general enough that such a consumer can generate, stream, query, edit, surface-extract, and persist material worlds through the public API. Product One’s controller, character, animation, route, population, and assets stay consumer-owned and are not deferred substrate work.

## Non-goals

- Implementing game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content.
- Shipping a game prototype or privileged in-tree consumer path that bypasses the public interface.
- Treating derived meshes, diagnostics, or validation presentation as authoritative world state.

## Confirmed vision constraints

- External consumers—including validation—use only public interfaces; they must not reach into storage, meshing, or scheduler internals.
- Generation is deterministic for the same versioned parameters and seed; mutations commit atomically through a bounded public command path; persistence restores the same authoritative material state.
- Derived meshes and diagnostics never become authority; streaming bounds resident work, exposes observable lifecycle, rejects stale background results, and surfaces typed failures to public consumers.
- Performance may be reported with machine identity; this vision does not set machine-specific correctness thresholds.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds agree that current delivery is the reusable substrate plus adjacent public-API validation, and that Product One is a later separate consumer.

## Seed synthesis

- **`README.md`:** Names the product Moria; points current authority at the program brief; limits repository deliverables to substrate commitments; treats the interface reference as supporting context that does not expand scope.
- **`docs/seeds/mixed-project-brief.md`:** Defines current identity (reusable Rust voxel-world substrate), public consumer boundary, correctness and validation commitments (including required adjacent validation executable/fixtures), explicit non-goals, and embeds Product One as non-authorizing future-consumer context.
- **`docs/seeds/substrate-interface-reference.md`:** Supports the brief with consumer-facing interface intent (identity, readiness/material observations, bounded mutations, streaming lifecycle observability, delta persistence, non-entity registered objects, non-mutable diagnostics) without adding deliverables or widening product scope.
