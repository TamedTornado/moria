# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. This repository delivers substrate crates that expose an authoritative voxel world through public interfaces, plus a minimal public-interface validation harness that is adjacent to the product and is not a game layer.

## Purpose

Moria exists so independent consumers can generate, stream, query, mutate, mesh, save, and restore the same authoritative voxel world without embedding game rules, presentation, or consumer-specific content in the substrate. External repositories integrate through the same public interfaces the validation harness exercises.

## Product boundary

**In product:** the Rust substrate’s public capabilities for world identity and generation, sparse authoritative material truth, bounded mutation with explicit lifecycle outcomes, streaming of resident work, derived regenerable meshing, deterministic world queries (including non-entity registered objects), persistence of authoritative change, and diagnostics that observe lifecycle, revision, and bounded work without exposing mutable internal handles.

**Adjacent delivery (not product identity):** a minimal public-interface validation harness delivered by this repository. It exercises the substrate through the public API. A headless fixture covers generation, streaming, mutation, queries, and persistence. A minimal visual fixture exercises meshing through the public interface. Harness presentation choices (such as a free-fly camera or diagnostic overlays) may be used to exercise that API; they do not enlarge the product into a game layer.

**Out of product:** game rules, characters, controllers, animation, authored routes, production assets, and other consumer-specific content. Those remain with external consumers and games built on Moria.

## Required product outcomes

- **World identity and generation.** A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
- **Authoritative material truth.** Sparse voxel storage preserves authoritative material truth as the source of world state consumers rely on.
- **Bounded mutation.** Consumers submit bounded mutations through a public command API. Admission, commit, and failure states are explicit.
- **Streaming and derived meshing.** Streaming bounds resident work and rejects stale background results. Meshing is a derived view of voxel truth and can be regenerated from that truth.
- **Deterministic query surface.** Registered objects can participate in deterministic world queries without becoming game entities. Consumers can query the authoritative world through public interfaces.
- **Persistence and restore.** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
- **Public completeness.** Through public interfaces, consumers can generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world.
- **Diagnostics and measurement stance.** Diagnostics expose lifecycle, revision, and bounded-work observations without mutable internal handles. Performance measurements include machine identity and are evidence, not portable correctness thresholds.
- **Validation delivery.** The repository delivers a minimal public-interface validation harness with headless coverage of generation, streaming, mutation, queries, and persistence, and minimal visual coverage of meshing through the public interface.

## Future products and enabling implications

Downstream products are external consumer applications and games in other repositories. They own gameplay, UX, controllers, characters, animation, authored content, and presentation. Moria enables them by providing a reusable, publicly integrated voxel-world substrate; it does not schedule those consumers’ features or content as substrate work.

## Non-goals

- Shipping a game, game rules layer, characters, controllers, animation, authored routes, or production assets as part of Moria.
- Treating the validation harness as a game layer or as privileged access beyond the public consumer API.
- Treating performance numbers as portable correctness gates rather than machine-identified evidence.
- Persisting derived meshes or transient scheduling state as authoritative truth.

## Confirmed vision constraints

- **Ecosystem:** the product is a Rust substrate delivered as crates for external consumers.
- **Public integration:** external consumers and the validation harness use the same public interface surface; the harness does not define a second, privileged product API.
- **Mutation lifecycle:** admission, commit, and failure are explicit for bounded mutations.
- **Truth vs derived state:** voxel material truth is authoritative; meshes are derived and regenerable; meshes and transient scheduling are not persistence truth.
- **Measurement:** performance results carry machine identity and do not stand as portable correctness thresholds.

## Deferred design decisions

- Concrete public API shapes, command encodings, and diagnostic schemas.
- Generation, streaming, meshing, and persistence algorithms, layouts, and encodings.
- Crate and workspace packaging structure.
- Harness presentation details, workloads, and any non-correctness performance targets.
- Depth and sequencing of capability delivery within the substrate outcomes above.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` names Moria as a reusable voxel-world substrate and points current scope to the clean boundary and substrate requirement seeds.
- `docs/seeds/clean-project-boundary.md` fixes product identity as a Rust substrate for external consumers, requires public generate/stream/query/mutate/mesh/save/restore completeness, places the minimal validation harness as repository delivery outside a game layer, and excludes game-owned concerns.
- `docs/seeds/clean-substrate-requirements.md` supplies the outcome families for identity and deterministic generation, sparse authoritative truth, mutation lifecycle, streaming, derived meshing, non-entity query participation, persistence restore semantics, public validation fixtures, diagnostics, and the non-portable performance-evidence stance.
