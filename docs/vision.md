# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped crates). It is the material foundation for natural-looking, fully mutable 3D worlds—not a game, not a character demo, and not a presentation layer for a particular title.

## Purpose

Games and tools need a shared world truth: terrain and underground that read as continuous nature, remain mutable anywhere, and stay queryable and editable without baking game rules into the engine. Moria exists so multiple downstream products can sit on one honest matter world instead of each rebuilding geology, mutation, and deep-Z from scratch. The substrate must stand alone with **no LLM or game-system dependency**.

## Product boundary

**In product:** the reusable world substrate and its public interfaces—generation of geological natural worlds, material truth and mutation, and the engine-facing surface that external consumers use to stream, query, edit, and present that world. Presentation mesh is a derived view of voxel truth, not authority.

**Out of product / adjacent:**
- The actual game (ARPG, fortress, descent, sandbox rules) is a separate downstream consumer and is not this repository.
- A walkable-world executable, if present, is only a **validation harness**: it must use the same public substrate APIs available to an external game, with no privileged game paths.
- Gameplay, UX, controllers, cameras, authored demo routes/content, characters, combat, AI, spells, gas policy, building-as-gameplay, and System/LLM layers belong to consumers—not Moria—except where a consumer is only proving substrate interfaces.

## Future products and enabling implications

Described future consumers (System ARPG, DF-style fortress/colony, Moria-style descent, pure sandbox) are **not** current product. High-level enabling implications the substrate is meant to support, without owning their content or policy:
- Natural surface worlds over continuous deep-Z material truth.
- Everywhere mutability and placement so dig/build and underground play are honest.
- Clean layering so game rules, economy, and agents live above matter, queries, and priced verbs.

Compatibility seams may be designed where substrate requirements demand them; those game layers are not implemented here.

## Non-goals

- Shipping a full game, combat, stats, AI, spells, gas metering, weather/seasons as product features, or System/LLM integration inside Moria.
- Owning consumer controllers, cameras, characters, demo narratives, or acceptance scenarios as product identity.
- Treating validation-harness content, presentation, workloads, or performance gates as substrate scope without an explicit boundary answer.

## Confirmed vision constraints

- Product identity and repository boundary: reusable substrate crate(s); game is external.
- Any in-repo walkable executable is a harness that consumes only public substrate interfaces (Cargo workspace consumer boundary).
- Substrate stands alone with zero LLM dependency; game rules live above it.

## Assumptions proposed for approval

1. **Product One’s seed world, character, camera, debug palette, milestones, and numeric gates** describe a validation-consumer scenario and proof points for the substrate—not an expansion of Moria’s product identity into a game or demo product.
2. **Long-horizon matter behaviors** motivated in the substrate seed (richer fluids, integrity, fire ecology, mechanisms, multiplayer readiness) shape identity as an enabling world engine, not a committed current feature inventory or roadmap.

## Questions for human review

**Q1.** Is a walkable-world executable a **required current deliverable** of Moria, or only a **permitted** adjacent validation consumer of the substrate crate?

- **Proposed safe answer:** Permitted, not mandatory. If built, it must exercise public APIs only; its controller, content, presentation, route, and gates are not substrate scope.
- **If different:** Making the harness mandatory keeps substrate identity but adds a required adjacent artifact to current delivery; importing its player, world tour, or acceptance numbers into product scope would redefine Moria as a demo product rather than a reusable engine.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident voxel-world substrate crate; walkable executable is a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Binding correction: product is the reusable substrate; game is out of repo; harness may exist but only via public APIs; System/LLM/spell/gas/combat/AI/building layers out of scope (seams only).
- **docs/seeds/product-one-seed.md** — First vertical-slice / demo framing (curated region, third-person run, dig proof, numbers, milestones). Used as motivation and possible harness scenario; detailed content, controls, and gates not imported as current substrate scope pending Q1.
- **docs/seeds/voxel-world-substrate.md** — Substrate purpose and design goals (natural look, mutability, deep-Z, substrate-not-game, GPU-resident layering); future game modes as consumers. Mechanisms and inventories deferred to design; high-level enabling outcomes retained.
