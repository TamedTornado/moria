# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**: a Rust crate (or small family of tightly scoped crates) that external games consume. It provides a natural-looking, fully material world foundation—mutable matter, deep underground as first-class space, and generation that supports honest dig-down geology—not a game, demo title, or content package.

## Purpose

Games that need overworld-quality terrain *and* Minecraft-grade mutability with deep-Z play should not each reinvent the world engine. Moria exists so multiple future titles can share one substrate for matter, world structure, mutation, and related queries, while game rules, presentation, and content stay above it.

## Product boundary

**This product owns** the reusable world substrate and its public consumer-facing interfaces: the capability for natural-looking, continuous 3D mutable voxel worlds suitable for surface traversal and deep underground play.

**Adjacent / not this product:**
- The eventual game (or games) that sit on Moria—separate downstream consumers, not this repository.
- A walkable-world executable, if present: a validation harness that must use the same public interfaces available to any external game; not a privileged game layer and not ownership of game-specific UX, content, or policy.

**Explicitly not substrate work:** game rules; System/LLM features; spells, gas policy, combat, AI, and building/gameplay layers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Future products and enabling implications

Described future consumers include a System-driven ARPG, a fortress/colony-style builder, a Moria-style descent experience, and pure sandbox modes. They motivate the substrate; they are not current scope.

**Enabling implications (high level only):** the substrate should leave room for games that need natural surface worlds over true material underground, large sparse regions with streaming-friendly residency, mutation that stays authoritative over any rendered view, and pluggable game policy above shared world verbs and queries. Gameplay systems, controllers, characters, authored routes, presentation, and acceptance scenarios remain consumer-owned.

## Non-goals

- Shipping a finished game, ARPG, fortress mode, or System/LLM product in this repository.
- Owning combat, economy, AI agents, spell/gas systems, or building-as-gameplay.
- Treating a curated demo world, character controller, camera, or marketing clip as the product identity.

## Confirmed vision constraints

- Delivered as Rust crate(s) with a clear workspace boundary between reusable substrate and any validation harness.
- GPU-resident world substrate; must stand alone with zero LLM/System dependency.
- Any in-repo harness consumes only public substrate interfaces (no privileged game-only paths).

## Assumptions proposed for approval

1. **Product-one demo specifics are harness/validation intent, not substrate identity.** Character control, camera, curated region content, demo route, and numeric performance gates describe how one might prove the substrate—not what the substrate product *is*.
2. **Long-horizon substrate richness is enabling, not a committed current inventory.** Broader matter and world behaviors sketched for future fortress/ARPG consumers (multi-tier fluids, structural integrity, weather ecology, mechanisms, room semantics, and similar) record direction for later design; they are not a first-delivery checklist in this vision.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required current deliverable** of this repository, or only **permitted** as an adjacent consumer?

- **Proposed safe answer:** Permitted and appropriate for proving public interfaces; not required to define product completeness. Substrate identity stands without mandating a specific harness.
- **If answered differently:** Making the harness mandatory expands current delivery to “substrate plus a public-API harness that exists,” while still leaving harness controls, content, presentation, and performance gates out of substrate scope unless separately decided.

## Seed synthesis

- **`README.md`:** Named the product Moria; stated reusable GPU-resident voxel-world substrate as Rust crate; positioned walkable-world executable as separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding current boundary—substrate crate(s) are the product; game is out of repo; harness may exist only via public APIs; workspace split; game/System/LLM/spell/gas/combat/AI/building layers out of scope with optional compatibility seams only.
- **`docs/seeds/product-one-seed.md`:** Motivated a product-shaped proof (natural walkable region, mutation honesty, continuous 3D); supplied demo non-goals and first-slice motivation. Its controller, content palette, milestones, platforms, and performance numbers were treated as adjacent validation/design detail, not fused into current product scope (see Q1 and assumptions).
- **`docs/seeds/voxel-world-substrate.md`:** Anchored substrate purpose (natural look over voxel truth; mutability; deep-Z; substrate-not-game; multi-game reuse) and long-horizon enabling implications for future consumers; implementation inventory, algorithms, and build order deferred past vision altitude.
