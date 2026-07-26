# Moria — product vision

Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.

## Current product

**Moria** is a reusable Rust voxel-world substrate. This repository ships that substrate and a minimal validation executable that exercises it. It does not ship a game.

Downstream games and tools consume Moria only through public crate interfaces.

## Purpose

Give multiple independent consumers a shared, correct foundation for large voxel worlds: create and identify a world, stream bounded regions, read authoritative material, apply bounded edits, extract surfaces for rendering, and persist material truth—without each consumer owning storage, meshing, or scheduling internals.

The substrate must stay useful across consumers; any single game’s rules, content, or presentation stay outside this product.

## Boundary

**In scope (this repository)**

- Public crate APIs for world identity, region requests, readiness, material queries, bounded mutation, persistence of authoritative deltas, and read-only diagnostics.
- Deterministic seed-based generation, sparse voxel storage, bounded streaming, atomic mutation commit, surface extraction, and lifecycle/observability needed to support those APIs.
- Headless fixtures and a small visual validation executable that use only the public API (e.g. free-fly camera and diagnostics). The executable is not a game prototype and has no privileged world path.

**Out of scope (consumers or later products)**

- Game rules, combat, inventory, AI, narrative, characters, animation, authored levels, production content, player controllers, character meshes, or curated traversal routes.
- Reaching into storage, meshing, or scheduler internals from outside the public surface.

**Future-consumer context (not current deliverables)**  
Seeds mention a later “Product One” explorer demo (third-person character, hills/forest/river/cave, skeletal animation, cliff-to-cave route) in a separate repository. That material pressures the interface toward portable generation, streaming, mutation, surfaces, and persistence; it does not authorize those gameplay systems, assets, or content here.

## Required product-level outcomes

1. **Portable public surface** — External consumers can create/identify a world, request bounded regions, observe readiness, query material truth, submit bounded edits, and persist/restore authoritative material state—without internal handles or private paths.
2. **Deterministic generation** — Same versioned parameters and seed produce the same world material truth.
3. **Bounded, observable streaming** — Resident work is bounded; lifecycle states (e.g. requested, loading, resident, evicted, failed) are visible; background work carries generation identity so stale results cannot overwrite newer truth.
4. **Authoritative mutation** — Edits enter through a bounded command API, with explicit admission failures and atomic commits; commit revisions are observable.
5. **Persistence of material truth** — Persistence records authoritative deltas (not derived meshes); restore yields the same material state.
6. **Derived data stays non-authoritative** — Meshes and diagnostics never become world truth.
7. **Typed, observable failures** — Failures remain typed and visible to public consumers.
8. **Validation without a game** — Headless fixtures cover generation, query, mutation, persistence, and lifecycle; a relocated visual consumer can render and edit only through the public API. Performance may be reported with machine identity; this vision does not set machine-specific correctness thresholds.

## Non-goals

- Implementing or shipping Product One or any other game in this repository.
- Game systems (rules, combat, inventory, AI, narrative), characters, animation, authored levels, or production content.
- Treating the validation executable as a prototype game or privileged world path.
- Expanding product scope from interface-reference detail (it describes needed surface pressure only).
- Machine-specific performance pass/fail gates as correctness commitments of this brief.

## Unresolved human questions

None that change product identity, purpose, or boundary. The seeds agree: current deliverable is the substrate; Product One and similar material are later/reference context only.

If stakeholders want Product One (or any named consumer demo) *in* this repository’s current scope, that would reverse the explicit seed boundary and should be decided before design proceeds as if the substrate-only mandate holds.

## Seed contribution account

| Source | Contribution to this vision |
| --- | --- |
| `README.md` | Names Moria as the substrate defined by the program brief; states that only current substrate commitments are deliverables; marks the interface reference as supporting technical context that does not expand scope; flags the later-product vision as embedded but non-binding for this repo. |
| `docs/seeds/mixed-project-brief.md` | Primary binding source: current product, public boundary, correctness and validation commitments, non-goals, and explicit separation of later Product One consumer vision from Moria deliverables. |
| `docs/seeds/substrate-interface-reference.md` | Refines the public surface (world identity, readiness/material queries, bounded mutations, streaming states, delta persistence, registered objects in queries without becoming game entities, diagnostics without mutable internals) without adding deliverables or gameplay. |

**Omitted from current scope (visible in seeds but not imported as product)**  
Product One setting, explorer fantasy, skeletal animation, forest/river/cave content, and curated cliff-to-cave traversal—retained only as interface-pressure context.

**Conflicts**  
None among the three manifest seeds regarding which product is current.
