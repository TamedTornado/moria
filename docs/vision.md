# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate. Downstream games and a minimal
validation executable consume it only through public crate interfaces. This
repository delivers that substrate, not any particular game.

## Purpose

Moria exists so multiple independent consumers can share one voxel-world
foundation: create and identify worlds, stream and query authoritative material
state, apply bounded edits, persist and restore that state, extract
non-authoritative surfaces, and observe readiness, lifecycle, and failures—
without each consumer reimplementing world truth or reaching into internals.

## Product boundary

- **Substrate ownership.** Moria owns deterministic seed-based generation,
  sparse voxel storage, bounded streaming, mutation, surface extraction,
  persistence, and read-only diagnostics as reusable capabilities for many
  consumers.
- **Public contract.** Consumers create a world whose identity combines format
  version, generation parameters, and seed; request bounded regions; observe
  readiness and streaming lifecycle; query authoritative material
  observations; submit bounded edits; and persist authoritative deltas.
  Failures are typed and observable. Storage, meshing, and scheduler
  internals stay private; no consumer has a privileged world path.
- **Not a game.** Moria does not own game rules, combat, inventory, AI,
  narrative, characters, animation, authored levels, production content,
  player controllers, or game-facing presentation.
- **Not Product One.** A later third-person explorer (separate repository) is a
  future consumer only; its controller, character, animation, scenery, route,
  and presentation are not Moria scope.
- **Adjacent validation delivery.** Headless fixtures cover generation, query,
  mutation, persistence, and lifecycle through the public API. A small visual
  fixture shows a relocated external consumer can render and edit through that
  same API; it is not a game prototype and owns no privileged path.
- **Harness details stay adjacent.** Controllers, scene content, presentation,
  platform choices, and performance gates of validation artifacts are not
  product identity.

## Required product outcomes

- **Deterministic generation.** The same versioned generation parameters and
  seed produce the same world material truth.
- **World identity and authoritative material access.** A world's identity
  combines format version, generation parameters, and seed. Consumers create
  and identify a world, request bounded regions, observe readiness, and obtain
  bounded authoritative material observations. Registered objects may
  participate in queries without becoming game entities.
- **Bounded atomic mutation.** Edits enter only through a bounded command
  surface, with admission failures visible, and commit as atomic revisions.
- **Bounded streaming with safe lifecycle.** Resident work stays bounded;
  lifecycle is observable (requested, loading, resident, evicted, failed);
  background results carry generation identity so stale work cannot replace
  newer truth; failures remain typed and consumer-visible.
- **Authority-only persistence; non-authoritative derivation.** Persistence
  records authoritative material deltas rather than derived meshes, and
  restores the same authoritative material state. Surface extraction and
  diagnostics never become authoritative; diagnostics report lifecycle and
  bounded work without exposing mutable internal handles.
- **Public-only integration.** Every consumer, including validation, uses only
  public crate interfaces. Validation must prove that path via the headless
  and visual fixtures above; performance reports include machine identity, with
  no machine-specific correctness threshold fixed here.

## Future products and enabling implications

**Product One** (separate repository, after the substrate ships) may place a
third-person explorer in a generated region with hills, a dense mixed forest, a
river, and a cave, using skeletal animation and a curated cliff-to-cave
traversal. That gameplay, content, and presentation are consumer-owned.

Moria’s enabling implication is only that the public substrate remain usable by
such games under the outcomes above. Future-consumer needs do not restore
excluded gameplay, building, presentation, or policy layers into the substrate.

## Non-goals

- Shipping a game, game prototype, or Product One in this repository.
- Implementing game systems (rules, combat, inventory, AI, narrative,
  characters, animation, authored levels, production content).
- Treating free-fly cameras, demo scenes, curated routes, or similar harness
  choices as substrate features.
- Granting any consumer privileged access to world internals.
- Using derived meshes or diagnostics as authoritative world state.
- Fixing machine-specific performance pass/fail thresholds as product
  correctness.

## Confirmed vision constraints

- Form is a reusable Rust substrate consumed via public crate interfaces.
- World identity combines format version, generation parameters, and seed;
  generation is deterministic for the same versioned parameters and seed.
- Mutation is admitted only through the bounded public command surface and
  commits atomically; persistence records authoritative deltas rather than
  derived meshes, and restores the same authoritative material state.
- Streaming bounds resident work, exposes observable lifecycle states, and
  prevents stale background results from overwriting newer truth; failures are
  typed and public-observable.
- Derived meshes and diagnostics are never authoritative.
- Validation and other consumers use only public interfaces (no privileged
  world path). Performance is reported with machine identity; seeds fix no
  machine-specific correctness threshold.

## Deferred design decisions

- Concrete public API shapes, crate packaging, and internal module structure.
- Generation algorithms, storage layout, streaming policy, meshing approach,
  and persistence encoding.
- Exact readiness and lifecycle presentation beyond required observable states
  and stale-work safety.
- Fixture design depth, visual harness presentation, and performance
  measurement methodology beyond machine-identified reporting.
- Delivery sequence and depth of individual substrate capabilities within the
  overall product responsibility.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds agree on one current product (the Rust voxel-world substrate),
treat Product One as a later consumer only, and bind validation artifacts as
adjacent repository commitments that exercise the public API without expanding
product identity.

## Seed synthesis

- **`README.md`** — Names Moria as the reusable voxel-world substrate, confines
  repository deliverables to current substrate commitments, and marks the
  interface reference as non-expanding context.
- **`docs/seeds/mixed-project-brief.md`** — Binding source for product identity,
  public boundary, correctness and validation commitments, non-goals, and the
  embedded Product One future-consumer vision (explicitly non-authorizing for
  Moria scope).
- **`docs/seeds/substrate-interface-reference.md`** — Supporting interface
  outcomes (world identity, query/mutation/streaming/persistence/diagnostics
  behaviors, non-entity registered objects) without adding deliverables or
  widening scope.
