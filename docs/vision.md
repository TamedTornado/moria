# Project vision

## What we are building now

Moria is a reusable **Rust voxel-world substrate**: a public-crate library that owns world generation, sparse material storage, bounded streaming, mutation, surface extraction, persistence, and read-only diagnostics. This repository delivers that substrate for multiple independent consumers; it does not deliver a game.

## Purpose

Give game teams and other tools a shared, trustworthy voxel-world foundation they can integrate through public crate interfaces—deterministic enough to share seeds and parameters, bounded enough to stream and edit safely, and strict enough that only material state is authoritative—without each consumer reinventing world truth or reaching into engine internals.

## Product boundary

**In product:** the Moria substrate and its public consumer-facing guarantees (world identity and creation, bounded region request and readiness, material query, bounded edit admission and commit, persistence of material truth, derived surface data, and non-authoritative diagnostics).

**Adjacent, not product identity:** a minimal validation executable and fixtures that exercise the substrate only through the same public interfaces. They may exist to prove the crate; they are not a game prototype and hold no privileged world path. Presentation choices for that executable (for example a free-fly camera) are adjacent-artifact detail, not substrate identity.

**Out of product / downstream:** any particular game, including a later third-person explorer demo in a separate repository; gameplay systems, controllers, characters, animation, authored content, production assets, and game-specific policy remain consumer-owned.

## Required product outcomes

- Expose a public Rust crate surface through which multiple consumers create and identify worlds, request bounded regions, observe readiness, query authoritative material, submit bounded edits, and persist deltas—without access to storage, meshing, or scheduler internals.
- Generate material worlds deterministically from the same versioned parameters and seed; keep sparse voxel material as authoritative world state, and keep derived meshes and diagnostics non-authoritative.
- Stream and retain work in bounds: expose observable region lifecycle, carry generation identity on background results so stale work cannot replace newer truth, and surface typed failures consumers can observe.
- Admit mutations through a bounded command API that commits atomically with observable failure and commit identity; restore the same authoritative material state from persisted material deltas (not from derived meshes).
- Provide surface extraction and read-only diagnostics of lifecycle and bounded work without handing out mutable internal handles; allow registered objects to participate in queries without becoming game entities.
- Deliver the program’s adjacent validation using the public surface: headless fixtures covering generation, query, mutation, persistence, and lifecycle; a small visual fixture showing a relocated external consumer can render and edit through that API; performance reports that include machine identity without a machine-specific correctness threshold established by this product.

## Future products and enabling implications

A separate **Product One** repository may later ship a third-person explorer demo in a generated region. That game is a future consumer, not a Moria deliverable. It motivates keeping the substrate multi-consumer, streamable, editable, and renderable through the public API. It does **not** pull player control, character presentation, animation, forest or route content, or other game assets into Moria.

## Non-goals

- Game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content.
- Treating any validation harness as a game prototype or privileged consumer of world internals.
- Machine-specific performance pass/fail thresholds as product correctness (reporting with machine identity is required; hard thresholds are not established here).

## Confirmed vision constraints

- Delivery ecosystem is **Rust public crates**; intended consumers are games and tools that link those interfaces.
- Correctness: same versioned parameters and seed yield the same generation; mutation commits are atomic; persistence round-trips authoritative material; meshes and diagnostics never become world truth.
- Safety of concurrency and streaming: bounded resident work, observable lifecycle states, generation-tagged background results, typed observable failures.
- Encapsulation: external consumers must not depend on storage, meshing, or scheduler internals.
- Validation artifacts that ship with the program must use exactly the public interfaces and own no privileged world path.

## Deferred design decisions

- Concrete algorithms, data layouts, streaming bounds, crate and workspace layout, and API shape beyond the outcome guarantees above.
- Exact headless and visual fixture workloads, any camera or presentation choices in the visual fixture, and any performance budgets or target machines.
- Depth and sequencing of capability delivery within the substrate program; structure and content of any later consumer game.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds agree that the current product is the Rust voxel-world substrate, that games (including Product One) are out of repository scope, and that validation is an adjacent public-API consumer with binding fixture obligations—not a second product identity.

## Seed synthesis

- `README.md` names Moria as the reusable voxel-world substrate, limits repository deliverables to current substrate commitments, and treats the interface reference as non-expanding support.
- `docs/seeds/mixed-project-brief.md` binds product identity, public boundary, correctness and validation commitments, non-goals, and marks the embedded Product One explorer as future-consumer context only.
- `docs/seeds/substrate-interface-reference.md` details the public surface (identity, query, mutation, streaming lifecycle, persistence, registered objects, diagnostics) as outcome pressure without new product deliverables.
