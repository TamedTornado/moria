# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world **substrate**: a Rust crate (or small family of tightly scoped crates) that other products consume. It provides a natural-looking, fully material world whose appearance is a view of mutable voxel truth—continuous terrain and deep underground—not a heightmap with decorative props.

## Purpose

Games and tools need a shared foundation for worlds that read as ordinary landscape yet remain diggable, placeable, and honest all the way down. Moria exists so those consumers do not each reimplement geology-backed matter, mutation, and queries. It stands alone as an engine layer with no dependency on any game rules system or LLM.

## Product boundary

**In product:** the substrate—world matter, generation of natural regions from seed, mutation and query surfaces, and the public interfaces an external game would use. GPU-resident simulation of that world is in scope at the outcome level.

**Out of product (this repository):** the actual game; game rules; System/LLM, spells, gas, combat, AI, and building layers (compatibility seams only where substrate needs demand them—not those layers themselves).

**Adjacent, not the product:** a walkable-world executable, if present, is only a **validation harness**. It must call the same public APIs available to an external game. Harness-owned concerns—character control, camera, authored demo route/content, debug presentation, scripted workloads, and performance gates—do not become substrate scope merely because they prove the crate.

Workspace separation between reusable crates and any harness is a required consumer boundary; exact crate layout is downstream design.

## Future products and enabling implications

Downstream consumers (not built here) include a System-driven ARPG, a fortress/colony game, a descent-style adventure, and pure sandboxes. The substrate should remain game-agnostic so each can layer policy, content, and presentation above shared matter and verbs.

**Enabling implications (not committed roadmap):** durable edit/persistence and streaming so large regions stay tractable; structural and fluid behavior rich enough for building and hydrology games; vegetation and objects that participate in the material world; seams for priced verbs and external authors—without implementing game economy, agents, or LLM authorship in Moria.

## Non-goals

- Shipping a playable game, combat, stats, AI entities, or building/ fortification gameplay in this repository
- Implementing System/LLM, spell, gas, or intent layers
- Treating mesh, dressing, or demo visuals as authoritative world state
- Absorbing harness UX, content inventory, or acceptance scenarios into the substrate product identity

## Confirmed vision constraints

- Product identity is the reusable substrate, not the game or a game-layer demo
- Delivery form is Rust crate(s) for external consumption; validation must not use privileged paths denied to external games
- World reads as natural landscape while remaining fully mutable voxel truth, including deep-Z underground as first-class space
- Substrate is GPU-resident and independent of any LLM or game-rules stack

## Assumptions proposed for approval

1. **Harness ownership stays adjacent.** Even if a walkable harness is delivered in-repo, its controller, character, camera, seed-route content, debug presentation, and numeric performance gates remain harness concerns used to exercise public APIs—not features of the Moria product identity.
2. **Long-horizon substrate depth is enabling, not current commitment.** Fuller fluid, integrity, weather, and multi-mode persistence capabilities described in design seeds motivate the architecture but are not a bound current delivery list; first-slice depth is downstream design after this vision is approved.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required current deliverable** of this repository, or only **permitted** as an optional adjacent artifact?

- **Proposed answer:** Required as a thin validation harness that consumes public substrate APIs only—not as a content-rich “product one” game demo.
- **If different:** Permitted-only means success is crates and public interfaces alone; the repository need not ship any walkable executable, and product-one-style demo proof moves fully outside current delivery.

## Seed synthesis

- **`README.md`:** Named Moria as GPU-resident voxel-world substrate consumed as a Rust crate; framed the walkable-world executable as separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Bound current product to reusable substrate crates; excluded the actual game and rule/System/spell/gas/combat/AI/building layers; allowed a harness only if it uses public interfaces; required workspace-level consumer separation.
- **`docs/seeds/product-one-seed.md`:** Motivated natural generated world plus mutability proof and a walkable validation path; supplied non-goals aligning with game exclusion; its controller, seed content, milestones, and performance numbers stayed out of vision scope as harness/design detail.
- **`docs/seeds/voxel-world-substrate.md`:** Anchored design goals (natural look, mutability everywhere, deep-Z, substrate-not-game, GPU-resident, LLM-independent); listed future game consumers and high-level enabling directions without transferring gameplay or feature inventory into current product identity.
