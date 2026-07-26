# Project vision

## What we are building now

**Moria** is a reusable Rust voxel-world substrate for external consumers. This repository delivers substrate crates and a minimal public-interface validation harness. The product is the world foundation—not a game, not a first-party title, and not consumer content.

## Purpose

Give other repositories a stable, public foundation for authoritative mutable voxel worlds so they can own gameplay, presentation, and content without reimplementing world identity, generation, authority, streaming, meshing, or persistence. The harness exists only to prove that foundation through the same public API another repository would use.

## Product boundary

**In scope**

- Substrate crates that expose public interfaces to generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world.
- A minimal validation harness that exercises those interfaces: a headless fixture for generation, streaming, mutation, queries, and persistence; and a minimal visual fixture with a free-fly camera that exercises meshing. Diagnostic overlays may aid observation.
- Public diagnostics for lifecycle, revision, and bounded-work observations—without mutable internal handles as the integration surface.

**Out of scope**

- Game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content.
- Treating the harness as a game layer, shipped UX, or product surface beyond contract validation.
- Portable performance pass/fail thresholds; measurements include machine identity and are evidence, not correctness gates.

**Repository role**

Ship the substrate and the public-interface harness. The harness is an adjacent consumer of the product under test, not part of the substrate’s product identity.

## Required product-level outcomes

These are vision-altitude capabilities the current product must provide. Design and delivery own mechanisms and sequencing.

1. **World identity and generation** — A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
2. **Authoritative storage and mutation** — Sparse voxel storage preserves material truth. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
3. **Streaming and derived views** — Streaming bounds resident work and rejects stale background results. Meshing is a derived view of voxel truth and can be regenerated. Registered objects can participate in deterministic world queries without becoming game entities.
4. **Persistence** — Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
5. **Public validation** — Headless and minimal visual fixtures exercise the public contract end to end so external consumers can rely on the same surface the harness uses.

## Non-goals

- Shipping a playable game, character stack, controller, animation, authored content, or production assets in this repository.
- Building consumer-specific presentation or gameplay on top of the substrate inside current scope.
- Exposing internal mutable handles as the integration contract.
- Treating performance numbers as portable correctness thresholds.

## Future products and enabling implications

No named downstream game or first-party title is in current scope. Future products are external consumers that will depend on Moria’s public world APIs.

**Enabling implications (context only—not current commitments):** external games and tools can build mutable voxel worlds without owning identity, authority, streaming, meshing, or save/restore; long-horizon consumer features (gameplay, characters, presentation, content) remain outside Moria.

## Assumptions proposed for approval

None. The supplied seeds agree on product identity, purpose, boundary, and required outcomes without fill-in.

## Questions for human review

None. Product identity (reusable Rust voxel-world substrate), repository delivery (crates plus minimal public-interface validation harness), and consumer ownership (gameplay, characters, controllers, content, game-layer presentation) are already fixed by the seeds. Capability depth, architecture, platforms, delivery sequence, and acceptance workloads are downstream design questions, not vision ambiguities.

## Seed synthesis

| Seed | Contribution |
|------|----------------|
| **README.md** | Names the product Moria; states that current scope is defined by the clean boundary and substrate seeds without embedding a separate downstream product vision. |
| **docs/seeds/clean-project-boundary.md** | Fixes current identity as a reusable Rust voxel-world substrate for external consumers; repository delivers substrate crates and a minimal public-interface validation harness; harness may use free-fly camera and diagnostics but is not a game layer; game rules, characters, controllers, animation, routes, production assets, and consumer content are outside current product; product must be complete enough to generate, stream, query, mutate, mesh, save, and restore through public interfaces. |
| **docs/seeds/clean-substrate-requirements.md** | Contributes vision-level responsibilities: deterministic identity/generation from versioned parameters and seed; authoritative sparse storage and explicit mutation contract; streaming bounds and regenerable meshing; registered objects in queries without game-entity status; versioned authoritative persistence (not derived/transient state); headless and minimal visual public validation; diagnostics without mutable internals; performance as machine-scoped evidence. |

**Not imported into current scope:** Other files under `docs/seeds/` (for example product-one or mixed-authority material) were outside the seed-document manifest for this synthesis and are treated as non-authoritative for current product identity.
