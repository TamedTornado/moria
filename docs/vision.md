# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate delivered as public crate interfaces. This repository ships the substrate and an adjacent validation executable that exercises those interfaces only. It does not ship a game.

## Purpose

Provide a stable, multi-consumer foundation for generating, streaming, querying, mutating, and persisting voxel worlds so downstream games and tools can build on shared material truth without owning world-engine internals.

## Product boundary

**Moria owns:** world identity and deterministic seed-based generation; sparse material storage; bounded region request and streaming lifecycle; atomic mutation admission and commit; surface extraction as a derived view; persistence of authoritative material deltas; and read-only diagnostics exposed to consumers.

**Adjacent delivery, not product identity:** a minimal validation executable and headless fixtures that consume only the public API to demonstrate generation, query, mutation, persistence, lifecycle, and that an external consumer can render and edit through the same path. Validation may include a free-fly camera and diagnostics sufficient to exercise the crate; it is not a game prototype and has no privileged world access.

**Downstream / excluded from Moria:** game rules, combat, inventory, AI, narrative, characters, animation, authored levels, production content, player controllers, curated routes, and any particular game’s presentation or policy.

External consumers create and identify worlds, request bounded regions, observe readiness, query material truth, submit bounded edits, and persist deltas. They must not reach storage, meshing, or scheduler internals.

## Required product outcomes

1. **Multi-consumer public surface** — Games and tools integrate solely through versioned public crate interfaces; no privileged internal path is required or available to any consumer, including validation.
2. **Deterministic material worlds** — The same versioned generation parameters and seed yield the same authoritative material state.
3. **Bounded, observable residency** — Consumers request regions, observe lifecycle states (including failure), and see resident work kept within bounds; stale background results cannot overwrite newer truth.
4. **Safe mutation and persistence** — Edits enter through a bounded command API, commit atomically with observable outcomes, and persist so restore recovers the same authoritative material state—not derived meshes or diagnostics.
5. **Derived views stay non-authoritative** — Surface extraction and diagnostics support consumers without becoming world truth.
6. **Provable public usability** — Headless coverage of generation, query, mutation, persistence, and lifecycle, plus a small visual fixture proving an external consumer can render and edit through the public API. Performance may be reported with machine identity; no machine-specific pass/fail threshold is part of the product promise.

## Future products and enabling implications

A separate Product One may later place a third-person explorer in a generated region (hills, forest, river, cave) with skeletal animation and a curated traversal. That is a future consumer repository, not Moria scope.

Enabling implication only: the substrate’s public generation, streaming, material query/mutation, and persistence outcomes must remain usable by such a game without embedding its controller, character, content, or presentation in Moria.

## Non-goals

- Implementing any game, prototype gameplay loop, or production content pack
- Game systems: rules, combat, inventory, AI, narrative, characters, animation
- Authored levels, curated demo routes, or forest/population design workloads
- Privileged validation paths or treating the visual fixture as a shippable game
- Machine-specific performance gates as correctness criteria

## Confirmed vision constraints

- Implementation ecosystem: Rust public crates consumed by Rust (and crate-linked) clients
- Determinism: same versioned parameters and seed → same generation outcome
- Authority: only material state is authoritative; meshes and diagnostics are not
- Isolation: consumers, including validation, use the public boundary only
- Streaming: bounded residency with typed, observable failures and generation-aware background results
- Validation is a required adjacent delivery that must not expand product identity into a game

## Deferred design decisions

- Concrete API shapes, crate layout, and scheduling/meshing mechanisms
- Exact streaming bounds, residency policy parameters, and command admission limits
- Persistence format details and surface-extraction technique
- Validation fixture interaction model beyond “public API only” (e.g. camera affordances)
- Performance measurement methodology and any later non-binding budgets

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds define one current product (the substrate), require validation as adjacent delivery without importing game scope, and clearly separate Product One as a later consumer.

## Seed synthesis

- **README.md** — Names Moria as the reusable voxel-world substrate, confines deliverables to current substrate commitments, and marks the interface reference as non-expanding context.
- **docs/seeds/mixed-project-brief.md** — Binding source for product identity, public boundary, correctness and validation commitments, non-goals, and the embedded later Product One consumer vision without transferring game scope into Moria.
- **docs/seeds/substrate-interface-reference.md** — Supporting surface semantics (identity, query, mutation, streaming lifecycle, persistence, diagnostics) that inform outcome language without adding deliverables.
