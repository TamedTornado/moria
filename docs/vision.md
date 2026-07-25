# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. It exposes public interfaces that generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world. This repository ships the substrate crates and a minimal validation harness that exercises those same public interfaces—not a game or consumer product.

## Purpose

Give other repositories a stable, reusable foundation for mutable voxel worlds so they can own gameplay, presentation, and content without reimplementing world identity, authority, or persistence. The substrate makes deterministic, queryable world truth available through a public contract suitable for independent consumers and automated validation.

## Product boundary

**In scope:** the substrate’s public contract for world identity and generation, resident streaming, authoritative mutation, derived meshing, world queries, and persistence of authoritative state; plus a minimal harness that only validates that contract through the public API (headless and minimal visual exercise).

**Out of scope (adjacent / consumer-owned):** game rules, characters, player or NPC controllers, animation, authored routes, production assets, consumer-specific content, and any game layer built on the substrate. Free-fly camera, diagnostic overlays, and similar harness presentation are harness-local aids, not product features or a shipped game UX.

**Repository role:** deliver substrate crates and the validation harness. The product identity is the substrate; the harness is an adjacent consumer of the product under test, required to exist and to use public interfaces only.

## Future products and enabling implications

No first-party game or named downstream title is in current scope. Future consumers are external products that will depend on Moria’s public world APIs.

**Enabling implications (not current roadmap commitments):**
- External games and tools can build mutable voxel worlds without owning world generation, authority, or save/restore.
- Independent repositories can validate integration against the same public interfaces the harness uses.
- Long-horizon consumer features (gameplay, characters, presentation, controllers) remain fully outside Moria.

## Non-goals

- Shipping a playable game, character, controller stack, animation, or authored content inside this repository.
- Treating performance numbers as portable pass/fail correctness thresholds.
- Exposing mutable internal handles as the integration surface; consumers and the harness use public interfaces and diagnostics only.

## Confirmed vision constraints

- World identity is defined by a versioned parameter set and seed; generation is deterministic and can materialize bounded regions without eagerly allocating the entire world.
- Authoritative material truth is distinct from derived views (for example meshing), which can be regenerated; persistence restores authoritative truth and identical query behavior, not derived meshes or transient scheduling state.
- Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure; streaming bounds resident work and rejects stale background results; diagnostics report lifecycle, revision, and bounded-work observations without mutable internal handles.

## Assumptions proposed for approval

None. The supplied seeds agree on current product identity, purpose, and boundary without needing a proposed fill-in.

## Questions for human review

None. The clean boundary and substrate seeds already fix product identity (reusable Rust voxel-world substrate), repository delivery (crates plus minimal public-interface validation harness), and consumer ownership (gameplay, characters, controllers, content, presentation). Capability depth, delivery sequence, architecture, platforms, and acceptance workloads remain downstream design.

## Seed synthesis

- **README.md** — Names the product Moria and points current scope to the clean boundary and substrate seeds, without embedding a separate product story.
- **docs/seeds/clean-project-boundary.md** — Establishes current identity as a reusable Rust voxel-world substrate for external consumers; repository delivers substrate crates and a minimal public-interface validation harness; harness may use free-fly and diagnostics but is not a game layer; game rules, characters, controllers, animation, routes, production assets, and consumer content are out of scope; product must be complete enough to generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces.
- **docs/seeds/clean-substrate-requirements.md** — Contributes vision-altitude responsibilities: deterministic identity/generation from versioned parameters and seed; authoritative storage and explicit mutation contract; streaming and meshing as derived views; registered objects in queries without becoming game entities; versioned authoritative persistence; headless and minimal visual public-interface validation; diagnostics without mutable internals; performance as machine-scoped evidence, not portable correctness gates. Mechanism inventories and thresholds stay downstream.
