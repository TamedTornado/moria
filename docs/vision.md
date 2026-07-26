# Project vision

## What we are building now

**Moria** is a reusable **Rust voxel-world substrate**: a library consumed through public crate interfaces by games and by a minimal validation executable. This repository delivers that substrate for multiple downstream consumers. It is not a game, demo title, or content package.

## Purpose

Provide a shared world foundation that games and a minimal validation executable integrate through public crate interfaces. Callers create and drive generated voxel worlds so generation, residency, mutation, material truth, and persistence stay coherent and useful across multiple downstream consumers.

## Product boundary

**In product:** public crate capabilities for world identity and creation; deterministic seed-based generation; sparse voxel storage of authoritative material; bounded region request and streaming with observable readiness; bounded mutation; surface extraction derived from material truth; persistence of authoritative material deltas; read-only diagnostics.

**Outside product identity, required as adjacent validation in this program:** headless fixtures that cover generation, query, mutation, persistence, and lifecycle; a small visual fixture that shows a relocated external consumer can render and edit only through the public API. Those artifacts are not the product and own no privileged world path.

**Outside this product entirely:** any particular game; gameplay rules, combat, inventory, AI, narrative, characters, animation, controllers, authored levels, production content, curated routes, and forest or traversal workloads. A free-fly camera or similar exercise UI may live only on an adjacent validation executable, not as substrate scope. General-purpose tools and independent tool-author integration are not current-product mandates.

Consumers must not reach into storage, meshing, or scheduler internals. There is no privileged integration path for validation or other callers.

## Required product outcomes

- **Public multi-consumer surface.** Games and a minimal validation executable consume the substrate through public crate interfaces that remain useful to more than one downstream consumer.
- **Identifiable, deterministic worlds.** A world is identified by format version, generation parameters, and seed. Generation for the same versioned parameters and seed is deterministic.
- **Bounded observation of material truth.** Consumers request bounded regions, observe readiness, and query authoritative material observations. Derived meshes and diagnostics never become authoritative world state.
- **Bounded, atomic mutation.** Edits enter only as bounded commands with explicit admission failure and atomic commit, with commit revisions observable to callers.
- **Bounded streaming with honest lifecycle.** Resident work stays bounded. Streaming exposes requested, loading, resident, evicted, and failed states. Background results carry generation identities so stale work cannot replace newer truth. Failures are typed and observable on the public surface.
- **Authoritative persistence and derived surface.** Persistence records and restores authoritative material deltas (not derived meshes). Surface extraction is available as a derived view of material truth. Registered objects may participate in queries without becoming game entities. Read-only diagnostics report lifecycle and bounded work without exposing mutable internal handles.

Adjacent validation must exercise these outcomes through the same public interfaces: headless coverage of generation, query, mutation, persistence, and lifecycle; plus a small visual demonstration that an external-style consumer can render and edit without privileged access. Performance reports include machine identity; this vision does not set machine-specific pass/fail thresholds.

## Future products and enabling implications

**Product One** (separate repository, after the substrate ships) is a future consumer: a third-person explorer in a generated region (hills, mixed forest, river, cave), possibly with skeletal animation and a curated cliff-to-cave traversal. That game’s controller, character, animation, assets, route, and presentation are not Moria work.

Moria’s enabling implication is only that a later game can generate, stream, query, mutate, extract surface, and persist a voxel world through the public substrate without privileged access to storage, meshing, or scheduler internals. No gameplay or content from Product One is in current scope.

## Non-goals

- Shipping a playable game, game prototype, or production content in this repository
- Game systems: rules, combat, inventory, AI, narrative, characters, animation, authored levels
- Privileged or internal access for any consumer, including validation
- Treating meshes, diagnostics, or validation UI as authoritative world state
- Machine-specific performance correctness gates as part of this product promise
- General-purpose tool integration as a current-product mandate

## Confirmed vision constraints

- Delivery form is a **Rust** library with **public crate interfaces**, consumed by **games** and a **minimal validation executable**, and kept useful to multiple downstream consumers
- Generation is **deterministic** for the same versioned parameters and seed
- Mutation is **bounded**, **admitted or rejected explicitly**, and **committed atomically**
- Persistence restores the **same authoritative material state**; deltas are material authority, not meshes
- Streaming **bounds resident work**, exposes **lifecycle states**, and uses **generation identities** against stale application; failures are **typed and observable** to public consumers
- Adjacent validation uses **exactly the public interfaces** and reports performance with **machine identity**

## Deferred design decisions

- Concrete API shapes, crate layout, algorithms, storage layouts, and scheduler design
- Numeric bounds, timing or memory targets, and benchmark workloads
- Validation fixture presentation (camera, HUD, platforms) beyond public-API-only exercise
- Depth and sequence of capability delivery within the substrate mandate
- How registered-object participation is modeled at the API level

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds already separate the current substrate from Product One and state validation as adjacent program commitments without redefining product identity.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable voxel-world substrate; binds current deliverables to the program brief; treats the interface reference as non-expanding support.
- **`docs/seeds/mixed-project-brief.md`:** Authoritative current product identity, public boundary (games and minimal validation executable), correctness and validation commitments, non-goals, and Product One as future-consumer context only.
- **`docs/seeds/substrate-interface-reference.md`:** Supporting public-surface detail that strengthens outcome families (identity, query, mutation, streaming states, persistence, diagnostics, registered non-entity query participation) without adding deliverables or widening scope.
