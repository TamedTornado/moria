# Moria — Product Vision

## Current product

**Moria** is a reusable Rust voxel-world substrate. This repository ships the substrate and a minimal validation executable that exercises it through public crate interfaces. It does not ship a game.

## Purpose

Provide a stable, consumer-facing foundation so multiple games and tools can create, stream, inspect, mutate, and persist voxel worlds without depending on storage, meshing, or scheduler internals. Downstream products should be able to relocate onto the public API and remain correct and observable.

## Boundary

**In scope**

- Public crate interfaces for world identity, bounded region requests, readiness, authoritative material queries, bounded edits, and persistence of deltas
- Deterministic seed-based generation with versioned parameters
- Sparse voxel storage, bounded streaming with observable lifecycle states, atomic mutation admission, surface extraction as derived (non-authoritative) data, and read-only diagnostics
- Headless fixtures for generation, query, mutation, persistence, and lifecycle
- A small visual validation fixture (e.g. free-fly camera and diagnostics) that uses only the public API—no privileged world path, not a game prototype

**Out of scope (consumers)**

External consumers must not reach into storage, meshing, or scheduler internals. Derived meshes and diagnostics never become authoritative world state.

## Required product-level outcomes

1. **Reusable consumption** — Games and tools depend only on public interfaces; the validation executable is itself an external-style consumer.
2. **Deterministic generation** — Same versioned parameters and seed yield the same world.
3. **Bounded, observable streaming** — Resident work is bounded; lifecycle states (requested, loading, resident, evicted, failed) are visible; background results carry generation identities so stale work cannot overwrite newer truth.
4. **Safe mutation** — Edits enter through a bounded command API, with admission failures and atomic commit; commit revisions are observable.
5. **Authoritative persistence** — Restored material state matches what was committed; persistence records deltas, not derived meshes.
6. **Typed, observable failure** — Failures remain typed and visible to public consumers.
7. **Non-authoritative derived data** — Meshes, diagnostics, and similar outputs inform consumers but never define world truth.
8. **Validation without a game** — Headless and minimal visual fixtures prove the API; performance may be reported with machine identity, without machine-specific correctness thresholds mandated by the brief.

## Non-goals

- Game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content
- Player controllers, character meshes, animation clips, biome population workloads, curated routes, or game assets
- Any particular finished game (including “Product One” or similar demos described only as later consumers)
- Expanding product scope via interface reference details beyond the outcomes above

## Unresolved human questions

None that change product identity, purpose, or boundary. The seeds consistently name Moria (the substrate) as the current deliverable and treat later game-facing material as non-binding context.

---

## Seed contributions

| Seed | Role |
|------|------|
| `README.md` | Names the repository product as Moria; states that only current substrate commitments are deliverables; points at the two seed docs below. |
| `docs/seeds/mixed-project-brief.md` | Binding program brief: current product, public boundary, correctness and validation commitments, non-goals, and an embedded later-consumer vision (Product One-style explorer) marked as future context only. |
| `docs/seeds/substrate-interface-reference.md` | Supporting technical surface (world identity, queries, mutations, streaming states, persistence of deltas, registered objects in queries without becoming game entities, diagnostics). Does not add deliverables or widen product scope. |

**Preserved as capability pressure, not imported content:** later-consumer ideas (third-person exploration, hills/forest/river/cave regions, skeletal animation, curated traversal) imply that the substrate must support deterministic generation, streaming, material truth, mutation, and persistence suitable for such games—without implementing those games or their content here.
