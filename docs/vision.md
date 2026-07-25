# Project vision

## What we are building now

**Moria** is a reusable **Rust** voxel-world substrate for external consumers. Through public interfaces it supports generating, streaming, querying, mutating, meshing, saving, and restoring an authoritative voxel world. This repository delivers that substrate and a minimal public-interface validation harness; the harness is an adjacent delivery, not a second product identity.

## Purpose

Moria exists so independent games and tools can share a deterministic, authoritative voxel-world foundation without owning or forking substrate internals. Consumers integrate the library surface; they do not become part of Moria’s product.

## Product boundary

**Belongs to Moria:** the substrate itself—public APIs and library surface that make an authoritative voxel world usable by other repositories.

**Delivered beside Moria, not the product identity:** a minimal validation harness that exercises the same public interfaces available to any external consumer. It may use a free-fly camera and diagnostic overlays only to validate those interfaces. It is not a game layer.

**Outside Moria:** game rules, characters, controllers, animation, authored routes, production assets, and all consumer-specific content, presentation, and policy.

## Future products and enabling implications

Downstream games and tools are future or adjacent *consumers* of Moria, not current product scope. Enabling implication: a stable public substrate for authoritative voxel worlds so those consumers can generate, stream, query, mutate, mesh, and persist world truth without privileged access. Their gameplay, UX, controllers, characters, content, and presentation remain entirely theirs.

## Non-goals

- Shipping a playable game, character, controller stack, or authored experience in this product
- Treating the validation harness as a privileged path, prototype game, or consumer of private internals
- Absorbing consumer-owned presentation, policy, or content into the substrate

## Confirmed vision constraints

- External consumers and the in-repo validation harness share the same public interfaces; the harness has no privileged access.
- Completeness for the current product means public generate, stream, query, mutate, mesh, save, and restore of an authoritative voxel world—not a game or content vertical.

## Assumptions proposed for approval

None.

## Questions for human review

None. The supplied seeds agree on product identity (Rust voxel-world substrate), consumer boundary, and required adjacent validation delivery without transferring harness or game ownership into the product.

## Seed synthesis

- **`README.md`**: Names the product Moria as a reusable voxel-world substrate and points current scope to the two clean boundary/requirements seeds; no competing identity.
- **`docs/seeds/clean-project-boundary.md`**: Fixes current identity as a Rust substrate for external consumers, requires substrate plus a minimal public-interface validation harness as repository delivery, and excludes game-layer ownership; compatible detail stays subordinate to design.
- **`docs/seeds/clean-substrate-requirements.md`**: Supplies product-level outcomes (deterministic identity, authoritative mutation, streaming and derived views, persistence of truth not derived state, public validation and diagnostics as evidence) that shape purpose and completeness without entering this brief as an operation inventory; detailed requirements remain subordinate input to downstream design.
