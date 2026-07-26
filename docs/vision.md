# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. The current product is the public substrate that lets those consumers generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world. This repository also delivers a minimal validation harness that exercises only those public interfaces; the harness is adjacent verification, not a game or product layer.

## Purpose

Moria exists so external products can own gameplay, presentation, and content while relying on a shared, deterministic voxel-world foundation. It concentrates world identity, authoritative material truth, streaming residency, derived meshing, public mutation, and persistence behind stable public interfaces that any external repository can call without privileged access.

## Product boundary

**Belongs to Moria**

- Public substrate capabilities for world generation, streaming, query, mutation, meshing, save, and restore.
- Diagnostics that observe lifecycle, revision, and bounded-work behavior without exposing mutable internal handles.
- A required adjacent validation delivery: headless coverage of generation, streaming, mutation, queries, and persistence, plus a minimal visual exercise of meshing through the same public interface available to other repositories.

**Does not belong to Moria**

- Game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content or policy.
- Gameplay, UX, presentation design, and acceptance scenarios owned by downstream games or other consumers.
- Portable performance thresholds, machine models, or consumer-selected runtime environments as product promises.

## Required product outcomes

- **Deterministic identity and bounded materialization.** A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
- **Authoritative truth and explicit mutation.** The substrate preserves authoritative material truth. Consumers submit bounded mutations through a public command interface whose admission, commit, and failure states are explicit.
- **Bounded streaming and derived meshing.** Streaming bounds resident work and rejects stale background results. Meshing is a derived view of voxel truth and can be regenerated from that truth.
- **Deterministic query participation.** Registered objects can participate in deterministic world queries without becoming game entities.
- **Authoritative persistence.** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
- **Public-only validation and observable operation.** Validation harnesses and external consumers use the same public interfaces. Diagnostics expose lifecycle, revision, and bounded-work observations without mutable internal handles. Performance measurements include machine identity and are evidence, not portable correctness thresholds.

## Future products and enabling implications

No named downstream game or consumer is in current scope. Future external products (for example games) are consumers of Moria, not expansions of Moria. The substrate’s enabling implication is that such products can build their own rules, characters, controllers, presentation, and content atop a reusable, deterministic voxel-world foundation without re-implementing world authority, streaming residency, derived meshing, or public persistence.

## Non-goals

- Shipping a game, playable campaign, character stack, or production content pack.
- Treating the validation harness as a game layer or importing consumer controllers, routes, assets, or acceptance scenarios into product identity.
- Treating derived meshes or transient scheduling state as authoritative persisted truth.
- Defining portable performance SLAs from measurement evidence.

## Confirmed vision constraints

- The product is a reusable **Rust** substrate intended for **external consumers** through public interfaces.
- World generation is **deterministic** given versioned parameters and seed.
- Mutation has **explicit admission, commit, and failure** states; consumers do not bypass public command boundaries.
- Persistence restores **identical query behavior** from versioned authoritative deltas only.
- Adjacent validation and diagnostics have **no privileged mutable internal access**.
- Performance numbers are **machine-identified evidence**, not portable correctness gates.

## Deferred design decisions

- Internal representation, algorithms, APIs, package layout, and synchronization for storage, streaming, meshing, and persistence.
- Depth and packaging of diagnostics; exact shape of headless versus visual validation fixtures.
- How far first delivery goes on each outcome family, and any non-portable benchmark environments used as evidence.

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds agree on product identity (Rust voxel-world substrate for external consumers), required outcome families, repository delivery of a minimal public-interface validation harness outside game-layer identity, and explicit exclusions of consumer-owned gameplay and content.

## Seed synthesis

- **`README.md`** — Names the product Moria and points current scope to the clean boundary and substrate seeds, without a separate downstream product vision.
- **`docs/seeds/clean-project-boundary.md`** — Fixes current identity as a reusable Rust voxel-world substrate, requires public-interface completeness for generate/stream/query/mutate/mesh/save/restore, requires a minimal validation harness as repository delivery, and excludes game-layer ownership.
- **`docs/seeds/clean-substrate-requirements.md`** — Supplies the binding outcome substance for identity, deterministic generation, authoritative mutation lifecycle, streaming, derived meshing, query participation, persistence, public validation, diagnostics, and non-portable performance evidence.
