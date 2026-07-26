# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate consumed through public crate interfaces. This repository’s product is that substrate, not any particular game.

## Purpose

Moria exists so multiple independent games and tools can share one authoritative, deterministic voxel world: generate from seed and versioned parameters, stream and mutate bounded regions, persist material truth, extract surfaces for consumer rendering, and inspect lifecycle and work through public interfaces—without each consumer reimplementing world authority or reaching into storage, meshing, or scheduling internals.

## Product boundary

**In product:** the reusable substrate and its public consumer surface—world create and identity, bounded region request and readiness, material-truth query, bounded edit admission and atomic commit, surface extraction for consumer use, persistence of authoritative material state, bounded streaming with observable lifecycle, and read-only diagnostics.

**Adjacent repository delivery (not product identity):** a minimal validation executable plus headless and small visual fixtures that exercise the substrate only through the public crate API. Their free-fly camera, diagnostic presentation, relocation scenario, coverage set, and performance reporting presentation remain validation behavior, not substrate features.

**Out of product / downstream:** any particular game, including a later Product One explorer demo. Controllers, characters, animation, authored routes, terrain population, production content, combat, inventory, AI, narrative, and game rules stay consumer-owned.

## Required product outcomes

- Consumers create and identify worlds and obtain deterministic generation for the same versioned parameters and seed.
- Consumers request bounded regions, observe streaming readiness and lifecycle, and query authoritative material observations without privileged internal access.
- Consumers submit bounded mutations that admit or fail with typed, observable outcomes and commit atomically; persistence restores the same authoritative material state.
- Surface extraction and diagnostics remain derived: meshes and diagnostic views never become authoritative world state.
- Streaming bounds resident work; background results carry generation identities so stale work cannot replace newer truth; failures stay typed and observable on the public surface.
- The substrate remains useful to multiple downstream consumers through the same public crate boundary.

## Future products and enabling implications

**Product One** (separate repository, after the reusable substrate ships) is a future consumer: a third-person explorer in a generated region with hills, dense mixed forest, river, and cave, using skeletal animation and a curated cliff-to-cave traversal. That vision implies Moria’s public world, material, streaming, edit, persistence, and surface capabilities must remain fit for an independent game client. It does not place controllers, character meshes, animation clips, forest workloads, curated routes, or game assets in Moria.

## Non-goals

- Game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content.
- Treating validation as a game prototype or giving it a privileged world path.
- Machine-specific performance correctness thresholds as product identity (machine identity may accompany reports; thresholds are not product promises).
- Absorbing Product One or any other game’s presentation, policy, or content into the substrate.

## Confirmed vision constraints

- Integration surface is Rust public crate interfaces; external consumers must not reach into storage, meshing, or scheduler internals.
- Material state after generation, mutation, and restore is authoritative; derived meshes and diagnostics are not.
- Mutation crosses the product boundary only as bounded commands with atomic commit semantics.
- Streaming exposes observable lifecycle states and generation-aware background results so consumers can distinguish current truth from stale or failed work.
- Adjacent validation in this repository uses exactly those public interfaces—no privileged substrate path.

## Deferred design decisions

- Concrete API shapes, crate layout, storage and meshing algorithms, streaming schedules, persistence encoding, and diagnostic schemas.
- Delivery depth and sequence within the substrate mandate; fixture workload design and any non-correctness performance gates.
- How surface results are handed to consumers without exposing meshing internals (outcome required; mechanism deferred).

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names Moria as the reusable voxel-world substrate, limits deliverables to current substrate commitments, and marks the interface reference as non-expanding support.
- `docs/seeds/mixed-project-brief.md` — Binding current-product identity, public boundary, correctness and validation commitments, non-goals, and explicit separation of later Product One from Moria scope.
- `docs/seeds/substrate-interface-reference.md` — Supporting vocabulary for identity, query, mutation, streaming, persistence, and diagnostics already authorized by the program brief; does not add deliverables.
