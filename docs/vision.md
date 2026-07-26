# Moria — Product Vision

## Current product

**Moria** is a reusable Rust voxel-world substrate delivered as crates plus a
minimal public-interface validation harness. External consumers build on it;
this repository does not ship a game.

## Purpose

Provide an authoritative voxel world that consumers can generate, stream,
query, mutate, mesh, save, and restore through public interfaces. The harness
exercises that API (it may use a free-fly camera and diagnostic overlays) without
becoming a game layer.

## Boundary

| In scope | Out of scope |
| --- | --- |
| Substrate crates and public APIs | Game rules, characters, controllers, animation |
| Authoritative sparse voxel truth | Authored routes, production assets, consumer content |
| Deterministic generation; registered objects in deterministic world queries without becoming game entities | Game entities |
| Streaming, meshing as derived views, persistence of truth | Treating derived meshes or scheduling state as truth |
| Headless and minimal visual validation fixtures that exercise capabilities | A playable product or consumer-specific experience |

The harness exercises the same API another repository would use. It is not a
game layer. Diagnostic overlays are optional presentation for the harness;
diagnostics themselves are required product outcomes.

## Required product-level outcomes

1. **World identity** — A versioned parameter set and seed define identity;
   generation is deterministic and can materialize bounded regions without
   eagerly allocating the full world.
2. **Authoritative storage and mutation** — Sparse storage holds material truth;
   consumers submit bounded mutations via a public command API with explicit
   admission, commit, and failure states.
3. **Streaming and derived views** — Streaming bounds resident work and rejects
   stale background results; meshing is regenerable from voxel truth; registered
   objects can participate in deterministic world queries without becoming game
   entities (queries are a supported role, not an exclusive one).
4. **Persistence** — Versioned authoritative deltas restore identical query
   behavior; derived meshes and transient scheduling state are not saved as truth.
5. **Public validation and diagnostics** — A headless fixture exercises
   generation, streaming, mutation, queries, and persistence; a minimal visual
   fixture with a free-fly camera exercises meshing through the public interface;
   diagnostics expose lifecycle, revision, and bounded-work observations without
   mutable internal handles. Presentation of diagnostics (e.g. overlays) is not
   mandated.
6. **Performance evidence** — Measurements include machine identity and are
   evidence, not portable correctness thresholds.

## Non-goals

- Shipping gameplay, characters, animation, controllers, or production content
- Embedding consumer-specific design into the substrate
- Treating the validation harness as a product surface beyond exercising the API
- Claiming global determinism beyond generation and registered-object world queries
- Promising public-interface stability as a product commitment
- Establishing a proof-of-correctness standard for the harness or fixtures
- Persisting derived or transient state as authoritative world truth
- Encoding performance numbers as portable pass/fail criteria

## Unresolved questions

None that change product identity, purpose, or boundary. The seeds agree on
current scope.

## Seed contributions

| Source | Contribution |
| --- | --- |
| `README.md` | Names the product (Moria), states it is a reusable voxel-world substrate, and points at the two clean seeds as the definition of current scope without downstream product vision. |
| `docs/seeds/clean-project-boundary.md` | Establishes identity (Rust substrate for external consumers), delivery shape (crates + minimal validation harness), in/out boundary (no game layer), optional harness presentation (free-fly camera, diagnostic overlays), and the capability completeness bar (generate, stream, query, mutate, mesh, save, restore via public interfaces—without a stability promise). |
| `docs/seeds/clean-substrate-requirements.md` | Supplies product-level outcomes: identity/generation (determinism scoped to generation), storage/mutation, streaming/derived views (including registered objects in deterministic queries without game-entity status), persistence, public validation/diagnostics (fixtures exercise capabilities; diagnostics required, not a specific overlay form), and how performance evidence is framed. |

Other files under `docs/seeds/` (e.g. product or reference material) were not
part of the vision seed manifest and are not imported into current product
scope.
