# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate delivered as public crate interfaces. This repository ships that substrate for games and other external consumers; it does not ship a game. A minimal validation executable and fixtures are adjacent repository deliveries that exercise the same public boundary; they are not part of the substrate’s product identity.

## Purpose

Moria exists so multiple downstream consumers can rely on one shared voxel-world foundation—deterministic generation, material truth, bounded edit and streaming behavior, surface extraction, persistence, and read-only diagnostics—without reimplementing that foundation or depending on game-specific systems.

## Product boundary

**In product (Moria):** the reusable substrate and its public crate surface: world creation and identity, deterministic seed-based generation, sparse voxel world material storage, bounded region request and readiness, material queries (including registered objects that can participate in queries without becoming game entities), bounded mutation, bounded streaming with observable lifecycle, surface extraction, persistence of authoritative material deltas, and read-only diagnostics. External consumers integrate only through that public surface and must not reach storage, meshing, or scheduler internals.

**Adjacent, not product identity:** a validation executable and fixtures delivered with the repository. They use exactly the public interfaces, own no privileged world path, and are not a game prototype. Particular cameras, character content, and presentation choices remain adjacent behavior; the required validation outcomes below stay repository commitments without becoming substrate identity.

**Out of product / downstream:** any particular game, including a later Product One explorer demo; game rules, combat, inventory, AI, narrative, characters, animation, controllers, authored levels, production content, presentation policy, and gameplay systems.

## Required product outcomes

- Consumers create and identify worlds (identity combining format version, generation parameters, and seed), request bounded regions, observe readiness, and query authoritative material observations through public crate interfaces only; registered objects may participate in those queries without becoming game entities.
- Generation is deterministic for the same versioned parameters and seed, and remains useful as a shared foundation across multiple downstream consumers.
- Bounded mutation is admitted through a command API and committed atomically; failures are typed and observable to public consumers.
- Streaming bounds resident work, exposes observable lifecycle states, and carries generation identities so stale background results cannot replace newer material truth.
- Persistence records and restores authoritative material state as deltas; derived meshes and diagnostics never become authoritative world state.
- The substrate provides surface extraction and read-only diagnostics usable by external consumers through the public boundary.

## Future products and enabling implications

A separate Product One repository may later place a third-person explorer in a generated region with hills, a dense mixed forest, a river, and a cave, using skeletal animation and a curated cliff-to-cave traversal to communicate the world. That game is a future consumer, not current Moria scope. Moria’s enabling implication is only that a reusable public substrate can supply generated material worlds and material queries such a presentation could consume. Edits, streaming, and persistence remain Moria product outcomes but are not established as Product One workloads. Player control, character presentation, animation, forest population, curated routes, and game assets stay with that future product.

## Non-goals

- Implementing game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content in Moria.
- Treating the validation executable as a game prototype or granting it privileged world access.
- Making derived meshes or diagnostics authoritative world state.
- Establishing machine-specific performance correctness thresholds as product truth.

## Confirmed vision constraints

- Integration ecosystem: Rust public crate interfaces for external consumers (games and validation alike).
- Authority model: persistence restores authoritative material state; derived meshes and diagnostics never become authoritative. The seeds do not establish that every non-material observation is categorically non-authoritative.
- Correctness qualities already bound to named outcomes: world identity from format version, generation parameters, and seed; deterministic generation for those versioned parameters and seed; atomic commit of admitted mutations; typed observable failures; streaming lifecycle observability with stale-work protection via generation identities.
- Consumer isolation: no privileged internal path into storage, meshing, or scheduler; validation uses the same public boundary as any other consumer.
- Adjacent validation is a required repository delivery: headless fixtures covering generation, query, mutation, persistence, and lifecycle; a small visual fixture showing a relocated external consumer can render and edit through the public API; and performance reporting with machine identity. No machine-specific performance correctness threshold is product-binding. Particular cameras, routes, and presentation remain adjacent behavior and do not define substrate identity.

## Deferred design decisions

- How public APIs, internal modules, and crates are factored; algorithms, data layouts, and streaming or persistence encodings.
- Depth and sequencing of substrate capability delivery across releases.
- Exact fixture protocols and visual demo presentation beyond the vision-level validation outcomes above.
- How far surface extraction, diagnostics detail, and multi-consumer packaging go in any given delivery slice.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names Moria as the reusable voxel-world substrate, confines repository deliverables to current substrate commitments, and marks the interface reference as non-expanding support.
- `docs/seeds/mixed-project-brief.md` — Binding source for current product identity, public boundary, correctness and validation commitments, non-goals, and the later Product One consumer context that must not enter Moria scope.
- `docs/seeds/substrate-interface-reference.md` — Supporting vocabulary refining world identity composition, query participation without entity-hood, mutations, streaming states, persistence of deltas vs meshes, and diagnostics; does not add deliverables or expand scope.
