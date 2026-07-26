# Moria vision

## Current product

**Moria** is a reusable Rust voxel-world substrate for external consumers. This
repository delivers substrate crates and a minimal public-interface validation
harness.

The harness exercises the same public API available to another repository; it
may use a free-fly camera and diagnostic overlays for that purpose. It is not a
game layer. External consumers depend on the substrate and its public
interfaces, not on the harness as a product surface.

## Purpose

Deliver an authoritative voxel world that external products can generate,
stream, query, mutate, mesh, save, and restore through public interfaces—without
embedding consumer-specific gameplay, content, or assets in this repository.

## Boundary

| In scope | Out of scope |
| --- | --- |
| Substrate crates and public APIs | Game rules, characters, controllers, animation |
| Versioned world identity (parameters + seed) | Authored routes, production assets, consumer-specific content |
| Deterministic generation of bounded regions without whole-world eager allocation | Treating derived meshes or scheduling state as truth |
| Sparse authoritative voxel storage | Mutable internal handles in diagnostics |
| Bounded mutation command API with explicit admit/commit/fail | Performance numbers as portable correctness gates |
| Streaming that bounds resident work and rejects stale work | Shipping a playable product from this repo |
| Regenerable meshing as a derived view of voxel truth | |
| Registered objects that can participate in deterministic world queries without becoming game entities | |
| Versioned delta persistence that restores identical query behavior | |
| Headless and minimal visual validation fixtures | |

## Required product-level outcomes

1. **World identity** — A versioned parameter set and seed define a world;
   generation is deterministic and can materialize bounded regions without
   eagerly allocating the complete world.
2. **Authoritative storage and mutation** — Sparse storage holds material truth;
   consumers submit bounded mutations through a public command API with explicit
   admission, commit, and failure states.
3. **Streaming and derived views** — Streaming bounds resident work and rejects
   stale background results; meshing is derived from voxel truth and can be
   regenerated; registered objects can participate in deterministic world
   queries without becoming game entities.
4. **Persistence** — Versioned authoritative deltas restore identical query
   behavior; derived meshes and transient scheduling state are not saved as
   truth.
5. **Public validation** — A headless fixture covers generation, streaming,
   mutation, queries, and persistence; a minimal visual fixture with free-fly
   camera exercises meshing via the public interface; diagnostics expose
   lifecycle, revision, and bounded-work observations without mutable internal
   handles.
6. **Performance evidence** — Measurements include machine identity and serve as
   evidence, not portable pass/fail thresholds.

## Non-goals

- Building or shipping a game or game layer in this repository
- Encoding game rules, characters, controllers, animation, authored routes,
  production assets, or consumer-specific content into the substrate
- Treating meshes, scheduling state, or harness UI as authoritative world truth
- Using harness-only performance numbers as cross-machine correctness criteria
- Exposing mutable internal handles through diagnostics

## Unresolved questions for humans

None that change product identity, purpose, or boundary. The supplied seeds
name Moria as the current product and consistently place consumer gameplay and
content outside this repository.

## Seed contributions

| Seed | Contribution |
| --- | --- |
| `README.md` | Names the product **Moria**, states it is a reusable voxel-world substrate, and points current scope at the two clean seed docs (not a separate product vision). |
| `docs/seeds/clean-project-boundary.md` | Fixes identity and boundary: reusable substrate for external consumers; crates plus minimal public-API validation harness; harness is not a game layer; complete enough for generate/stream/query/mutate/mesh/save/restore through public interfaces. |
| `docs/seeds/clean-substrate-requirements.md` | Supplies product-level outcomes for identity/generation (including generation determinism and bounded materialization), storage/mutation, streaming/derived views (including query determinism for registered objects), persistence, validation/diagnostics, and performance-as-evidence. |

Other files under `docs/seeds/` (for example product-one and architecture
reference seeds) were not part of this manifest’s binding set. They are treated
as historical or reference context only; their gameplay, content, and
implementation detail are not imported into current scope.
