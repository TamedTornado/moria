# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. This repository delivers substrate crates and a minimal public-interface validation harness that is adjacent to the product, not a game layer.

## Purpose

Moria exists so independent consumers can generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces, without embedding game rules, characters, controllers, animation, authored routes, production assets, or other consumer-specific content in the substrate.

## Product boundary

**In product:** the reusable voxel-world substrate exposed through public interfaces—world identity and generation, sparse authoritative material storage, bounded mutation admission and commit, streaming of resident work, regenerable meshing as a derived view, deterministic world queries (including registered objects that are not game entities), versioned authoritative persistence, and diagnostic observations of lifecycle, revision, and bounded work without granting mutable internal handles for diagnostics.

**Adjacent repository delivery, not product identity:** a minimal public-interface validation harness. The harness exercises the same public API another repository would use. It must deliver a headless fixture that exercises generation, streaming, mutation, queries, and persistence, and a minimal visual fixture with a free-fly camera that exercises meshing through that public interface. The harness may use diagnostic overlays. Harness controllers, presentation, and fixture content are not substrate features.

**Out of product:** game rules, characters, controllers, animation, authored routes, production assets, consumer-specific content, and any game layer built on the substrate.

## Required product outcomes

- **World identity and generation.** A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
- **Authoritative material truth and mutation.** Sparse voxel storage preserves authoritative material truth. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
- **Streaming and derived meshing.** Streaming bounds resident work and rejects stale background results. Meshing is a derived view of voxel truth and can be regenerated from that truth.
- **Deterministic queries.** Registered objects can participate in deterministic world queries without becoming game entities. Consumers can query the authoritative world through public interfaces.
- **Persistence.** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
- **Public validation and diagnostics.** The repository must deliver a headless fixture that exercises generation, streaming, mutation, queries, and persistence, and a minimal visual fixture with a free-fly camera that exercises meshing, both only through public interfaces. Diagnostics expose lifecycle, revision, and bounded-work observations without mutable internal handles. Performance measurements include machine identity and are evidence, not portable correctness thresholds.

## Future products and enabling implications

No future consumer product is specified in the supplied seeds. External repositories and games may consume the substrate; their gameplay, content, presentation, controllers, and policy remain consumer-owned. The substrate’s enabling implication is that such consumers can rely on public, authoritative world operations without privileged internal access.

## Non-goals

- Shipping a game, game rules, characters, controllers, animation, authored routes, production assets, or consumer-specific content as part of Moria.
- Treating the validation harness as a game layer or as the product’s identity.
- Persisting derived meshes or transient scheduling state as authoritative truth.
- Treating performance measurements as portable correctness thresholds across machines.

## Confirmed vision constraints

- The product is a **Rust** substrate intended for **external** consumers.
- Consumers and the validation harness use **public interfaces** for world operations. **Diagnostics** expose observations without mutable internal handles.
- **Generation** and **world queries** are **deterministic** under the stated identity and registration model.
- Mutation has **explicit admission, commit, and failure** states.
- Persistence **records versioned authoritative deltas** and restores **identical query behavior**; derived meshes and transient scheduling state are not truth.
- Performance results are **machine-identified evidence**, not portable gates.

## Deferred design decisions

- Concrete public API shapes, crate layout, storage encodings, meshing algorithms, streaming schedules, and persistence formats.
- Depth and sequencing of substrate capabilities within releases.
- Exact harness fixture workloads, platforms, and presentation choices beyond the seed-level free-fly visual and headless exercise points.
- How diagnostics surface observations in tooling beyond the required observation kinds.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** names Moria as a reusable voxel-world substrate and points current scope to the clean boundary and substrate requirement seeds.
- **docs/seeds/clean-project-boundary.md** fixes current product identity as a Rust voxel-world substrate for external consumers, requires repository delivery of substrate crates plus a minimal public-interface validation harness, excludes game-layer ownership, permits free-fly camera and diagnostic overlays on the harness, and mandates public generate/stream/query/mutate/mesh/save/restore completeness.
- **docs/seeds/clean-substrate-requirements.md** supplies the outcome-level substrate mandates for identity and deterministic generation, sparse truth and mutation lifecycle, streaming and regenerable meshing, query participation of registered objects, persistence of authoritative deltas, required public headless and minimal visual validation fixtures, diagnostics without internal handles, and machine-bound performance evidence.
