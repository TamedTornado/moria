# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. This repository delivers the substrate as consumable crates plus a minimal validation harness that exercises the same public interfaces available to another repository. The harness is an adjacent validation artifact, not a game or product surface.

## Purpose

Moria exists so independent consumers can build against an authoritative voxel world they do not own: generate, stream, query, mutate, mesh, save, and restore that world through public interfaces, without privileged access to internals.

## Product boundary

**Belongs to Moria**

- Reusable substrate capability for an authoritative voxel world exposed only through public interfaces.
- World identity, deterministic generation of bounded regions, streaming of resident work, material mutation with explicit lifecycle, queries (including registered objects that are not game entities), regenerable meshing as a derived view, and persistence of authoritative truth.
- Diagnostics that expose lifecycle, revision, and bounded-work observations without mutable internal handles.
- Repository delivery of a minimal public-interface validation harness (adjacent to the product identity).

**Does not belong to Moria**

- Game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content.
- Any game layer, gameplay policy, or consumer presentation beyond what an adjacent harness needs to call the public API.
- Portable performance thresholds or correctness gates derived from harness measurements.

## Required product outcomes

- A versioned parameter set and seed define world identity; generation is deterministic and materializes bounded regions without eagerly allocating the complete world.
- Authoritative material truth is preserved; consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
- Streaming bounds resident work and rejects stale background results; meshing is a derived view of voxel truth and can be regenerated from that truth.
- Registered objects can participate in deterministic world queries without becoming game entities.
- Persistence records versioned authoritative deltas and restores identical query behavior; derived meshes and transient scheduling state are not saved as truth.
- Public interfaces alone suffice for consumers (and the validation harness) to generate, stream, query, mutate, mesh, save, and restore the world; diagnostics report lifecycle, revision, and bounded-work observations without mutable internal handles.

## Future products and enabling implications

No named downstream game or consumer product is in current scope. External repositories are the intended consumers. The substrate must remain complete enough that such consumers can own gameplay, content, presentation, and policy on top of public world interfaces. Long-horizon consumer features are not a committed Moria roadmap.

## Non-goals

- Shipping a playable game, character stack, controller scheme, animation system, or authored content pack.
- Treating the validation harness as a product UI or embedding game-specific acceptance scenarios into substrate identity.
- Treating performance measurements as portable correctness thresholds.

## Confirmed vision constraints

- The product is a Rust substrate packaged for external consumption through public interfaces; the harness must use that same public surface available to another repository.
- Performance measurements include machine identity and are evidence only, not portable correctness thresholds.
- The repository delivers a minimal public-interface validation harness as an adjacent artifact; its free-fly camera, overlays, fixture split, workloads, and presentation remain harness concerns, not substrate identity.

## Deferred design decisions

- Depth and sequencing of substrate capability within the authorized outcome set.
- Public API shape, crate packaging, and internal structure (provided adjacent consumers have no privileged access).
- How the adjacent harness is organized (e.g. headless versus visual fixtures) and what diagnostic presentation it uses.
- Concrete algorithms, storage layouts, streaming policies, meshing methods, and persistence encodings.
- Any numeric performance budgets or benchmark environments (measurements remain non-portable evidence).

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names the product Moria and points current scope at the clean boundary and substrate requirement seeds.
- `docs/seeds/clean-project-boundary.md` — Fixes identity as a reusable Rust voxel-world substrate for external consumers; requires public-interface completeness for generate/stream/query/mutate/mesh/save/restore; places game-owned concerns outside; requires a minimal public-interface validation harness as repository delivery, not a game layer.
- `docs/seeds/clean-substrate-requirements.md` — Supplies binding outcome substance for identity, deterministic bounded generation, authoritative mutation lifecycle, streaming, derived meshing, queryable registered objects, persistence of authoritative deltas, public validation and diagnostics, and non-portable performance evidence.
