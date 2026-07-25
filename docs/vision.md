# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate: a public crate surface through which adjacent games and a minimal validation executable create, stream, query, mutate, and persist shared voxel worlds. This repository delivers that substrate, not a playable game.

## Purpose

Moria exists so multiple downstream consumers can rely on one authoritative, deterministic voxel world without each owning generation, residency, material truth, or persistence. The substrate absorbs that shared world responsibility so games and tools stay consumers of a stable public interface.

## Product boundary

**In product:** the reusable world substrate and its public crate interfaces—seed-parameterized generation, sparse world representation, bounded streaming and mutation, surface extraction for consumers, persistence of authoritative material state, and read-only diagnostics useful across consumers.

**Adjacent, required delivery (not product identity):** validation that exercises only those public interfaces—headless behavioral fixtures and a small external visual fixture that can render and edit through the same API. The harness may exist in-repo as a consumer under test; its camera, presentation, and exercise content are not Moria’s product.

**Out of product:** any particular game, player control scheme, characters, animation, authored levels, production content, combat, inventory, AI, narrative, or privileged paths into storage, meshing, or scheduler internals.

## Future products and enabling implications

**Product One** (separate repository, after the substrate ships) is a later third-person exploration demo in a generated region. It is future-consumer context only: it does not put controllers, characters, animation, forest population, curated routes, or game assets into Moria.

Enabling implication: public world identity, readiness, material query/edit, streaming lifecycle, and persistence must remain usable by relocated external consumers such as that demo—without transferring demo ownership into this product.

## Non-goals

- Game rules, gameplay systems, characters, animation, authored content, or production assets.
- Treating the validation executable as a game prototype or giving it privileged world access.
- Expanding this repository into Product One or any other specific title.

## Confirmed vision constraints

- Consumers (including validation) integrate only through public crate interfaces; they have no privileged access to substrate internals.
- Same versioned parameters and seed yield deterministic generation; derived meshes and diagnostics are never authoritative world state.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as the reusable voxel-world substrate scoped by the program brief, treats only current substrate commitments as repository deliverables, and marks the interface reference as non-scoping support; its boundary correction is fused into product identity above, with no remaining parallel authority.
- **docs/seeds/mixed-project-brief.md** — Supplies the binding current product (Rust substrate + public consumers), validation-as-adjacent-delivery, correctness and non-goal boundaries, and the embedded Product One future-consumer vision; compatible operational and fixture detail stays subordinate input to downstream design.
- **docs/seeds/substrate-interface-reference.md** — Confirms the consumer-facing surface shape (identity, query, mutation, streaming lifecycle, persistence, diagnostics) without adding deliverables; its operation-level detail remains subordinate design input, not a second product definition.
