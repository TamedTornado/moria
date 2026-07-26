# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate: library crates that external consumers use to generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces only. This repository also delivers a minimal validation harness that exercises those same public interfaces; the harness is an adjacent delivery, not a game or product layer.

## Purpose

Independent games and tools need a shared, trustworthy voxel-world foundation without owning world generation, sparse material truth, streaming residency, derived meshing, or persistence themselves. Moria exists so those consumers can build on a deterministic, publicly integrated substrate rather than reimplementing world authority.

## Product boundary

**Belongs to Moria**

- Substrate crates that own world identity, generation, sparse material storage, mutation admission, streaming residency, derived meshing, persistence of authoritative state, deterministic world queries over registered objects, and public diagnostics.
- A minimal public-interface validation harness (headless and minimal visual) that proves the substrate without privileged access.

**Does not belong to Moria**

- Game rules, characters, player or AI controllers, animation, authored routes, production assets, and any consumer-specific content or presentation.
- Consumer UX, gameplay policy, and game-entity models built on top of the substrate.

The harness may use a free-fly camera and diagnostic overlays solely to exercise the public API available to any other repository. It is not a game layer and does not import consumer-owned features into the substrate.

## Required product outcomes

- **World identity and generation.** A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
- **Authoritative material and mutation.** Sparse storage preserves material truth. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure.
- **Streaming and derived views.** Streaming bounds resident work and rejects stale background results. Meshing is a regenerable derived view of voxel truth, not a second authority.
- **Queries without game entities.** Registered objects can participate in deterministic world queries without becoming game entities.
- **Persistence of truth.** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
- **Public validation and evidence.** Headless validation exercises generation, streaming, mutation, queries, and persistence; a minimal visual fixture exercises meshing through the public interface. Diagnostics expose lifecycle, revision, and bounded-work observations without mutable internal handles. Performance measurements include machine identity and are evidence, not portable correctness thresholds.

## Future products and enabling implications

No named future game or tool is in the supplied seeds. Downstream consumers are any external repositories that integrate the public Rust substrate. Enabling implication only: the substrate must remain complete enough that such consumers can own gameplay, content, and presentation without needing privileged access to Moria internals. No consumer feature set is committed here.

## Non-goals

- Shipping a game, character, controller scheme, animation system, authored content, or production art pipeline.
- Treating the validation harness as a playable product or design surface for consumer UX.
- Portable performance numbers as pass/fail correctness gates.
- Granting adjacent consumers privileged or internal-only access beyond the public interface.

## Confirmed vision constraints

- Delivery form is Rust substrate crates plus a minimal public-interface validation harness in this repository.
- Integration boundary is public interfaces only; the harness uses the same surface external consumers get.
- Worlds are deterministic given versioned parameters and seed; mutations and queries have explicit public lifecycle semantics.
- Authoritative truth is sparse material state and versioned deltas; meshes and transient scheduling are derived or non-persistent.
- Performance results are machine-attributed evidence, not portable thresholds.

## Deferred design decisions

- Concrete public API shapes, command vocabulary, and crate packaging.
- Generation, storage, streaming, meshing, and persistence algorithms and data layouts.
- Exact harness packaging, platforms, workloads, and diagnostic presentation (beyond free-fly/minimal visual and headless coverage already required at outcome level).
- How deep each capability is in the first deliverable slice and how milestones are ordered.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names Moria as a reusable voxel-world substrate and points current scope solely to the two clean seeds, excluding downstream product vision from this synthesis.
- `docs/seeds/clean-project-boundary.md` — Fixes product identity as a Rust substrate for external consumers, requires repository delivery of crates plus a minimal public validation harness, and excludes game/content layers from current scope while mandating generate/stream/query/mutate/mesh/save/restore through public interfaces.
- `docs/seeds/clean-substrate-requirements.md` — Supplies the binding outcome substance for identity, generation, storage, mutation, streaming, meshing, queries, persistence, public validation fixtures, diagnostics, and non-portable performance evidence.
