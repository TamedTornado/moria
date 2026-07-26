# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world product for games—not a game, not a demo title, and not an LLM-backed system.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface and deep underground whose voxel grid is the authoritative truth for inspection and mutation. Game rules, presentation, and policies live above the substrate; the substrate stands alone with no LLM dependency and supplies matter, physics-oriented world behavior, queries, and mutation for downstream titles.

## Product boundary

**This product owns** the reusable voxel-world substrate: world generation foundations, GPU-resident matter, non-authoritative mesh/dressing views of that matter, streaming and persistence foundations, and the public mutation/query surface external games integrate against.

**Adjacent, not identity.** A walkable-world executable may exist beside the crate stack as a validation harness. When present, it must use the same public interfaces available to an external game and must not own privileged or game-specific substrate paths. Whether shipping that harness is part of current delivery is unresolved (see Q1). Its character, camera, controls, authored region content, presentation, demo routes, workloads, and performance gates are harness- or consumer-owned, not substrate product scope.

**Out of this product and repository.** The actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and building gameplay layers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

Downstream design must make these consumer-visible guarantees true:

- **Integrable substrate.** External games and any co-located harness consume Moria only through public Rust crate interfaces, with no privileged internal access path.
- **Material world that reads natural.** Rolling terrain, forests, water, cliffs, and similar surface features read as an ordinary world while remaining fully material: what you see is backed by mutable voxel truth, and cuts or scars read as real matter changes—not decorative geometry outside the world model.
- **Mutable everywhere, deep Z first-class.** Any reachable volume can be destroyed, altered, or filled; underground strata, voids, and buried materials are continuous content space with the surface, not a shallow painted floor.
- **Geology-first, sparse large worlds.** Worlds are generated as geology and related structure that can materialize on demand so large regions stay cheap until touched, rather than as a heightmap with rock painted underneath.
- **Matter API boundary.** Consumers mutate and inspect the world through substrate verbs and queries; layers above matter do not touch raw voxel storage directly. That seam is the multi-game and sandbox boundary.
- **Streaming and persistence foundations.** Authoritative world state is generation plus edit history such that untouched volume stays cheap to store and mutated regions can be reloaded; active play can focus work near anchors without treating the whole region as always resident.

## Future products and enabling implications

Described future consumers include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate substrate generality (injectable pricing/policy above shared verbs, System as a game-layer client of the same mirrors and registries, fortress and adventure reuse of the same material world). Their gameplay, UX, controllers, characters, authored content, presentation, and game-specific policy are not current Moria scope.

A first walkable “product one” region-and-character slice is an adjacent validation and audience artifact for the substrate, not a second product identity and not a narrowing of the substrate’s purpose. Long-horizon matter behaviors that future games need (richer fluids, structural failure, vegetation as interactable objects, ambient ecology) remain enabling implications of the substrate’s role; delivery depth and sequence are design choices, not a committed multi-title roadmap in this brief.

## Non-goals

- Shipping the actual game or implementing System/LLM, spell, gas, combat, AI, or building gameplay layers in this repository
- Treating the walkable harness’s demo fantasy, controls, content set, or benchmark theater as the product definition
- Making the substrate depend on an LLM to function
- Redefining the product as a single-game vertical slice rather than a reusable world substrate

## Confirmed vision constraints

- Delivered in the Rust crate ecosystem (one crate or a small tightly scoped family)
- GPU-resident voxel-world substrate
- Substrate must stand alone with zero LLM dependency
- Any validation harness in-repo consumes only public interfaces (no privileged game-only substrate paths)
- Game rules and the listed future game layers stay above/outside this product

## Deferred design decisions

- Exact crate family split and internal module boundaries
- Voxel scale, LOD approach, object-layer capacity, and similar fidelity/cost tradeoffs
- Which substrate outcome depths ship in which milestone; harness content and acceptance theater if a harness is delivered
- How far multi-tier fluids, structural integrity, vegetation object lifecycle, and ambient simulation are taken in any given delivery
- Backend graphics stack choices, target machines, and numeric performance budgets (not fixed by product identity seeds)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required adjacent delivery** of current Moria work, or only **permitted** to exist beside the substrate crates?

- **Proposed answer:** Permitted only. Current product identity and required outcomes center on the reusable substrate; a harness may exist and, if present, must use public interfaces, but shipping it is not required to complete the substrate product.
- **If different:** Requiring it keeps product identity as the substrate but adds a mandatory adjacent deliverable (still not game scope) that design must plan for; “permitted only” allows crate-first delivery without a shippable walkable binary.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and positions a walkable-world executable as a separate consumer/validation harness rather than a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the Rust substrate crate(s), requires public-interface-only harness consumption, excludes the game and listed game layers from this repository, and treats workspace separation as enforcement of that consumer boundary.
- **docs/seeds/product-one-seed.md** — Describes a first walkable validation/demo slice that motivates mutability proof and substrate exercise; its character, content, platforms, and numeric gates stay consumer/harness-owned and do not redefine product identity.
- **docs/seeds/voxel-world-substrate.md** — Authorizes the substrate’s outcome families (natural material world, deep-Z mutability, geology-first generation, matter API layering, streaming/persistence role) without pulling mechanism inventory, game layers, or implementation detail into this brief.
