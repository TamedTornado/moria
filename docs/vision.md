# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

## Current product

**Moria** is a reusable Rust voxel-world substrate: a family of crates (and a minimal validation harness) that generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces.

The repository ships substrate crates plus harnesses that exercise only those public APIs. It does not ship a game.

## Purpose

Give external consumers a complete, stable substrate for material voxel worlds—deterministic identity and generation, sparse authoritative storage, bounded mutation, streaming of resident work, derived meshing, object-aware world queries, and versioned persistence—so games and tools can be built elsewhere without forking or embedding privileged engine paths.

## Boundary

| In scope | Out of scope |
|---|---|
| Substrate crates and their public command/query surfaces | Game rules, characters, controllers, animation |
| Deterministic generation from versioned parameters + seed | Authored content, production assets, consumer-specific worlds |
| Sparse storage of authoritative material truth | Combat, AI, building UI, spells, gas, LLM/System layers |
| Streaming, meshing as a regenerable derived view | Privileged harness-only mutation or query paths |
| Persistence of versioned authoritative deltas | Treating meshes or scheduling state as truth |
| Headless and minimal visual fixtures (e.g. free-fly camera, diagnostics) as public-API validation only | A walkable demo, third-person character, or product-shaped game |

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

- Implementing any game, demo character, traversal fantasy, or content pack in this repository
- Importing Product One / walkable-world gameplay, seed-world composition, material palettes, or art direction as current deliverables
- Embedding System, LLM, spell, gas, combat, AI, or building layers (compatibility seams only where substrate needs demand them)
- Shipping privileged APIs usable only by in-repo harnesses
- Treating derived views (meshes, scheduling) as authoritative or durable truth
- Fixing portable performance SLAs as correctness requirements

## Unresolved human questions

None that change product identity, purpose, or boundary. The clean seeds and root README align: Moria is the substrate; walkable demos and games are external or future consumers. Downstream design may still need crate-split detail, meshing strategy, and voxel scale—those are technical choices, not vision gaps.

## Seed contributions

| Source | Role in this vision |
|---|---|
| `README.md` | Names the product (Moria) and points current scope at the clean boundary and substrate-requirements seeds, without a downstream product vision. |
| `docs/seeds/clean-project-boundary.md` | Defines identity: reusable Rust substrate for external consumers; crates + public-interface validation harness; harness is not a game; lists excluded game/content concerns; requires completeness for generate/stream/query/mutate/mesh/save/restore. |
| `docs/seeds/clean-substrate-requirements.md` | Supplies product-level capability outcomes: identity & deterministic generation, sparse storage & mutation API, streaming & derived meshing, registered-object queries, versioned persistence, headless/visual fixtures, diagnostics, and performance-as-evidence. |

**Reference only (not current scope):** Older seeds under `docs/seeds/` (`product-one-seed.md`, `voxel-world-substrate.md`, `project-boundary.md`, seeds README) describe a walkable-world milestone, character controller, curated demo region, geology pipeline, and game-adjacent architecture. They inform *why* the substrate must support material mutability, deep continuous space, streaming sparsity, and clean consumer boundaries—but their gameplay, content, characters, assets, and implementation detail are not imported into current product scope. The older `project-boundary.md` agrees that the substrate is the product and any walkable executable is only a validation harness; the clean seeds supersede it for current wording.
