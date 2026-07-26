# Project vision

## What we are building now

Moria is a reusable **Rust voxel-world substrate** delivered as public crate interfaces. This repository ships the substrate and adjacent validation that exercises it; it does not ship a game.

## Purpose

Games and tools need a shared, trustworthy world layer they can integrate without owning generation, storage, streaming, mutation, meshing, or persistence themselves. Moria exists so multiple independent consumers can create, stream, query, edit, and restore the same kind of authoritative voxel world through one stable public surface.

## Product boundary

**In product (Moria):**

- Public crate APIs for world identity, bounded region requests, readiness, material queries, bounded mutation, persistence, surface extraction, and read-only diagnostics.
- Substrate ownership of deterministic seed-based generation, sparse voxel material truth, bounded streaming with observable lifecycle, atomic mutation commit, and persistence of authoritative deltas.
- Enforcement that external consumers use only the public surface—no privileged path into storage, meshing, or scheduler internals.

**Adjacent to product (repository delivery, not product identity):**

- Headless fixtures covering generation, query, mutation, persistence, and lifecycle.
- A minimal validation executable and small visual fixture that use only public interfaces (for example free-fly camera and diagnostics). They are not a game prototype and own no privileged world path.

**Outside product:**

- Any particular game, including a later third-person explorer demo and all of its gameplay, content, presentation, controllers, characters, animation, and authored routes.

## Required product outcomes

- **Integrable substrate:** Independent consumers (games and the validation executable) create and identify worlds and drive them only through versioned public crate interfaces.
- **Deterministic generation:** The same versioned parameters and seed produce the same authoritative material world.
- **Bounded, observable streaming:** Consumers request bounded regions, observe readiness and lifecycle (including requested, loading, resident, evicted, and failed), and never accept stale background results in place of newer truth; failures stay typed and visible.
- **Authoritative material truth:** Queries expose readiness and bounded material observations; registered objects may participate in queries without becoming game entities; derived meshes and diagnostics never become world truth.
- **Safe mutation and restore:** Bounded edit commands are admitted or rejected explicitly, commit atomically with revisions, and persistence restores the same authoritative material state from deltas—not from derived meshes.
- **Consumer-visible diagnostics:** Lifecycle and bounded work are reportable without exposing mutable internal handles.
- **Validation exists and stays honest:** Headless and small visual validation prove the public API from an external consumer stance; performance may be reported with machine identity, but this vision does not set machine-specific pass/fail thresholds.

## Future products and enabling implications

A separate later product (Product One) may place a third-person explorer in a generated region and use curated traversal and presentation to communicate the world. That work lives outside this repository.

Moria only needs to remain a reusable world substrate those future games can consume. Enabling implications already in current outcomes—deterministic generation, streaming, mutation, surface extraction, persistence, and a strict public boundary—are enough at vision altitude. No player controller, character, animation, forest population, curated route, or game asset is implied as Moria scope.

## Non-goals

- Game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content.
- A game prototype, privileged consumer path, or any transfer of Product One presentation or gameplay into the substrate.
- Machine-specific performance correctness thresholds or consumer-owned hardware/backend choices as part of product identity.

## Confirmed vision constraints

- Delivery form is a **Rust crate / public crate interface** substrate, not an ecosystem-neutral or game-first product.
- Correctness commitments: seed-and-parameter determinism; atomic bounded mutation commit; persistence restores authoritative material state; meshes and diagnostics are non-authoritative.
- Streaming bounds resident work, exposes lifecycle, and protects against stale background replacement via generation identity.
- Adjacent validation is required repository delivery and must exercise only public interfaces; it does not redefine product identity.
- References to later consumers pressure the interface only; they do not authorize game systems in Moria.

## Deferred design decisions

- Concrete API shapes, data layouts, algorithms, crate splits, and internal scheduling or storage mechanisms.
- Exact streaming bounds, lifecycle machinery detail, and persistence encoding.
- Validation fixture contents beyond the mandate that they exist, stay public-API-only, and cover the named behavior families; visual fixture presentation beyond the non-game stance.
- How performance reporting is collected or displayed (beyond optional machine identity on reports).

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds settle a single current product (the reusable Rust voxel-world substrate), require adjacent public-API validation without making it the product, and place Product One firmly as a future external consumer.

## Seed synthesis

- **README.md:** Names Moria as the reusable voxel-world substrate, binds current deliverables to the program brief, and marks the interface reference as non-expanding support.
- **docs/seeds/mixed-project-brief.md:** Authoritative current product identity, public boundary, correctness and validation commitments, non-goals, and explicit separation of later Product One consumer vision from repository scope.
- **docs/seeds/substrate-interface-reference.md:** Supporting surface semantics (identity, query, mutation, streaming states, persistence of deltas, registered objects, diagnostics) fused into outcomes without adding deliverables.
