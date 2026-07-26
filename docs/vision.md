# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. The repository ships substrate crates and a minimal public-interface validation harness that exercises the same API available to another repository.

## Purpose

Moria exists so independent consumers can generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces alone, without owning world identity, material truth, or persistence rules inside each product.

## Product boundary

Belongs to Moria:

- Substrate capabilities that make an authoritative voxel world usable end-to-end through public interfaces: generation, streaming, query, mutation, meshing, save, and restore.
- A minimal validation harness that uses only those public interfaces (not a game layer).
- Diagnostics that expose lifecycle, revision, and bounded-work observations without giving consumers mutable internal handles.

Does not belong to Moria:

- Game rules, characters, controllers, animation, authored routes, production assets, and other consumer-specific content or policy.
- Presentation, UX, or workload choices that a game or other consumer invents beyond what the public substrate must enable.
- Portable performance thresholds or machine-agnostic correctness gates derived from benchmark runs.

The harness may use a free-fly camera and diagnostic overlays to exercise the API. Those harness details validate the substrate; they are not product features that games must adopt. Gameplay and consumer presentation remain adjacent ownership.

## Required product outcomes

- External consumers can, through public interfaces only, generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world.
- A versioned parameter set and seed define world identity; generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
- Sparse storage preserves authoritative material truth. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
- Streaming keeps resident work bounded and rejects stale background results. Meshing is a derived, regenerable view of voxel truth, not authoritative truth.
- Registered objects can participate in deterministic world queries without becoming game entities.
- Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.

The repository delivers public validation as an adjacent harness: a headless fixture that exercises generation, streaming, mutation, queries, and persistence; and a minimal visual fixture that exercises meshing through the public interface. Diagnostics cover lifecycle, revision, and bounded-work observations without mutable internal handles.

## Future products and enabling implications

No named downstream game is in current scope. Future products are external consumers in separate repositories. Enabling implication: the public substrate must remain sufficient for such consumers to own their own rules, content, controllers, and presentation while relying on Moria for authoritative world generation, mutation, query, meshing, streaming, and persistence.

## Non-goals

- Building a game, character layer, animation system, or authored gameplay content in this product.
- Treating the validation harness as a shippable game experience or privileged internal API surface.
- Making performance numbers from any machine into portable product correctness requirements.
- Absorbing consumer-specific policy, assets, routes, or UX into the substrate.

## Confirmed vision constraints

- Integration form is a reusable Rust substrate for external consumers (library crates), not an ecosystem-neutral or game-ship product.
- Consumers and the validation harness share the same public interfaces; adjacent consumers have no privileged access to internals.
- World behavior that must match across generate, mutate, query, and restore is deterministic under the versioned identity model.
- Authoritative truth is material/voxel state and versioned persistence deltas; meshes and transient scheduling are not truth.
- Mutation outcomes expose explicit admission, commit, and failure states.
- Performance evidence is machine-identified and non-portable as correctness thresholds.

## Deferred design decisions

- Concrete APIs, crate layout, data layouts, algorithms, and enforcement of the public-only boundary.
- Exact streaming, meshing, mutation, and persistence designs within the outcome mandates above.
- Harness implementation depth beyond the minimal public-interface role (controls, overlays, scenes).
- How and where performance is measured operationally, short of inventing portable gates.
- Delivery sequence and slice depth for substrate capabilities.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md`: Names the product Moria as a reusable voxel-world substrate and points current scope to the clean boundary and requirements seeds only.
- `docs/seeds/clean-project-boundary.md`: Fixes identity as a Rust substrate for external consumers, requires public end-to-end world operations, places game concerns outside the product, and commits the repository to a minimal public-interface validation harness that is not a game layer.
- `docs/seeds/clean-substrate-requirements.md`: Supplies binding outcome-level mandates for identity, deterministic bounded generation, sparse authoritative storage, public mutation lifecycle, streaming, regenerable meshing, query-capable registered objects, versioned delta persistence, public fixtures, diagnostics, and machine-identified performance evidence.
