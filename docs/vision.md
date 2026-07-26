# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate. It is consumed through public crate interfaces by games and other external consumers. This repository delivers that substrate, not any particular game.

## Purpose

Moria exists so multiple downstream products can share one voxel-world foundation: create and identify worlds, stream and observe regions, query material truth, apply bounded edits, persist authoritative state, and diagnose lifecycle and work—without each consumer owning or reaching into world-engine internals.

## Product boundary

**In product:** deterministic seed-based generation; sparse voxel material storage; bounded streaming; mutation; surface extraction; persistence of authoritative material state; read-only diagnostics; and the public crate surface that exposes those capabilities.

**Out of product:** game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, and production content. Consumer-owned presentation, controllers, characters, gameplay policy, and authored world dressing stay with adjacent games.

**Adjacent, not identity:** the repository’s validation path (headless fixtures and a small visual fixture / minimal validation executable) exercises Moria only through the same public interfaces. It is not a game prototype and owns no privileged world path. Fixture-specific controls, scenes, workloads, and presentation are not substrate scope.

**Not this product:** a later third-person explorer (Product One) in a separate repository is future-consumer context only.

## Required product outcomes

- External consumers create and identify a world (format version, generation parameters, and seed), request bounded regions, observe readiness, query authoritative material observations, submit bounded edits, and persist authoritative deltas—without access to storage, meshing, or scheduler internals.
- Generation is deterministic for the same versioned parameters and seed. Mutation is admitted through a bounded command API with explicit admission failures and atomic commit; persistence restores the same authoritative material state.
- Streaming bounds resident work and exposes observable lifecycle states (including requested, loading, resident, evicted, and failed). Background results carry generation identities so stale work cannot replace newer truth. Failures are typed and observable to public consumers.
- Surface extraction and diagnostics remain derived and non-authoritative; diagnostics report lifecycle and bounded work without exposing mutable internal handles. Registered objects may participate in queries without becoming game entities.
- The substrate remains useful to multiple downstream consumers through that public boundary alone.
- Adjacent validation covers generation, query, mutation, persistence, and lifecycle in headless fixtures, and demonstrates that a relocated external consumer can render and edit through the public API. Performance is reported with machine identity; no machine-specific correctness threshold is part of the product promise.

## Future products and enabling implications

After the substrate ships, a separate Product One repository may present a third-person explorer in a generated region (hills, dense mixed forest, river, cave) with skeletal animation and curated traversal. That material pressures interface usefulness only. It does not put a player controller, character mesh, animation clips, forest population workload, curated route, or game assets into Moria. Enabling implication: keep the public world contract sufficient for such a consumer without absorbing its gameplay or content.

## Non-goals

- Shipping a game, game prototype, or game systems (rules, combat, inventory, AI, narrative, characters, animation).
- Authoring production content, curated demo routes, or forest/river/cave dressing inside Moria.
- Treating derived meshes or diagnostics as authoritative world state.
- Privileged validation or consumer paths into storage, meshing, or scheduler internals.

## Confirmed vision constraints

- Implementation ecosystem is Rust public crates; external consumers integrate only through that public surface.
- Generation determinism is keyed to versioned parameters and seed; mutation commits are atomic; persistence is of authoritative material deltas, not derived meshes.
- Streaming keeps resident work bounded; stale background application is prevented via generation identity; public failures are typed and observable.
- Validation uses exactly the public interfaces; it neither expands substrate scope nor defines hardware, OS, or performance pass/fail bars for the product.
- Later-consumer paragraphs in the program brief do not authorize current Moria gameplay, content, or presentation work.

## Deferred design decisions

- Concrete API shapes, data layouts, algorithms, crate splits, and enforcement structure for the public boundary.
- Numeric streaming bounds, timing or memory thresholds, benchmark workloads, and how performance numbers are collected beyond machine identity on reports.
- Detailed models for surface extraction, registered objects, and diagnostic payloads.
- Release depth and sequencing of substrate capabilities (what ships first versus later within the product).
- Visual-fixture presentation and controls beyond “public API only, not a game.”

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Establishes Moria as the reusable substrate named by the program brief; only current substrate commitments are repository deliverables; the interface reference is supporting context that must not expand scope.
- `docs/seeds/mixed-project-brief.md` — Binding source for current product identity, owned capabilities, public boundary, correctness, validation delivery, non-goals, and Product One as embedded future-consumer vision only.
- `docs/seeds/substrate-interface-reference.md` — Supporting public-surface behaviors (world identity, query/readiness, mutation commands, streaming lifecycle states, delta persistence, registered objects, diagnostics) without adding deliverables.
