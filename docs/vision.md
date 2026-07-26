# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. It
provides public interfaces to generate, stream, query, mutate, mesh, save, and
restore an authoritative voxel world. This repository delivers that substrate
and public-interface validation fixtures; those fixtures are adjacent repository
deliveries that exercise the product, not part of the substrate’s product
identity.

## Purpose

External consumers need a shared, authoritative voxel world they can integrate
without rebuilding world generation, storage, mutation, streaming, meshing, and
persistence. Moria exists so other repositories can own their games, tools, and
presentation while relying on one substrate for world truth and the public
operations that surround it.

## Product boundary

**Belongs to Moria**

- The reusable substrate and its public interfaces for world identity,
  generation, sparse material truth, bounded mutation, streaming, queries,
  meshing, persistence, and diagnostics.
- Completeness enough that an external consumer can perform the full
  generate–stream–query–mutate–mesh–save–restore path through those interfaces.

**Adjacent repository delivery (not product identity)**

- A headless validation fixture that exercises generation, streaming, mutation,
  queries, and persistence through the public API.
- A minimal visual validation fixture with a free-fly camera that exercises
  meshing through the public interface.
- Diagnostic overlays may accompany the visual fixture. Fixture mechanisms and
  further depth remain harness behavior.

**Outside current product**

- Game rules, characters, controllers, animation, authored routes, production
  assets, and consumer-specific content. Validation fixtures are not a game
  layer.

## Required product outcomes

- A versioned parameter set and seed define world identity. Generation is
  deterministic and can materialize bounded regions without eagerly allocating
  the complete world.
- Sparse voxel storage preserves authoritative material truth. Consumers submit
  bounded mutations through a public command API whose admission, commit, and
  failure states are explicit.
- Streaming bounds resident work and rejects stale background results. Meshing
  is a derived view of voxel truth and can be regenerated.
- Registered objects can participate in deterministic world queries without
  becoming game entities.
- Persistence records versioned authoritative deltas and restores identical
  query behavior. Derived meshes and transient scheduling state are not saved as
  truth.
- Diagnostics expose lifecycle, revision, and bounded-work observations without
  mutable internal handles. Repository validation must include the headless and
  visual fixture roles above, each exercising its required public capabilities
  rather than privileged internals.

## Future products and enabling implications

No specific downstream game or tool is in current product scope. Future external
consumers (games, editors, tools in other repositories) are the intended users
of the substrate. Enabling implication: the public generate–stream–query–
mutate–mesh–save–restore path and diagnostics must remain usable without
privileged internal access. Gameplay, UX, controllers, authored content, and
presentation stay with those consumers.

## Non-goals

- Building a game, character stack, controller stack, animation system, or
  authored content pipeline inside this product.
- Treating validation fixtures as a playable product or game layer.
- Treating performance measurements as portable correctness thresholds.
- Saving derived meshes or transient scheduling state as authoritative truth.

## Confirmed vision constraints

- The substrate is a Rust library surface aimed at external consumers in other
  repositories; consumers integrate through public interfaces.
- Performance measurements include machine identity and are evidence, not
  portable correctness gates.
- Validation fixtures use the same public API available to another repository;
  free-fly camera use on the visual fixture exercises meshing and does not
  expand product scope into a game layer.

## Deferred design decisions

- Concrete APIs, crate layout, storage encodings, algorithms, and streaming or
  meshing schedules.
- Fixture implementation mechanisms and any coverage depth beyond the required
  headless and visual roles and their mandated capability sets.
- Any performance targets, hardware matrices, or benchmark workloads.
- How consumers register objects, author parameters, or compose higher-level
  gameplay on top of queries and mutations.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names the product Moria and points current scope at the clean
  boundary and substrate requirement seeds.
- `docs/seeds/clean-project-boundary.md` — Establishes reusable Rust substrate
  identity, repository delivery of substrate plus public-interface validation,
  public-API-only exercise, exclusion of game-layer concerns, and the required
  world capability path.
- `docs/seeds/clean-substrate-requirements.md` — Supplies binding outcome-level
  substance for identity, generation, storage, mutation lifecycle, streaming,
  meshing, queries, persistence, diagnostics, the two validation fixture roles
  and their required coverage, and the role of performance evidence.
