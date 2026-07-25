# Project vision

## What we are building now

**Moria** is a reusable voxel-world substrate for external consumers. Through public interfaces, it makes an authoritative voxel world that can be generated, streamed, queried, mutated, meshed, saved, and restored.

This repository also delivers a minimal public-interface validation harness. The harness is an adjacent consumer of the substrate under test, not part of the product’s identity.

## Purpose

Moria exists so downstream games and tools can depend on a stable, authoritative voxel world without owning world generation, material truth, streaming, derived views, or persistence themselves. External work should integrate through the same public surface the substrate exposes to any other repository.

## Product boundary

**In product:** the reusable substrate and its public interfaces for world identity, generation, sparse material truth, streaming, mutation, queries, meshing as a derived view, and persistence of authoritative state.

**Adjacent, not identity:** a minimal validation harness that exercises those public interfaces (including optional free-fly viewing and diagnostic overlays). Harness-specific controls, presentation, routes, workloads, and acceptance scenarios are not product scope.

**Outside product:** game rules, characters, player or game controllers, animation, authored content, production assets, and any consumer-specific policy or presentation.

## Future products and enabling implications

Future products are external consumers (other repositories’ games or tools), not further slices of this product’s identity.

Enabling implications already supported by the seeds: consumers can depend on deterministic world identity, bounded generation without eager full-world allocation, public mutation and query access, regenerated derived meshes, and restore of identical query behavior from versioned authoritative deltas—without privileged internal access.

## Non-goals

- Shipping a game layer, characters, gameplay systems, or production content in this product.
- Giving the validation harness (or any adjacent consumer) privileged access beyond the public interface.
- Treating harness presentation, controllers, workloads, or performance gates as product requirements.

## Confirmed vision constraints

- World identity is defined by a versioned parameter set and seed; generation is deterministic and can materialize bounded regions without allocating the complete world.
- Consumers and the harness use public interfaces only; sparse storage holds authoritative material truth, and meshing is a regenerable derived view—not saved as truth.
- Persistence records versioned authoritative deltas and restores identical query behavior; diagnostics observe lifecycle and bounded work without exposing mutable internal handles. Performance measurements are evidence, not portable correctness thresholds.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **`README.md`:** Names the product Moria and states it is a reusable voxel-world substrate; points current scope to the clean boundary and requirements seeds (no independent expansion).
- **`docs/seeds/clean-project-boundary.md`:** Fixes current product identity as the reusable substrate for external consumers; requires repository delivery of substrate plus a minimal public-interface validation harness; excludes game rules, characters, controllers, animation, authored routes, production assets, and consumer content; requires public-interface completeness for generate, stream, query, mutate, mesh, save, and restore.
- **`docs/seeds/clean-substrate-requirements.md`:** Contributes vision-level outcomes for identity and deterministic bounded generation, authoritative sparse truth and public mutation, streaming and derived meshing, persistence of authoritative deltas, public validation/diagnostics posture, and non-portable performance evidence—without promoting mechanism inventories into product identity.
