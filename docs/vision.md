# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate. Downstream games and a minimal validation executable consume it only through public crate interfaces. This repository delivers that substrate, not any particular game.

## Purpose

Moria exists so multiple independent consumers can rely on one reusable authoritative voxel-world substrate: create and identify worlds, stream and observe regions, read material truth, apply bounded edits, extract surfaces for presentation, and persist deltas—without each consumer owning world storage, generation, or lifecycle machinery.

## Product boundary

**In product.** The substrate owns deterministic seed-based generation, sparse voxel storage, bounded streaming, mutation, surface extraction, persistence of authoritative material deltas, and read-only diagnostics. Its public surface lets consumers create and identify a world, request bounded regions, observe readiness, query material truth, submit bounded edits, and persist deltas. Consumers must not reach into storage, meshing, or scheduler internals.

**Adjacent, not identity.** A minimal validation executable, headless fixtures, and a small visual fixture are current program deliveries that exercise the substrate. They are not a game prototype. The validation executable and the visual fixture (as a relocated external public-API consumer) use exactly the public interfaces and own no privileged world path; free-fly viewing, diagnostic presentation, and similar harness choices stay with those artifacts, not with substrate identity. Headless fixtures cover generation, query, mutation, persistence, and lifecycle behavior through the public surface without the same executable-level harness mandate.

**Out of product.** Game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, production content, player controllers, curated routes, and any particular game’s presentation or policy remain consumer-owned. A later explorer demo (Product One) is a separate future consumer, not a Moria deliverable.

## Required product outcomes

- **World identity and deterministic generation.** A world is identified by format version, generation parameters, and seed. Generation is deterministic for the same versioned parameters and seed, and remains useful to multiple consumers.
- **Authoritative material truth with sparse storage.** Sparse voxel storage is a substrate capability. Material state is authoritative: consumers obtain readiness and bounded authoritative material observations; persistence records authoritative material deltas. Derived meshes and diagnostics never become authoritative world state. Authority attaches to material state, not to the storage representation.
- **Bounded streaming with explicit lifecycle states.** Consumers request bounded regions; resident work is bounded. Streaming exposes the consumer-visible states requested, loading, resident, evicted, and failed. Background results carry generation identities so stale work cannot replace newer truth; failures are typed and observable on the public surface.
- **Bounded mutation and material persistence.** Edits enter through a bounded command API with admission failures and commit revisions, and admitted mutations commit atomically. Persistence records authoritative deltas (not derived meshes) and restores the same authoritative material state.
- **Consumer-facing derived views.** The substrate provides surface extraction for external rendering without elevating geometry to world authority; read-only diagnostics report lifecycle and bounded work without mutable internal handles; registered objects may participate in queries without becoming game entities.
- **Public-only integration and adjacent validation.** The public crate API is sufficient for external consumers to create, stream, query, mutate, persist, and observe lifecycle without privileged access. Headless fixtures cover generation, query, mutation, persistence, and lifecycle. The validation executable uses exactly the public interfaces and owns no privileged world path. A small visual fixture shows a relocated external consumer can render and edit through that API. Performance reports include machine identity; this brief establishes no machine-specific correctness threshold.

## Future products and enabling implications

After the substrate ships, a separate Product One repository may present a third-person explorer in a generated region (hills, mixed forest, river, cave) with skeletal animation and a curated cliff-to-cave traversal. That material is future-consumer context only. It motivates keeping generation, streaming, material query, mutation, surface extraction, and persistence reusable and public; it does not put controllers, characters, animation, forest population, curated routes, or game assets into Moria.

## Non-goals

- Implementing any game, game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content inside Moria.
- Privileged validation-executable or game paths into storage, meshing, or scheduler internals.
- Treating derived meshes or diagnostics as authoritative world state.
- Shipping Product One (or any other particular game) from this repository.

## Confirmed vision constraints

- Delivery form is a reusable Rust substrate consumed through public crate interfaces.
- Generation is deterministic for the same versioned parameters and seed.
- Mutation is admitted through the bounded public command path and commits atomically; persistence restores authoritative material state while meshes and diagnostics remain non-authoritative.
- Streaming bounds resident work, exposes the states requested, loading, resident, evicted, and failed, carries generation identities against stale replacement, and surfaces typed observable failures.
- The validation executable and the visual fixture (relocated external public-API consumer) use exactly the public interfaces and own no privileged world path.
- Performance reporting carries machine identity; this brief does not establish a machine-specific correctness threshold.

## Deferred design decisions

- Internal representation, algorithms, data layouts, crate packaging, and enforcement of the public-only boundary.
- Exact command shapes, persistence encoding, and diagnostics schema beyond the outcome mandates above.
- Depth and sequence of substrate capability slices for intermediate milestones.
- Harness UX details (camera, on-screen diagnostics layout) and any workload or platform choices for fixtures.
- Product One design, content, and acceptance as a separate product.
- Whether later acceptance introduces machine-specific performance gates; the current brief only requires machine-identified reporting and does not set such a threshold.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds agree on a single current product (the Rust voxel-world substrate), keep validation as required adjacent delivery outside product identity, and place Product One strictly in later-consumer context.

## Seed synthesis

- **README.md** — Names Moria as the reusable voxel-world substrate, limits repository deliverables to current substrate commitments, and classifies the interface reference as non-expanding support.
- **docs/seeds/mixed-project-brief.md** — Binding source for identity, public boundary, correctness, validation deliveries, non-goals, and the embedded Product One future-consumer distinction.
- **docs/seeds/substrate-interface-reference.md** — Supporting public-surface detail translated into world-identity, query, mutation, streaming lifecycle states, persistence, registered-object, and diagnostics outcomes without adding deliverables.
