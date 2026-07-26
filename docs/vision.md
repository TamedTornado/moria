# Moria — Product Vision

## Current product

**Moria** is a reusable **Rust voxel-world substrate**: a library (crates plus a
minimal public-interface validation harness) that external consumers can depend
on to host an authoritative voxel world. It is infrastructure for games and
tools, not a game itself.

## Purpose

Give external repositories a stable, public way to **generate, stream, query,
mutate, mesh, save, and restore** a deterministic voxel world without embedding
game rules, content, or consumer-specific systems in this repo.

The substrate exists so other products can treat world identity, sparse
material truth, derived meshes, and persistence as shared plumbing rather than
reimplementing them per title.

## Boundary

| In scope | Out of scope |
| -------- | ------------ |
| Substrate crates and public APIs | Game rules, characters, controllers, animation |
| Deterministic world identity (versioned parameters + seed) | Authored routes, production assets, consumer content |
| Sparse authoritative voxel storage and bounded mutation commands | Game entities or entity-framework semantics for registered objects |
| Streaming of resident work; meshing as a regenerable derived view | Shipping a playable game or product vertical |
| Persistence of versioned authoritative deltas (not derived meshes or scheduling state) | Portable performance thresholds as correctness gates |
| Headless + minimal visual fixtures that exercise only the public interface | Mutable internal handles exposed as the consumer contract |
| Free-fly camera and diagnostic overlays in the harness for validation | Treating the harness as a game layer |

The harness must validate the same public surface another repository would use.
It may observe lifecycle, revision, and bounded-work diagnostics; it must not
become a second product.

## Required product-level outcomes

When the current product is done, an external consumer (or the validation
harness standing in for one) can:

1. **Identify a world** from a versioned parameter set and seed, and regenerate
   the same material truth deterministically.
2. **Materialize bounded regions** without eagerly allocating the entire world.
3. **Hold sparse authoritative material truth** and submit **bounded mutations**
   through a public command API with explicit admission, commit, and failure.
4. **Stream** work under residence bounds and reject stale background results.
5. **Treat meshing as a derived view** of voxel truth that can be regenerated,
   not as saved truth.
6. **Register objects** that participate in deterministic world queries without
   becoming game entities.
7. **Persist and restore** versioned authoritative deltas so restored query
   behavior matches the pre-save world (derived meshes and transient scheduling
   state are not part of that truth).
8. **Validate the public contract** via a headless fixture (generation,
   streaming, mutation, queries, persistence) and a minimal visual fixture
   (meshing through the public interface with free-fly camera).
9. **Observe** lifecycle, revision, and bounded-work diagnostics without relying
   on mutable internal handles; performance numbers include machine identity and
   remain evidence, not portable pass/fail thresholds.

## Non-goals

- Building or shipping a game, genre vertical, or narrative product in this repo.
- Owning characters, controllers, animation, authored content, or production art.
- Encoding consumer game rules or entity systems into the substrate.
- Persisting derived meshes or scheduler ephemera as authoritative world state.
- Defining cross-machine performance SLAs as correctness criteria.
- Exposing internal mutability as the supported integration surface.

## Unresolved questions for humans

None that change product identity, purpose, or boundary. The clean seeds agree
that the **current** product is the substrate and harness only; any future
games, sample titles, or reference implementations are context for capability
needs, not current deliverables.

If product owners later want a first-party sample game, a different primary
consumer, or a narrower/wider public surface, that decision should amend this
vision explicitly.

## Seed contributions

| Seed | Contribution |
| ---- | ------------ |
| `README.md` | Names the product **Moria** as a reusable voxel-world substrate and points current scope solely at the two clean seed docs (no separate product vision elsewhere). |
| `docs/seeds/clean-project-boundary.md` | States current product (Rust substrate crates + minimal public-interface validation harness), hard non-goals (game layer, content, controllers, assets), and the capability bar: generate, stream, query, mutate, mesh, save, restore through public interfaces. |
| `docs/seeds/clean-substrate-requirements.md` | Product-level outcomes for identity/generation, sparse storage and mutation admission, streaming and derived meshing, registered-object queries, versioned delta persistence, headless/visual public fixtures, and diagnostics/performance-as-evidence. |

Other files under `docs/seeds/` (if present) are not part of the stated current
manifest and were not used to define this vision.
