# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate delivered as public crate interfaces. This repository ships the substrate, not a game. Games and a minimal validation executable consume it only through that public surface.

## Purpose

Moria exists so multiple downstream games and tools can share one authoritative voxel world substrate: create and identify worlds, generate material truth deterministically from versioned parameters and seed, stream and edit bounded regions, derive non-authoritative surfaces, persist and restore material state, and observe readiness, lifecycle, and failures without privileged access to internals.

## Product boundary

**In product:** the reusable world substrate and its public crate contract—world identity, deterministic generation, sparse material storage, bounded streaming, mutation, surface extraction, persistence of authoritative material deltas, and read-only diagnostics.

**Adjacent, not the product:** a minimal validation executable and fixtures that exercise the public API. That validation delivery is part of the program (headless coverage plus a small visual demonstration that an external consumer can render and edit through the public API). It is not a game prototype and has no privileged world path. Its camera, presentation, and exercise-only diagnostics belong to the harness, not to Moria’s identity.

**Out of product:** any particular game, gameplay systems, controllers, characters, animation, authored content, curated routes, production assets, and later-product demos. Interface pressure from possible consumers does not transfer consumer-owned work into Moria.

## Required product outcomes

- **Reusable multi-consumer substrate.** Downstream games and tools integrate through public crate interfaces only; the substrate remains useful beyond a single title and denies privileged access to storage, meshing, or scheduler internals.
- **Deterministic world generation and identity.** A world is identified by format version, generation parameters, and seed; the same versioned parameters and seed produce the same authoritative material generation.
- **Authoritative material access and bounded change.** Consumers create worlds, request bounded regions, observe readiness, query bounded authoritative material observations, and submit bounded mutation commands that admit or fail explicitly and commit atomically under revisions. Registered objects may participate in queries without becoming game entities.
- **Bounded streaming, persistence, and observable truth.** Resident work is bounded; streaming exposes observable lifecycle states; background results carry generation identities so stale work cannot replace newer material truth; persistence records and restores authoritative material deltas (not derived meshes); failures are typed and observable on the public surface.
- **Derived views stay non-authoritative.** Surface extraction and diagnostics report derived or read-only views of lifecycle and bounded work; they never become authoritative world state and never expose mutable internal handles.
- **Program validation via public API.** Headless fixtures cover generation, query, mutation, persistence, and lifecycle behavior. A small visual fixture demonstrates that a relocated external consumer can render and edit through the same public interfaces. Performance may be reported with machine identity; that reporting does not define a machine-specific correctness threshold for the product.

## Future products and enabling implications

A separate later Product One may present a third-person explorer in a generated region (hills, mixed forest, river, cave) with skeletal animation and a curated traversal. That product is a future consumer, not Moria.

Enabling implication only: Moria’s public substrate must remain sufficient for external games to build such experiences later—generated regions, material query/edit, streaming readiness, and persistence—without Moria owning player control, character presentation, animation, population workloads, routes, or game assets.

## Non-goals

- Game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content.
- Shipping Product One (or any title) inside this repository.
- Treating validation harness presentation, free-fly camera, or exercise-only UI as substrate features.
- Machine-specific performance pass/fail gates as product correctness criteria.

## Confirmed vision constraints

- Implementation ecosystem is Rust public crates; consumers integrate at that boundary.
- Generation is deterministic for the same versioned parameters and seed.
- Mutation is bounded, admitted through the command API, and committed atomically; failures stay typed and observable.
- Derived meshes and diagnostics are never authoritative world state.
- Streaming bounds resident work and exposes lifecycle; generation identities prevent stale replacement of newer truth.
- Adjacent consumers, including validation, have no privileged world path and must not reach into storage, meshing, or scheduler internals.

## Deferred design decisions

- Concrete API shapes, crate layout, data layouts, algorithms, and persistence encodings.
- Numeric bounds for streaming/residency, exact lifecycle presentation beyond the required observability, and surface-extraction strategy details.
- Validation harness content, controls, platforms, workloads, and any performance measurement setup beyond reporting machine identity.
- Delivery sequence and depth of individual substrate capabilities within the current product.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds agree that the current product is the reusable Rust voxel-world substrate; validation is an adjacent required program delivery that uses only public interfaces; Product One and other games are future or external consumers.

## Seed synthesis

- `README.md` — Names Moria as the reusable voxel-world substrate, confines repository deliverables to current substrate commitments, and positions the interface reference as non-expanding support.
- `docs/seeds/mixed-project-brief.md` — Binding current-product identity, public boundary, correctness and validation commitments, non-goals, and explicit future-consumer Product One context that must not enlarge Moria.
- `docs/seeds/substrate-interface-reference.md` — Supporting surface detail (identity, query/mutation/streaming/persistence/diagnostics, registered objects) translated here only into outcome-level mandates, without new deliverables.
