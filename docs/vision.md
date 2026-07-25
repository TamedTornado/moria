# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation: natural-looking terrain whose truth is fully material and mutable, including deep underground, exposed through public interfaces for generation, matter, mutation, and queries. It is not a game, not a demo experience, and not a gameplay stack.

## Purpose

Multiple games need the same underlying claim—surface worlds that read as normal landscapes while remaining editable matter all the way down—without each title re-owning geology, mutability, and world truth. Moria exists so downstream games can share one substrate and keep rules, presentation, content, and policy above it.

## Product boundary

**Belongs here:** the reusable substrate and its public crate interfaces; engine-level responsibility for natural mutable voxel worlds (matter, world services, mutation/query seams) that games consume. A Cargo workspace may separate substrate crates from any adjacent harness.

**Does not belong here:** the actual game (separate downstream consumer, out of this repository); game rules; System/LLM, spell, gas, combat, AI, and building *layers* as implementations (compatibility seams only where substrate requirements demand them); harness- or game-owned controllers, characters, authored demo content, presentation, routes, workloads, and acceptance gates.

**Harness rule:** this repository may include a walkable-world executable, but only as a validation harness that consumes the same public interfaces available to an external game—never privileged or game-specific paths.

## Future products and enabling implications

Future consumers (not current product): a System-driven ARPG, a fortress/colony-style game, a Moria-style descent experience, and pure sandbox play.

High-level enabling implications for the substrate: worlds that look natural while remaining voxel-true; mutability everywhere, including deep Z; dig/place and related matter verbs as substrate capabilities; clean layering so gas pricing, game policy, and any LLM/System client stay outside the substrate.

## Non-goals

- Shipping the commercial game or implementing game systems (combat, AI, spells, gas economy, building UX, LLM System)
- Treating a validation harness’s character, controls, curated tour, or clip criteria as product identity
- Decorative non-material geometry as world authority; substrate-coupled LLM dependency

## Confirmed vision constraints

- Product identity is the reusable substrate, not the game; the game lives elsewhere
- Any in-repo walkable executable is harness-only and must use public substrate interfaces
- Substrate stands alone with zero LLM dependency; the System is a game-layer client if used at all
- Delivery form is Rust crate(s), with a consumer boundary from any harness that is not optional

## Assumptions proposed for approval

1. High-level outcomes already assigned to the substrate (readable natural surface, material truth, deep mutability, multi-game reuse) define current identity; mechanisms, slice inventory, platforms, and performance numbers remain downstream design.
2. Product One’s region, third-person traversal, and benchmark tables describe a harness scenario for proving the substrate, not additions to substrate scope.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required** current delivery of this repository, or only **permitted** as an adjacent consumer of the substrate?

**Proposed answer:** Permitted only—the committed product is the substrate crates and public interfaces; a harness may exist to validate them but is not mandatory for product completeness.

**If different:** Making the harness mandatory expands “done” to require shipping a playable validation executable alongside the crates, which changes repository delivery boundary (still without making the harness a game).

## Seed synthesis

- **README.md** — Named the product Moria; fixed identity as a reusable GPU-resident voxel-world substrate consumed as a Rust crate; stated the walkable-world executable is a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Binding boundary: substrate as crate(s); game out of repo; harness optional and public-interface-only; Cargo workspace as the concrete consumer split; game/System/LLM/spell/gas/combat/AI/building layers out of scope except compatibility seams.
- **docs/seeds/product-one-seed.md** — Motivated a walkable proof of “voxel truth that looks good”; supplied harness-shaped non-goals and demo ambitions; treated as adjacent validation/consumer material, not transferred controller, content, or acceptance detail into current product scope.
- **docs/seeds/voxel-world-substrate.md** — Supplied substrate purpose and outcomes (natural look, full mutability, deep Z, substrate-not-game, multi-consumer layering) and long-horizon enabling implications; design mechanisms and build order deferred downstream.
