# Project vision

## What we are building now

**Moria** is a reusable Rust **voxel-world substrate**. Games and tools consume it through public crate interfaces. This repository delivers that substrate—not a game, not game content, and not a privileged in-repo world path.

A **minimal validation executable** sits beside the crate. It exercises the same public interfaces an external consumer would use (including free-fly camera and diagnostics sufficient to prove the API). It is not a game prototype and does not expand product identity.

## Purpose

Moria exists so multiple downstream consumers can share one foundation for material voxel worlds: create and identify a world, stream and observe bounded regions, query authoritative material truth, submit bounded edits, extract surfaces for presentation, and persist and restore material state—without embedding any particular game’s rules, content, or presentation.

## Product boundary

**In product**

- Public crate surface for world create/identify, bounded region requests, readiness observation, material queries, bounded mutation commands, and delta persistence.
- Substrate-owned capabilities: deterministic seed-based generation, sparse voxel storage, bounded streaming with observable lifecycle, atomic mutation commit, surface extraction, persistence of authoritative material deltas, and read-only diagnostics.
- Correctness properties: same versioned parameters and seed yield the same generation; derived meshes and diagnostics never become authoritative truth; background work carries generation identity so stale results cannot overwrite newer truth; failures are typed and observable to public consumers.

**Adjacent, not identity**

- A small validation executable and headless fixtures that consume only the public API. Performance may be reported with machine identity; no machine-specific correctness threshold is part of product identity.

**Out of product**

- Any particular game, including later “Product One” or similar demos: player controllers, character meshes, animation, curated routes, forest population, authored levels, production content, combat, inventory, AI, and narrative systems.
- Reach-through into storage, meshing, or scheduler internals by external consumers.

## Required product outcomes

1. **Reusable public substrate** — External consumers (games and the validation executable) integrate only through public crate interfaces; no privileged internal path exists for in-repo tools.
2. **Deterministic generation** — For the same versioned parameters and seed, generation is deterministic and remains useful across multiple consumers.
3. **Bounded streaming with observable lifecycle** — Resident work is bounded; consumers can observe states such as requested, loading, resident, evicted, and failed; generation identities prevent stale background work from replacing current truth.
4. **Authoritative material query and mutation** — Queries return readiness and bounded material observations; mutations are admitted as bounded commands, can fail admission explicitly, and commit atomically with revision identity.
5. **Surface extraction as derived view** — Meshes (and similar presentation products) are derived from material truth and are never the authority for world state.
6. **Persistence of material truth** — Persistence records authoritative deltas (not derived meshes) and restores the same material state.
7. **Diagnostics without authority** — Read-only diagnostics report lifecycle and bounded work without exposing mutable internal handles or elevating diagnostic data to world truth.
8. **Validated public contract** — Headless fixtures cover generation, query, mutation, persistence, and lifecycle; a small visual fixture shows that a relocated external consumer can render and edit through the public API.

## Future products and enabling implications

After the substrate ships, a separate repository may host a third-person explorer in a generated region (hills, mixed forest, river, cave) with skeletal animation and a curated cliff-to-cave traversal. That later material is **consumer context only**. It pressures the substrate toward reusable generation, streaming, mutation, surface extraction, and persistence—but does **not** authorize player control, characters, animation clips, forest workloads, curated routes, or game assets inside Moria.

## Non-goals

- Implementing game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content.
- Treating the validation executable as a game prototype or as a second, privileged implementation of the world.
- Expanding scope from interface-reference details into new deliverables not implied by the current program brief.
- Making derived presentation (meshes, diagnostics) authoritative over material state.

## Confirmed vision constraints

- Product identity is the **reusable voxel-world substrate**, not any game and not the validation harness.
- Delivery form is a **Rust crate** (public interfaces) plus a **minimal public-API validation path**.
- World identity combines format version, generation parameters, and seed.
- Mutations are command-shaped with explicit bounds, admission failures, and commit revisions.
- Streaming exposes lifecycle states; persistence stores authoritative deltas; registered objects may participate in queries without becoming game entities.
- Later-consumer vision embedded in the program brief does not authorize game deliverables in this repository.

## Deferred design decisions

- Crate layout, API naming, storage representation, meshing strategy, streaming ring policy, and persistence encoding.
- Numeric performance targets and machine-specific gates (reporting with machine identity is allowed; thresholds are not settled here).
- How far “registered objects” go beyond material participation in queries (objects must not become game entities).
- Sequencing and depth of surface-extraction and diagnostic surfaces beyond what public consumers need to render and observe.

## Assumptions proposed for approval

None beyond the reading that the mixed program brief’s **Current product / Current non-goals / Later consumer vision** sections correctly separate binding scope from future-consumer context. The root README states the same separation.

## Questions for human review

None. The seed-document manifest is consistent: current product is the substrate; Product One–style material is reference-only consumer pressure.

## Seed synthesis

- **`README.md`** — Names Moria as the reusable voxel-world substrate defined by the mixed program brief; states that only current substrate commitments are deliverables; positions the interface reference as supporting technical context without expanding product scope.
- **`docs/seeds/mixed-project-brief.md`** — Binding source for current product, public boundary, correctness, validation, non-goals, and the embedded later-consumer vision (explicitly non-authorizing for Moria deliverables). Establishes world create/identify, bounded regions, readiness, material query, bounded edits, persistence, deterministic generation, sparse storage, streaming, mutation, surface extraction, diagnostics, and the public-API-only validation executable.
- **`docs/seeds/substrate-interface-reference.md`** — Supports the brief without adding deliverables: world identity (format version, parameters, seed); readiness and bounded material observations; mutation commands with bounds, admission failure, and commit revisions; streaming lifecycle states; delta persistence; registered objects in queries without game-entity status; diagnostics without mutable internal handles.

Other files under `docs/seeds/` (for example older Product One or architecture notes) are **outside this vision’s seed-document manifest** and were not used to set product identity or scope.
