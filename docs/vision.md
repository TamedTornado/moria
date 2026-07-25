# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate (or a small family of tightly scoped crates). It provides natural-looking, fully mutable volumetric worlds whose material truth can be generated, queried, mutated, and consumed by external games—without embedding those games’ rules, content, or presentation.

## Purpose

Downstream games need a shared foundation where the world *is* matter, not decorative geometry over a heightmap: continuous natural surface, deep underground as first-class space, and mutation that remains honest after dig and place. Moria exists so those games reuse one engine-layer world instead of each rebuilding geology, matter, and mutation seams.

## Product boundary

**This product owns** the reusable substrate: volumetric world identity, natural-looking material worlds with deep-Z as a first-class dimension, mutation and query seams that keep consumers off raw voxel storage, and GPU-resident matter as the runtime stance. If a walkable-world executable ships in-repo, it is a validation consumer of those public interfaces, not a privileged game path.

**Adjacent / not this product:** the eventual game repository and all game rules; System/LLM, spells, gas policy, combat, AI, and building/economy layers; harness- or game-owned controllers, characters, cameras, authored demo routes, presentation, and performance acceptance scenarios. Compatibility seams may be designed where substrate needs demand them; those layers are not implemented here.

## Future products and enabling implications

Future consumers named in the seeds include a System-driven ARPG, a fortress/colony builder, a descent-style adventure, and pure sandbox modes. They motivate substrate generality; they are not current deliverables.

Enabling implications for those consumers (high level only): a mutable material world that reads as natural terrain; geology-backed depth rather than a thin floor; public verbs/queries so agents and tools share one mutation path; and a consumer boundary that keeps pricing policy and game logic above the substrate. Gameplay loops, content, UI, and controllers stay with those products.

## Non-goals

- Implementing the game, System/LLM features, spells, gas metering, combat, AI, or building/work-order gameplay in this repository.
- Treating the validation harness’s character, route, content palette, or benchmark theater as the product identity.
- Making the substrate depend on an LLM or on any single game’s rules.

## Confirmed vision constraints

- Product identity is the reusable substrate crate stack; the actual game is a separate downstream consumer outside this product’s ownership.
- Any in-repo walkable-world executable must exercise the same public interfaces an external game would use (Cargo workspace separation is the intended enforcement shape; exact crate split is design).
- The substrate must stand alone with zero LLM dependency.

## Assumptions proposed for approval

1. **Substrate responsibility vs. demo motivation.** High-level duties for natural world generation, fully mutable matter (including deep Z), and consumer-facing mutation/query seams remain *current substrate* responsibilities even when a walkable demo or future game is what motivates them. Mechanism depth and first-slice inventory stay for design.
2. **Long-horizon matter systems are not silent roadmap.** Broader substrate capabilities described for later fidelity (richer fluids, integrity, ambient ecology, full building machinery, and so on) are enabling design horizon, not committed near-term vision scope merely because a seed inventories them.

## Questions for human review

**Q1.** Is an in-repo walkable-world executable a *required current deliverable*, or only a *permitted* validation consumer of the substrate?

- **Proposed safe answer:** Permitted and encouraged as an adjacent harness that proves public interfaces; the product we are building now remains the substrate crates alone.
- **If answered differently:** Making the harness mandatory expands current delivery to include a shippable validation app, but still must not import its controller, character, content, presentation, or acceptance gates into substrate scope unless separately decided.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate; positions the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding product-boundary correction—substrate crate(s) are the product; game is out of repo; harness may exist only via public interfaces; game/System/LLM/spell/gas/combat/AI/building layers are out of scope.
- **`docs/seeds/product-one-seed.md`:** Describes a first product-shaped walkable demo (region, character, dig/place proof, milestones, targets). Contributes the “undeniable mutable natural world” outcome and harness motivation; does **not** transfer demo controls, content, or gates into substrate identity pending Q1.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies substrate design goals and long-horizon matter/world responsibilities (natural look, mutability, deep Z, substrate-not-game, multi-game reuse, GPU-resident stance). Implementation inventories, algorithms, and open technical questions remain downstream design input, not vision commitments.
