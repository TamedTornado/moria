# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate for external consumers. This repository delivers that substrate and a minimal public-interface validation harness. The product under design is the substrate; the harness is an adjacent delivery that proves the public surface, not a second product identity.

## Purpose

Moria exists so external consumers can generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces, without each consumer re-implementing world identity, material truth, streaming, meshing, or persistence.

## Product boundary

**In product scope**

- The reusable Rust voxel-world substrate and its public consumer-facing interfaces.
- Capabilities that make an authoritative voxel world operable end to end: identity and generation, sparse material truth, bounded mutation, streaming of resident work, derived meshing, world queries over registered objects, and versioned persistence with restore.

**Adjacent, not product identity**

- A minimal validation harness that exercises the public interface available to another repository. It may use a free-fly camera and diagnostic overlays. It is not a game layer.
- Headless and minimal visual fixtures that validate substrate operations through that public surface.

**Outside current product**

- Game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content.

## Required product outcomes

- **Authoritative operable world.** Through public interfaces, consumers can generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world.
- **World identity and deterministic generation.** A versioned parameter set and seed define world identity. Generation is deterministic and can materialize bounded regions without eagerly allocating the complete world.
- **Authoritative material truth and explicit mutation.** Sparse storage preserves material truth. Consumers submit bounded mutations through a public command API with explicit admission, commit, and failure states.
- **Bounded streaming and regenerable derived views.** Streaming bounds resident work and rejects stale background results. Meshing is a derived view of voxel truth and can be regenerated. Registered objects participate in deterministic world queries without becoming game entities.
- **Versioned persistence of truth only.** Persistence records versioned authoritative deltas and restores identical query behavior. Derived meshes and transient scheduling state are not saved as truth.
- **Public validation and observational diagnostics.** A headless fixture exercises generation, streaming, mutation, queries, and persistence. A minimal visual fixture with a free-fly camera exercises meshing through the public interface. Diagnostics expose lifecycle, revision, and bounded-work observations without mutable internal handles. Performance measurements include machine identity and are evidence, not portable correctness thresholds.

## Future products and enabling implications

Downstream products are external consumers in other repositories that integrate through Moria’s public interfaces. The substrate’s enabling implication is that those consumers can rely on a public, authoritative voxel-world surface without embedding game rules or consumer-specific content in Moria. No specific future game, content set, or presentation layer is in current scope.

## Non-goals

- Building a game, character/controller stack, animation system, authored routes, or production content inside Moria.
- Treating the validation harness as a shippable game or as a privileged second API.
- Treating performance numbers as portable pass/fail correctness thresholds.
- Persisting derived meshes or transient scheduling state as authoritative world truth.

## Confirmed vision constraints

- Integration surface is Rust and public interfaces usable by external consumers; the harness exercises the same API another repository would use.
- Mutation lifecycle states (admission, commit, failure) are explicit at the public command boundary.
- Generation and world-query participation for registered objects are deterministic under the product’s identity and query rules.
- Persistence restores identical query behavior from versioned authoritative deltas only.
- Validation diagnostics must not expose mutable internal handles.
- Performance evidence is machine-identified and non-portable as correctness criteria.

## Deferred design decisions

- Concrete parameter schemas, seed encoding, storage layout, streaming policy, meshing approach, and persistence encoding.
- Public API shape, package split, and how fixtures are organized in the repository.
- Exact diagnostic fields, fixture workloads, and any quantitative performance targets (beyond the rule that measurements are evidence, not portable thresholds).
- Depth and sequencing of delivery within the substrate outcomes above.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names the product Moria as a reusable voxel-world substrate and points current scope to the two clean boundary/requirements seeds.
- **docs/seeds/clean-project-boundary.md** — Fixes current identity (Rust substrate for external consumers), repository delivery (substrate plus minimal public validation harness), end-to-end operable-world mandate, and explicit exclusion of game-layer concerns.
- **docs/seeds/clean-substrate-requirements.md** — Supplies the outcome families for identity/generation, storage/mutation, streaming/derived views, persistence, and public validation/diagnostics that the substrate must make true.
