# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate. Downstream games and a
minimal validation executable consume it only through public crate
interfaces. This repository delivers that substrate, not any particular game.

## Purpose

Moria exists so multiple independent consumers can create, stream, query,
mutate, extract surfaces from, persist, and diagnose voxel worlds without
privileged access to storage, meshing, or scheduler internals. The substrate
keeps world-material capabilities reusable across those consumers.

## Product boundary

**In product scope.** Public crate APIs for world creation and world
identity, sparse voxel storage, bounded region request and streaming,
readiness and material query, bounded mutation, surface extraction,
persistence of deltas with restoration of authoritative material state, and
read-only diagnostics. Correctness properties of generation, mutation,
streaming, and persistence belong to the substrate.

**Adjacent delivery, not product identity.** Headless fixtures and a small
visual fixture exercise the public API. They are validation consumers of the
substrate under test, not a game prototype and not a second product. They
own no privileged world path.

**Out of product scope.** Game rules, combat, inventory, AI, narrative,
characters, animation, authored levels, production content, player
controllers, curated routes, and game-facing presentation or policy. Future
Product One content and controls remain in a separate consumer repository.

## Required product outcomes

- Consumers integrate only through public Rust crate interfaces; they must
  not reach storage, meshing, or scheduler internals. Capabilities remain
  useful to multiple downstream consumers.
- Consumers can create worlds and identify them. World identity combines
  format version, generation parameters, and seed. Generation is
  deterministic for the same versioned parameters and seed. Worlds use
  sparse voxel storage as a substrate property.
- Consumers request bounded regions, observe streaming lifecycle and
  readiness, and query bounded authoritative material truth. Resident work
  stays bounded; background results carry generation identity so stale work
  cannot replace newer truth.
- Bounded edit commands admit or fail explicitly, commit atomically with
  revisions, and leave failures typed and observable on the public surface.
- Surface extraction never becomes authoritative world state. Persistence
  records authoritative deltas rather than derived meshes and restores the
  same authoritative material state. Read-only diagnostics report lifecycle
  and bounded work without exposing mutable internal handles, and never
  become authoritative world state.
- Adjacent validation delivery covers generation, query, mutation,
  persistence, and lifecycle via headless fixtures, plus a small visual
  fixture that renders and edits only through the public API as a relocated
  external consumer.

## Future products and enabling implications

A later Product One in a separate repository may present a third-person
explorer in a generated region (hills, mixed forest, river, cave) using its
own controllers, animation, and authored traversal. That consumer motivates
reusable world generation, material truth, streaming, mutation, surfaces, and
persistence; it does not pull gameplay, characters, animation, content, or
routes into Moria.

## Non-goals

- Shipping any particular game, demo gameplay loop, or production content
  pack as Moria.
- Implementing game systems (rules, combat, inventory, AI, narrative,
  characters, animation, authored levels).
- Treating derived meshes or diagnostics as authoritative world state.
- Granting validation fixtures or any consumer a privileged world path.
- Establishing machine-specific performance correctness thresholds for the
  substrate.

## Confirmed vision constraints

- Delivery form is a Rust crate surface consumed by external programs.
- Sparse voxel storage is a mandatory substrate property alongside generation,
  streaming, mutation, surface extraction, persistence, and diagnostics.
- Generation determinism is keyed to versioned parameters and seed.
- Mutation is bounded, admitted through a command API, and committed
  atomically; streaming exposes observable lifecycle states and bounds
  resident work.
- Persistence records authoritative deltas rather than derived meshes and
  restores the same authoritative material state; meshes and diagnostics
  never authoritatively redefine that state.
- Stale background results must not overwrite newer truth; public failures
  are typed and observable. Validation performance reporting includes
  machine identity and does not establish machine-specific correctness
  thresholds.

## Deferred design decisions

- Crate layout, concrete API shapes, algorithms, and streaming or
  persistence encodings.
- Exact surface-extraction approach, diagnostics payload design, and any
  registered-object participation model detail.
- Fixture workload choices, visual presentation, and any quantitative
  performance targets.
- Delivery sequence and depth within the substrate outcome set.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds agree that Moria is the Rust voxel-world substrate, that
validation fixtures are required adjacent delivery using only public
interfaces, and that Product One is a future separate consumer.

## Seed synthesis

- `README.md` names Moria as the reusable voxel-world substrate, limits
  repository deliverables to current substrate commitments, and marks the
  interface reference as non-expanding context.
- `docs/seeds/mixed-project-brief.md` binds current product identity,
  public boundary, correctness and validation commitments, non-goals, and
  the later Product One consumer vision as non-authorizing context.
- `docs/seeds/substrate-interface-reference.md` supports the brief’s
  consumer-visible world, query, mutation, streaming, persistence, and
  diagnostics outcomes without adding deliverables or redefining the
  product.
