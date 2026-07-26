# Project vision

## What we are building now

**Moria** is a reusable **Rust voxel-world substrate**, consumed through public crate interfaces by games and by a minimal validation executable. This repository delivers the substrate—not any particular game.

## Purpose

Downstream products need a shared, trustworthy world layer they can integrate without each owning generation, storage, streaming, mutation, surface extraction, or persistence. Moria exists so multiple independent consumers can create, stream, query, edit, restore, and inspect the same kind of authoritative voxel world through one stable public surface.

## Product boundary

**In product (Moria):**

- Public crate APIs for world identity, bounded region requests, readiness observation, material query, bounded mutation, persistence, surface extraction, and read-only diagnostics.
- Substrate ownership of deterministic seed-based generation, sparse voxel material truth, bounded streaming with observable lifecycle, atomic mutation commit, and persistence of authoritative deltas.
- A hard external boundary: consumers must not reach into storage, meshing, or scheduler internals.

**Adjacent delivery (repository, not product identity):**

- Headless fixtures covering generation, query, mutation, persistence, and lifecycle behavior.
- A minimal validation executable and small visual fixture that use only the public interfaces (for example free-fly camera and diagnostics). They prove an external consumer can render and edit through the API; they are not a game prototype and own no privileged world path.

**Outside product:**

- Any particular game or demo experience, including a later third-person explorer and all of its gameplay, content, presentation, controllers, characters, animation, and authored routes.

## Required product outcomes

- **Integrable substrate:** Independent consumers create and identify worlds and drive them only through versioned public crate interfaces.
- **Deterministic generation:** The same versioned parameters and seed produce the same authoritative material world.
- **Bounded, observable streaming:** Consumers request bounded regions and observe readiness and lifecycle (requested, loading, resident, evicted, failed). Background results carry generation identities so stale work cannot replace newer truth; failures remain typed and visible to public consumers.
- **Authoritative material truth:** Queries expose readiness and bounded material observations. Registered objects may participate in queries without becoming game entities. Derived meshes and diagnostics never become world truth.
- **Safe mutation and restore:** Bounded edit commands are admitted or rejected explicitly, commit atomically with revisions, and persistence restores the same authoritative material state from deltas—not from derived meshes.
- **Consumer-visible diagnostics:** Lifecycle and bounded work are reportable without exposing mutable internal handles.
- **Honest validation:** Headless and small visual validation exercise the public API from an external-consumer stance. Performance reporting is a required validation outcome and must include machine identity; this vision does not establish machine-specific pass/fail thresholds.

## Future products and enabling implications

A separate later product (Product One), in another repository, may place a third-person explorer in a generated region and use curated traversal and presentation to communicate the world. That material is future-consumer context embedded for interface pressure only.

Moria enables such consumers by remaining a multi-consumer world substrate. Enabling implications already present in current outcomes—deterministic generation, streaming, mutation, surface extraction, persistence, and a strict public boundary—are sufficient at vision altitude. No player controller, character mesh, animation, forest population, curated route, or game asset is in Moria scope.

## Non-goals

- Game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content.
- A game prototype, privileged consumer path, or transfer of any later product’s presentation or gameplay into the substrate.
- Machine-specific performance correctness thresholds, or consumer-owned hardware and backend choices as part of product identity.

## Confirmed vision constraints

- Delivery form is a **Rust crate / public crate interface** substrate, not a game-first product.
- Correctness: seed-and-parameter determinism; atomic bounded mutation commit; persistence restores authoritative material state; meshes and diagnostics are non-authoritative.
- Streaming bounds resident work, exposes lifecycle, and protects against stale background replacement via generation identity.
- Adjacent validation is required repository delivery and must exercise only public interfaces; it does not redefine product identity. Performance reports must include machine identity; no machine-specific correctness threshold is established.
- References to later consumers pressure the interface only; they do not authorize game systems in Moria.

## Deferred design decisions

- Concrete API shapes, data layouts, algorithms, crate splits, and internal scheduling or storage mechanisms.
- Exact streaming bounds, lifecycle machinery detail, and persistence encoding.
- Validation fixture contents beyond the mandate that they exist, stay public-API-only, and cover the named behavior families; visual fixture presentation beyond the non-game stance.
- How performance reporting is collected or displayed (beyond the requirement that reports include machine identity).

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds agree on a single current product (the reusable Rust voxel-world substrate), treat validation as required adjacent delivery outside product identity, and place Product One firmly as a future external consumer.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable voxel-world substrate defined by the program brief, binds repository deliverables to current substrate commitments only, and marks the interface reference as supporting technical context that does not expand product scope.
- **`docs/seeds/mixed-project-brief.md`:** Authoritative current product identity, public consumer boundary, correctness and validation commitments, non-goals, and explicit separation of later Product One consumer vision from repository scope. Operational and fixture detail is preserved at outcome altitude, not as a feature inventory.
- **`docs/seeds/substrate-interface-reference.md`:** Supporting surface semantics (world identity, query/readiness, mutation commands, streaming lifecycle states, persistence of authoritative deltas, registered objects, diagnostics) fused into outcomes without adding deliverables.
