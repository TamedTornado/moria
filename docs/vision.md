# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate delivered as public crate interfaces. This repository ships that substrate—not a game, demo, or player-facing product.

## Purpose

Moria exists so multiple downstream games and tools can share one world substrate: deterministic seed-based generation, material truth, bounded streaming and mutation, surface extraction, persistence, and read-only diagnostics—without each consumer reimplementing world authority or reaching into substrate internals.

## Product boundary

- **In product:** The reusable Rust substrate and its public crate surface for world identity, generation, sparse voxel material state, bounded region requests and streaming lifecycle, mutation, surface extraction, persistence of authoritative deltas, and read-only diagnostics.
- **Adjacent, not identity:** A minimal validation executable and fixtures that exercise the substrate only through the same public interfaces; they are not a game prototype and have no privileged world path.
- **Out of product:** Any particular game, player controller, character, animation, authored content, presentation layer, combat, inventory, AI, narrative, or production content pipeline.
- **Consumers:** Games and the validation executable consume Moria through public crates. No consumer may depend on storage, meshing, or scheduler internals.
- **Later program:** Product One and similar explorers are separate future consumers in other repositories; their gameplay and content do not expand Moria’s boundary.

## Required product outcomes

- **Multi-consumer public substrate:** Downstream games and tools integrate only through public Rust crate interfaces. Capabilities remain useful across consumers; none gain privileged access to internals.
- **World identity and deterministic generation:** Consumers create and identify worlds from format version, generation parameters, and seed. Generation is deterministic for the same versioned parameters and seed.
- **Authoritative material truth:** Consumers request bounded regions, observe readiness, and query bounded authoritative material. Sparse voxel storage and material authority live in the substrate. Registered objects may participate in queries without becoming game entities.
- **Bounded mutation:** Edits enter through a bounded command API with admission failures and commit revisions; admitted mutations commit atomically. Failures are typed and observable on the public surface.
- **Streaming, surfaces, persistence, diagnostics:** Streaming bounds resident work and exposes observable lifecycle states; background results carry generation identities so stale work cannot replace newer truth. Surface extraction and diagnostics never become authoritative world state. Persistence records authoritative deltas (not derived meshes) and restores the same material state. Diagnostics report lifecycle and bounded work without exposing mutable internal handles.
- **Adjacent validation:** Headless fixtures cover generation, query, mutation, persistence, and lifecycle behavior. A small visual fixture shows that a relocated external consumer can render and edit only through the public API; it may include a free-fly camera and diagnostics sufficient to exercise the crate. Performance reports include machine identity; this vision sets no machine-specific correctness threshold.

## Future products and enabling implications

After the substrate ships, a separate Product One (or similar) repository may build a third-person explorer in a generated region. That game is a future consumer. Moria’s enabling implication is a reusable public world surface those games can call—not ownership of controllers, characters, animation, terrain content, curated routes, or presentation.

## Non-goals

- Implementing game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content.
- Shipping any particular game or game prototype in this repository.
- Treating derived meshes or diagnostics as authoritative world state.
- Giving the validation executable or any other consumer a privileged path around the public API.

## Confirmed vision constraints

- Product form is a reusable Rust substrate consumed via public crate interfaces.
- Generation must be deterministic for the same versioned parameters and seed.
- Derived meshes and diagnostics must never become authoritative world state.
- Validation and other external consumers must use exactly the public interfaces, with no privileged world path.
- Performance reporting includes machine identity; machine-specific correctness thresholds are not part of this product promise.

## Deferred design decisions

- Concrete algorithms, data layouts, streaming bounds, persistence encodings, surface-extraction methods, and crate packaging beyond the public-interface mandate.
- Depth and sequencing of capability delivery within the substrate outcome families above.
- Validation fixture workloads, routes, presentation choices, and any performance gates beyond reporting with machine identity.
- How future games compose the substrate into gameplay, content, and UX.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` names Moria as the reusable voxel-world substrate for this repository, limits deliverables to current substrate commitments, and frames the interface reference as non-expanding support.
- `docs/seeds/mixed-project-brief.md` is the binding program brief for product identity, public boundary, correctness and validation commitments, non-goals, and Product One as future-consumer context only.
- `docs/seeds/substrate-interface-reference.md` supports outcome families on the public surface (world identity, queries, mutations, streaming states, persistence of deltas, registered objects, diagnostics) without adding deliverables or redefining identity.
