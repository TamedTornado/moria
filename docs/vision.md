# Project vision

## What we are building now

**Moria** is a reusable Rust voxel-world substrate: a public-crate world engine that other programs create, stream, query, edit, and persist through stable interfaces. This repository delivers that substrate, not a game.

## Purpose

Games and tools need a shared, multi-consumer foundation for deterministic, mutable voxel worlds whose material truth stays authoritative under streaming, edits, and restore. Moria exists so downstream products can build on that foundation without re-implementing world truth or reaching into substrate internals.

## Product boundary

**In product:** the reusable substrate and its public consumer surface—world identity, bounded region requests, readiness observation, material query, bounded mutation, persistence of authoritative deltas, streaming lifecycle visibility, and read-only diagnostics. Generation, sparse world representation, bounded streaming, mutation, surface extraction for consumers, and persistence are substrate responsibilities at this altitude. Derived presentation data and diagnostics never become authoritative world state.

**Adjacent, not the product identity:** a minimal validation executable (and fixtures) that exercise only those public interfaces. It is not a game prototype and owns no privileged world path. Its specific controls, camera, presentation, and workloads are harness concerns, not substrate identity.

**Out of product:** any particular game, player experience, authored content, or production title. External consumers must not depend on storage, meshing, or scheduler internals.

## Future products and enabling implications

**Product One** (separate repository, after the substrate ships) is a later game-facing demo: a third-person explorer in a generated region used to communicate the world. That consumer may pressure the substrate for large mutable outdoor regions and traversal, but its character, animation, curated route, forest population, and assets are not Moria deliverables.

Enabling implication only: the substrate should remain useful to multiple games that need deterministic generation, bounded streaming, mutation, and durable material state.

## Non-goals

- Game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content.
- Treating any single downstream demo’s presentation, controller, or acceptance scenario as current Moria scope.
- Exposing internals as the integration path, or elevating meshes/diagnostics to authoritative truth.

## Confirmed vision constraints

- Consumed through public crate interfaces; multi-consumer usefulness is required.
- Same versioned parameters and seed yield deterministic generation; mutations admit through a bounded command path and commit atomically; persistence restores the same authoritative material state.
- Streaming bounds resident work, exposes observable lifecycle, prevents stale results from replacing newer truth, and surfaces typed failures to public consumers.
- This brief does not invent machine-specific performance correctness thresholds.

## Assumptions proposed for approval

1. **Validation stays non-game:** whatever validation ships, it only proves public-interface fitness (headless and/or small visual exercise) and does not become a second product identity or a playable prototype.
2. **Interface reference is non-expanding:** world identity, readiness, bounded material observations, command revisions, streaming states, delta persistence, non-entity registered objects, and lifecycle diagnostics describe the intended public surface shape without adding new product scope.

## Questions for human review

**Q1.** Is a minimal validation harness a **required current deliverable of this repository**, or only a **permitted adjacent consumer** of the public crates?

- **Proposed answer:** Required as an adjacent in-repo harness that uses only public interfaces to exercise the substrate; not a game, and its camera/controls/presentation are not product scope.
- **If different:** If only permitted, this repository ships the substrate crates alone and validation may live elsewhere; repository boundary and “done” criteria shrink to crates without an in-repo consumer.

## Seed synthesis

- **`README.md`:** Names the product Moria; binds current deliverables to the substrate program brief; states the embedded later-product vision is not in-repo work; positions the interface reference as supporting context that does not expand scope.
- **`docs/seeds/mixed-project-brief.md`:** Supplies current product identity (reusable Rust voxel substrate), public boundary, correctness and validation commitments, non-goals, and explicitly quarantines Product One (explorer demo, animation, curated traversal, content) as future-consumer context only.
- **`docs/seeds/substrate-interface-reference.md`:** Contributes non-binding surface shape (identity, query/readiness, mutation commands, streaming states, delta persistence, registered objects, diagnostics) without new deliverables or scope expansion.
