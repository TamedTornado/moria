# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate. It is consumed through public crate interfaces by games and by a minimal validation executable. This repository delivers the substrate, not any particular game.

## Purpose

Moria exists so multiple downstream consumers can share one authoritative voxel world: create and identify worlds from versioned generation parameters and seeds, stream and observe bounded regions, query material truth, apply bounded edits, obtain derived surfaces, persist and restore authoritative state, and inspect lifecycle and work through read-only diagnostics—without game systems living in the substrate.

## Product boundary

**Moria (current product)** owns the reusable world substrate and its public crate surface: world identity and creation, deterministic seed-based generation, sparse voxel storage, bounded streaming and readiness, bounded mutation, surface extraction, persistence of authoritative deltas, and read-only diagnostics. Consumers must not reach into storage, meshing, or scheduler internals.

**Adjacent validation** is a required delivery of this repository but not part of product identity: headless fixtures, a minimal validation executable, and a small visual fixture that prove generation, query, mutation, persistence, lifecycle, and external render/edit through the public API only. Validation is not a game prototype and owns no privileged world path. Free-fly camera or similar exercise aids may exist on that executable; they are not substrate features.

**Downstream / future** products (including a separate Product One repository) own gameplay, controllers, characters, animation, presentation, authored content, routes, and game policy. References to those consumers explain interface pressure only.

## Required product outcomes

1. **Multi-consumer public substrate** — Capabilities remain useful to multiple consumers through public crate interfaces only; no privileged or internal paths for any consumer, including validation.

2. **World identity and deterministic generation** — Consumers create and identify worlds whose identity combines format version, generation parameters, and seed. Generation is deterministic for the same versioned parameters and seed.

3. **Bounded streaming and readiness** — Consumers request bounded regions, observe readiness, and see observable streaming lifecycle states. Resident work is bounded. Background results carry generation identities so stale work cannot replace newer truth. Failures are typed and observable.

4. **Authoritative material query and non-authority of derivatives** — Consumers query readiness and bounded authoritative material observations. Derived meshes and diagnostics never become authoritative world state. Registered objects may participate in queries without becoming game entities.

5. **Bounded mutation, surface extraction, and persistence** — Mutations are bounded commands with admission failures and commit revisions, committed atomically. The substrate provides surface extraction as a reusable, non-authoritative derived view of material truth. Persistence records authoritative deltas (not derived meshes) and restores the same authoritative material state.

6. **Diagnostics and public-API validation** — Read-only diagnostics report lifecycle and bounded work without exposing mutable internal handles. Headless fixtures cover generation, query, mutation, persistence, and lifecycle. A minimal validation executable and small visual fixture exercise the crate only through the public API, including that a relocated external consumer can render and edit that way. Performance may be reported with machine identity; no machine-specific correctness threshold is part of the product promise.

## Future products and enabling implications

After the reusable substrate ships, a separate Product One repository may place a third-person explorer in a generated region with hills, dense mixed forest, a river, and a cave, using skeletal animation and a curated cliff-to-cave traversal. That is future-consumer context. It reinforces that Moria must stay a multi-consumer public substrate; it does not authorize player controllers, character meshes, animation clips, forest population workloads, curated routes, or game assets in Moria.

## Non-goals

- Game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content.
- Shipping any particular game or game prototype as the product.
- Treating derived meshes or diagnostics as authoritative world state.
- Privileged world access for validation or other consumers.
- Machine-specific performance correctness thresholds.

## Confirmed vision constraints

- Ecosystem and delivery form: Rust public crate interfaces.
- Deterministic generation for the same versioned parameters and seed.
- Atomic mutation commits; persistence restores authoritative material state.
- Streaming bounds resident work; generation identities prevent stale overwrite.
- Typed, observable public failures; diagnostics are read-only and non-authoritative.
- Repository deliverables are current substrate commitments plus adjacent public-API validation, not later-consumer products.

## Deferred design decisions

- API shapes, data layouts, algorithms, packaging, and how the public boundary is enforced in the workspace.
- Exact streaming bounds, lifecycle machinery, and surface-extraction design.
- Validation fixture content, camera or control choices, platforms, workloads, and any performance targets.
- Depth and sequence of capability delivery within the current product.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Establishes Moria as the reusable voxel-world substrate, limits repository deliverables to current substrate commitments, and frames the interface reference as supporting context that does not expand scope.
- `docs/seeds/mixed-project-brief.md` — Binding source for identity, public boundary, correctness and validation commitments, non-goals, and the embedded later Product One vision (explicitly non-authorizing for Moria).
- `docs/seeds/substrate-interface-reference.md` — Supports outcome-level surface mandates (identity composition, query and mutation shape, streaming lifecycle, persistence deltas, registered objects, diagnostics limits) without adding deliverables or redefining product identity.
