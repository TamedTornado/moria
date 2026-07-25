# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer world of mutable matter that can look like a continuous natural landscape while remaining fully material underneath—not a game, not a character demo, and not a heightmap with props.

## Purpose

Future games need a shared foundation for deep, diggable, placeable worlds where the visible surface and the underground are the same material truth. Moria exists so those games can consume generation, matter, mutation, and queries without re-owning the world engine—and without embedding game rules in the substrate.

## Product boundary

**This product owns** the substrate: the reusable world/matter engine surface that adjacent and external consumers call through public interfaces. High-level substrate responsibilities include a natural-looking yet fully mutable material world, first-class deep underground extent, and the engine outcomes that make dig/place and related matter queries honest (the mesh is a view; material truth is authoritative).

**This product does not own** the actual game, game rules, or the future System, LLM, spell, gas, combat, AI, or building *gameplay* layers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

**Adjacent, not identity:** a walkable-world executable, if present in the repository, is a validation harness and separate consumer. It must use the same public interfaces available to an external game—no privileged or game-specific paths inside the substrate. Harness-specific controller, character, camera, authored demo content, presentation, route, workload, platform, and performance gates are not part of the substrate product identity.

## Future products and enabling implications

Downstream consumers (not current delivery) include a System/LLM ARPG, a fortress/colony-style builder, a descent-style adventure, and pure sandboxes. The substrate is meant to enable a material world that reads as normal landscape, mutability everywhere including deep Z, and clean layering so game policy stays above the engine. Gameplay, UX, controllers, authored content, presentation, and per-game policy remain consumer-owned.

## Non-goals

- Shipping a game, combat, entities/AI, System/LLM features, spells, gas economy, or building-as-gameplay in this product
- Treating the walkable demo’s character, content, or acceptance scenario as the product itself
- Implementing excluded game layers “for validation” inside privileged substrate paths

## Confirmed vision constraints

- Delivery form is the Rust crate ecosystem; intended consumers integrate as Rust dependents of the substrate.
- Adjacent consumers (including any in-repo harness) have no privileged access—only the public substrate interfaces an external game would use.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world **validation harness** a **mandatory current delivery**, or only **permitted** beside the substrate?

- **Proposed safe answer:** Permitted only. Current committed product is the substrate; a harness may exist and must stay a same-API consumer, but is not required for the product to be complete.
- **If answered otherwise:** Making the harness mandatory keeps product identity on the substrate but expands *repository delivery* to require an adjacent walkable consumer; it still must not import that consumer’s controls, content, presentation, or acceptance details into substrate scope.

## Seed synthesis

- **README.md** — Named the product Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and cast the walkable-world executable as a separate validation consumer, not a game layer; no extra binding requirements beyond that framing.
- **docs/seeds/project-boundary.md** — Settled current product identity (substrate crate(s)), repository exclusion of the actual game, public-interface-only harness rule, and explicit out-of-scope game layers; workspace packaging detail remains subordinate to downstream design.
- **docs/seeds/product-one-seed.md** — Motivated a product-shaped walkable proof and a first vertical slice of substrate-backed world outcomes; its controller, seed-world content, milestones, and performance gates are harness/consumer or design input, not fused into current substrate identity (see Q1).
- **docs/seeds/voxel-world-substrate.md** — Supplied the long-horizon substrate purpose (natural look over mutable voxel truth, deep Z, substrate-not-game) and future consumer set; compatible mechanism, layer, and milestone detail remains subordinate design source and does not expand current committed scope.
