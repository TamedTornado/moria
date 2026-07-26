# Moria vision

## Current product

**Moria** is a reusable Rust voxel-world substrate: crates plus a minimal
public-interface validation harness that external consumers can depend on.

It is not a game, game layer, or content product. The harness may use a free-fly
camera and diagnostic overlays only to exercise the same public API another
repository would call.

## Purpose

Deliver an authoritative, deterministic voxel world that external products can
generate, stream, query, mutate, mesh, save, and restore through stable public
interfaces—without embedding consumer-specific gameplay, content, or assets in
this repository.

## Boundary

| In scope | Out of scope |
| --- | --- |
| Substrate crates and public APIs | Game rules, characters, controllers, animation |
| Versioned world identity (parameters + seed) | Authored routes, production assets, consumer content |
| Deterministic generation of bounded regions | Full-world eager allocation as the default model |
| Sparse authoritative voxel storage | Treating derived meshes or scheduling state as truth |
| Bounded mutation command API with explicit admit/commit/fail | Mutable internal handles exposed as the consumer surface |
| Streaming that bounds resident work and rejects stale work | Performance numbers as portable correctness gates |
| Regenerable meshing as a derived view | Game entities; registered objects are query participants only |
| Versioned delta persistence that restores query behavior | Broader game, combat, AI, LLM, spell, gas, or building systems |
| Headless and minimal visual validation fixtures | Shipping a playable product from this repo |

## Required product-level outcomes

1. **World identity** — A versioned parameter set and seed define a world;
   generation is deterministic and can materialize bounded regions without
   loading the entire world.
2. **Authoritative storage and mutation** — Sparse storage holds material truth;
   consumers submit bounded mutations through a public command API with explicit
   admission, commit, and failure states.
3. **Streaming and derived views** — Streaming bounds resident work and rejects
   stale background results; meshing is derived from voxel truth and can be
   regenerated; registered objects may participate in deterministic queries
   without becoming game entities.
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

- Building or shipping a game, character stack, or production content pipeline
  in this repository
- Encoding consumer-specific gameplay, routes, or assets into the substrate
- Treating meshes, scheduling state, or harness UI as authoritative world truth
- Using harness-only performance numbers as cross-machine correctness criteria
- Expanding scope into adjacent product domains (combat, AI, LLM, systems beyond
  the voxel substrate)

## Unresolved questions for humans

None that change product identity, purpose, or boundary. The seed set names
Moria as the current product and consistently frames downstream games and
examples as external consumers, not in-repo scope.

If later work reintroduces older seeds (`product-one-seed`,
`voxel-world-substrate`, non-clean boundary notes) as binding, re-confirm
whether any walkable-world or consumer-game validation remains a substrate
milestone versus pure reference.

## Seed contributions

| Seed | Contribution |
| --- | --- |
| `README.md` | Names the product **Moria**, states it is a reusable voxel-world substrate, and points current scope at the two clean seed docs (not a separate product vision). |
| `docs/seeds/clean-project-boundary.md` | Fixes identity and boundary: Rust substrate crates + minimal public-API harness; harness is not a game layer; complete enough for generate/stream/query/mutate/mesh/save/restore. |
| `docs/seeds/clean-substrate-requirements.md` | Supplies product-level outcomes for identity/generation, storage/mutation, streaming/derived views, persistence, validation/diagnostics, and performance-as-evidence. |

Other files under `docs/seeds/` (for example product-one and architecture
reference seeds) were not part of this manifest’s binding set. They are treated
as historical or reference context only; their gameplay, content, and
implementation detail are not imported into current scope.
