# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. It is a library-style product: substrate crates that expose public interfaces for an authoritative voxel world. This repository also delivers a minimal validation harness that exercises those same public interfaces; the harness is an adjacent repository delivery, not part of the substrate’s product identity.

## Purpose

Moria exists so independent consumers can generate, stream, query, mutate, mesh, save, and restore a shared, authoritative voxel world without owning world identity, material truth, or the lifecycle of those operations themselves. It is infrastructure for voxel-world games and tools, not a game.

## Product boundary

**In product:** Public substrate capability to create and operate an authoritative voxel world—world identity and deterministic generation, sparse material storage, bounded mutation with explicit outcomes, streaming of resident work, regenerable meshing as a derived view, deterministic world queries (including participation by registered objects that are not game entities), persistence of authoritative state, and diagnostics that expose lifecycle, revision, and bounded-work observations without granting mutable internal handles.

**Repository delivery, outside product identity:** A minimal public-interface validation harness (including headless and minimal visual fixtures) that uses the same API available to any other repository. Fixture-specific controls, presentation, and exercise protocol are harness behavior, not substrate product surface.

**Out of product:** Game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content or policy. Consumers own gameplay, UX, and presentation built on the substrate.

## Required product outcomes

1. **World identity and generation** — A versioned parameter set and seed define world identity. Generation is deterministic and materializes bounded regions without eagerly allocating the complete world.
2. **Authoritative material truth and mutation** — Sparse voxel storage holds authoritative material truth. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
3. **Streaming and derived views** — Streaming bounds resident work and rejects stale background results. Meshing is a regenerable derived view of voxel truth, not a second source of truth.
4. **Queries** — Consumers can run deterministic world queries; registered objects may participate in those queries without becoming game entities.
5. **Persistence** — Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
6. **Public boundary and validation** — All of the above is available through public interfaces. Repository delivery includes a minimal harness that exercises generation, streaming, mutation, queries, persistence, and meshing via that public boundary. Diagnostics expose lifecycle, revision, and bounded-work observations without mutable internal handles.

## Future products and enabling implications

No future consumer product is named in the current seeds. Downstream games and tools are implied external consumers of the substrate. Enabling implication only: a complete public world substrate (identity through persistence and meshing) so such consumers need not reimplement authoritative voxel-world mechanics. Their gameplay, content, controllers, characters, animation, and presentation remain consumer-owned and are not substrate scope.

## Non-goals

- Shipping a game, game layer, or production gameplay experience in this product.
- Owning characters, player/game controllers, animation, authored routes, production assets, or consumer-specific content.
- Treating derived meshes or transient scheduling state as authoritative persisted truth.
- Portable performance correctness thresholds; performance measurements are evidence tied to machine identity, not product pass/fail guarantees.

## Confirmed vision constraints

- **Ecosystem:** Rust substrate crates intended for external consumers in other repositories.
- **Authority model:** Voxel material state is authoritative; meshes are derived and regenerable; persistence restores authoritative query behavior, not derived or scheduling state.
- **Determinism:** World identity (versioned parameters + seed) and generation yield deterministic worlds; queries that the product offers are deterministic.
- **Public integration:** Consumers and the validation harness use public interfaces; diagnostics must not expose mutable internal handles.
- **Harness role:** The repository delivers a minimal public-interface validation harness; it is not a game layer.
- **Performance evidence:** Measurements include machine identity and do not define portable correctness thresholds.

## Deferred design decisions

- Concrete APIs, data layouts, algorithms, crate and workspace layout, and enforcement of package boundaries.
- Streaming, meshing, mutation, and persistence implementation strategies and encodings.
- Depth and sequence of capability delivery within the approved product scope.
- Harness fixture design details (what each fixture covers, how visuals or headless runs are structured) beyond the vision-level requirement that public interfaces be exercised.
- Any performance budgets, target hardware, or benchmark suites (evidence-only stance is fixed; numeric gates are not).

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds define one coherent current product (Rust voxel-world substrate), a settled adjacent harness delivery, and explicit exclusions for game-layer concerns.

## Seed synthesis

- **README.md** — Names the product Moria and points current scope to the clean boundary and requirements seeds only.
- **docs/seeds/clean-project-boundary.md** — Establishes Rust external-consumer substrate identity, repository harness delivery, game-layer exclusions, and the end-to-end public capability set (generate through restore).
- **docs/seeds/clean-substrate-requirements.md** — Supplies the binding product-level guarantees for identity, generation, storage, mutation lifecycle, streaming, meshing, queries, persistence, diagnostics, public validation fixtures, and non-portable performance evidence.
