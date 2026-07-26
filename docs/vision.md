# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates for external game consumers. It is an engine-layer foundation for material worlds—not a game, not an authored content pack, and not a title-specific rules stack.

## Purpose

Moria exists so multiple games can share one foundation for natural-looking, fully material voxel worlds: landscape that reads as ordinary terrain while remaining mutable matter all the way down, including deep underground. Game rules, presentation, and policy stay above the substrate so the same stack can underpin different titles without embedding any one game’s systems.

## Product boundary

**In product:** the reusable substrate—generation foundation for geological worlds, GPU-resident mutable matter as world truth, visual presentation derived as a non-authoritative view of that matter, and public mutation and query interfaces for consumers.

**Adjacent to the product:** a walkable-world executable may exist as a validation harness that consumes those same public interfaces (whether it is a current repository delivery is Q1). Downstream games are separate consumers and are not this repository’s product.

**Consumer-owned (not substrate scope):** character control, cameras, authored demo regions and routes, harness workloads and acceptance metrics, game presentation, and title-specific content or policy.

## Required product outcomes

1. **Material world truth.** The world is a voxel matter field that can be destroyed, moved, and placed throughout the volume, including deep underground—not decorative geometry over a fixed heightmap shell.
2. **Normal-looking surface from matter.** Consumers can present rolling terrain, vegetation, water bodies, cliffs, and similar natural features that read as an ordinary world while remaining grounded in voxel truth; the rendered surface is a view, never the authority.
3. **Deep Z as content.** Underground space is continuous first-class volume—geology, caves, strata, and buried features—not a thin floor under the skybox.
4. **Public mutation and query surface.** External games and any in-repo harness inspect and change the world only through the substrate’s public interfaces; nothing above the matter core gets privileged direct voxel access.
5. **Large-region residence.** Playable regions can be large enough that sparsity and streaming are real: the full volume need not sit in memory as dense raw voxels at once.
6. **Standalone reusability.** The substrate stands alone with no LLM dependency and no built-in game policy, and remains usable as a shared foundation across multiple game genres.

## Future products and enabling implications

Described future consumers include a System-driven ARPG, a fortress/colony-style builder, a descent-style adventure game, and pure sandboxes. Moria enables them by providing matter, world behavior relevant to physics and traversal, queries, and mutation—not by shipping their gameplay, controllers, AI, combat, spells, gas policy, building UX, or authored content. Compatibility seams may be designed where substrate requirements demand them; those game layers are not implemented in this product.

## Non-goals

- Shipping the actual game, or any game-rules product, in this repository
- Implementing System/LLM features, spells, gas policy, combat, AI, or building layers here
- Owning consumer presentation, character control, or harness/demo content as product identity
- Collapsing product identity to a single first demo route, seed region, or milestone catalog

## Confirmed vision constraints

- Delivery form is the Rust crate ecosystem (one crate or a small tightly scoped family).
- Any validation harness in this repository must use the same public interfaces available to an external game; privileged or game-specific implementation paths are excluded.
- The substrate has zero LLM dependency and must remain usable without it.
- The consumer/substrate separation is mandatory at the product boundary; how packages are split to enforce it is a later design choice.

## Deferred design decisions

- Delivery depth and sequence within generation, matter behavior, and the public interface surface
- Concrete representation, meshing, streaming, persistence, and simulation mechanisms
- Exact package layout inside the Rust workspace boundary
- Performance budgets, target machines, and harness workloads if a harness is delivered
- Open fidelity choices left unresolved in the substrate seed (for example voxel scale and distant representation)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world executable a required current repository delivery, or only a permitted adjacent validation pattern?

**Proposed answer:** Permitted only—current product identity and scope stay the reusable substrate; a harness may exist as an adjacent validation artifact but is not committed as a product deliverable in this brief.

**If different:** Making the harness required adds a repository delivery obligation for an adjacent validation executable without changing substrate identity; it still must not import that harness’s controller, content, route, or acceptance metrics into product outcomes.

## Seed synthesis

- **README.md:** Names Moria as the GPU-resident voxel-world substrate Rust crate and frames the walkable-world executable as a separate consumer and validation harness.
- **docs/seeds/project-boundary.md:** Binding product boundary—substrate is the product; the actual game and named game layers are out of scope; any harness must use public interfaces; consumer separation is mandatory.
- **docs/seeds/product-one-seed.md:** Motivates validation through a walkable material world and dig/place proof; demo content, controls, hardware, and gates stay adjacent (harness delivery status: Q1).
- **docs/seeds/voxel-world-substrate.md:** Supplies purpose and outcome altitude for material worlds, deep-Z, natural look versus voxel truth, geology-first generation, and reusable layering for multiple future games.
