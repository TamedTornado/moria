# Moria — product vision

Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.

## Current product

**Moria** is a reusable Rust voxel-world substrate. This repository ships that substrate and a minimal validation executable that exercises it. It does not ship a game.

Downstream games and the validation executable consume Moria only through public crate interfaces.

## Purpose

Give multiple independent consumers a shared, correct foundation for voxel worlds: create and identify a world, stream bounded regions, read bounded authoritative material observations, apply bounded edits, extract surfaces for rendering, and persist material truth—without each consumer owning storage, meshing, or scheduling internals.

The substrate must stay useful across consumers; any single game’s rules, content, or presentation stay outside this product.

## Boundary

**In scope (this repository)**

- Public crate APIs for world identity, region requests, readiness, bounded authoritative material observations, bounded mutation, persistence of authoritative deltas, and read-only diagnostics.
- Deterministic seed-based generation, sparse voxel storage, bounded streaming, atomic mutation commit, surface extraction, and lifecycle/observability needed to support those APIs.
- Headless fixtures and a small visual validation executable that use only the public API (e.g. free-fly camera and diagnostics). The executable is not a game prototype and has no privileged world path. A small visual fixture demonstrates that a relocated external consumer can render and edit through the public API.

**Out of scope (consumers or later products)**

- Game rules, combat, inventory, AI, narrative, characters, animation, authored levels, production content, player controllers, character meshes, or curated traversal routes.
- Reaching into storage, meshing, or scheduler internals from outside the public surface.

**Future-consumer context (not current deliverables)**  
Seeds mention a later “Product One” explorer demo (third-person character, hills/forest/river/cave, skeletal animation, cliff-to-cave route) in a separate repository. That material is later-consumer context only; it pressures interface usefulness toward generation, streaming, mutation, surfaces, and persistence, but does not authorize those gameplay systems, assets, content, or Product One’s forest-population workload in this repository.

## Required product-level outcomes

1. **Public consumer surface** — External consumers can create/identify a world, request bounded regions, observe readiness, obtain bounded authoritative material observations, submit bounded edits, and persist authoritative deltas—without internal handles or private paths.
2. **Deterministic generation** — Same versioned parameters and seed produce the same world material truth.
3. **Bounded, observable streaming** — Resident work is bounded; lifecycle states (e.g. requested, loading, resident, evicted, failed) are visible; background results carry generation identities so stale results cannot overwrite newer truth.
4. **Authoritative mutation** — Edits enter through a bounded command API, with explicit admission failures and atomic commits; commit revisions are observable.
5. **Persistence of material truth** — Persistence records authoritative deltas (not derived meshes); restore yields the same material state.
6. **Derived data stays non-authoritative** — Meshes and diagnostics never become world truth.
7. **Typed, observable failures** — Failures remain typed and visible to public consumers.
8. **Validation without a game** — Headless fixtures cover generation, query, mutation, persistence, and lifecycle; a relocated visual consumer can render and edit only through the public API. Performance is reported with machine identity; this vision does not set machine-specific correctness thresholds.

## Non-goals

- Implementing or shipping Product One or any other game in this repository.
- Game systems (rules, combat, inventory, AI, narrative), characters, animation, authored levels, or production content.
- Product One’s forest-population workload, curated cliff-to-cave route, player controller, character mesh, animation clips, or game assets as Moria deliverables.
- Treating the validation executable as a prototype game or privileged world path.
- Expanding product scope from interface-reference detail (it describes needed surface pressure only).
- Machine-specific performance pass/fail gates as correctness commitments of this brief.

## Unresolved human questions

None that change product identity, purpose, or boundary. The seeds agree and settle: the current deliverable is the substrate; Product One and similar material are later/reference context in a separate repository, not current scope here.

## Seed contribution account

| Source | Contribution to this vision |
| --- | --- |
| `README.md` | Names Moria as the substrate defined by the program brief; states that only current substrate commitments are deliverables; marks the interface reference as supporting technical context that does not expand scope; flags the later-product vision as embedded but non-binding for this repo. |
| `docs/seeds/mixed-project-brief.md` | Primary binding source: current product, public boundary (consumers persist deltas; restoration is a product-level persistence guarantee), correctness and validation commitments (including that performance is reported with machine identity; background *results* carry generation identities), non-goals, and explicit separation of later Product One consumer vision from Moria deliverables. |
| `docs/seeds/substrate-interface-reference.md` | Refines the public surface (world identity, readiness and bounded authoritative material observations, bounded mutations, streaming states, delta persistence, registered objects in queries without becoming game entities, diagnostics without mutable internals) without adding deliverables or gameplay. |

**Omitted from current scope (visible in seeds but not imported as product)**  
Product One setting, explorer fantasy, skeletal animation, forest/river/cave content, forest-population workload, and curated cliff-to-cave traversal—retained only as interface-pressure context.

**Conflicts**  
None among the three manifest seeds regarding which product is current.

**Not settled by seeds (deliberately not constrained here)**  
Whether tools are a distinct consumer class, general world scale beyond Product One’s excluded forest-population workload, and platform-portable generation. The brief’s unmodified determinism commitment is not qualified by platform constraints in this vision.
