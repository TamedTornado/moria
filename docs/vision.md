# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate. This repository delivers that substrate as public crate interfaces for games and other consumers. It does not deliver any particular game.

## Purpose

Moria exists so multiple independent consumers can create, stream, inspect, edit, and persist voxel worlds through one shared public surface. Authoritative material truth stays in the substrate; presentation, gameplay, and content stay with consumers.

## Product boundary

**In product:** the reusable world substrate and its public crate surface—world identity, deterministic generation, sparse voxel material storage, bounded streaming, mutation, surface extraction as derived geometry, persistence of authoritative deltas, and read-only diagnostics.

**Adjacent (outside product identity, still program-delivered):** headless fixtures that cover generation, query, mutation, persistence, and lifecycle behavior; and a small visual fixture that shows a relocated external consumer can render and edit through the public API. Both use exactly those public interfaces, are not a game prototype, and own no privileged world path.

**Out of product:** any particular game, including a later Product One explorer demo in a separate repository. Gameplay systems, controllers, characters, animation, authored levels, production content, and game policy belong to consumers.

## Required product outcomes

- **Public multi-consumer surface.** Consumers create and identify a world (format version, generation parameters, and seed), request bounded regions, observe readiness, query authoritative material truth, submit bounded edits, and persist deltas—without reaching into storage, meshing, or scheduler internals.
- **Deterministic generation.** The same versioned parameters and seed produce the same world. The surface remains useful to multiple downstream consumers.
- **Authoritative material mutation.** Edits enter as bounded commands with admission failures and atomic commit revisions. Material state is authoritative; derived meshes and diagnostics are not.
- **Bounded streaming with honest lifecycle.** Resident work is bounded; consumers observe requested, loading, resident, evicted, and failed states. Background results carry generation identity so stale work cannot replace newer truth. Failures are typed and observable on the public surface.
- **Derived surfaces and safe diagnostics.** Surface extraction produces non-authoritative geometry. Diagnostics report lifecycle and bounded work without exposing mutable internal handles. Registered objects may participate in queries without becoming game entities.
- **Persistence of material truth.** Persistence records authoritative deltas rather than derived meshes and restores the same authoritative material state.

## Future products and enabling implications

A separate Product One repository may later ship a third-person explorer in a generated region (hills, mixed forest, river, cave) with skeletal animation and a curated traversal. That is future-consumer context only. It pressures the substrate to remain generatable, streamable, queryable, editable, and presentable through public interfaces; it does not add controllers, characters, animation, forest workloads, curated routes, or game assets to Moria.

## Non-goals

- Game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content.
- Player controllers, character meshes, animation clips, curated game routes, or other consumer-owned presentation and policy inside Moria.
- Privileged consumer access to storage, meshing, or scheduler internals.
- Treating derived meshes or diagnostics as authoritative world state.
- Shipping Product One or any other particular game from this repository.

## Confirmed vision constraints

- Implementation ecosystem: Rust public crate interfaces.
- Generation determinism for the same versioned parameters and seed.
- Atomic commit of admitted mutations; typed, observable public failures.
- Streaming lifecycle states are observable; generation identity prevents stale overwrite.
- Adjacent validation uses exactly the public interfaces and has no privileged world path.
- Performance reporting includes machine identity; this vision does not establish a machine-specific correctness threshold.

## Deferred design decisions

- Internal algorithms, data layouts, crate splits, and scheduling mechanisms.
- Exact streaming bounds, revision encodings, and persistence formats.
- Depth and sequence of capability delivery across releases.
- Visual-fixture presentation choices (for example free-fly camera detail) and headless fixture workload design.
- Any performance budgets or target environments beyond reporting machine identity with results.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds fix product identity as the Rust voxel-world substrate, settle validation as program-delivered adjacent artifacts outside product identity, and place Product One as a later separate consumer.

## Seed synthesis

- `README.md` — Names Moria as the reusable voxel-world substrate, limits this repository to current substrate commitments, and treats the interface reference as non-expanding support.
- `docs/seeds/mixed-project-brief.md` — Binding current-product identity, public boundary, correctness and validation commitments, non-goals, and embedded later Product One consumer context that must not enter Moria scope.
- `docs/seeds/substrate-interface-reference.md` — Supporting detail for the same substrate surface (identity, query, mutation, streaming lifecycle, persistence, registered objects, diagnostics), fused here only as outcome-level mandates.
