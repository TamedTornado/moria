# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or small family of tightly scoped Rust crates. It is the material world layer for natural-looking, fully mutable voxel worlds—not a game, and not a presentation shell.

## Purpose

Moria exists so multiple games can share one honest world substrate: terrain that reads as a normal surface world, voxel truth that is mutable everywhere including deep underground, and public mutation and query interfaces that keep game rules above the matter layer. The product stands alone with no dependency on LLM or System features.

## Product boundary

**In product**
- The reusable substrate: geology-first world structure, material matter, natural (non-cubic) presentation of that matter, mutation and query surfaces, and the persistence and streaming responsibilities a live world consumer needs.
- An adjacent walkable-world executable only as validation of that substrate through the same public interfaces an external game would use (see Q1). A Cargo workspace boundary between substrate and harness is required whenever the harness exists.

**Out of product**
- The actual game and its rules, UX, controllers, characters, authored content, combat, AI, building gameplay, System/LLM, spells, gas policy, and related layers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.
- Harness-owned specifics: character controls, demo route, seed-world content, debug presentation, scripted benchmarks, platform and performance gates—unless human review makes the harness mandatory without absorbing those details into the substrate.

## Future products and enabling implications

Described future consumers include a System-driven ARPG, a Dwarf Fortress–style fortress game, a Moria-style descent experience, and a pure sandbox. They remain downstream products.

High-level enabling implications (not a committed inventory or roadmap): continuous 3D material worlds that look natural rather than cubic; dig and place as first-class material verbs; deep-Z geology and underground space as content; sparse, streamable, delta-persistent worlds; matter-coupled vegetation and static fluid bodies as world truth rather than decoupled props.

## Non-goals

- Shipping any full game mode, economy, combat, AI, spell/System/LLM stack, or building/fortress gameplay in this repository.
- Treating a validation harness as a game layer or giving it privileged paths around the public substrate API.
- Making the substrate depend on LLM or System features to function.

## Confirmed vision constraints

- Substrate, not game: game rules live above; the substrate provides matter, world structure, queries, and mutation through public interfaces.
- GPU-resident voxel world intended for reuse by external consumers as Rust crate(s).
- Walkable-world executable, if present, is validation only and must use the same public interfaces as an external game.
- Out of repository implementation scope: System, LLM, spells, gas, combat, AI, and building layers (seams only where required).

## Assumptions proposed for approval

1. Product-one details (region composition, character controls, demo milestones, performance numbers, platforms) describe how an adjacent harness may prove the substrate, not extra identity or scope for the substrate itself.
2. Long-horizon matter behaviors sketched for fortress/ARPG fantasies (advanced fluids, fire ecology, integrity, mechanisms) are enabling direction for substrate identity, not a committed near-term delivery list.

## Questions for human review

**Q1.** Is the walkable-world executable a **mandatory current delivery** of this repository, or only a **permitted** adjacent validation consumer?
*Proposed safe answer:* Permitted adjacent consumer; if included, it must exercise the substrate only through public interfaces and does not redefine the product as a demo game.
*If answered differently:* Making it mandatory expands current delivery to include a first-party harness executable (still not a game), so repository “done” includes shipping that consumer without transferring its controls, content, or acceptance scenario into substrate scope.

## Seed synthesis

- **README.md** — Named the product Moria; stated reusable GPU-resident voxel-world substrate as a Rust crate; positioned the walkable-world executable as a separate validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Locked current product to the substrate crate(s); excluded the actual game from this repository; permitted a validation harness only through public interfaces; required a workspace boundary when both exist; excluded game rules and System/LLM/spell/gas/combat/AI/building layers from implementation while allowing compatibility seams.
- **docs/seeds/product-one-seed.md** — Motivated a walkable natural region as the undeniable proof of a material world; kept dig/place as mutability proof; listed demo non-goals aligned with substrate-not-game; supplied harness-facing detail treated here as validation intent, not substrate identity expansion.
- **docs/seeds/voxel-world-substrate.md** — Contributed substrate goals (natural look, mutability everywhere, deep Z, reusable substrate, GPU residency); framed future game modes as consumers; assigned high-level world/matter responsibilities to the substrate without importing mechanism inventories; noted System as a game-layer client with zero substrate LLM dependency.
