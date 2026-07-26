# Moria — Vision

## Current product

**Moria** is a reusable Rust voxel-world **substrate**: a library consumed through public crate interfaces by games and by a minimal validation executable. This repository delivers the substrate, not any particular game.

## Purpose

Provide a correct, deterministic, streamable voxel world that multiple independent downstream consumers can generate, query, edit, and persist through a stable public API—without depending on game rules, content, or privileged internal paths.

## Boundary

**In scope**

- Public crate interfaces for world identity, bounded region requests, readiness observation, material-truth queries, bounded edits, and delta persistence.
- Deterministic seed-based generation, sparse voxel storage, bounded streaming, mutation, surface extraction, and read-only diagnostics—kept useful to more than one consumer.
- A minimal validation executable that exercises **only** those public interfaces (optional free-fly camera and diagnostics). It is not a game prototype and owns no privileged world path.
- Headless fixtures for generation, query, mutation, persistence, and lifecycle; a small visual fixture showing an external consumer can render and edit via the public API. Performance may be reported with machine identity; no machine-specific correctness bar is set here.

**Out of scope (non-goals)**

- Game rules, combat, inventory, AI, narrative, characters, animation, authored levels, or production content.
- Any later consumer product (e.g. a third-person explorer demo, forest/river/cave content, skeletal animation, curated traversal routes, or game assets). Such material is **future-consumer context** only; it explains interface pressure, not deliverables.

**Consumer isolation**

External code must not reach into storage, meshing, or scheduler internals. Derived meshes and diagnostics are never authoritative world state.

## Required product-level outcomes

1. **Reusable public surface** — Consumers can create and identify a world, request bounded regions, observe readiness, query material truth, submit bounded edits, and persist deltas without private hooks.
2. **Deterministic generation** — Same versioned parameters and seed produce the same authoritative material state.
3. **Bounded, observable mutation** — Edits enter through a command API with explicit bounds, admission failures, and atomic commit (with revisions).
4. **Faithful persistence** — Restored state matches the same authoritative material truth; deltas, not derived meshes, are the source of record.
5. **Bounded streaming with safe concurrency** — Resident work is bounded; lifecycle states are observable; background results carry generation identities so stale work cannot overwrite newer truth; failures are typed and visible to public consumers.
6. **Validation without product capture** — Headless and small visual fixtures prove the crate through the same public path a relocated external consumer would use.

## Non-goals (product identity)

Moria is not a game, demo, or content pipeline. Interface-supporting concepts (e.g. registered objects participating in queries without becoming game entities) remain substrate concerns; they do not import gameplay systems into this repository.

## Unresolved human questions

None from the seed set that would change product identity, purpose, or boundary. Seeds agree: current deliverable is the substrate; later Product One–style material is explicit future-consumer context only.

If product owners later want the validation executable, surface-extraction depth, or “registered objects” semantics to carry more (or less) product weight, that would be a deliberate scope decision—not a conflict already present in the seeds.

## Seed contributions

| Source | Role in this vision |
|--------|---------------------|
| **`README.md`** | Names the product (Moria), states that only substrate commitments are deliverables, and frames the brief’s later-product paragraphs as non-binding embedded context. Points at the interface reference as supporting technical context only. |
| **`docs/seeds/mixed-project-brief.md`** | Binding definition of current product, public boundary, correctness and validation commitments, non-goals, and the later consumer vision—with an explicit disclaimer that those paragraphs do not authorize game systems or assets in Moria. Primary source for purpose, outcomes, and non-goals. |
| **`docs/seeds/substrate-interface-reference.md`** | Technical color for the public surface (world identity, readiness/material queries, mutation commands, streaming lifecycle, delta persistence, registered objects, diagnostics). Used to sharpen boundary and outcomes; **does not** add deliverables or expand product scope. |

Material **not** imported into current scope: Product One gameplay and presentation (third-person explorer, hills/forest/river/cave scene, skeletal animation, curated cliff-to-cave route, character/content assets). High-level capabilities those consumers would need—deterministic worlds, streaming, mutation, persistence, queryable material truth—remain as substrate outcomes above.
