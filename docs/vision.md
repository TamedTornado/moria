# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate consumed through public crate interfaces. This repository delivers that substrate for multiple downstream consumers; it does not deliver a game.

## Purpose

Moria exists so games and other external consumers can create, identify, stream, query, mutate, derive surfaces from, persist, and diagnose voxel worlds through one shared public boundary—without each consumer owning generation, material storage, or lifecycle internals.

## Product boundary

**This product owns** the substrate’s public world capabilities: world creation and identity; deterministic seed-based generation; sparse material world state; bounded region request and streaming with observable lifecycle; material queries; bounded mutation; surface extraction as derived non-authoritative output; persistence of authoritative material deltas; and read-only diagnostics. Public failures and lifecycle remain typed and observable.

**Adjacent to this product** are a minimal validation executable and headless and visual fixtures that exercise the substrate through those same public interfaces. They check and demonstrate the crate; they are not a game prototype and own no privileged world path. Controllers, characters, presentation choices, authored content, routes, and machine-specific performance gates stay with those adjacent artifacts or with later consumers—not in Moria’s identity.

**Outside this product** are particular games, including the later Product One explorer. Game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, and production content are consumer-owned. External consumers must not reach into storage, meshing, or scheduler internals.

## Required product outcomes

- **Public multi-consumer substrate.** Games and a minimal validation executable integrate only through public crate interfaces. The substrate stays useful to multiple consumers and grants no privileged access to storage, meshing, or scheduler internals.
- **Identifiable deterministic worlds.** Consumers create and identify a world whose identity combines format version, generation parameters, and seed. Generation is deterministic for the same versioned parameters and seed.
- **Bounded material truth and streaming.** Consumers request bounded regions, observe readiness, and obtain bounded authoritative material observations. Streaming bounds resident work and exposes observable lifecycle states (requested, loading, resident, evicted, failed). Background results carry generation identities so stale work cannot replace newer truth.
- **Bounded atomic mutation.** Consumers submit bounded edit commands with explicit admission failures; admitted mutations commit atomically with commit revisions visible on the public surface.
- **Material authority, derived surfaces, and non-game participants.** Persistence records and restores authoritative material state as deltas, not derived meshes. Surface extraction and diagnostics never become authoritative world state. Registered objects may participate in queries without becoming game entities. Read-only diagnostics report lifecycle and bounded work without exposing mutable internal handles. Failures remain typed and observable to public consumers.
- **Public-API validation coverage.** Headless fixtures cover generation, query, mutation, persistence, and lifecycle behavior. A small visual fixture shows that a relocated external consumer can render and edit through the public API. Performance is reported with machine identity; this program does not establish machine-specific correctness thresholds.

## Future products and enabling implications

After the reusable substrate ships, a separate Product One repository may place a third-person explorer in a generated region with hills, dense mixed forest, river, and cave, and may use skeletal animation and a curated cliff-to-cave traversal to communicate the world. That future consumer motivates a capable multi-consumer substrate; it does not authorize a player controller, character mesh, animation clips, forest population workload, curated route, or game asset inside Moria.

## Non-goals

- Game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content
- Shipping any particular game or game prototype from this repository
- Treating derived meshes or diagnostics as authoritative world state
- Privileged consumer access to storage, meshing, or scheduler internals
- Machine-specific performance thresholds as substrate correctness criteria

## Confirmed vision constraints

- Intended integration is Rust public crate interfaces
- Repository product deliverable is the substrate, not a game
- Validation artifacts use exactly the public interfaces and own no privileged world path
- Generation determinism is bound to versioned parameters and seed; mutation commits are atomic; streaming work is bounded and lifecycle-observable
- Persistence restores the same authoritative material state; stale background work must not overwrite newer truth
- Validation performance reporting includes machine identity without binding machine-specific thresholds

## Deferred design decisions

- Public API shapes, crate layout, and internal architecture
- Generation methods, storage layout, streaming bound policy, and surface-extraction methods
- Persistence encoding and any synchronization patterns
- Delivery depth and sequence within the substrate program
- Packaging of headless fixtures and non-product presentation choices of the visual fixture
- How performance is measured and compared across machines without becoming a correctness bar

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` establishes Moria as the reusable voxel-world substrate defined by the program brief, limits this repository’s deliverables to current substrate commitments, and treats the interface reference as supporting context that does not expand scope.
- `docs/seeds/mixed-project-brief.md` supplies binding current identity, public boundary, correctness, validation commitments, and non-goals, and embeds Product One only as future-consumer context that must not enlarge Moria.
- `docs/seeds/substrate-interface-reference.md` supports the same substrate surface—world identity, queries, mutation, streaming lifecycle, persistence, registered objects, and diagnostics—translated here into outcome-level mandates without new deliverables.
