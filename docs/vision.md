# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. Downstream games consume it as an engine layer for natural-looking, fully material, continuously three-dimensional worlds. It is not a game.

## Purpose

Moria exists so adventure, fortress, sandbox, and related games can share one material world foundation: mutable voxel matter as truth, presentation as a non-authoritative view, geology-backed generation, deep underground as real volume, and mutation and query surfaces that keep game rules above the substrate. The substrate must stand alone with no dependency on an LLM or any particular game ruleset.

## Product boundary

**This product owns** the reusable world substrate and its public Rust integration surface: material world representation, geology-oriented generation, non-authoritative meshing and matter-driven dressing, mutation and query APIs, and world-scale sparsity, streaming, and generation-plus-delta persistence so external games need not own a private world engine.

**Adjacent, not identity.** A walkable-world executable may exist as a validation harness that exercises terrain generation, streaming, meshing, editing, collision against voxel truth, persistence, and performance through the same public interfaces available to an external game (see Q1). That harness is not a game layer and does not define product identity.

**Not this product.** The actual game; game rules; System and LLM features; spells; gas and pricing policy; combat; AI; and building as a game layer. Gameplay, UX, character control, cameras, authored demo routes and content, presentation policy, and consumer-chosen platforms or acceptance gates belong to adjacent consumers or future products.

**Access rule.** Adjacent consumers have no privileged substrate paths. Compatibility seams may be designed where substrate requirements demand them; excluded layers are not implemented here.

## Required product outcomes

1. **Integrable substrate.** External games (and any harness) integrate only through public Rust crate interfaces for matter mutation and queries—not through privileged or game-specific voxel paths.
2. **Looks normal, is material.** Surface worlds read as continuous natural terrain, not heightmaps with decorative props, while visible solids remain voxel-backed material truth and the mesh is a regenerated view.
3. **Mutable everywhere.** Material volume can be destroyed, modified, or filled through substrate verbs; scars stay honest matter; physics and gameplay-facing checks run against voxel truth, not the render mesh.
4. **Deep Z is first-class.** Underground volume—caves, strata, buried structure—is continuous three-dimensional content space, not a floor under a skybox.
5. **Geology-first generation.** Worlds generate as independent, lazy-evaluable geology so dig-down encounters true layers and voids; homogeneous or untouched volume stays sparse without eager full-region materialization.
6. **Reusable under many games.** The same substrate serves multiple game styles by providing matter, physics-facing behavior, and query/mutation surfaces while leaving pricing, rules, and content authorship above it.

## Future products and enabling implications

**Future consumers (not current product):** a System-driven ARPG, a fortress/colony-style game, a descent-style experience, and pure sandbox modes. They own gameplay, content, controllers, characters, and presentation.

**Enabling implications** (motivated by substrate goals; delivery depth is design): richer fluid and structural behavior on matter, fire and ambient ecology on aggregates, and voxel-backed interactive objects with physics coupling—available for those consumers without implementing their game layers here.

## Non-goals

- Shipping the actual game or its rules in this repository
- Implementing System, LLM, spell, gas, combat, AI, or building game layers
- Folding harness-specific characters, cameras, demo routes, content palettes, or performance gates into product identity
- Depending on an LLM inside the substrate

## Confirmed vision constraints

- Delivery form is a Rust crate or tightly scoped Rust crate family for Rust game consumers
- Material world representation is GPU-resident
- Zero LLM dependency; substrate stands alone as an engine layer
- Any validation harness uses only public interfaces—no privileged game-specific paths
- Excluded game layers stay unimplemented here; seams only where substrate requirements demand them

## Deferred design decisions

- Internal crate split and packaging of the substrate family
- Capability depth and sequence for first versus later substrate slices
- Voxel resolution, distant presentation strategy, object-layer scaling, and fluid or integrity fidelity
- Performance targets, hardware tiers, and graphics-backend portability details
- How far command/mirror architecture is treated as multiplayer-ready in scope statements versus later work

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required adjacent delivery** of the current Moria effort, or only **permitted** alongside the substrate crates?

*Proposed answer:* Permitted only—product identity and completeness center on the reusable substrate crates; a harness may exist to validate public interfaces but is not mandatory for the product to be complete.

*If different:* Making the harness mandatory adds an adjacent deliverable commitment (still not game identity) so “done” depends on a walkable consumer artifact; forbidding it in-repo would remove the allowed validation executable from this repository’s scope.

## Seed synthesis

- **README.md:** Names Moria as the GPU-resident Rust-crate substrate and classifies the walkable-world executable as a separate validation consumer for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md:** Fixes current identity on the reusable substrate, excludes the game and listed game layers, and requires public-interface-only harness access.
- **docs/seeds/product-one-seed.md:** Describes a first walkable demo consumer and proof scenario; motivates mutability and validation concerns without transferring demo content, controls, or platform gates into product identity.
- **docs/seeds/voxel-world-substrate.md:** Supplies substrate design goals—natural-looking material worlds, mutability, deep Z, game-agnostic layering, GPU-resident matter—and long-horizon consumer implications at outcome altitude.
