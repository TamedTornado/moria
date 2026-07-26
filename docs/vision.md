# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate. It is delivered as public crate interfaces for external games and other consumers. This repository delivers that substrate, not any particular game.

## Purpose

Moria exists so multiple downstream games and tools can create, stream, observe, edit, and persist deterministic voxel material worlds through one shared public surface—without reimplementing those responsibilities or reaching into substrate internals.

## Product boundary

- **This product owns:** world creation and identity; deterministic seed-based generation; sparse voxel material storage; bounded streaming with observable lifecycle; bounded mutation; surface extraction as non-authoritative derived representation; persistence of authoritative material state; read-only diagnostics; exclusive access through public crate interfaces.
- **Adjacent settled validation delivery (not product identity):** a minimal validation executable and fixtures that exercise only those public interfaces—headless coverage of generation, query, mutation, persistence, and lifecycle, plus a small visual demonstration that a relocated external consumer can render and edit through the public API. They are not game prototypes and hold no privileged world path. Free-fly camera and diagnostic presentation, when present, belong to those artifacts.
- **Consumer-owned / out of product:** game rules, combat, inventory, AI, narrative, characters, animation, player controllers, authored levels, production content, curated traversal or scenery workloads, and any particular game’s presentation or policy.
- **Future, separate product:** Product One (separate repository) is a later game-facing consumer, not a Moria deliverable.

## Required product outcomes

- External consumers can create and identify a world from versioned generation parameters and seed, request bounded regions, observe readiness, and obtain bounded authoritative material observations.
- Generation is deterministic for the same versioned parameters and seed. Capabilities stay useful to multiple downstream consumers through the public surface alone.
- Consumers submit bounded edits only through a command API: admission can fail observably; admitted mutations commit atomically with visible revision; storage, meshing, and scheduler internals stay unreachable.
- Streaming bounds resident work, exposes observable lifecycle states, and tags background results so stale work cannot replace newer truth. Failures are typed and visible to public consumers.
- Surface extraction yields derived representations consumers may use for their own rendering; persistence records and restores authoritative material deltas so reloaded state matches prior material truth. Derived meshes and diagnostics never become authoritative world state.
- Read-only diagnostics report lifecycle and bounded work without exposing mutable internal handles. Registered objects may participate in queries without becoming game entities.

## Future products and enabling implications

After the substrate ships, a separate Product One repository may host a third-person explorer in a generated region (hills, mixed forest, river, cave) with skeletal animation and a curated cliff-to-cave traversal. That is future-consumer context only.

Moria’s enabling implication is that an external game crate can rely on public generation, streaming, material query and edit, surface extraction for rendering, and persistence—without privileged access and without Moria owning controllers, characters, animation, authored scenery, or game content.

## Non-goals

- Implementing game systems, characters, animation, authored levels, or production content inside Moria.
- Treating the validation executable or visual fixture as a game prototype or privileged consumer.
- Establishing machine-specific performance correctness thresholds as product law.
- Shipping Product One or any other particular game from this repository.

## Confirmed vision constraints

- Integration form is Rust public crate interfaces; adjacent consumers have no privileged world path.
- Same versioned parameters and seed yield deterministic generation; mutation commits are atomic; persistence restores the same authoritative material state.
- Derived meshes and diagnostics are never authoritative; failures affecting public consumers are typed and observable.
- Performance, when reported for validation, includes machine identity; this vision does not fix machine-specific correctness thresholds.
- Validation artifacts that exercise the substrate use the same public interfaces as external games.

## Deferred design decisions

- Concrete algorithms, data layouts, crate/package layout, and internal scheduling or meshing design.
- Exact public API shapes, command schemas, and lifecycle-state naming beyond the outcomes above.
- Delivery depth and sequence among owned capabilities; performance budgets and benchmark workloads.
- Validation-executable presentation details (camera, HUD, platforms) beyond public-API-only access.
- How far surface extraction and diagnostics are specialized for particular consumer renderers.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` names Moria as the reusable voxel-world substrate defined by the program brief, limits repository deliverables to current substrate commitments, and treats the interface reference as non-scoping technical context.
- `docs/seeds/mixed-project-brief.md` binds current product identity, public boundary, correctness and validation commitments, non-goals, and the explicit exclusion of embedded Product One consumer vision from Moria scope.
- `docs/seeds/substrate-interface-reference.md` supports the brief’s public surface (world identity, query, mutation, streaming lifecycle, persistence of deltas, registered objects, diagnostics) without adding deliverables or expanding product scope.
