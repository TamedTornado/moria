# Project vision

## What we are building now

**Moria** is a reusable **Rust voxel-world substrate**: library crates that give external consumers an authoritative, deterministic voxel world they can generate, stream, query, mutate, mesh, save, and restore through public interfaces only.

This repository also ships a **minimal public-interface validation harness** (headless and minimal visual) that exercises those same interfaces. The harness is an adjacent delivery for validation, not a game and not part of the product identity.

## Purpose

External Rust consumers need a stable world foundation they can integrate without forking engine internals or inventing their own voxel authority. Moria exists so those consumers can rely on one public surface for world identity, sparse material truth, bounded work, derived meshing, and durable restore—while owning gameplay, presentation, and content themselves.

## Product boundary

**In product (substrate):**
- Public APIs for world identity, generation, streaming, mutation, queries, meshing, and persistence
- Authoritative sparse material truth and regenerable derived views
- Diagnostics that observe lifecycle, revision, and bounded-work state without exposing mutable internal handles
- Completeness to generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world via public interfaces

**In repository, outside product identity:**
- Minimal validation harnesses that use only the public API (headless coverage of core operations; minimal visual coverage of meshing). The harness may use a free-fly camera and diagnostic overlays; those are harness affordances, not product features.

**Out of current product (consumer-owned):**
- Game rules, characters, player/NPC controllers, animation, authored routes, production assets, and consumer-specific content or policy

Adjacent consumers have no privileged access: anything the harness can do must be possible through the same public interfaces available to another repository.

## Required product outcomes

1. **Stable world identity** — A versioned parameter set and seed define each world; generation is deterministic and can materialize bounded regions without eagerly allocating the entire world.
2. **Authoritative material truth** — Sparse storage holds material reality; consumers apply bounded mutations through a public command API with explicit admission, commit, and failure outcomes.
3. **Bounded streaming and fresh work** — Streaming limits resident work and rejects stale background results so consumers can progress without unbounded memory or obsolete commits.
4. **Regenerable derived meshing** — Meshing is a derived view of voxel truth and can be rebuilt; it is not authoritative truth.
5. **Deterministic world queries** — Registered objects can participate in deterministic queries without becoming game entities.
6. **Durable restore of authority** — Persistence records versioned authoritative deltas and restores identical query behavior; derived meshes and transient scheduling state are not saved as truth.
7. **Public-only validation** — Headless and minimal visual fixtures prove generation, streaming, mutation, queries, persistence, and meshing through the public surface; diagnostics support observation without internal mutation handles.
8. **Evidence-not-gates performance** — Performance measurements include machine identity and serve as evidence, not portable pass/fail correctness thresholds.

## Future products and enabling implications

No future consumer product is specified in the supplied seeds. Downstream games or tools are expected external consumers of this substrate. Enabling implication only: a complete public world surface (generate through restore, plus meshing and diagnostics) so those consumers need not reimplement world authority. Their gameplay, UX, content, and presentation remain theirs.

## Non-goals

- Building a game, character stack, controller layer, animation system, or authored content pipeline
- Treating the validation harness as a playable product or privileged co-engine
- Shipping production assets or consumer-specific presentation as substrate deliverables
- Encoding portable performance SLAs as product correctness gates
- Expanding current scope into gameplay policy, entity frameworks, or engine modules beyond the world substrate outcomes above

## Confirmed vision constraints

- **Ecosystem:** Rust library crates for external consumers; integration is via public interfaces.
- **Authority model:** Sparse material truth is authoritative; meshes and scheduling state are derived or transient.
- **Determinism:** Generation and query-relevant restore behavior are deterministic for a given identity (parameters + seed) and recorded deltas.
- **API fairness:** Validation and external consumers share the same public surface; no privileged internal handles for harnesses.
- **Harness delivery:** The repository must provide headless and minimal visual public-interface validation fixtures; they do not redefine product identity.
- **Performance stance:** Measurements are machine-identified evidence, not portable correctness thresholds.

## Deferred design decisions

- Public API shape, crate layout, and package boundaries
- Concrete generation, storage, streaming, meshing, and persistence mechanisms
- Exact mutation command vocabulary, failure taxonomy, and diagnostic field set
- How far the minimal visual fixture goes beyond “exercise meshing on the public API”
- Benchmark workloads and any non-binding performance targets used as engineering evidence

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds agree on product identity (Rust voxel-world substrate), repository delivery of a public-interface validation harness outside game scope, outcome families (generate through restore plus meshing, queries, diagnostics), and consumer-owned exclusions.

## Seed synthesis

- **`README.md`** — Names Moria as a reusable voxel-world substrate and points current scope to the two clean boundary/requirements seeds.
- **`docs/seeds/clean-project-boundary.md`** — Fixes current identity as a Rust substrate for external consumers, requires public completeness (generate–restore including mesh), ships a non-game public-interface harness, and excludes game/content layers.
- **`docs/seeds/clean-substrate-requirements.md`** — Supplies binding outcome substance: identity and deterministic bounded generation, sparse truth and mutation admission, streaming freshness, derived meshing, non-entity query objects, delta persistence, public headless/visual validation, diagnostics without internal mutation, and evidence-only performance.
