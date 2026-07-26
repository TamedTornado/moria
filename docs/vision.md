# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

## Current product

**Moria** is a reusable Rust voxel-world substrate: a family of crates (and a minimal validation harness) that generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces.

The repository ships substrate crates plus harnesses that exercise only those public APIs. It does not ship a game.

## Purpose

Give external consumers a complete substrate for material voxel worlds—deterministic identity and generation, sparse authoritative storage, bounded mutation, streaming of resident work, derived meshing, object-aware world queries, and versioned persistence—so games and tools can be built elsewhere without forking or embedding privileged engine paths.

## Boundary

| In scope | Out of scope |
|---|---|
| Substrate crates and their public command/query surfaces | Game rules, characters, controllers, animation |
| Deterministic generation from versioned parameters + seed | Authored routes, production assets, consumer-specific content |
| Sparse storage of authoritative material truth | A game layer (the harness is validation only) |
| Streaming; meshing as a regenerable derived view | Privileged harness-only mutation or query paths |
| Persistence of versioned authoritative deltas | Treating meshes or scheduling state as truth |
| Headless and minimal visual fixtures (free-fly camera, diagnostics) as public-API validation only | Mutable internal handles exposed as diagnostics |

The validation harness may look like a thin viewer; it is not a game layer. Anything it does must use the same interfaces available to another repository.

## Required product-level outcomes

1. **World identity** — A versioned parameter set and seed fully define a world; generation is deterministic and can materialize bounded regions without eagerly allocating the entire world.
2. **Authoritative matter** — Sparse storage holds material truth; consumers submit bounded mutations through a public command API with explicit admission, commit, and failure.
3. **Bounded work** — Streaming keeps resident work in bounds and rejects stale background results; meshing is a derived view that can be regenerated from truth.
4. **Queryable world** — Registered objects can participate in deterministic world queries without becoming game entities.
5. **Persistence** — Versioned authoritative deltas restore identical query behavior; derived meshes and transient scheduling state are not saved as truth.
6. **Public validation** — A headless fixture exercises generation, streaming, mutation, queries, and persistence; a minimal visual fixture exercises meshing through the public interface; diagnostics expose lifecycle, revision, and bounded-work observations without mutable internal handles.
7. **Evidence, not thresholds** — Performance measurements include machine identity and serve as evidence, not portable correctness bars.

## Non-goals

- Implementing a game layer, game rules, characters, controllers, animation, authored routes, production assets, or consumer-specific content in this repository
- Shipping privileged APIs usable only by in-repo harnesses
- Treating derived views (meshes, scheduling) as authoritative or durable truth
- Exposing mutable internal handles through diagnostics
- Fixing portable performance SLAs as correctness requirements

## Unresolved human questions

None that change product identity, purpose, or boundary. The clean seeds and root README align: Moria is the substrate; walkable demos and games are external or future consumers. Downstream design may still need crate-split detail, meshing strategy, and voxel scale—those are technical choices, not vision gaps.

## Seed contributions

| Source | Role in this vision |
|---|---|
| `README.md` | Names the product (Moria) and points current scope at the clean boundary and substrate-requirements seeds, without a downstream product vision. |
| `docs/seeds/clean-project-boundary.md` | Defines identity: reusable Rust substrate for external consumers; crates + public-interface validation harness; harness is not a game; lists excluded game/content concerns; requires completeness for generate/stream/query/mutate/mesh/save/restore. |
| `docs/seeds/clean-substrate-requirements.md` | Supplies product-level capability outcomes: identity & deterministic generation, sparse storage & mutation API, streaming & derived meshing, registered-object queries, versioned persistence, headless/visual fixtures, diagnostics, and performance-as-evidence. |
