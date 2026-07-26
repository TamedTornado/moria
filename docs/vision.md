# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate: public crates that external consumers use to generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world. The repository also delivers a minimal public-interface validation harness; that harness is an adjacent delivery, not the product identity.

## Purpose

Moria exists so other repositories can build on a reusable voxel-world substrate without owning world identity, storage truth, mutation admission, streaming residency, meshing derivation, or persistence themselves. It provides the reusable world engine surface; it does not provide a game.

## Product boundary

**This product owns**

- Substrate crates and the public interfaces that let external consumers operate an authoritative voxel world.
- World identity, generation, sparse material truth, mutation commands, streaming residency bounds, regenerable derived meshes, deterministic world queries (including registered objects that are not game entities), and persistence of authoritative state.
- Diagnostics that expose lifecycle, revision, and bounded-work observations without handing out mutable internal handles.

**Adjacent repository delivery (not product identity)**

- A minimal validation harness that uses only the same public interfaces available to another repository. The harness is not a game layer. Its headless fixture exercises generation, streaming, mutation, queries, and persistence. Its minimal visual fixture includes a free-fly camera and exercises meshing through the public interface. Diagnostic overlays may be used.

**Outside this product**

- Game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content and policy.

## Required product outcomes

- **World identity and generation.** A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
- **Authoritative material and mutation.** Sparse storage preserves authoritative material truth. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
- **Streaming and derived mesh.** Streaming bounds resident work and rejects stale background results. Meshing is a regenerable derived view of voxel truth, not a second source of truth.
- **Queries.** Registered objects can participate in deterministic world queries without becoming game entities.
- **Persistence.** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
- **Public completeness and validation.** Through public interfaces alone, consumers can generate, stream, query, mutate, mesh, save, and restore the world. A headless fixture exercises generation, streaming, mutation, queries, and persistence. A minimal visual fixture with a free-fly camera exercises meshing through the public interface. Performance measurements include machine identity and are evidence, not portable correctness thresholds.

## Future products and enabling implications

No named future game or product is in scope. The intended consumers are external repositories that integrate the substrate crates. Enabling implication: those consumers can build their own gameplay, presentation, and content on a world that remains identity-stable, query-consistent after restore, and operable without privileged internal access. Their gameplay, UX, controllers, assets, and policies stay with those consumers.

## Non-goals

- Shipping a game, game layer, or consumer-owned rules, characters, controllers, animation, routes, or production content inside this product.
- Treating derived meshes or transient scheduling state as authoritative truth.
- Using performance numbers as portable correctness gates independent of machine identity.
- Granting consumers or the validation harness privileged access beyond the public interface.

## Confirmed vision constraints

- The substrate is a Rust library surface for external consumers.
- Harness and consumers exercise the same public API; no privileged internal access for validation or integration.
- World generation and the query surface that includes registered objects are deterministic as specified for those outcomes.
- Persistence records versioned authoritative deltas; restore must yield identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
- Performance figures are machine-identified evidence only, not portable product pass/fail thresholds.

## Deferred design decisions

- Concrete API shapes, crate layout, storage encoding, streaming and meshing strategies, and mutation admission policy details.
- Exact diagnostic fields, fixture workloads, free-fly camera implementation details, and how overlay tooling is implemented in the harness.
- Depth and sequence of capability delivery within the outcomes above.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` names Moria as a reusable voxel-world substrate and points current scope to the two clean seed documents.
- `docs/seeds/clean-project-boundary.md` fixes product identity (Rust substrate crates for external consumers), required world operations through public interfaces, required adjacent harness delivery without making it a game, and explicit exclusion of gameplay and consumer content.
- `docs/seeds/clean-substrate-requirements.md` supplies the outcome families for identity, generation, storage, mutation, streaming, meshing, queries, persistence, public validation fixtures, diagnostics, and the status of performance evidence.
