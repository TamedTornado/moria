# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate. Games and a minimal validation executable consume it only through public crate interfaces. This repository delivers that substrate, not any particular game.

## Purpose

Provide a shared, multi-consumer foundation for voxel worlds: deterministic generation, authoritative material state, bounded streaming and mutation, surface extraction, persistence, and read-only diagnostics—usable without privileged access to internal storage, meshing, or scheduling.

## Product boundary

**In product:** the substrate crate surface and the world capabilities listed under required outcomes; public creation and identification of a world; bounded region requests; readiness; material queries; bounded edits; persistence of authoritative deltas; lifecycle-visible streaming; typed, observable failures; and read-only diagnostics.

**Adjacent, not identity:** a minimal validation executable and headless/visual fixtures that exercise only public interfaces. Delivery of those validation artifacts is a current program commitment; their camera, presentation, route, workload, and performance gates are not substrate product scope.

**Out of product:** any particular game, gameplay systems, controllers, characters, animation, authored levels, production content, or privileged consumer world paths.

## Required product outcomes

- **Multi-consumer substrate:** the same public interfaces serve multiple downstream games and the validation executable without any consumer reaching storage, meshing, or scheduler internals.
- **Deterministic generation:** the same versioned parameters and seed yield the same world generation.
- **Authoritative material world:** sparse voxel truth supports bounded queries and edits; mutation is admitted through a bounded command API and committed atomically; derived meshes and diagnostics never become authoritative state.
- **Bounded streaming with honest lifecycle:** resident work stays bounded; consumers observe states such as requested, loading, resident, evicted, and failed; background results carry generation identities so stale work cannot replace newer truth.
- **Persistence of material truth:** persistence records and restores the same authoritative material state via deltas, not derived meshes.
- **Observable correctness surface:** failures are typed and visible to public consumers; diagnostics report lifecycle and bounded work without exposing mutable internal handles; surface extraction supports consumer rendering without elevating meshes to world authority.

## Future products and enabling implications

A separate Product One (third-person explorer in a generated region with hills, mixed forest, river, and cave; skeletal animation; curated traversal) is a later game-facing consumer in another repository. It pressures interface usefulness only. It does not authorize player controllers, character meshes, animation clips, forest population workloads, curated routes, or game assets in Moria. Enabling implication: public APIs and world capabilities must remain general enough for such explorers without embedding their content or presentation.

## Non-goals

- Game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content.
- Treating the validation executable as a game prototype or granting it a privileged world path.
- Machine-specific correctness or performance thresholds as product identity (performance may be reported with machine identity for observation only).

## Confirmed vision constraints

- Implementation ecosystem: Rust public crate interfaces.
- Consumers have no privileged access to storage, meshing, or scheduler internals.
- Validation must use exactly the public interfaces; headless coverage of generation, query, mutation, persistence, and lifecycle; a small visual fixture that a relocated external consumer can render and edit through the public API.
- Generation determinism is tied to versioned parameters plus seed; mutation commits are atomic; persistence restores authoritative material state.

## Deferred design decisions

- Concrete API shapes, data layouts, storage and meshing strategies, scheduler design, and crate/workspace packaging.
- Streaming bounds, generation-identity schemes, and persistence encodings beyond the outcome guarantees above.
- Validation fixture presentation details (camera, diagnostics UI) and any non-threshold performance reporting format.
- Depth and prioritization of surface extraction and related consumer-facing derived views.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names Moria as the reusable voxel-world substrate, points at the mixed brief as the binding current-product definition, and treats the interface reference as non-expanding technical context.
- `docs/seeds/mixed-project-brief.md` — Authoritative current product, public boundary, correctness and validation commitments, non-goals, and explicit separation of later Product One consumer vision from Moria deliverables.
- `docs/seeds/substrate-interface-reference.md` — Supporting surface vocabulary (world identity, query/mutation/streaming/persistence/diagnostics outcomes) without adding deliverables or widening product scope.
