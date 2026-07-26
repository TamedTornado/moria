# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate. Downstream games and validation artifacts consume it only through public crate interfaces. This repository delivers that substrate, not a game.

## Purpose

Moria exists so multiple independent consumers can create, stream, query, edit, and persist generated voxel worlds through one shared public surface—without each game owning world storage, generation, meshing internals, or privileged engine paths.

## Product boundary

**Belongs to Moria:** world create/identity; deterministic seed-based generation; storage of authoritative voxel material truth; bounded region request and streaming with observable lifecycle; material queries and readiness; bounded mutation with atomic commit; surface extraction as non-authoritative derived output; persistence of authoritative deltas; read-only diagnostics; typed, observable failures to public consumers.

**Adjacent, not product identity:** a minimal validation executable and fixtures that exercise the same public interfaces (no privileged world path; not a game prototype). Headless coverage and a small visual fixture that shows a relocated external consumer can render and edit through the public API are program validation commitments. Performance reporting may include machine identity; no machine-specific correctness threshold is part of the product promise.

**Not Moria:** any particular game; game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content; consumer-owned controllers, presentation, routes, or content. Consumers must not reach into storage, meshing, or scheduler internals.

## Required product outcomes

1. **Public-only consumption.** Games and validation consumers use public crate interfaces only; adjacent validation has no privileged world path.

2. **Identifiable, deterministic worlds.** A world is identified by format version, generation parameters, and seed. Generation is deterministic for the same versioned parameters and seed.

3. **Bounded stream and observe.** Consumers request bounded regions, observe readiness and streaming lifecycle, and obtain bounded authoritative material observations. Resident work stays bounded. Background results carry generation identities so stale work cannot replace newer truth.

4. **Bounded, atomic mutation.** Edits enter through a bounded command API with admission outcomes and commit revisions; admitted mutations commit atomically.

5. **Authoritative persistence.** Persistence records and restores authoritative material deltas (and thus the same authoritative material state). Derived meshes and diagnostics never become authoritative world state.

6. **Derived surface and diagnostics.** Surface extraction supports consumer rendering needs without authority. Diagnostics report lifecycle and bounded work without exposing mutable internal handles. Registered objects may participate in queries without becoming game entities. Failures remain typed and observable on the public surface.

7. **Multi-consumer usefulness.** These capabilities remain useful to multiple downstream consumers. Validation covers generation, query, mutation, persistence, and lifecycle behavior through public interfaces.

## Future products and enabling implications

**Product One** (separate repository, after the substrate ships) may place a third-person explorer in a generated region (hills, dense mixed forest, river, cave) and may use skeletal animation and a curated cliff-to-cave traversal. That material is future-consumer context only. It does not put player controllers, character meshes, animation clips, forest population workloads, curated routes, or game assets into Moria.

Enabling implication: a public, multi-consumer world substrate with generation, streaming, query, edit, surface derivation, and persistence is what such a later demo would consume.

## Non-goals

- Shipping a game, game prototype, or Product One content in this repository
- Gameplay systems (rules, combat, inventory, AI, narrative, characters, animation)
- Authored levels or production content
- Privileged internal access for any consumer, including validation
- Machine-specific performance pass/fail as product correctness

## Confirmed vision constraints

- Integration form is a Rust crate (public crate interfaces).
- External consumers stay outside storage, meshing, and scheduler internals.
- Meshes and diagnostics are never authoritative world state.
- Streaming exposes observable lifecycle; failures are typed and public-observable.
- Validation uses exactly the public interfaces; it is not a game and owns no privileged path.
- Performance may be reported with machine identity; this vision sets no machine-specific correctness threshold.

## Deferred design decisions

- Concrete API shapes, command and revision encodings, and streaming state machine detail
- Storage layout, generation algorithms, meshing approach, and scheduler design
- Persistence encoding and delta format
- Fixture workloads, visual presentation of the validation executable, and how performance is measured or displayed
- Delivery sequence and depth of each substrate capability within the first shippable slice

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds agree that the current product is the reusable Rust substrate; validation is an adjacent program commitment outside product identity; Product One is a later consumer only.

## Seed synthesis

- `README.md` names Moria as the reusable voxel-world substrate, limits repository deliverables to current substrate commitments, and marks the interface reference as non-expanding support.
- `docs/seeds/mixed-project-brief.md` is the binding current-product authority: Rust substrate identity, public outcomes, correctness, validation adjacency, non-goals, and Product One as non-authorizing future-consumer context.
- `docs/seeds/substrate-interface-reference.md` supports the same public surface (identity, query, mutation, streaming lifecycle, persistence, registered objects, diagnostics) without adding deliverables or widening product scope.
