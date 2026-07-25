# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer foundation for natural-looking, fully material, fully mutable voxel worlds—not a game and not a demo as product identity.

## Purpose

Future games and tools need a shared world layer where what you see is voxel truth: terrain can look continuous and natural while remaining diggable, placeable, and continuous in depth. Moria exists so consumers share one matter-true substrate instead of each reimplementing mutable worlds, geology, and mutation boundaries.

## Product boundary

**This product owns** the reusable substrate: world matter and generation outcomes, mutation and query surfaces usable by external consumers, and the high-level responsibility for a natural-looking presentation of voxel truth (mesh is a view; matter is authoritative). Compatibility seams may be designed where substrate requirements demand them.

**This product does not own** the actual game, game rules, or the System / LLM, spells, gas, combat, AI, or building-game layers. Controllers, characters, cameras, authored demo content, presentation polish, and game-specific policy remain consumer-owned.

**Adjacent:** a walkable-world executable may live in-repo only as a **validation harness**. It must consume the substrate through the same public interfaces available to an external game. Its specific controls, character, content, route, and acceptance theater are not substrate scope unless human review expands that boundary (see Q1).

Repository boundary: the real game is a separate downstream consumer, not part of this repository. A Cargo workspace separation between substrate and harness is required as a consumer-boundary expression; exact crate splits are downstream design.

## Future products and enabling implications

Described future **consumers** (not current product): a System-driven ARPG, a Dwarf Fortress–style fortress/colony mode, a Moria-style descent adventure, and pure sandbox tools. They motivate substrate reuse; their gameplay, content, UI, and policies stay out of Moria.

**Enabling implications** (substrate-owned outcomes that make those consumers viable, not a committed feature roadmap): fully mutable material worlds all the way down; deep-Z geology and voids as first-class space; natural surface worlds that still dig and scar honestly; clean verb/query boundaries so pricing, agents, and rules plug in above matter.

## Non-goals

- Shipping game systems (combat, economy, AI, spells, gas policy, building UX, LLM/System behavior) inside this repository.
- Making the substrate depend on an LLM or System to function.
- Treating harness-specific demo content, controllers, or marketing numbers as the definition of Moria.

## Confirmed vision constraints

- Product identity is the reusable substrate crate(s); the game lives elsewhere.
- Any in-repo walkable executable is a harness that uses only public substrate interfaces—no privileged game-only paths.
- The substrate must stand alone as an engine layer with zero LLM dependency.

## Assumptions proposed for approval

1. **Delivery depth is design, not identity.** Seeds describe both a long-horizon substrate surface and a thinner first validation slice. Vision treats high-level substrate responsibilities (natural-looking material worlds, free mutation, deep-Z) as product-owned; which sim systems run in the first engineering slice stays downstream.
2. **Harness details stay out of product scope.** Curated regions, character traversal, debug presentation, and numeric performance gates in the seeds inform later design and validation intent; they do not redefine Moria as a third-person demo product.

## Questions for human review

**Q1.** Is an in-repo walkable-world validation harness a **mandatory current deliverable**, or only a **permitted** adjacent consumer of the substrate?

- **Proposed answer (safe):** Permitted and expected as the primary validation path, but **not** required to define product completeness. “Done” for Moria is reusable substrate crates and public interfaces a game could consume; shipping a particular harness executable is helpful, not identity-defining.
- **If answered differently:** Making the harness mandatory expands repository delivery commitments to include a shippable validation binary as part of current scope, while still keeping its character, content, controls, and demo theater out of the substrate crate unless further expanded.

## Seed synthesis

- **`README.md`:** Names the product Moria; states GPU-resident voxel-world substrate as a Rust crate; walkable executable is a separate consumer/validation harness, not a game layer; points at seeds as preserved inputs.
- **`docs/seeds/project-boundary.md`:** Binding boundary—substrate is the product; real game is out of repo; harness must use public interfaces; workspace split expresses the consumer boundary; game/System/LLM/spell/gas/combat/AI/building layers are out of scope (seams only where substrate needs them).
- **`docs/seeds/product-one-seed.md`:** Motivates a product-shaped walkable proof of material world + mutability; lists non-goals aligned with excluding game systems; contributes validation-oriented first-slice intent, dig/place-as-proof, and demo/acceptance detail kept out of current product scope pending Q1 and downstream design.
- **`docs/seeds/voxel-world-substrate.md`:** Long-horizon substrate design goals—natural look over voxel truth, mutability everywhere, deep-Z, substrate-not-game reuse for multiple future titles, GPU-resident stance, generation-as-geology, and layering that keeps rules above matter; detailed mechanisms and open engineering questions deferred past vision altitude.
