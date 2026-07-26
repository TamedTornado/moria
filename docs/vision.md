# Moria — Product Vision

## Current product

**Moria** is a reusable Rust voxel-world substrate delivered as crates plus a
minimal public-interface validation harness. External consumers build on it;
this repository does not ship a game.

## Purpose

Provide an authoritative, deterministic voxel world that consumers can generate,
stream, query, mutate, mesh, save, and restore through stable public interfaces.
The harness proves those interfaces work (including a free-fly camera and
diagnostic overlays) without becoming a game layer.

## Boundary

| In scope | Out of scope |
| --- | --- |
| Substrate crates and public APIs | Game rules, characters, controllers, animation |
| Authoritative sparse voxel truth | Authored routes, production assets, consumer content |
| Deterministic generation and bounded mutations | Game entities (registered objects support queries only) |
| Streaming, meshing as derived views, persistence of truth | Treating derived meshes or scheduling state as truth |
| Headless and minimal visual validation fixtures | A playable product or consumer-specific experience |

The harness may exercise the same API another repository would use. It is not a
game layer.

## Required product-level outcomes

1. **World identity** — A versioned parameter set and seed define identity;
   generation is deterministic and can materialize bounded regions without
   eagerly allocating the full world.
2. **Authoritative storage and mutation** — Sparse storage holds material truth;
   consumers submit bounded mutations via a public command API with explicit
   admission, commit, and failure states.
3. **Streaming and derived views** — Streaming bounds resident work and rejects
   stale background results; meshing is regenerable from voxel truth; registered
   objects can participate in deterministic queries without becoming game entities.
4. **Persistence** — Versioned authoritative deltas restore identical query
   behavior; derived meshes and transient scheduling state are not saved as truth.
5. **Public validation and diagnostics** — A headless fixture exercises
   generation, streaming, mutation, queries, and persistence; a minimal visual
   fixture with a free-fly camera exercises meshing through the public interface;
   diagnostics expose lifecycle, revision, and bounded-work observations without
   mutable internal handles.
6. **Performance evidence** — Measurements include machine identity and are
   evidence, not portable correctness thresholds.

## Non-goals

- Shipping gameplay, characters, animation, controllers, or production content
- Embedding consumer-specific design into the substrate
- Treating the validation harness as a product surface beyond interface proof
- Persisting derived or transient state as authoritative world truth
- Encoding performance numbers as portable pass/fail criteria

## Unresolved questions

None that change product identity, purpose, or boundary. The seeds agree on
current scope.

## Seed contributions

| Source | Contribution |
| --- | --- |
| `README.md` | Names the product (Moria), states it is a reusable voxel-world substrate, and points at the two clean seeds as the definition of current scope without downstream product vision. |
| `docs/seeds/clean-project-boundary.md` | Establishes identity (Rust substrate for external consumers), delivery shape (crates + minimal harness), in/out boundary (no game layer), and the capability completeness bar (generate, stream, query, mutate, mesh, save, restore via public interfaces). |
| `docs/seeds/clean-substrate-requirements.md` | Supplies product-level outcomes: identity/generation, storage/mutation, streaming/derived views, persistence, public validation/diagnostics, and how performance evidence is framed. |

Other files under `docs/seeds/` (e.g. product or reference material) were not
part of the vision seed manifest and are not imported into current product
scope.
