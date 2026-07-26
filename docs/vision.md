# Project vision

## What we are building now

**Moria** is a reusable Rust voxel-world substrate. This repository delivers that substrate as public crate interfaces consumed by games and by a minimal validation executable. It does not deliver any particular game.

## Purpose

Provide a shared, deterministic foundation for voxel worlds so multiple independent downstream consumers can create, stream, query, edit, and persist material world state without reimplementing generation, storage, streaming, mutation, surface extraction, persistence, or diagnostics—and without reaching into substrate internals.

## Product boundary

**In scope.** The substrate and its public consumer surface: world create and identity; deterministic seed-based generation; sparse voxel storage; bounded region request and streaming; readiness and material query; bounded mutation; surface extraction; persistence of authoritative material deltas; read-only diagnostics.

**Adjacent validation (not product identity).** Headless fixtures covering generation, query, mutation, persistence, and lifecycle. A small visual fixture showing that a relocated external consumer can render and edit through the public API. A minimal validation executable that uses only those public interfaces; it may include a free-fly camera and diagnostics sufficient to exercise the crate, but it is not a game prototype and owns no privileged world path.

**Out of scope.** Any particular game (including a later Product One explorer), game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, production content, and any privileged path into storage, meshing, or scheduler internals.

## Required product-level outcomes

1. **Public multi-consumer substrate.** External consumers create and identify a world, request bounded regions, observe readiness, query material truth, submit bounded edits, and persist deltas—using only the public crate surface.
2. **Deterministic generation.** The same versioned parameters and seed produce the same generation.
3. **Bounded streaming with observable lifecycle.** Resident work stays bounded. Streaming exposes requested, loading, resident, evicted, and failed states. Background results carry generation identities so stale work cannot replace newer truth. Failures are typed and observable to public consumers.
4. **Authoritative material, non-authoritative derived views.** Mutation is admitted through a bounded command API (explicit bounds, admission failures, commit revisions) and committed atomically. Persistence records and restores the same authoritative material state via deltas. Derived meshes and diagnostics never become authoritative world state.
5. **Surface extraction and read-only diagnostics.** Consumers can obtain extracted surfaces for their own use. Diagnostics report lifecycle and bounded work without exposing mutable internal handles.
6. **Queryable registered objects without game-entity status.** Registered objects may participate in queries without becoming game entities owned by the substrate.
7. **Validation through the public API only.** Headless and visual fixtures, and the validation executable, exercise the same public interfaces available to external consumers. Performance is reported with machine identity; this vision does not set a machine-specific correctness threshold.

## Future products (context only)

After the substrate ships, a separate Product One repository may place a third-person explorer in a generated region (hills, dense mixed forest, river, cave), possibly with skeletal animation and a curated cliff-to-cave traversal. That material is future-consumer context embedded in the program brief. It explains interface pressure on the substrate; it does not authorize a player controller, character mesh, animation clips, forest population workload, curated route, or game asset in Moria.

Enabling implication for this product: public world identity, material query, streaming, mutation, surface extraction, and persistence outcomes must remain suitable for independent game repositories.

## Non-goals

- Implementing game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content inside Moria.
- Shipping Product One (or any other game) from this repository.
- Treating the validation executable or visual fixture as a game prototype or as a privileged substrate surface.
- Establishing machine-specific performance correctness thresholds as product pass/fail gates.
- Letting derived meshes or diagnostics become authoritative world state.

## Unresolved human questions

None. The seeds agree on current product identity (reusable voxel-world substrate), repository boundary (substrate and public-API validation only), and demotion of Product One to later-consumer context. No remaining disagreement would change product identity, purpose, or boundary.

Deferred to design (not vision blockers): concrete API shapes, crate packaging, algorithms, data layouts, streaming/eviction policy, and depth of the visual fixture beyond free-fly and diagnostics permission.

## Seed contributions

| Source | Contribution |
|---|---|
| `README.md` | Names Moria as the reusable voxel-world substrate defined by the mixed brief; limits repository deliverables to current substrate commitments; marks the interface reference as non-scoping support. |
| `docs/seeds/mixed-project-brief.md` | Binding current product, public boundary, correctness and validation commitments, non-goals, and explicitly demoted later Product One consumer vision. |
| `docs/seeds/substrate-interface-reference.md` | Supporting outcome detail for world identity, query/mutation/streaming/persistence shape, registered objects, and diagnostics—without adding deliverables or redefining product identity. |

**Intentionally omitted from vision.** Product One gameplay, content, characters, assets, and traversal narrative (kept as interface-pressure context only). Implementation prescriptions beyond product-level outcomes. Machine-specific performance gates.
