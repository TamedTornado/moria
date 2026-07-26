# Project vision

## What we are building now

**Moria** is a reusable **Rust voxel-world substrate**. This repository delivers that substrate—consumed through public crate interfaces by games and by a minimal validation executable—not any particular game.

## Purpose

Provide a shared, authoritative material world layer that multiple downstream consumers can generate, stream, query, edit, and persist without reimplementing world machinery or reaching into substrate internals. The product must remain useful to many consumers, not tuned to a single title.

## Product boundary

**In product**

- The substrate crate surface and the capabilities it owns: deterministic seed-based generation, sparse voxel storage, bounded streaming, mutation, surface extraction, persistence, and read-only diagnostics.
- A public consumer boundary: create and identify a world; request bounded regions; observe readiness; query material truth; submit bounded edits; persist deltas.
- Validation that proves those interfaces: headless fixtures for generation, query, mutation, persistence, and lifecycle; a small visual fixture that shows a relocated external-style consumer can render and edit through the same public API. The validation executable may include a free-fly camera and diagnostics sufficient to exercise the crate; it is not a game prototype and owns no privileged world path.

**Out of product**

- Any game, game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content.
- Storage, meshing, and scheduler internals as part of the public contract—consumers must not reach them.
- Later consumer fantasies (see below) as deliverables, assets, or in-repo game paths.

**Authority rule**

Material voxel state is authoritative. Derived meshes and diagnostics never become world truth. Mutation is admitted through a bounded command API and committed atomically. Persistence restores the same authoritative material state.

## Required product-level outcomes

These are consumer-visible guarantees the substrate must make true—not a feature inventory or delivery sequence:

1. **Deterministic generation.** The same versioned parameters and seed produce the same world identity and material results.
2. **Bounded, observable streaming.** Resident work is bounded; lifecycle states (at least requested, loading, resident, evicted, failed) are visible; background results carry generation identities so stale work cannot replace newer truth.
3. **Authoritative material queries.** Consumers can observe readiness and bounded material truth without treating meshes or diagnostics as source of truth.
4. **Bounded, atomic mutation.** Edits enter through an explicit command API with admission failures and commit revisions; committed material state is the authority.
5. **Durable material deltas.** Persistence records authoritative deltas (not derived meshes) and restores the same material state.
6. **Hard public boundary.** External consumers—including the in-repo validation executable—use only the public interfaces; failures remain typed and observable.
7. **Multi-consumer utility.** The surface stays general enough for multiple downstream games and tools, not a single demo’s private world path.

Supporting interface shape (from the reference seed; not extra scope): world identity combines format version, generation parameters, and seed; registered objects may participate in queries without becoming game entities; diagnostics report lifecycle and bounded work without exposing mutable internal handles.

## Future products and enabling implications

The program brief embeds a **later consumer vision**: after the substrate ships, a separate Product One repository may place a third-person explorer in a generated region (hills, mixed forest, river, cave) with skeletal animation and a curated cliff-to-cave traversal.

That material is **future-consumer context only**. It pressures the substrate toward general generation, streaming, mutation, meshing-for-view, and public-API consumption. It does **not** authorize a player controller, character mesh, animation clips, forest population workload, curated route, or game assets in Moria.

## Non-goals

- Implementing game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content.
- Shipping Product One or any other title in this repository.
- Giving the validation executable a privileged world path or treating free-fly/diagnostics as a game prototype.
- Making derived meshes or diagnostics authoritative world state.
- Establishing machine-specific performance correctness thresholds in this vision (performance may be reported with machine identity; thresholds are not product identity).

## Confirmed vision constraints

- Product identity is the reusable Rust voxel-world substrate, not a game.
- Only current substrate commitments are deliverables; later-product paragraphs do not expand scope.
- Validation uses exactly the public interfaces available to external consumers.
- Correctness centers on deterministic generation, atomic bounded mutation, material-authoritative persistence, bounded streaming with generation-aware stale rejection, and typed observable failures.

## Assumptions proposed for approval

1. **Validation is in current delivery, not product identity.** Headless and small visual fixtures are required to prove the public surface; they do not make Moria a walkable game or demo product.
2. **Interface-reference detail is binding shape, not a second product.** The reference seed elaborates how consumers meet the brief’s boundary; it does not add crates, games, or features beyond that boundary.

## Questions for human review

None. The seeds agree on current product (substrate), purpose (multi-consumer material world layer), boundary (public crate APIs only), validation role (public-API consumer, not game), and the status of Product One (future context, not deliverable). No ambiguity would change product identity, purpose, or boundary without contradicting the binding brief.

## Seed synthesis

- **`README.md`** — Names the repository product as Moria, the reusable voxel-world substrate defined by the mixed program brief; states that only current substrate commitments are deliverables; positions the substrate interface reference as supporting technical context without expanding scope; flags that the brief deliberately embeds a small later-product vision inside an otherwise binding current-product document.
- **`docs/seeds/mixed-project-brief.md`** — Binding source for current product, public boundary, correctness and validation commitments, non-goals, and the explicit demotion of Product One (third-person explorer, biome postcard, animation, curated traversal) to future-consumer context that must not import game assets or privileged paths into Moria.
- **`docs/seeds/substrate-interface-reference.md`** — Supporting elaboration of the reusable public surface (world identity, readiness and material queries, bounded mutation commands, streaming lifecycle states, delta persistence, registered objects without game-entity status, read-only diagnostics). Contributes interface pressure and vocabulary; does not add deliverables or change product identity.
