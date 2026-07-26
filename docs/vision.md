# Project vision

## What we are building now

**Moria** is a reusable Rust voxel-world substrate for external consumers. This
repository delivers substrate crates and a minimal public-interface validation
harness. The harness is not the product: it exercises the same public API
another repository would use.

## Purpose

Moria exists so external games and tools can generate, stream, query, mutate,
mesh, save, and restore an authoritative voxel world through public interfaces,
while owning their own gameplay, presentation, and content.

## Product boundary

**Belongs to Moria**

- Authoritative voxel-world identity, generation, sparse storage, mutation,
  streaming, meshing as a derived view, deterministic world queries (including
  registered objects that are not game entities), and persistence of
  authoritative truth.
- Public interfaces as the sole integration surface for external consumers.
- A minimal validation harness (headless and minimal visual) that exercises
  those interfaces. Free-fly camera and diagnostic overlays are harness means
  only, not game features.

**Does not belong to Moria**

- Game rules, characters, player or game controllers, animation, authored
  routes, production assets, and any consumer-specific content or policy.
- A game layer, title, or gameplay product built on the substrate.
- Portable performance gates; measurements are evidence and must carry machine
  identity.

## Required product outcomes

1. **World identity and generation** — A versioned parameter set and seed define
   world identity. Generation is deterministic and materializes bounded regions
   without eagerly allocating the complete world.
2. **Authoritative storage and mutation** — Sparse voxel storage holds
   authoritative material truth. Consumers submit bounded mutations through a
   public command API with explicit admission, commit, and failure states.
3. **Streaming and derived views** — Streaming bounds resident work and rejects
   stale background results. Meshing is a regenerable derived view of voxel
   truth, not a second source of truth. Registered objects may participate in
   deterministic world queries without becoming game entities.
4. **Persistence of truth** — Persistence records versioned authoritative deltas
   and restores identical query behavior. Derived meshes and transient
   scheduling state are not saved as truth.
5. **Public validation and diagnostics** — A headless fixture exercises
   generation, streaming, mutation, queries, and persistence. A minimal visual
   fixture with a free-fly camera exercises meshing through the public
   interface. Diagnostics expose lifecycle, revision, and bounded-work
   observations without mutable internal handles.
6. **Complete enough for external use** — Through public interfaces alone,
   consumers can generate, stream, query, mutate, mesh, save, and restore an
   authoritative voxel world.

## Future products and enabling implications

No future consumer product is in current scope. Downstream games or tools may
consume Moria; they own gameplay, UX, content, and presentation. The substrate’s
enabling implication is a stable public world API that such consumers can
integrate without privileged access. No specific title, genre, or consumer
feature set is committed here.

## Non-goals

- Shipping a game, character stack, animation system, or production content
  pipeline.
- Treating the validation harness as a playable product or game layer.
- Encoding harness controllers, routes, assets, or performance numbers as
  substrate acceptance criteria.
- Making performance measurements portable correctness thresholds independent of
  machine identity.
- Persisting derived meshes or transient scheduling state as authoritative truth.

## Confirmed vision constraints

- **Rust substrate for external consumers.** Integration is through public
  interfaces; consumers have no privileged access to internals.
- **Deterministic world identity.** Versioned parameters plus seed define
  identity; generation is deterministic for a given identity.
- **Explicit mutation lifecycle.** Bounded mutations use a public command API
  with admission, commit, and failure states.
- **Authoritative truth vs derived views.** Voxels and versioned deltas are
  truth; meshes and transient scheduling state are not persisted as truth.
- **Diagnostics without internal mutability.** Lifecycle, revision, and
  bounded-work observations are exposed without mutable internal handles.
- **Performance as evidence.** Measurements include machine identity and do not
  define portable correctness thresholds.
- **Harness is repository delivery, not product identity.** Headless and
  minimal visual fixtures are required to prove the public boundary; they are
  not a game layer.

## Deferred design decisions

- Concrete APIs, data layouts, algorithms, crate and package structure, and how
  the public boundary is enforced in the workspace.
- Streaming, meshing, and persistence encodings; synchronization patterns; how
  registered objects are modeled beyond “not game entities.”
- Harness implementation detail beyond exercising public interfaces (exact
  overlays, workloads, platforms).
- Any quantitative timing, memory, or throughput targets and the environments
  used to collect evidence.

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds agree on current product identity (Rust voxel-world
substrate), required repository delivery of a minimal public-interface
validation harness outside game/product identity, outcome families, and
exclusions. Other files under `docs/seeds/` (for example product-one or
architecture reference material) are not part of the current-scope manifest
named by the repository README and were not used to expand product identity.

## Seed synthesis

- **`README.md`** — Names the product Moria and points current scope only to the
  clean boundary and clean substrate-requirements seeds, without a downstream
  product vision.
- **`docs/seeds/clean-project-boundary.md`** — Fixes current identity as a
  reusable Rust voxel-world substrate for external consumers; repository
  delivers substrate crates plus a minimal public-interface validation harness
  that is not a game layer; lists game/content exclusions; requires completeness
  to generate, stream, query, mutate, mesh, save, and restore through public
  interfaces.
- **`docs/seeds/clean-substrate-requirements.md`** — Supplies binding outcome
  families for identity/generation, storage/mutation, streaming/derived views,
  persistence, public validation fixtures, diagnostics, and
  performance-as-evidence (not a portable gate).
