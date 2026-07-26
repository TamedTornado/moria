# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate delivered as public crate interfaces. This repository delivers that substrate for consumption by games and other downstream consumers, not any particular game. Adjacent validation exercises the same public boundary; it is not the product identity.

## Purpose

Moria exists so multiple downstream consumers can share a deterministic voxel world: create and identify worlds, stream and observe bounded regions, query authoritative material truth, apply bounded edits, extract derived surfaces, persist and restore material state, and obtain read-only diagnostics—without privileged access to substrate internals.

## Product boundary

**In product:** world creation and identity; deterministic seed-based generation; voxel material storage and query; bounded streaming with observable lifecycle; bounded mutation; surface extraction as derived (non-authoritative) output; persistence of authoritative material deltas; read-only diagnostics; the public consumer-facing interface contract.

**Adjacent, not product identity:** a minimal validation executable and fixtures that consume only public interfaces and hold no privileged world path. This repository delivers that adjacent exercise of the public boundary; harness controls, presentation, fixture protocols, workloads, and machine-specific performance gates remain adjacent, not product scope.

**Out of product:** any game, game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, production content, player controllers, or curated gameplay routes. Possible later games explain interface pressure only.

## Required product outcomes

- Consumers create and identify a world, request bounded regions, observe readiness, query authoritative material truth, submit bounded edits, and persist deltas through public crate interfaces only; they do not reach into storage, meshing, or scheduler internals.
- Generation is deterministic for the same versioned parameters and seed; mutation is admitted through a bounded command API and committed atomically; failures are typed and observable to public consumers.
- Streaming bounds resident work, exposes observable lifecycle states, and keeps background results from replacing newer truth when generation identities show work is stale.
- Persistence restores the same authoritative material state; derived meshes and diagnostics never become authoritative world state.
- Surface extraction and read-only diagnostics remain useful, non-authoritative support for multiple downstream consumers of the same substrate.
- Adjacent validation exercises the public product boundary without a privileged world path.

## Future products and enabling implications

A separate Product One repository may later ship a third-person explorer in a generated region (hills, mixed forest, river, cave) with skeletal animation and a curated cliff-to-cave traversal. That is future-consumer context only.

Enabling implication: Moria’s public world, generation, streaming, mutation, surface, persistence, and diagnostics outcomes are intended to support such later games without embedding their controllers, characters, animation, content, presentation, or gameplay policy in Moria.

## Non-goals

- Implementing game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content.
- Treating the validation executable as a game prototype or granting it privileged substrate access.
- Establishing machine-specific performance correctness thresholds from this vision.
- Delivering Product One (or any particular game) in this repository.

## Confirmed vision constraints

- Integration surface is public Rust crate interfaces; consumers have no privileged access to storage, meshing, or scheduler internals.
- Adjacent validation uses exactly those public interfaces and holds no privileged world path.
- This vision does not set a machine-specific performance correctness bar.

## Deferred design decisions

- Delivery depth and sequence across substrate capabilities (first slice vs later depth).
- Algorithms, data layouts, internal module structure, and concrete API shapes.
- Exact streaming residency policy, lifecycle presentation, and generation-identity mechanics beyond the outcome guarantees above.
- Validation fixture inventory, visual harness UX, workloads, and any performance measurement protocol beyond reporting with machine identity.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds agree that the current product is the reusable substrate; validation is an adjacent public-boundary consumer with repository delivery commitments; Product One is a later separate consumer and does not expand Moria’s scope.

## Seed synthesis

- `README.md` — Names Moria as the reusable voxel-world substrate; only current substrate commitments are repository deliverables; the interface reference is supporting and non-expanding.
- `docs/seeds/mixed-project-brief.md` — Binding current-product identity, public boundary, correctness, validation delivery, non-goals, and explicit future-consumer (Product One) separation.
- `docs/seeds/substrate-interface-reference.md` — Supporting vocabulary for world identity, query/mutation/streaming/persistence/diagnostics semantics; does not add deliverables or expand scope.
