# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate that exposes an authoritative voxel world to external consumers through public interfaces. The repository also delivers a minimal public-interface validation harness as an adjacent artifact; that harness is not the product.

## Purpose

Moria exists so other repositories can depend on a shared voxel-world substrate instead of owning world generation, mutation, streaming, meshing, and persistence themselves. Consumers integrate the substrate and build their own gameplay, presentation, and content on top of its public contracts.

## Product boundary

**In product scope**

- Authoritative voxel-world capabilities delivered as Rust substrate crates for external consumers.
- Public interfaces sufficient to generate, stream, query, mutate, mesh, save, and restore a world.
- Substrate-owned world identity, material truth, mutation admission, streaming residency, derived meshing, deterministic object queries, and versioned persistence of authoritative deltas.

**Out of product scope (adjacent or consumer-owned)**

- Game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content.
- Presentation, UX, and game-layer policy of any consuming product.
- Harness-owned unspecified scenarios, fixture data, extra controls beyond the mandated free-fly camera, overlay design, routes, and acceptance details not fixed by the required validation outcome families.

**Repository-adjacent delivery (not product identity)**

- This repository delivers a minimal public-interface validation harness kept outside product identity; it does not make Moria a game or tool application.
- A headless fixture must exercise generation, streaming, mutation, queries, and persistence through the public interface.
- A minimal visual fixture with a free-fly camera must exercise meshing through the public interface.
- Both fixtures use the same public surface available to another repository.

## Required product outcomes

1. **Public world lifecycle.** External consumers can generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world using only public interfaces.
2. **Deterministic world identity.** A versioned parameter set and seed define world identity; generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
3. **Authoritative mutation.** Authoritative material truth is preserved sparsely. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
4. **Bounded residency and derived views.** Streaming bounds resident work and rejects stale background results. Meshing is a regenerable derived view of voxel truth, not independent truth. Registered objects can participate in deterministic world queries without becoming game entities.
5. **Truthful persistence.** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
6. **Observable public operation.** Diagnostics expose lifecycle, revision, and bounded-work observations without granting mutable internal handles, so consumers and fixtures can validate behavior through the same public surface.

## Future products and enabling implications

No named future game or consumer product is in current scope. Downstream products in other repositories are expected consumers of the substrate. Enabling implication: anything that needs a shared, authoritative, streamable, meshable, and restorable voxel world can build on Moria without re-implementing those substrate outcomes. Gameplay, content, and presentation remain those products’ responsibility.

## Non-goals

- Shipping a playable game, character systems, animation, or authored gameplay content in this product.
- Treating the validation harness as a game layer or as the primary product experience.
- Promoting performance measurements into portable correctness thresholds.
- Making derived meshes or transient scheduling state part of authoritative saved truth.
- Granting consumers or fixtures privileged mutable access to internal substrate handles.

## Confirmed vision constraints

- Integration ecosystem is Rust substrate crates consumed by external repositories.
- Consumers and validation fixtures use the same public interfaces; diagnostics do not expose mutable internal handles.
- Performance measurements include machine identity and are evidence only, not portable correctness thresholds.
- Repository delivery includes a headless public-interface fixture covering generation, streaming, mutation, queries, and persistence, and a minimal visual public-interface fixture with a free-fly camera covering meshing; both remain outside product identity.

## Deferred design decisions

- Concrete APIs, data layouts, algorithms, crate splits, and workspace packaging.
- Streaming, meshing, and mutation scheduling strategies and resource budgets.
- Persistence encoding and revision representation details.
- Unspecified fixture scenarios, data, extra controls, overlay design, and implementation details beyond the settled headless and visual coverage outcome families.
- Depth and sequencing of capability delivery within the approved product outcomes.

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds and human review settle a single current product (Rust voxel-world substrate), a clear consumer boundary, required substrate outcomes, and binding repository-adjacent validation slices without elevating the harness into product identity.

## Seed synthesis

- `README.md` — Names the product Moria and points current scope to the clean boundary and substrate-requirements seeds only.
- `docs/seeds/clean-project-boundary.md` — Establishes the Rust substrate identity for external consumers, repository delivery of substrate plus a minimal public-interface harness, exclusion of game-layer concerns, and the end-to-end public world lifecycle mandate.
- `docs/seeds/clean-substrate-requirements.md` — Supplies the binding substrate outcomes for identity and generation, storage and mutation, streaming and derived views, persistence, mandated headless and visual validation coverage, diagnostics, and the non-portable nature of performance evidence.
