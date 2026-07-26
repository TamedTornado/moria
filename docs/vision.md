# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate delivered as public crate interfaces. Games and a minimal validation executable consume those interfaces. This repository delivers the substrate, not any particular game.

## Purpose

Moria exists so multiple independent games and tools can create, stream, query, mutate, and persist voxel worlds through one shared, correct substrate without each owning generation, storage, streaming, mutation, surface extraction, persistence, or diagnostic machinery.

## Product boundary

**In product scope**

- Public crate surface for world create/identify, bounded region request, readiness observation, material-truth query, bounded edit submission, and delta persistence.
- Substrate-owned world capabilities: deterministic seed-based generation, sparse voxel storage, bounded streaming, mutation, surface extraction, persistence, and read-only diagnostics usable by many consumers.
- Adjacent validation that exercises only the public interfaces (headless behavior coverage and a small visual fixture proving a relocated external consumer can render and edit through that API).

**Out of product scope (consumer- or later-product-owned)**

- Game rules, combat, inventory, AI, narrative, characters, animation, authored levels, production content, player controllers, curated routes, and game-facing presentation.
- Any privileged path into storage, meshing, or scheduler internals.
- Machine-specific performance pass/fail thresholds (performance may be reported with machine identity only).

The validation executable and fixtures are adjacent artifacts that validate the substrate; they are not a game prototype and do not redefine product identity.

## Required product outcomes

- Downstream consumers integrate solely through public Rust crate interfaces and can create and identify a world, request bounded regions, observe readiness, query authoritative material truth, submit bounded edits, and persist deltas.
- For the same versioned generation parameters and seed, generation produces the same world; consumers can rely on that determinism across runs.
- Mutation is admitted only as bounded commands and commits atomically; persistence restores the same authoritative material state; derived meshes and diagnostics never become authoritative world state.
- Streaming keeps resident work bounded and exposes observable lifecycle; background results carry generation identities so stale work cannot replace newer truth; failures are typed and observable to public consumers.
- Substrate capabilities remain useful to multiple independent consumers, not specialized to one game.
- Headless fixtures cover generation, query, mutation, persistence, and lifecycle behavior; a small visual fixture demonstrates that an external consumer can render and edit solely through the public API.

## Future products and enabling implications

**Product One** is a separate later repository and future consumer: a third-person explorer in a generated region (hills, mixed forest, river, cave) that may use skeletal animation and a curated cliff-to-cave traversal. That material is future-consumer context only.

Enabling implication only: Moria’s public world, streaming, mutation, material query, and persistence outcomes must remain adequate for such an external game to build on without privileged substrate access. Product One’s controller, character, animation, content, presentation, and curated traversal are not Moria scope and are not authorized by the embedded later vision.

## Non-goals

- Implementing any particular game, demo game loop, or production content pack inside this product.
- Game systems: rules, combat, inventory, AI, narrative, characters, animation, authored levels.
- Privileged consumer access to storage, meshing, or scheduler internals.
- Establishing machine-specific correctness or performance pass criteria for the substrate.
- Shipping Product One or any other game-facing product from this repository.

## Confirmed vision constraints

- Delivery form is a Rust substrate consumed via public crate interfaces.
- Generation is deterministic for identical versioned parameters and seed.
- Mutation uses a bounded command path with atomic commit; persistence restores authoritative material state only.
- Derived meshes and diagnostics are non-authoritative.
- Streaming bounds resident work, exposes lifecycle states, and protects truth with generation identities on background results; failures are typed and public-observable.
- Validation consumers use exactly the public interfaces and own no privileged world path.
- Performance reporting may include machine identity; this program does not fix a machine-specific correctness threshold.

## Deferred design decisions

- Concrete API shapes, data layouts, algorithms, crate packaging, and internal module structure.
- Exact streaming bounds, lifecycle detail beyond the required observability outcomes, and persistence encoding.
- Fixture and validation-executable presentation depth (beyond public-API-only exercise and proof of external render/edit).
- Performance measurement workloads and any future thresholds.
- Delivery sequencing and depth of substrate capability slices within the current product.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` names the product Moria, points at the program brief as the binding current-product definition, limits this repository to substrate commitments, and treats the interface reference as non-scoping support.
- `docs/seeds/mixed-project-brief.md` is the primary authority for current identity, public boundary, correctness and validation commitments, non-goals, and the embedded Product One future-consumer context that must not expand Moria scope.
- `docs/seeds/substrate-interface-reference.md` clarifies the consumer-facing world surface (identity, query, mutation, streaming lifecycle, persistence, diagnostics) without adding deliverables or widening product scope.
