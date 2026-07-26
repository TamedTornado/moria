# Project vision

## What we are building now

**Moria** is a reusable Rust voxel-world substrate for external consumers. It exposes public interfaces that generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world. This repository delivers the substrate crates and a minimal public-interface validation harness; the product identity is the substrate, not a game or playable experience.

## Purpose

Give independent repositories a reusable world foundation so they can own gameplay, characters, controllers, animation, authored routes, production assets, and consumer-specific content without reimplementing world identity, authority, mutation, meshing, or persistence. The substrate makes deterministic, queryable world truth and regenerable derived meshing available through a public contract suitable for external consumers and for validation that uses only that same contract.

## Product boundary

**In product:** the substrate’s public contract for world identity and generation, resident streaming, authoritative mutation, derived meshing, world queries (including registered objects that are not game entities), persistence of authoritative state, and diagnostics that expose lifecycle, revision, and bounded-work observations without mutable internal handles.

**Adjacent repository delivery (not product identity):** a minimal validation harness that exercises the public interfaces—a headless fixture for generation, streaming, mutation, queries, and persistence, and a minimal visual fixture with a free-fly camera that exercises meshing. The free-fly camera is required for that visual fixture and remains harness-local; diagnostic overlays may be used as harness-local aids. Neither is a product feature or shipped game UX.

**Out of product:** game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content. Any game or tool built on Moria is a separate consumer.

## Required product outcomes

A downstream design must make these true:

1. **Public, reusable substrate.** External consumers integrate through ordinary public interfaces only; the harness uses that same surface, with no privileged internal path.
2. **Identity and deterministic generation.** A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
3. **Authoritative material truth and bounded mutation.** Sparse storage preserves authoritative material truth. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
4. **Streaming and derived meshing.** Streaming bounds resident work and rejects stale background results. Meshing is a regenerable derived view of voxel truth, not authority.
5. **Queryable world.** Deterministic world queries are available; registered objects can participate in those queries without becoming game entities.
6. **Persistence of authority.** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
7. **Observable operation.** Diagnostics expose lifecycle, revision, and bounded-work observations without granting mutable internal handles.
8. **Public-interface validation.** A headless fixture exercises generation, streaming, mutation, queries, and persistence through the public interface. A minimal visual fixture with a free-fly camera exercises meshing through the public interface.

## Future products and enabling implications

No first-party game or named downstream title is in current scope. Future consumers are external products that depend on Moria’s public world APIs.

**Enabling implications (not current roadmap commitments):**

- External games and tools can own mutable voxel worlds without reimplementing generation, authority, streaming, meshing, queries, or save/restore.
- Independent repositories can integrate and validate against the same public interfaces the harness uses.
- Game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content remain outside Moria; derived meshing and its public visual-fixture exercise stay substrate-owned.

## Non-goals

- Shipping a playable game, character, controller stack, animation, authored routes, or production content in this product.
- Treating the validation harness as a game layer or as the product’s identity.
- Treating performance measurements as portable pass/fail correctness thresholds.
- Saving derived meshes or transient scheduling state as authoritative truth.
- Exposing mutable internal handles as the consumer integration surface.

## Confirmed vision constraints

- Delivered as a Rust substrate for external consumers; integration is through public interfaces shared with any other repository.
- World identity uses a versioned parameter set and seed; generation is deterministic; regions may materialize without full-world eager allocation.
- Authoritative material truth is distinct from regenerable derived views; persistence restores authoritative truth and identical query behavior, not meshes or transient scheduling state.
- Bounded mutations use a public command API with explicit admission, commit, and failure; streaming bounds resident work and rejects stale background results.
- Diagnostics report lifecycle, revision, and bounded-work observations without mutable internal handles.
- Performance measurements include machine identity and are evidence only, not portable correctness thresholds.

## Deferred design decisions

- Algorithms, data layouts, crate packaging, and exact public API shapes.
- Streaming topology, meshing approach, mutation admission policy details, and persistence encoding.
- How validation fixtures are structured beyond the required headless exercise and the required minimal visual fixture with free-fly camera.
- Capability depth and delivery sequence within the outcomes above.
- Concrete free-fly camera and diagnostic overlay design details (presence of the free-fly camera on the visual fixture is not deferred).

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds fix product identity (reusable Rust voxel-world substrate), repository delivery (substrate crates plus minimal public-interface validation harness), outcome families (generate, stream, query, mutate, mesh, save/restore with the constraints above), and consumer ownership limited to game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content—not presentation as a whole. Platform targets, performance budgets, architecture, and acceptance workloads remain downstream design.

## Seed synthesis

- **README.md** — Names the product Moria as a reusable voxel-world substrate and points current scope to the clean boundary and substrate seeds.
- **docs/seeds/clean-project-boundary.md** — Binds current identity to a reusable Rust voxel-world substrate for external consumers; repository delivers substrate crates and a minimal public-interface validation harness; harness is not a game layer; game rules, characters, controllers, animation, routes, production assets, and consumer content are out of scope; product must generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces.
- **docs/seeds/clean-substrate-requirements.md** — Supplies the fused outcome substance: deterministic identity/generation from versioned parameters and seed; sparse authoritative storage and explicit mutation contract; streaming and regenerable meshing; registered objects in queries without game-entity status; versioned authoritative persistence; headless fixture exercise of generation, streaming, mutation, queries, and persistence; minimal visual fixture with free-fly camera to exercise meshing through the public interface; diagnostics without mutable internals; performance as machine-scoped evidence, not portable correctness gates.
