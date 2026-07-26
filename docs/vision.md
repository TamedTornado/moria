# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. This repository delivers substrate crates plus a minimal public-interface validation harness. The harness is an adjacent validation artifact, not part of the substrate’s product identity.

## Purpose

Moria exists so independent consumers can build on a shared, authoritative voxel world without reimplementing world identity, generation, storage, mutation, streaming, meshing, query, or persistence. External products integrate through the same public interfaces the validation harness exercises.

## Product boundary

**In product (substrate):**

- Rust substrate crates that define, generate, hold, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces.
- Diagnostics that expose lifecycle, revision, and bounded-work observations without granting mutable internal handles.
- Public command and query surfaces suitable for external repositories with no privileged access path.

**Adjacent delivery (not product identity):**

- A minimal public-interface validation harness, including a headless fixture and a minimal visual fixture. The harness exercises the same API available to another repository. Fixture controllers, camera mode, overlays, presentation, and workloads are harness-owned, not substrate product features.

**Out of product:**

- Game rules, characters, controllers, animation, authored routes, production assets, and other consumer-specific content or policy.
- Any game layer; the harness is not a game.

## Required product outcomes

A downstream design must make these consumer-visible guarantees true:

- **World identity and generation.** A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
- **Authoritative material truth and mutation.** The substrate preserves authoritative material truth. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
- **Streaming and meshing.** Streaming bounds resident work and rejects stale background results. Meshing is a derived view of voxel truth and can be regenerated from that truth.
- **Deterministic queries.** The world supports deterministic queries. Registered objects can participate in those queries without becoming game entities.
- **Persistence of authority.** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
- **Public integration and observability.** Generate, stream, query, mutate, mesh, save, and restore are available through public interfaces shared by external consumers and the validation harness. Diagnostics expose lifecycle, revision, and bounded-work observations without mutable internal handles.

The repository also delivers headless and minimal visual validation fixtures that exercise those public capabilities (generation, streaming, mutation, queries, persistence, and meshing). Fixtures validate the substrate; they do not define gameplay or consumer content.

## Future products and enabling implications

No named downstream game is in current scope. Future consumers are external products that integrate via public interfaces. Enabling implication: the substrate’s public world, mutation, streaming, meshing, query, and persistence capabilities must be sufficient for independent games and tools to own their own rules, presentation, controllers, and content. Gameplay, UX, authored content, and game-specific policy remain consumer-owned.

## Non-goals

- Shipping a game, character systems, player controllers, animation, authored routes, or production assets as part of Moria.
- Treating the validation harness as a game layer or as the product’s identity.
- Treating performance measurements as portable correctness thresholds.
- Persisting derived meshes or transient scheduling state as authoritative truth.
- Giving consumers mutable internal handles in place of public diagnostics and APIs.

## Confirmed vision constraints

- Product form is a Rust substrate for external consumers (crate-delivered).
- Generation and world queries that the substrate defines as deterministic must be deterministic.
- Consumers and harnesses use public interfaces; validation does not require a privileged internal API.
- Diagnostics must not expose mutable internal handles.
- Performance measurements include machine identity and count as evidence only, not portable correctness thresholds.
- Persistence authority is limited to versioned authoritative deltas; restore must match prior query behavior for that authority.

## Deferred design decisions

- Internal storage layouts, algorithms, crate splits, and API surface shape beyond the outcome guarantees above.
- Streaming, meshing, and scheduling mechanisms and any timing or memory budgets.
- Exact fixture presentation, diagnostic overlay design, and harness workload selection.
- Depth and sequencing of capability delivery within the outcome families above.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` names Moria as a reusable voxel-world substrate and points current scope to the clean boundary and substrate requirement seeds.
- `docs/seeds/clean-project-boundary.md` fixes product identity as a reusable Rust voxel-world substrate, requires substrate crates plus a minimal public-interface validation harness, excludes game-layer concerns, and mandates public-interface completeness for generate, stream, query, mutate, mesh, save, and restore.
- `docs/seeds/clean-substrate-requirements.md` supplies the binding outcome families for identity, generation, storage and mutation, streaming, meshing, queries, persistence, diagnostics, and the headless and minimal visual validation fixtures, and states that performance figures are machine-bound evidence rather than portable thresholds.
