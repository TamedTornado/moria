# Moria — Product Vision

## Current product

**Moria** is a reusable Rust voxel-world substrate. This repository ships that
substrate as public crate interfaces for games and other tools, plus a minimal
validation executable that exercises only those interfaces. It does not ship a
game.

## Purpose

Give multiple downstream consumers a shared, trustworthy voxel world they can
create, stream, inspect, edit, and persist without owning generation, storage,
meshing, or scheduling internals. Success means the same public surface is
useful to more than one consumer and remains correct under streaming, mutation,
and restore.

## Boundary

**In scope for this product**

- Deterministic seed-based world generation under versioned parameters
- Sparse voxel storage and bounded streaming of world regions
- Bounded, atomic mutation of authoritative material state
- Surface extraction as a derived view (never authoritative)
- Persistence of authoritative material deltas
- Read-only diagnostics and observable lifecycle / failure reporting
- Public crate APIs for world identity, region request, readiness, material
  query, edit admission, and persist/restore
- A minimal validation executable (optional free-fly camera and diagnostics)
  that uses only the public API and holds no privileged world path
- Headless fixtures for generation, query, mutation, persistence, and lifecycle

**Out of consumer reach**

External callers must not reach into storage, meshing, or scheduler internals.
Derived meshes and diagnostics must never become authoritative world state.

## Required product-level outcomes

1. **Reusable public surface** — Consumers can create and identify a world,
   request bounded regions, observe readiness, query material truth, submit
   bounded edits, and persist deltas through documented crate interfaces.
2. **Deterministic generation** — The same versioned parameters and seed yield
   the same world material truth.
3. **Trusted mutation** — Edits enter only through a bounded command API,
   with admission failures and atomic commit (including revision identity).
4. **Faithful persistence** — Restore recovers the same authoritative material
   state; persistence records deltas of that truth, not derived meshes.
5. **Bounded streaming** — Resident work stays bounded; lifecycle states
   (requested, loading, resident, evicted, failed) are observable; background
   results carry generation identities so stale work cannot overwrite newer
   truth.
6. **Observable failures** — Failures are typed and visible to public
   consumers without exposing mutable internal handles.
7. **Validated without becoming a game** — Headless coverage and a small
   visual fixture prove that a relocated external consumer can render and edit
   through the public API; performance may be reported with machine identity
   but is not a machine-specific correctness gate in this vision.

These outcomes preserve the *capabilities* later games need (streamed
generated terrain, edits, restore, meshable surfaces, diagnostics). They do
not pull later gameplay, content, characters, or assets into Moria’s scope.

## Non-goals

Moria does not implement:

- Game rules, combat, inventory, AI, or narrative systems
- Characters, animation, player controllers, or production content
- Authored levels or curated traversal routes as product deliverables
- Forest population workloads, river/cave set pieces, or other scene content
  as substrate features
- A game prototype or any privileged path around the public API

References to possible games exist only to pressure the interface design.

## Future / reference context (not current deliverables)

After the substrate ships, a separate **Product One** repository may build a
third-person explorer in a generated region (hills, mixed forest, river, cave)
with skeletal animation and a curated cliff-to-cave traversal. That material is
embedded in the program brief as later-consumer context. It does **not**
authorize player control, character meshes, animation clips, forest workloads,
curated routes, or game assets in this repository.

The substrate interface reference describes the public surface consumers need
(world identity, readiness and material queries, mutation commands and
revisions, streaming states, delta persistence, registered objects in queries
without game-entity semantics, lifecycle diagnostics). It supports the brief; it
does not expand product scope.

## Unresolved human questions

None that change product identity, purpose, or boundary. The seeds agree that
the current product is the reusable substrate and that Product One and the
interface reference are context, not concurrent deliverables.

Open items for later design (not vision blockers): concrete crate layout and
API names; mesh extraction scheduling details; exact persistence format; how
“registered objects” participate in queries; machine baselines for performance
reporting.

## Seed contribution map

| Seed | Contribution to this vision |
| --- | --- |
| `README.md` | Names the product Moria; states that only current substrate commitments are deliverables; positions the interface reference as non-expanding support; flags the later-product vision as deliberately embedded but non-binding for this repo. |
| `docs/seeds/mixed-project-brief.md` | Primary definition of current product, public boundary, correctness and validation commitments, non-goals, and the Product One future-consumer vignette (explicitly non-authorizing for Moria). |
| `docs/seeds/substrate-interface-reference.md` | Interface-level capability pressure (identity, query, mutation, streaming lifecycle, delta persistence, registered objects, diagnostics) without adding deliverables or gameplay. |

**Omitted from current product scope (visible in seeds, not imported here):**
Product One’s explorer fantasy, terrain set pieces, skeletal animation, and
curated traversal—retained only as rationale that the substrate must stay
generically useful to multiple consumers.
