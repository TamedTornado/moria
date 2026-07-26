# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. This repository delivers the substrate crates and a minimal public-interface validation harness. The harness is an adjacent validation artifact that exercises the same public API other repositories use; it is not a game layer and is not part of the substrate’s product identity.

## Purpose

Moria exists so independent consumers can generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces, without owning world-authority machinery themselves and without privileged access to substrate internals.

## Product boundary

- **In product:** The reusable Rust substrate that owns authoritative voxel-world identity, generation, sparse material truth, bounded mutation, streaming of resident work, regenerable derived mesh views, deterministic world queries involving registered objects (without elevating them to game entities), and versioned persistence of authoritative deltas.
- **In repository, outside product identity:** A minimal validation harness (headless and minimal visual exercise of the public interface, including free-fly camera and diagnostic overlays as harness concerns only) that proves consumers can use the public API.
- **Out of product:** Game rules, characters, controllers, animation, authored routes, production assets, consumer-specific content, presentation policy, and any gameplay or UX layer.
- **Consumer-owned:** Integration choices, content, controls, and acceptance scenarios of downstream products; harness-specific controls, overlays, and presentation.

## Required product outcomes

- **World identity and generation:** A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
- **Authoritative storage and mutation:** Sparse storage preserves authoritative material truth. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
- **Streaming and derived views:** Streaming bounds resident work and rejects stale background results. Meshing is a derived, regenerable view of voxel truth, not authoritative truth. Registered objects can participate in deterministic world queries without becoming game entities.
- **Persistence:** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
- **Public validation and diagnostics:** Validation artifacts exercise generation, streaming, mutation, queries, persistence, and meshing only through public interfaces. Diagnostics expose lifecycle, revision, and bounded-work observations without mutable internal handles. Performance measurements include machine identity and are evidence, not portable correctness thresholds.

## Future products and enabling implications

No named downstream game or application is in current scope. Future external consumers (games or tools in other repositories) are the intended users of the substrate. Enabling implication: public, non-privileged interfaces must be sufficient for those consumers to build their own gameplay, content, and presentation on top of authoritative voxel worlds. Consumer-owned gameplay, characters, controllers, animation, routes, assets, and UX remain outside Moria.

## Non-goals

- Delivering a game, game rules, characters, controllers, animation, authored content, production assets, or consumer-specific presentation.
- Treating the validation harness as a product surface or game layer.
- Portable performance numbers as correctness gates.
- Privileged or internal-handle access for adjacent consumers or validation fixtures.

## Confirmed vision constraints

- Current product is a **Rust** substrate for **external** consumers; integration is through **public interfaces** only.
- The repository **delivers** substrate crates **and** a minimal public-interface validation harness (adjacent artifact; not product identity).
- World generation is **deterministic** under versioned parameters and seed; mutation lifecycle states are **explicit**; persistence restores **identical query behavior** from authoritative deltas only.
- Performance data carries **machine identity** and does not define portable correctness thresholds.

## Deferred design decisions

- Crate layout, API surface shape, algorithms, data layouts, and persistence encodings.
- Streaming, meshing, and scheduling policies beyond the outcome mandates above.
- How headless and visual validation are packaged relative to the substrate crates.
- Platforms, hardware targets, and any non-Rust integration stories.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md`: Names the product Moria and points current scope to the clean boundary and substrate-requirement seeds without adding downstream product vision.
- `docs/seeds/clean-project-boundary.md`: Establishes identity as a reusable Rust voxel-world substrate for external consumers; repository delivery of crates plus a minimal public validation harness; excludes game layer and consumer-owned content; requires public-interface completeness for generate, stream, query, mutate, mesh, save, and restore.
- `docs/seeds/clean-substrate-requirements.md`: Supplies the binding outcome substance for identity and deterministic bounded generation, sparse truth and public mutation lifecycle, streaming and regenerable derived views, query participation of registered objects, authoritative delta persistence, public validation fixtures, diagnostics without internal handles, and machine-identified performance evidence.
