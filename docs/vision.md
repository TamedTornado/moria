# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. This repository delivers substrate crates and a minimal public-interface validation harness that exercises the same API available to another repository. The harness is not a game layer and is not the product’s identity; it is an adjacent validation delivery that proves the public substrate works end to end.

## Purpose

Moria exists so independent consumers can generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces—without depending on game rules, characters, controllers, animation, authored routes, production assets, or other consumer-specific content.

## Product boundary

**In product**
- Reusable Rust substrate crates that expose public interfaces for an authoritative voxel world.
- World identity, deterministic generation of bounded regions, sparse authoritative material truth, bounded public mutation, streaming of resident work, regenerable meshing as a derived view, deterministic world queries that can include registered objects, and persistence of authoritative deltas with restore of identical query behavior.
- Diagnostics that expose lifecycle, revision, and bounded-work observations without handing out mutable internal handles.
- Delivery of a minimal public-interface validation harness (adjacent to product identity) that uses only the public API available to external repositories.

**Out of product**
- Game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content.
- Any game layer built on top of the substrate.
- Treating performance measurements as portable correctness thresholds; they are evidence that include machine identity.

## Required product outcomes

- **Authoritative world through public interfaces.** Consumers can generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world solely through public interfaces; no privileged internal access is required for correct use or for the validation harness.
- **Identity and deterministic generation.** A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
- **Authoritative mutation.** Sparse storage preserves authoritative material truth. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
- **Streaming and derived views.** Streaming bounds resident work and rejects stale background results. Meshing is a derived, regenerable view of voxel truth. Registered objects can participate in deterministic world queries without becoming game entities.
- **Persistence of truth, not derived state.** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
- **Public validation and diagnostics.** A headless fixture exercises generation, streaming, mutation, queries, and persistence. A minimal visual fixture with a free-fly camera exercises meshing through the public interface. Diagnostics expose lifecycle, revision, and bounded-work observations without mutable internal handles. Performance measurements include machine identity and remain evidence, not portable correctness gates.

## Future products and enabling implications

Downstream games and other external repositories are future or adjacent consumers of Moria, not part of the current product. Enabling implication: a complete public substrate for authoritative voxel worlds lets those consumers own gameplay, presentation, content, and policy while sharing generation, streaming, mutation, meshing, query, and persistence capability. No specific consumer title, slice, or acceptance scenario is committed here.

## Non-goals

- Shipping a game, character controller, animation system, authored routes, or production assets as Moria.
- Embedding game entities or game policy inside the substrate’s world model.
- Defining portable performance pass/fail thresholds for the substrate.
- Granting consumers or the validation harness privileged access to mutable internals.

## Confirmed vision constraints

- The product is a Rust substrate aimed at external consumers and other repositories.
- Validation must use the same public interfaces available outside this repository.
- Generation and world queries that participate in world identity are deterministic under the versioned parameter set and seed model.
- Persistence restores identical query behavior from versioned authoritative deltas; derived meshes and transient scheduling state are not truth.
- Performance data is machine-identified evidence, not a portable correctness contract.

## Deferred design decisions

- Internal crate layout, APIs beyond the public-interface obligation, algorithms, data layouts, and synchronization patterns.
- How streaming, meshing, mutation admission, and persistence encodings are implemented.
- Concrete fixture platforms, presentation beyond the required free-fly visual exercise of meshing, diagnostic UI detail, and any numeric performance budgets.
- Delivery sequence and depth of substrate capability within the outcome families above.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names Moria as a reusable voxel-world substrate and points current scope to the clean boundary and requirements seeds.
- `docs/seeds/clean-project-boundary.md` — Fixes current product identity (Rust substrate for external consumers), repository delivery of crates plus a minimal public validation harness, game-layer exclusions, and the end-to-end public-interface world capability set.
- `docs/seeds/clean-substrate-requirements.md` — Supplies outcome families for identity/generation, storage/mutation, streaming/derived views, persistence, public fixtures, diagnostics, and non-portable performance evidence; fused above into product mandates without mechanism inventory.
