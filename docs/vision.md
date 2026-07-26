# Moria — Product Vision

**Status:** Proposal for human approval and handoff to design  
**Authority:** Synthesized from the seed-document manifest only

## Current product

**Moria** is a reusable Rust **voxel-world substrate**: a library (crate) consumed through public interfaces by games and by a minimal validation executable. This repository delivers the substrate, not a game.

## Purpose

Give multiple downstream consumers a shared, trustworthy world foundation they can create, stream, query, edit, and persist—while the substrate itself owns surface extraction into render-consumable form—without each game reimplementing voxel storage, generation, or world lifecycle. Correctness and consumption only through a public boundary matter more than any single demo experience.

## Boundary

| In scope | Out of scope |
| --- | --- |
| Public crate APIs for world identity, bounded regions, readiness, material truth, bounded mutation, and persistence | Game rules, combat, inventory, AI, narrative, characters, animation, authored levels, production content |
| Deterministic seed-based generation, sparse voxel storage, bounded streaming, surface extraction, read-only diagnostics | Player controllers, character meshes, animation clips, forest/river/cave population as product content |
| A minimal validation executable that exercises only the public API (e.g. free-fly camera and diagnostics) | A game prototype or any privileged path into storage, meshing, or scheduler internals |

External consumers must not reach into storage, meshing, or scheduler internals. The validation executable uses the same public surface as any other consumer and owns no special world path.

**Later-consumer context (not current deliverables):** After the substrate ships, a separate Product One repository may host a third-person explorer in a generated region (hills, mixed forest, river, cave) with skeletal animation and a curated cliff-to-cave traversal. That vision pressures the substrate toward reusable world capabilities; it does **not** authorize game systems, assets, or curated content inside Moria.

## Required product-level outcomes

1. **Deterministic generation** — Same versioned parameters and seed produce the same world.
2. **Authoritative material truth** — Queries expose readiness and bounded material observations; derived meshes and diagnostics never become authoritative state.
3. **Bounded mutation** — Edits enter through a bounded command API, admit or fail explicitly, and commit atomically; commits carry revisions.
4. **Bounded streaming** — Resident work is bounded; lifecycle states (requested, loading, resident, evicted, failed) are observable; background results carry generation identity so stale work cannot overwrite newer truth.
5. **Persistence of authority** — Persistence restores the same authoritative material state. Persistence records authoritative deltas rather than derived meshes.
6. **Substrate-owned surface extraction** — The substrate produces render-consumable surface results for consumers; consumers do not implement or reach into meshing internals.
7. **Public-only consumption** — Worlds are created and identified (format version, generation parameters, seed); consumers request regions, query, edit, and persist only through the public surface.
8. **Observable failure and diagnostics** — Failures are typed and visible to public consumers; diagnostics report lifecycle and bounded work without exposing mutable internal handles.
9. **Multi-consumer usefulness** — Substrate capabilities remain useful to more than one downstream product.
10. **Validation without a game** — Headless fixtures cover generation, query, mutation, persistence, and lifecycle; a small visual fixture demonstrates that a relocated external consumer can render and edit through the public API. Performance is reported with machine identity; no machine-specific correctness threshold is established by the seeds.

## Non-goals

- Implementing any particular game, demo gameplay loop, or production content pack
- Game systems: rules, combat, inventory, AI, narrative, characters, animation
- Authored levels, curated traversal routes, or forest/river/cave as Moria-owned content
- Exposing or depending on storage, meshing, or scheduler internals for external consumers
- Treating derived geometry or diagnostics as source of truth
- Establishing machine-specific performance pass/fail gates as product correctness
- Promising stability or compatibility of the public boundary beyond requiring consumption through it and prohibiting internal access

## Unresolved human questions

None that change product identity, purpose, or boundary. The seeds agree that the current product is the substrate and that embedded Product One material is future-consumer context only.

If humans later need to decide optional scope (e.g. how far the validation executable should go visually, or whether any performance reporting becomes a contractual gate), those choices belong to design or program planning—not to redefining what Moria is.

## Seed contribution account

| Seed | Contribution to this vision |
| --- | --- |
| `README.md` | Names the product (Moria), states that only current substrate commitments are repository deliverables, and frames the interface reference as supporting context that does not expand scope. |
| `docs/seeds/mixed-project-brief.md` | Binding definition of current product, public boundary, correctness and validation commitments, non-goals, and the embedded later Product One paragraphs as non-authorizing consumer context. Primary source for purpose, outcomes, and boundary—including state-restoring persistence, surface extraction ownership, relocated visual fixture, and performance reported with machine identity. |
| `docs/seeds/substrate-interface-reference.md` | Supporting detail on the reusable consumer surface (world identity, readiness/material queries, mutation commands with commit revisions, streaming states, delta-vs-mesh persistence recording, registered objects, diagnostics). Informs wording of outcomes; adds no new deliverables. Registered-object querying is reference-only and is not elevated to a required product outcome. |

**Omitted from current scope (present only as future or reference pressure):** Product One’s third-person explorer, terrain vignette (hills, forest, river, cave), skeletal animation, and curated cliff-to-cave route. High-level substrate capabilities those consumers would need (generation, streaming, query, mutation, surface extraction, persistence, public API) are retained; gameplay, content, characters, assets, and game implementation are not. Interface-reference-only functions (e.g. registered-object query participation) are not imported as current deliverables.
