# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer foundation for natural-looking, fully mutable 3D worlds—not a game, demo character, or authored campaign.

## Purpose

Downstream games need one shared world of matter they can generate, stream, query, mutate, collide with, mesh for display, and persist—without each game re-implementing the material world. Moria exists so those games consume a stable substrate through public interfaces: voxel truth underneath a non-cube look, deep underground as first-class space, and mutation anywhere in the volume. It must stand alone with no dependency on LLM, combat, economy, or other game-policy systems.

## Product boundary

**In product:** the reusable substrate: world matter representation and mutation, generation of natural geology and surface worlds, meshing/view of voxel truth, collision against matter (not against a privileged render mesh), streaming and persistence of generated world plus edits, and the public APIs consumers use for those outcomes. Compatibility seams for higher layers may be designed where substrate requirements demand them; those layers are not implemented here.

**Adjacent, not identity:** a walkable-world executable may exist in-repo as a **validation harness**. It must use the same public interfaces an external game would use—no privileged or game-specific paths. Its controller, character, route, presentation, seed content, workloads, and performance gates are harness concerns, not the product’s identity.

**Out of repository / downstream consumers:** the actual game(s). Game rules and future System/LLM, spell, gas, combat, AI, and building *gameplay* layers are not part of Moria. Future titles (System ARPG, fortress/colony, descent roguelike, pure sandbox) consume Moria; they do not redefine its current scope.

## Required product outcomes

- **Material world truth:** any solid volume is mutable matter (destroy, move, place); nothing player-facing as “world” is decorative geometry outside that truth.
- **Natural-looking world from voxel truth:** the world reads as continuous terrain and geology (hills, surface dressing, underground), not as a cube aesthetic; the mesh/view is regenerated and non-authoritative.
- **Deep Z as content:** underground volume (caves, strata, descent) is first-class playable space, not a floor under a heightmap.
- **Generate, stream, edit, collide, persist:** consumers can obtain a generated region, stream it, dig/place through public verbs, collide against voxel occupancy, and save/load via generation-plus-edit-delta style persistence.
- **Reusable engine boundary:** same substrate supports multiple game genres above it; game policy (pricing, rules, agents, UI, content packs) stays above; consumers have no privileged access relative to each other.
- **Rust/wgpu ecosystem:** integrate as Rust crate(s) on portable GPU compute/graphics (wgpu-class), suitable for real-time world interaction.

## Future products and enabling implications

Future consumers include a System-driven ARPG, fortress/colony play, Moria-style descent, and sandbox modes. They motivate substrate capabilities (queryable matter, mutation verbs, deep geology, structural and fluid *hooks* over time) but own their own gameplay, UX, controllers, characters, authored content, presentation, and policy.

**Enabling implications (not committed roadmap):** the substrate should remain a viable foundation for those genres—natural surface worlds, honest dig/build volumes, and deep underground—without embedding any one game’s systems. Long-horizon matter behaviors (active fluids, fire/integrity simulation, building machinery, agent labor) may be enabled later; they are not current-product commitments merely because long-form seeds describe them.

## Non-goals

- Shipping a complete game, combat, stats, AI, or multiplayer product in this repository.
- Implementing System/LLM, spells, gas/pricing policy, or building-game UX/blueprints/work orders as Moria features.
- Treating the validation harness’s character, camera, demo route, seed postcard, or benchmark gates as the product definition.
- Making the substrate depend on any LLM or game-policy stack.

## Confirmed vision constraints

- Product identity is the **substrate**, not the game or harness presentation layer.
- Harness (if present) and external games share **public interfaces only**—no privileged substrate forks for demos.
- Substrate is **GPU-resident** and intended for real-time interactive worlds.
- Delivery form is **Rust crate(s)** (Cargo-separated from any harness); exact crate split is design, not vision.
- Load-bearing portability preference: **wgpu/WGSL**-class stack rather than a native-only GPU fork in core layers.
- Higher game layers (System, spells, gas, combat, AI, building policy) stay out of implementation here.

## Deferred design decisions

- First vertical-slice mechanism set, milestones, and how deep generation/matter/sim go before later phases.
- Meshing, storage, streaming, and sim algorithms; voxel scale and performance budgets.
- Exact public API shape, crate graph, and how far structural/fluid/vegetation object behaviors ship initially.
- Harness design: controller, content seed, platforms, and acceptance numbers (once harness delivery is settled).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world validation harness a **required current delivery** alongside the substrate, or only **permitted** as an adjacent artifact?

- **Proposed safe answer:** Required as a delivery obligation of the project, but **outside product identity**—it proves substrate outcomes via public APIs only; its controls, character, content, presentation, route, and performance gates stay harness-owned and out of Moria’s product scope.
- **If different:** If only permitted, current scope is substrate crates alone and no harness must ship; if required *and* product-shaped (character + curated demo world as product), identity shifts from engine substrate toward a demo application and this brief’s boundary section must be rewritten.

## Seed synthesis

- **README.md:** Names Moria as the GPU-resident Rust voxel-world substrate; frames the walkable executable as consumer/harness for core world capabilities, not a game layer.
- **docs/seeds/project-boundary.md:** Binding product boundary—substrate as Rust crate(s); game out of repo; harness non-privileged; game-policy layers excluded.
- **docs/seeds/product-one-seed.md:** Motivates first-slice proof points and harness-shaped demo concerns; informs outcomes and exclusions without transferring demo content/controls into substrate identity.
- **docs/seeds/voxel-world-substrate.md:** Long-horizon substrate vision (material world, natural look, deep Z, reuse across genres); supplies purpose and enabling implications, not a mechanism or roadmap commitment in this brief.
