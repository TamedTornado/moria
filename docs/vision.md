# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. This repository delivers substrate crates and a minimal public-interface validation harness. The product is the substrate: an authoritative voxel world that consumers generate, stream, query, mutate, mesh, save, and restore through public interfaces. The harness is an adjacent repository delivery that exercises those interfaces; it is not a game and is not the product identity.

## Purpose

Moria exists so independent consumers can build on a shared, reusable voxel world without owning world generation, authoritative material storage, streaming, meshing, mutation admission, or persistence themselves. It provides deterministic world identity and public operations so external code can integrate against the same contract the repository’s validation harness uses.

## Product boundary

**Belongs to Moria (current product)**

- Public substrate capabilities for an authoritative voxel world: identity and generation, sparse material truth, streaming, mutation, queries, meshing as a derived view, and persistence of authoritative state.
- Diagnostics that expose lifecycle, revision, and bounded-work observations without granting mutable internal handles.
- Public interfaces as the integration surface for external consumers and for the repository’s validation harness.

**Adjacent repository delivery (not product identity)**

- A minimal public-interface validation harness, including headless exercise of generation, streaming, mutation, queries, and persistence, and a minimal visual exercise of meshing. Fixture controls, overlays, workloads, and presentation details remain harness behavior, not substrate product features.

**Outside the current product**

- Game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content or policy.
- Any game layer or consumer product built on Moria.

## Required product outcomes

- **Authoritative world operations.** Through public interfaces, consumers can generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world.
- **Deterministic identity and generation.** A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
- **Sparse material truth and bounded mutation.** Sparse storage preserves authoritative material truth. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
- **Streaming and derived meshing.** Streaming bounds resident work and rejects stale background results. Meshing is a regenerable derived view of voxel truth, not authoritative state.
- **Queries without game entities.** Registered objects can participate in deterministic world queries without becoming game entities.
- **Persistence of truth only.** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.

## Future products and enabling implications

Downstream games and other external applications are future or adjacent consumers of Moria, not part of the current product. Enabling implication: the substrate must remain complete enough at the public boundary for such consumers to generate, stream, query, mutate, mesh, and persist worlds without privileged access. Gameplay, UX, controllers, authored content, presentation, and game-specific policy stay consumer-owned.

## Non-goals

- Shipping a game, character, controller, animation system, authored content, or production asset pipeline.
- Treating the validation harness as a game layer or as the product consumers integrate against instead of the public substrate interfaces.
- Portable performance correctness thresholds derived from harness measurements.
- Consumer-specific rules, routes, or presentation policy inside the substrate.

## Confirmed vision constraints

- Moria is delivered as a Rust substrate for external consumers; integration is through public interfaces shared with validation.
- The repository delivers a minimal public-interface validation harness in addition to substrate crates.
- Diagnostics expose lifecycle, revision, and bounded-work observations without mutable internal handles.
- Performance measurements include machine identity and are evidence, not portable correctness thresholds.

## Deferred design decisions

- Concrete APIs, crate layout, data layouts, algorithms, and persistence encodings.
- Streaming, meshing, and scheduling mechanisms and their internal lifecycle details.
- Harness fixture protocol, coverage depth, rendering, and workloads.
- Capability depth and delivery sequence within the outcomes above.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names the product Moria and points current scope to the clean boundary and substrate requirement seeds.
- `docs/seeds/clean-project-boundary.md` — Establishes Moria as a reusable Rust voxel-world substrate, repository delivery of crates plus a non-game public-interface harness, exclusion of game-layer concerns, and the end-to-end public world operations mandate.
- `docs/seeds/clean-substrate-requirements.md` — Supplies the binding substrate outcomes for identity, generation, storage, mutation, streaming, meshing, queries, persistence, diagnostics, and validation posture without expanding product identity into harness or game detail.
