# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate delivered as public crates.
Games and other tools consume it only through those public interfaces. This
repository’s product is the substrate itself, not any particular game.

## Purpose

Moria exists so multiple downstream consumers can share one authoritative
voxel-world foundation: create and identify worlds, obtain bounded material
truth, stream and mutate regions under explicit bounds, extract surfaces for
presentation, persist and restore authoritative state, and observe readiness
and failures—without each consumer rebuilding world infrastructure or reaching
into substrate internals.

## Product boundary

- **Moria owns** the reusable world substrate: deterministic seed-based
  generation, sparse voxel material truth, bounded streaming with observable
  lifecycle, bounded mutation, surface extraction, persistence of authoritative
  material state, and read-only diagnostics.
- **Consumers own** gameplay, presentation, controllers, cameras, characters,
  animation, authored content, and any game-specific policy.
- **Encapsulation:** external consumers create and identify worlds, request
  bounded regions, observe readiness, query material truth, submit bounded
  edits, and persist deltas; they must not reach into storage, meshing, or
  scheduler internals.
- **Adjacent validation delivery:** this repository also delivers headless
  fixtures and a minimal validation executable that exercise the public crate
  surface. They are not the product identity, are not a game prototype, and own
  no privileged world path. Their controls, content, presentation, workloads,
  and acceptance scenarios stay adjacent.
- **Not in this product:** game rules, combat, inventory, AI, narrative systems,
  production content packs, or any particular titled game.

## Required product outcomes

- **Public multi-consumer substrate.** Independent games and tools integrate
  only through public Rust crate interfaces and can perform material world work
  without internal coupling; the same capabilities remain useful across
  consumers.
- **Deterministic worlds and material truth.** Consumers create and identify a
  world from versioned generation parameters and a seed; generation is
  deterministic for the same parameters and seed; bounded regions can be
  requested, readiness observed, and authoritative material state queried.
- **Bounded streaming and mutation.** Resident work stays bounded with
  observable lifecycle; background results cannot replace newer truth via stale
  completion; edits enter through a bounded command surface and commit
  atomically as authoritative change.
- **Derived presentation support without false authority.** The substrate
  provides surface extraction for consumer presentation; derived meshes and
  diagnostics never become authoritative world state.
- **Persistence, failures, and diagnostics.** Authoritative material deltas
  persist and restore to the same material truth; failures are typed and
  visible at the public boundary; diagnostics report lifecycle and bounded work
  without exposing mutable internal handles.
- **Public-boundary validation.** Repository validation exercises generation,
  query, mutation, persistence, and lifecycle behavior through the public
  interfaces only.

## Future products and enabling implications

**Product One** is a separate later game-facing demo (third-person explorer in a
generated region). It is a future consumer of Moria, not current Moria scope.
It may motivate interface pressure; it does not authorize player controllers,
character assets, animation, curated routes, forest workloads, or game content
inside this product.

Enabling implication already supported by current outcomes: a later explorer
can host presentation and gameplay on deterministic generation, streaming,
mutation, surface extraction, and persistence without forking world internals.

## Non-goals

- Shipping any particular game, prototype, or production content pack
- Implementing gameplay systems (rules, combat, inventory, AI, narrative)
- Owning characters, animation, authored levels, or consumer UX and controllers
- Establishing machine-specific performance correctness thresholds as product
  identity
- Expanding product scope from supporting interface notes or future-consumer
  descriptions

## Confirmed vision constraints

- **Ecosystem:** delivered and consumed as Rust public crates.
- **Authority:** only material world state is authoritative; meshes and
  diagnostics are derived or observational.
- **Encapsulation:** external consumers must not depend on storage, meshing, or
  scheduler internals.
- **Determinism:** same versioned parameters and seed yield the same generation.
- **Admission and observability:** mutation is bounded and admitted; streaming
  and failures remain observable at the public boundary.
- **Reuse:** capabilities must remain useful to more than one downstream
  consumer.

## Deferred design decisions

- Depth and sequencing of substrate capabilities across releases
- Concrete crate and API shape, data layouts, algorithms, and encodings
- Streaming residency policy, concurrency, and generation-identity mechanics
- Surface-extraction and persistence formats at the implementation level
- Validation fixture design, workloads, platforms, and performance reporting
  practice beyond the public-boundary obligation

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds agree that the current product is the reusable Rust
voxel-world substrate; validation is an adjacent public-boundary consumer
delivery; Product One is a later separate consumer.

## Seed synthesis

- **README.md** — Names Moria as the reusable voxel-world substrate, limits
  repository deliverables to current substrate commitments, and marks the
  interface reference as non-expanding support.
- **docs/seeds/mixed-project-brief.md** — Binding current-product authority for
  identity, public boundary, correctness, validation at the public surface,
  non-goals, and Product One as future-consumer context only.
- **docs/seeds/substrate-interface-reference.md** — Non-expanding supporting
  vocabulary for world identity, readiness, bounded mutation, streaming
  lifecycle, persistence of deltas, and diagnostics; adds no deliverables.
