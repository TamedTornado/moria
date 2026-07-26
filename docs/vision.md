# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer product for external games: it owns matter, world representation, mutation and query surfaces, and the operational services those need. It is not a game and not a game repository.

## Purpose

Moria exists so multiple future games can share one foundation for a **natural-looking surface world over fully mutable voxel matter**, including **deep underground** play. The voxel grid is material truth; presentation is a view of that truth. Game rules, content policy, and LLM-driven systems live above the substrate. The substrate must stand alone with **no LLM dependency**.

## Product boundary

**This product owns**

- The reusable voxel-world substrate and its public consumer interfaces.
- World/matter outcomes: generation of diggable geology, sparse large-world residency, mutability, meshed natural appearance as a non-authoritative view, streaming, persistence of generation-plus-edits, and interaction against voxel truth (including collision).
- Integration as Rust crates for external game consumers.

**Adjacent, not identity**

- A walkable-world executable **may exist** only as a validation harness for the substrate. Whether that harness is a required current delivery is **unresolved (Q1)**. If present, it must use the **same public interfaces** available to an external game—no privileged or game-specific implementation paths.

**Downstream consumers own**

- Actual games (System ARPG, fortress/colony, descent/roguelike, pure sandbox, and similar).
- Gameplay, UX, controllers, characters, cameras, authored routes and content, presentation policy, and game-specific rules and pricing.

**Explicitly not this product**

- Game rules and the System, LLM, spell, gas, combat, AI, and building layers (compatibility seams may be designed where substrate needs demand them; those layers are not implemented here).
- The actual game as a repository citizen.

## Required product outcomes

Downstream design must make these true:

1. **Natural material world** — Consumers can present continuous, natural-looking terrain (rolling ground, forests, rivers, cliffs, meadows and similar) while every visible solid is backed by mutable voxel matter. The look is not “decorative geometry outside the material world,” and the product is not a heightmap-with-props substitute.
2. **Mutable everywhere, deep Z first-class** — Matter can be destroyed, moved, or placed throughout the volume. Underground (caves, strata, buried material interest) is real content, not a shallow floor under a surface shell.
3. **Geology-capable, large-world residency** — Worlds are generated as diggable geology from seed and parameters, materializing work on demand so large regions remain tractable; idle or homogeneous volume does not force full raw residency.
4. **Public mutation and inspection** — Consumers change and query the world only through the product’s public interfaces. Rendered surfaces are views regenerated from matter; they are not the authoritative or sole saved truth.
5. **Operational services for live play** — The substrate supports streaming around activity, persistence as generation plus edit deltas, and collision/interaction against voxel truth—so external games (and any harness) can walk, edit, and return to a consistent world without owning engine internals.
6. **Reusable engine layer** — The same crate surface can underpin different game genres without baking any one game’s rules, economy, or LLM stack into the substrate.

## Future products and enabling implications

Future **consumers** (not this repo’s product) include a System/LLM ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style descent experience, and pure sandbox modes. They motivate a rich matter and query surface (for example deeper fluid behavior, structural honesty, interactable vegetation, and placement-heavy play) but do **not** pull gameplay, content, controllers, or those games’ first slices into current-product scope.

Enabling implication only: keep the public verb/query boundary and matter-truth model fit for those consumers later; do not implement their game layers here.

## Non-goals

- Shipping the actual game, its systems, or its content in this product.
- Implementing System/LLM, spells, gas policy, combat, AI, or building-game layers here.
- Treating a demo character, camera, curated postcard route, seed-world set-dressing list, or marketing milestone plan as product identity.
- Importing a harness’s chosen hardware, frame-time gates, or platform stack as the substrate’s product promise.

## Confirmed vision constraints

- **Rust crate consumption** — The product is delivered for Rust consumers (one crate or a small tightly scoped family).
- **GPU-resident substrate** — Core world/matter residency is GPU-resident as part of product identity.
- **No privileged adjacent paths** — Any in-repo validation consumer uses the same public interfaces as an external game.
- **Standalone substrate** — Zero LLM dependency; the System is a future client, not a substrate feature.
- **Out-of-scope game layers stay out** — Game rules and System, LLM, spell, gas, combat, AI, and building layers are not implemented in this product (seams only where requirements demand them).

## Deferred design decisions

- Internal crate split, APIs, and enforcement layout (beyond the consumer-boundary outcome).
- Representation and algorithm choices (voxel scale, meshing approach, sparsity encoding, streaming ring policy, persistence encoding).
- How deep each matter-simulation family goes in which delivery, and in what order.
- Shape of any validation harness (controls, content, scenes, benchmarks)—after Q1.
- Concrete performance targets and supported hardware matrices.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required current delivery** of this repository, or only a **permitted adjacent artifact**?

- **Proposed safe answer:** Permitted adjacent artifact only; if present, it must consume the substrate solely through public interfaces.
- **If answered “required”:** Delivery planning must include that executable as a repository deliverable, still outside product identity and still without transferring its controller, content, presentation, or acceptance numbers into the substrate’s scope.

## Seed synthesis

- **`README.md`** — Names Moria as a reusable GPU-resident voxel-world substrate for Rust game consumers and describes a walkable-world executable as a separate validation consumer of core world services, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binding identity and boundary: substrate as Rust crate(s); game out of repo; harness may exist only via public interfaces; game/System/LLM/spell/gas/combat/AI/building layers out of scope.
- **`docs/seeds/product-one-seed.md`** — Adjacent first walkable-demo consumer: motivates why dig/edit proof and live world services matter; its character, route, content palette, milestones, and machine/performance gates are not product scope.
- **`docs/seeds/voxel-world-substrate.md`** — Authoritative substrate outcome families (natural look over voxel truth, full mutability, deep Z, geology generation, matter/query/mutation engine layer, streaming and persistence) without making its mechanism inventory or future game features current delivery law.
