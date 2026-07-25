# Project vision

## What we are building now

**Moria** is a reusable voxel-world substrate. This repository delivers that substrate through public crate interfaces so multiple downstream consumers can create, stream, inspect, mutate, and persist authoritative material worlds. It is not a game, demo title, or content product.

## Purpose

Moria exists so future games and tools do not each reimplement a durable, multi-consumer voxel world core. It provides a shared, deterministic foundation for natural-looking mutable voxel worlds while keeping gameplay, presentation policy, and authored experience outside the substrate.

## Product boundary

**This product owns** the reusable world substrate: world identity and seed-based generation, authoritative material truth, bounded region request and residency, admitted mutation, durable persistence of material state, surface extraction sufficient for external rendering, and read-only diagnostics. Consumers interact only through the public surface; storage, meshing, and scheduler internals stay encapsulated.

**Adjacent, not identity:** any validation executable or fixture is a consumer of that public surface. It must not gain a privileged world path and is not a game prototype.

**Outside this product:** game rules, combat, inventory, AI, narrative, characters, animation, controllers, authored levels, production content, and any particular game’s presentation or acceptance scenario.

## Future products and enabling implications

**Product One** (separate repository, after the substrate ships) is a later game-facing explorer demo: a third-person traversal through a generated region used to communicate the world. That consumer may pressure the substrate toward multi-consumer usefulness, observable readiness, and renderable/mutable material worlds.

Enabling implication only: Moria should remain fit for such external games without absorbing their controller, character, animation, route, forest/river/cave content, or curated experience. Those remain Product One’s responsibility.

## Non-goals

- Shipping a playable game, character stack, or production content pack in this repository.
- Treating derived meshes, diagnostics, or consumer-side objects as authoritative world state.
- Embedding game entities or gameplay policy into the substrate under the guise of world objects.

## Confirmed vision constraints

- Repository deliverable is the substrate, not any particular game; the same public interfaces must serve multiple consumers.
- Generation is deterministic for the same versioned parameters and seed; mutation is bounded and committed so material authority stays coherent; persistence restores that material authority.
- Streaming bounds resident work and exposes lifecycle that consumers can observe; failures are typed and visible; stale background work must not overwrite newer truth.

## Assumptions proposed for approval

1. **Language and packaging:** “Reusable Rust substrate consumed as public crates” is part of current product identity (not a deferred portability decision).
2. **Validation placement:** If a harness is required (see Q1), it lives as an adjacent in-repo consumer of public APIs only—never a second product identity or a privileged engine path.

## Questions for human review

**Q1.** Is a minimal validation harness (headless and small visual fixtures that exercise the public substrate APIs) a **required current delivery** of this repository, or only a **permitted** adjacent consumer pattern?

- **Proposed safe answer:** Required as adjacent validation that the substrate works through public interfaces; not part of product identity; no game controls, content, or acceptance scenario imported into Moria scope.
- **If different:** Treating the harness as optional shrinks repository commitments to library-only delivery; treating its specific camera, workload, or performance gates as product scope would blur substrate identity with a demo product.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable voxel-world substrate; states only current substrate commitments are repository deliverables; flags the brief’s embedded later-product vision as non-deliverable here; positions the interface reference as supporting context without scope expansion.
- **`docs/seeds/mixed-project-brief.md`:** Supplies binding current identity (reusable substrate vs game), public consumer boundary, correctness and multi-consumer stance, explicit non-goals, and later Product One paragraphs clearly labeled as future-consumer context that must not authorize game systems or assets in Moria; also states validation commitments that create Q1’s mandatory-vs-permitted tension.
- **`docs/seeds/substrate-interface-reference.md`:** Confirms a stable public surface (identity, readiness/material query, bounded mutation, streaming lifecycle, material-delta persistence, non-entity registered objects, non-mutating diagnostics) without adding deliverables or transferring consumer-owned presentation into the substrate.
