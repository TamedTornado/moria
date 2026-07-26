# Project vision

## What we are building now

Moria is a reusable **Rust voxel-world substrate**: a library consumed through public crate interfaces by games and other tools. This repository ships that substrate—not a finished game, not a genre prototype, and not production content.

## Purpose

Give multiple independent consumers a shared, trustworthy voxel world they can create, stream, inspect, edit, and persist without forking engine internals. Correctness and a strict public boundary matter so games and validation tools remain interchangeable clients of one substrate.

## Product boundary

**Belongs to Moria:** world identity and lifecycle; deterministic seed-based generation; sparse voxel authority; bounded region request and streaming; material query; bounded mutation; surface extraction for consumers that render; persistence of authoritative material deltas; read-only diagnostics; typed, observable failures.

**Does not belong to Moria:** game rules, combat, inventory, AI, narrative, characters, animation, controllers, authored levels, production assets, or any player-facing product.

**Adjacent, not identity:** a minimal validation executable and headless/visual fixtures that exercise the same public interfaces as external consumers. They prove the crate is usable from outside; they are not a game prototype and get no privileged world path. Their specific camera, character, content, route, platform, or performance gates are not substrate scope.

## Required product outcomes

- External consumers can create and identify a world, request bounded regions, observe readiness, query authoritative material, submit bounded edits, and persist deltas—only through public crate APIs, never storage, meshing, or scheduler internals.
- Generation is deterministic for the same versioned parameters and seed; mutation is admitted through a bounded command surface and committed atomically; persistence restores the same authoritative material state.
- Streaming keeps resident work bounded and exposes observable lifecycle states; background results carry generation identities so stale work cannot replace newer truth.
- Derived meshes and diagnostics never become authoritative world state; diagnostics report lifecycle and bounded work without exposing mutable internal handles; registered objects may participate in queries without becoming game entities.
- Failures are typed and observable to public consumers.
- Validation exists as an adjacent consumer of those public interfaces: headless coverage of generation, query, mutation, persistence, and lifecycle; a small visual fixture shows a relocated external consumer can render and edit through the public API. Performance may be reported with machine identity without machine-specific correctness thresholds in this vision.

## Future products and enabling implications

A separate later repository may ship **Product One**: a third-person explorer in a generated region (hills, mixed forest, river, cave) with skeletal animation and a curated traversal. That is a future game consumer, not Moria scope. It implies the substrate must remain multi-consumer and public-API-only so an independent game can render and edit without privileged access. It does not add player controllers, character meshes, animation clips, forest workloads, curated routes, or game assets to Moria.

## Non-goals

- Implementing game systems, characters, animation, authored levels, or production content in this repository.
- Treating the validation executable as a game prototype or granting it private substrate paths.
- Making derived presentation or diagnostics authoritative world truth.
- Importing Product One’s gameplay, presentation, or acceptance scenario into current delivery.

## Confirmed vision constraints

- Delivery form is a **Rust crate** with public interfaces as the sole integration surface for games and validation.
- The substrate must stay useful to multiple downstream consumers, not tuned as a single game’s private core.
- Determinism under versioned generation parameters and seed is a product promise.
- Consumers—including validation—have no privileged access to internals.
- Performance reporting, when present, includes machine identity; this vision does not fix machine-specific pass/fail thresholds.

## Deferred design decisions

- Concrete API shapes, command/query schemas, and crate package layout.
- Storage layout, streaming policy numbers, meshing approach, and scheduler design.
- Exact validation fixture platforms, cameras, workloads, and any numeric performance gates.
- Milestone order and how surface extraction is packaged for different consumers.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds fix current identity on the reusable Rust substrate, place Product One strictly as a future consumer, and treat validation as a required adjacent public-API consumer without folding its controls or content into product identity.

## Seed synthesis

- `README.md` — Names Moria as the reusable substrate, binds current deliverables to the program brief, and marks the interface reference as non-expanding support.
- `docs/seeds/mixed-project-brief.md` — Authoritative current-product boundary, correctness and validation commitments, non-goals, and explicit separation of later Product One context from Moria scope.
- `docs/seeds/substrate-interface-reference.md` — Supporting public-surface vocabulary (identity, readiness, bounded mutation, streaming states, delta persistence, diagnostics) fused into outcomes above; adds no new deliverables.
