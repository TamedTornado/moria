# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. This repository delivers substrate crates plus adjacent public-interface validation fixtures. It is not a game.

## Purpose

Moria exists so external consumers can generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces, without privileged access to substrate internals. It supplies the shared world substrate those consumers integrate against; it does not supply gameplay, presentation, or consumer-specific content.

## Product boundary

**In product:** the Rust substrate that owns authoritative voxel-world identity, generation, sparse material truth, bounded mutation admission, streaming of resident work, regenerable meshing as a derived view, deterministic world queries (including registered objects that are not game entities), versioned persistence of authoritative deltas, and diagnostics of lifecycle, revision, and bounded-work state—all exposed through public interfaces.

**Adjacent delivery (not product identity):** a headless validation fixture that exercises generation, streaming, mutation, queries, and persistence through those public interfaces; and a minimal visual fixture that exercises meshing through the same public surface (free-fly camera and diagnostic overlays are permitted for that purpose). These fixtures validate the substrate; they are not a game layer.

**Out of product:** game rules, characters, controllers, animation, authored routes, production assets, and other consumer-specific content, presentation, or policy. Downstream games and tools consume Moria; they are not part of Moria.

## Required product outcomes

- **World identity and generation.** A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
- **Authoritative material truth and mutation.** Sparse voxel storage preserves authoritative material truth. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
- **Streaming and derived views.** Streaming bounds resident work and rejects stale background results. Meshing is a regenerable derived view of voxel truth, not independent authority.
- **Deterministic queries.** Registered objects can participate in deterministic world queries without becoming game entities.
- **Persistence.** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
- **Public validation and diagnostics.** Validation fixtures exercise the public surface for generation, streaming, mutation, queries, persistence, and meshing. Diagnostics expose lifecycle, revision, and bounded-work observations without granting mutable internal handles. Performance measurements include machine identity and are evidence, not portable correctness thresholds.

## Future products and enabling implications

Downstream games and tools are future or external consumers of Moria. The substrate’s public world lifecycle enables those consumers; their gameplay, UX, controllers, characters, animation, authored content, presentation, and game-specific policy remain consumer-owned. No specific consumer product is in current scope.

## Non-goals

- Shipping a playable game, game rules, characters, controllers, animation, or production content as part of Moria.
- Treating validation fixtures as a game layer or as privileged consumers of internal APIs.
- Treating performance numbers as portable pass/fail correctness thresholds.
- Owning consumer-specific routes, assets, presentation, or policy.

## Confirmed vision constraints

- Ecosystem: Rust substrate crates for external consumers; integration is through public interfaces only.
- Authoritative truth is voxel material state (and versioned deltas of it); meshes and transient scheduling state are not truth.
- Mutation lifecycle states (admission, commit, failure) are explicit at the public command boundary.
- Validation must use the same public API surface available to other repositories.
- Performance evidence must carry machine identity and must not be read as portable correctness gates.

## Deferred design decisions

- Concrete parameter schemas, storage layouts, meshing algorithms, streaming schedules, and command encodings.
- Crate and workspace packaging, API surface shape, and diagnostic payload formats.
- Fixture implementation detail beyond the required public-interface exercise roles (exact camera, overlays, workloads, platforms, or thresholds).
- Delivery depth and sequencing of substrate capabilities within the outcome families above.

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds define a single current product (Rust voxel-world substrate), settle adjacent validation delivery without transferring fixture design into product identity, and do not present unresolved identity or boundary conflicts.

## Seed synthesis

- `README.md` names Moria as a reusable voxel-world substrate and points current scope to the two clean seeds.
- `docs/seeds/clean-project-boundary.md` fixes identity as a Rust substrate for external consumers, requires substrate crates plus a minimal public-interface validation harness, excludes game-layer ownership, and mandates public generate/stream/query/mutate/mesh/save/restore completeness.
- `docs/seeds/clean-substrate-requirements.md` supplies the binding outcome families (identity, generation, storage, mutation lifecycle, streaming, meshing, queries, persistence, diagnostics, and validation fixtures) and the performance-evidence constraint, without redefining product identity.
