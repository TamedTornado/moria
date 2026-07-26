# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. This
repository delivers substrate crates and a minimal public-interface validation
harness. The product is the substrate, not a game.

## Purpose

Provide an authoritative voxel-world foundation that external repositories can
generate, stream, query, mutate, mesh, save, and restore through public
interfaces—so each consumer does not reimplement world truth, lifecycle, or
persistence.

## Product boundary

**In scope.** The reusable substrate: deterministic world identity and
generation, sparse authoritative material storage, bounded mutation with
explicit admission and outcomes, streaming that bounds resident work, meshing
as a regenerable derived view, versioned persistence of authoritative deltas,
deterministic world queries involving registered objects, and read-only
diagnostics.

**Validation harness (adjacent delivery).** A minimal harness exercises the
same public API available to another repository. It may use a free-fly camera
and diagnostic overlays. Headless fixtures exercise generation, streaming,
mutation, queries, and persistence; a minimal visual fixture exercises meshing
through the public interface. The harness is not a game layer and owns no
privileged path into substrate internals.

**Out of scope as product identity.** Game rules, characters, controllers,
animation, authored routes, production assets, and consumer-specific content.
Those belong to external consumers.

## Required product-level outcomes

1. **Deterministic identity and generation.** A versioned parameter set and seed
   define world identity. Generation is deterministic and can materialize
   bounded regions without eagerly allocating the complete world.
2. **Sparse material truth and bounded mutation.** Sparse voxel storage
   preserves authoritative material truth. Consumers submit bounded mutations
   through a public command API with explicit admission, commit, and failure
   states.
3. **Streaming and derived views.** Streaming bounds resident work and rejects
   stale background results. Meshing is a derived view of voxel truth and can
   be regenerated. Registered objects can participate in deterministic world
   queries without becoming game entities.
4. **Persistence of authority, not views.** Persistence records versioned
   authoritative deltas and restores identical query behavior. Derived meshes
   and transient scheduling state are not saved as truth.
5. **Public validation and diagnostics.** Public fixtures prove generation,
   streaming, mutation, queries, persistence, and meshing through the shared
   API. Diagnostics expose lifecycle, revision, and bounded-work observations
   without mutable internal handles. Performance measurements include machine
   identity and are evidence, not portable correctness thresholds.

## Non-goals

- Shipping a game, playable demo, or production content pack as the product.
- Implementing game rules, characters, controllers, animation, authored
  routes, or consumer-specific assets and content.
- Treating meshes, scheduling state, or diagnostics as authoritative world
  truth.
- Encoding machine-specific performance numbers as portable product
  correctness gates.

## Unresolved human questions

None. The three seed documents agree that the current product is the reusable
Rust voxel-world substrate with an adjacent public-API validation harness, and
that game-facing layers are outside this repository.

## Seed contribution account

- **`README.md`** — Names Moria as a reusable voxel-world substrate and points
  current scope to the two clean seed documents, without a downstream product
  vision.
- **`docs/seeds/clean-project-boundary.md`** — Establishes product identity
  (substrate crates + minimal public-interface validation harness), the free-fly
  / diagnostic harness role (not a game layer), explicit exclusion of game
  rules and consumer content, and the completeness bar: generate, stream,
  query, mutate, mesh, save, and restore through public interfaces.
- **`docs/seeds/clean-substrate-requirements.md`** — Supplies product-level
  capability outcomes for identity and generation, storage and mutation,
  streaming and derived views (including meshing and registered-object
  queries), persistence of authoritative deltas, and public validation plus
  diagnostics; treats performance data as machine-bound evidence.

Other documents under `docs/seeds/` (for example older product demos or
architecture references) were not part of the prescribed seed manifest and are
not imported into current product scope. Any high-level capabilities they
illustrate for future consumers are already covered by the outcomes above;
their gameplay, content, characters, assets, and implementation detail remain
out of scope here.
