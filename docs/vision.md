# Project vision

## What we are building now

**Moria** is a reusable Rust voxel-world substrate delivered in this repository as public crate interfaces. Downstream games and tools consume that surface; this repository does not deliver any particular game.

## Purpose

Moria exists so multiple independent consumers can share one correct foundation for generated voxel worlds: create and identify worlds, stream and observe material truth, apply bounded edits, extract non-authoritative surfaces, and persist authoritative state—without reaching into substrate internals or treating derived data as truth.

## Product boundary

**In product (Moria):** world create/identify; deterministic seed-based generation; sparse voxel material authority; bounded streaming with observable lifecycle; mutation admission and commit; surface extraction as derived output; persistence of authoritative material deltas; read-only diagnostics—all only through public crate APIs, kept useful to multiple consumers.

**Adjacent, not product identity:** a minimal validation executable and fixtures that exercise the substrate solely via those public interfaces, with no privileged world path. Headless fixtures covering generation, query, mutation, persistence, and lifecycle behavior, plus a small visual fixture that can render and edit through the public API, are current program deliveries outside the product identity. That visual fixture may include a free-fly camera and diagnostics sufficient to exercise the crate; it is not a game prototype. Validation reports performance with machine identity without defining a correctness threshold.

**Out of product:** any game, gameplay systems, player controllers, characters, animation, authored levels, production content, presentation policy, and consumer-chosen environments or performance gates. Consumers must not reach into storage, meshing, or scheduler internals.

## Required product outcomes

- **World identity and generation.** Consumers create and identify worlds whose identity combines format version, generation parameters, and seed. Generation is deterministic for the same versioned parameters and seed.
- **Bounded observe and stream.** Consumers request bounded regions, observe readiness, and obtain bounded authoritative material observations. Streaming bounds resident work, exposes observable lifecycle states, and protects newer truth from stale background results via generation identities. Registered objects may participate in queries without becoming game entities.
- **Bounded mutation.** Consumers submit edits only as bounded commands; admission may fail observably; accepted mutations commit atomically and yield commit revisions.
- **Authoritative persistence.** Consumers persist authoritative material deltas and restore the same authoritative material state. Derived meshes and diagnostics never become authoritative world state.
- **Derived surfaces and diagnostics.** The substrate provides surface extraction from material truth as non-authoritative derived results, and read-only diagnostics that report lifecycle and bounded work without exposing mutable internal handles. Failures are typed and observable on the public surface.
- **Shared public integration.** Games and validation use the same public crate surface; capabilities remain useful to multiple downstream consumers without privileged internal access.

## Future products and enabling implications

After the reusable substrate ships, a separate **Product One** repository may host a third-person explorer demo in a generated region (hills, mixed forest, river, cave) with skeletal animation and a curated cliff-to-cave traversal. That game is a future consumer, not Moria.

**Enabling implication only:** Moria’s public world, generation, streaming, query, mutation, surface, and persistence outcomes must remain sufficient for such a later game to build on. Product One’s controller, character, animation, route, content, and presentation stay out of Moria.

## Non-goals

- Game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content.
- Shipping any particular game or game prototype in this repository.
- Elevating meshes, diagnostics, or other derived data to authoritative world state.
- Privileged validation or game access into storage, meshing, or scheduler internals.
- Machine-specific performance correctness thresholds for the substrate.

## Confirmed vision constraints

- The product is a **Rust** substrate consumed as **public crate interfaces**.
- Generation determinism is defined over matching versioned parameters and seed.
- Mutation commits are atomic; streaming bounds resident work; failures are typed and consumer-observable.
- Only material truth (including restored deltas) is authoritative; meshes and diagnostics are not.
- Validation consumers own no privileged world path and must use the public interfaces exactly.
- The program does not establish a machine-specific performance correctness threshold for the substrate.

## Deferred design decisions

- Algorithms, data layouts, crate packaging, API shape, and enforcement structure for the public surface.
- Streaming, generation, surface-extraction, and persistence mechanisms and encodings.
- Delivery depth and sequence of substrate capabilities within the product identity.
- Validation presentation details (for example camera and on-screen diagnostics) beyond public-API-only exercise of the crate.
- Any portable performance targets or benchmark workloads.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds define one current product (the Moria substrate), settle adjacent validation as program delivery outside product identity, and place Product One as a future consumer without transferring game scope into Moria.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable voxel-world substrate in this repository, limits deliverables to current substrate commitments, and treats the interface reference as non-expanding context.
- **`docs/seeds/mixed-project-brief.md`:** Binding current-product identity, public boundary, correctness and validation commitments, non-goals, and embedded later Product One consumer context that does not enlarge Moria.
- **`docs/seeds/substrate-interface-reference.md`:** Supports outcome-level public-surface mandates (identity, query, mutation, streaming lifecycle, persistence authority, registered objects, diagnostics) without adding deliverables or redefining product identity.
