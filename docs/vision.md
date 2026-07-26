# Project vision

## What we are building now

**Moria** is a reusable Rust voxel-world substrate delivered in this repository as public crate interfaces. It is consumed by games and by a minimal validation executable; this repository does not deliver any particular game.

## Purpose

Moria exists so multiple independent consumers can share one correct foundation for generated voxel worlds: create and identify worlds, stream and observe material truth, apply bounded edits, extract surfaces, and persist and restore authoritative material state—without reaching into substrate internals or treating derived meshes or diagnostics as world truth.

## Product boundary

**In product (Moria):** world create/identify; deterministic seed-based generation; sparse voxel material authority; bounded streaming with observable lifecycle; mutation admission and commit; surface extraction; persistence of authoritative material deltas; read-only diagnostics—all only through public crate APIs, kept useful to multiple downstream consumers.

**Adjacent, not product identity:** a minimal validation executable and fixtures that exercise the substrate solely via those public interfaces, with no privileged world path. The validation executable may provide a free-fly camera and diagnostics sufficient to exercise the crate; it is not a game prototype. Headless fixtures cover generation, query, mutation, persistence, and lifecycle behavior. A small visual fixture demonstrates that a relocated external consumer can render and edit through the public API. Performance is reported with machine identity; the program establishes no machine-specific correctness threshold.

**Out of product:** any game, gameplay systems, player controllers, characters, animation, authored levels, production content, presentation policy, and consumer-chosen environments or performance gates. Consumers must not reach into storage, meshing, or scheduler internals.

## Required product outcomes

- **World identity and generation.** Consumers create and identify worlds whose identity combines format version, generation parameters, and seed. Generation is deterministic for the same versioned parameters and seed.
- **Bounded observe and stream.** Consumers request bounded regions, observe readiness, and obtain bounded authoritative material observations. Streaming bounds resident work, exposes observable lifecycle states, and protects newer truth from stale background results via generation identities.
- **Bounded mutation.** Consumers submit edits only as bounded commands; admission may fail observably; accepted mutations commit atomically and yield commit revisions.
- **Authoritative persistence.** Consumers persist authoritative material deltas and restore the same authoritative material state. Derived meshes and diagnostics never become authoritative world state.
- **Surface extraction and diagnostics.** The substrate provides surface extraction from material truth, and read-only diagnostics that report lifecycle and bounded work without exposing mutable internal handles. Failures are typed and observable on the public surface.
- **Shared public integration.** Games and validation use the same public crate surface; capabilities remain useful to multiple downstream consumers without privileged internal access.

## Future products and enabling implications

After the reusable substrate ships, a separate **Product One** repository may host a third-person explorer demo in a generated region (hills, mixed forest, river, cave) with skeletal animation and a curated cliff-to-cave traversal. That game is a future consumer, not Moria. Mentions of possible consumers, including Product One, explain interface pressure only; they do not authorize player controllers, character meshes, animation, routes, content, or game assets in Moria.

**Enabling implication only:** Moria’s substrate capabilities remain useful to multiple downstream consumers. Product One’s controller, character, animation, route, content, and presentation stay out of Moria.

## Non-goals

- Game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content.
- Shipping any particular game or game prototype in this repository.
- Elevating derived meshes or diagnostics to authoritative world state.
- Privileged validation or game access into storage, meshing, or scheduler internals.
- Establishing a machine-specific performance correctness threshold.

## Confirmed vision constraints

- The product is a **Rust** substrate consumed as **public crate interfaces**.
- Generation determinism is defined over matching versioned parameters and seed.
- Mutation commits are atomic; streaming bounds resident work; failures are typed and consumer-observable.
- Persistence restores the same authoritative material state; derived meshes and diagnostics are not authoritative world state.
- Validation consumers own no privileged world path and must use the public interfaces exactly.
- Performance reporting includes machine identity; no machine-specific correctness threshold is established.

## Deferred design decisions

- Algorithms, data layouts, crate packaging, API shape, and enforcement structure for the public surface.
- Streaming, generation, surface-extraction, and persistence mechanisms and encodings.
- Delivery depth and sequence of substrate capabilities within the product identity.
- Validation presentation details beyond public-API-only exercise of the crate (including how free-fly camera or diagnostics appear, if at all, on a given validation artifact).
- Performance targets (portable or otherwise), benchmark workloads, and whether any non-machine-specific threshold is adopted.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds define one current product (the Moria substrate), settle adjacent validation as program delivery outside product identity, and place Product One as a future consumer without transferring game scope into Moria.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable voxel-world substrate in this repository, limits deliverables to current substrate commitments, and treats the interface reference as non-expanding context.
- **`docs/seeds/mixed-project-brief.md`:** Binding current-product identity, public boundary, correctness and validation commitments, non-goals, and embedded later Product One consumer context that does not enlarge Moria.
- **`docs/seeds/substrate-interface-reference.md`:** Supports outcome-level public-surface detail (identity, material query, mutation, streaming lifecycle, persistence authority, diagnostics) without adding deliverables or redefining product identity; reference-only details such as registered-object query participation are not mandatory product scope.
