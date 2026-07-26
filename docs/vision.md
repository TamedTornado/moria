# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. It provides public interfaces through which consumers generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world. This repository also delivers a minimal public-interface validation harness that exercises those same interfaces; the harness is not the product and is not a game layer.

## Purpose

Moria exists so independent products can rely on one shared, authoritative voxel world—identity, generation, mutation, streaming, derived views, queries, and persistence—without embedding game rules, content, or presentation in the substrate.

## Product boundary

**In product scope**

- Public substrate capabilities for an authoritative voxel world: generation, streaming, query, mutation, meshing, save, and restore.
- World identity, deterministic materialization of bounded regions, authoritative material truth, explicit mutation outcomes, regenerable derived meshes, object registration for world queries, and persistence that restores identical query behavior.
- Diagnostics that expose lifecycle, revision, and bounded-work observations without granting mutable internal handles.
- A headless validation fixture and a minimal visual validation fixture that exercise the public interface (including meshing in the visual case).

**Outside product scope (adjacent or consumer-owned)**

- Game rules, characters, controllers, animation, authored routes, production assets, and any consumer-specific content or policy.
- Harness-only presentation choices (for example free-fly viewing or diagnostic overlays) used to exercise the public API; they do not become game or product features.
- Downstream games and other repositories that consume Moria; their workloads, UX, platforms, and acceptance scenarios are not Moria’s product surface.

The repository delivers the substrate and the minimal validation harness. Product identity remains the substrate; the harness is an adjacent delivery that proves the public contract.

## Required product outcomes

- **Public world lifecycle.** External consumers can generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world only through public interfaces.
- **Stable world identity and deterministic generation.** A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
- **Authoritative truth and bounded mutation.** Storage preserves authoritative material truth. Consumers submit bounded mutations through a public command API whose admission, commit, and failure states are explicit.
- **Bounded streaming and regenerable derived views.** Streaming keeps resident work bounded and rejects stale background results. Meshing is a derived view of voxel truth and can be regenerated from that truth.
- **Queries without game entities.** Registered objects may participate in deterministic world queries without becoming game entities.
- **Persistence of truth, not derived noise.** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
- **Public validation and safe diagnostics.** A headless fixture exercises generation, streaming, mutation, queries, and persistence. A minimal visual fixture exercises meshing through the public interface. Diagnostics expose lifecycle, revision, and bounded-work observations without mutable internal handles.

## Future products and enabling implications

No named future game or consumer product is in the current seed set. Intended users are external consumers in other repositories that integrate the Rust substrate through the same public interfaces the validation harness uses. Completeness of those interfaces is the enabling implication: consumers can build their own gameplay, content, and presentation on Moria without privileged access to substrate internals.

## Non-goals

- Building a game, game layer, or default gameplay loop inside Moria.
- Owning characters, player controllers, animation systems, authored routes, or production game assets.
- Treating harness viewing aids or overlays as product features.
- Portable performance pass/fail thresholds; measurements are evidence tied to machine identity, not correctness contracts.

## Confirmed vision constraints

- Delivery ecosystem: Rust substrate usable by external consumers.
- All consumer-facing world operations go through public interfaces; validation uses those same interfaces.
- Generation is deterministic for a given versioned parameter set and seed.
- Mutation outcomes expose explicit admission, commit, and failure states.
- Persistence restores identical query behavior from versioned authoritative deltas only.
- Diagnostics do not expose mutable internal handles.
- Performance numbers include machine identity and are evidence, not portable correctness thresholds.

## Deferred design decisions

- Concrete public API shape, packaging of substrate units, and internal data layouts.
- How streaming, meshing, and background work are scheduled and invalidated in detail.
- Persistence encoding and delta representation.
- Exact design of headless and visual fixtures beyond the obligation to exercise the public contract.
- Which environments, machines, or workloads are used when collecting performance evidence.

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds describe one coherent current product: a Rust voxel-world substrate with a repository-delivered public-interface validation harness, and a clear exclusion of game and consumer content layers.

## Seed synthesis

- `README.md` — Names the product Moria and points current scope at the clean boundary and substrate requirement seeds.
- `docs/seeds/clean-project-boundary.md` — Fixes identity as a reusable Rust voxel-world substrate for external consumers; requires public generate/stream/query/mutate/mesh/save/restore; places the minimal validation harness in repository delivery while excluding game content and controls.
- `docs/seeds/clean-substrate-requirements.md` — Supplies binding substrate outcomes for identity, deterministic bounded generation, authoritative mutation lifecycle, streaming, derived meshing, query registration, persistence of truth, public fixtures, diagnostics, and non-portable performance evidence.
