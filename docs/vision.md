# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate. It is delivered in this repository as public crate interfaces for games and for a minimal validation executable. The repository delivers the substrate, not any particular game.

## Purpose

Moria exists so multiple downstream consumers can create, stream, inspect, edit, and persist voxel material worlds through one shared public boundary—without privileged access to internals and without embedding game systems in the substrate.

## Product boundary

**In product:** world creation and identity; deterministic seed-based generation; sparse voxel material truth; bounded region request and streaming with observable readiness; bounded mutation; surface extraction; persistence of authoritative material state; read-only diagnostics exposed to public consumers.

**Adjacent repository delivery (not product identity):** a minimal validation executable and fixtures that exercise only the public crate interfaces—headless coverage of substrate behavior and a small visual path showing a relocated external consumer can use the public API. Camera, controls, presentation, fixture content, and workloads remain harness-owned, not Moria product features.

**Out of product:** any particular game; game rules, combat, inventory, AI, narrative, characters, animation, authored levels, and production content. Consumers must not reach into storage, meshing, or scheduler internals.

## Required product outcomes

- Consumers can create and identify a world from versioned parameters and seed; generation is deterministic for the same versioned parameters and seed.
- Consumers can request bounded regions, observe readiness and streaming lifecycle, and rely on bounded resident work; background results carry generation identities so stale work cannot replace newer truth.
- Consumers can query authoritative material observations and use substrate surface extraction; derived meshes and diagnostics never become authoritative world state.
- Consumers can submit bounded edits through a command API; admitted mutations commit atomically; failures are typed and observable at the public boundary.
- Consumers can persist and restore the same authoritative material state; persistence records material deltas, not derived meshes.
- Public consumers, including validation, integrate only through the crate boundary; the repository’s adjacent validation exercises that boundary without a privileged world path.

## Future products and enabling implications

A separate **Product One** repository may later ship a third-person explorer in a generated region (hills, mixed forest, river, cave) with skeletal animation and a curated traversal. That is future-consumer context only. It does not place a player controller, character mesh, animation clips, forest population workload, curated route, or game assets in Moria.

Enabling implication at vision altitude: the substrate’s generation, streaming, material query, mutation, surface extraction, and persistence outcomes must remain useful to such game consumers after the substrate ships.

## Non-goals

- Implementing game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content.
- Shipping a game prototype or privileged world path for validation.
- Treating derived meshes or diagnostics as authoritative world state.
- Expanding current scope from later-consumer fiction embedded in program materials.

## Confirmed vision constraints

- Integration form is public Rust crate interfaces consumed by external games and validation code.
- Mutation is admitted only through the bounded public command path; consumers have no privileged internal access.
- Streaming exposes observable lifecycle states; public failures remain typed and observable.
- Validation that demonstrates relocation uses the public API only; it is not a game and owns no privileged world path.
- Performance reporting includes machine identity; this vision sets no machine-specific correctness threshold.

## Deferred design decisions

- Concrete API shapes, data layouts, algorithms, crate splits, and package layout beyond the public-boundary outcome.
- Streaming and residency policy details, exact lifecycle vocabulary implementation, and generation-identity machinery.
- Persistence encoding, surface-extraction approach, and diagnostic payload design.
- Validation fixture content, camera/controls presentation, workloads, and any quantitative performance gates.
- Delivery sequence and depth of substrate capabilities within the current product.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names Moria as the reusable voxel-world substrate, states that only current substrate commitments are repository deliverables, and marks the interface reference as non-expanding support.
- `docs/seeds/mixed-project-brief.md` — Binding current-product identity, public boundary, correctness and validation commitments, non-goals, and explicit separation of the later Product One consumer vision.
- `docs/seeds/substrate-interface-reference.md` — Supporting vocabulary for world identity, query/mutation/streaming/persistence/diagnostics semantics already required by the program brief; does not add deliverables.
