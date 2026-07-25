# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is the material foundation for natural-looking, fully mutable 3D worlds—not a game, not a demo title, and not a presentation or control layer for any particular experience.

## Purpose

Downstream games and tools need a shared world truth: surface and underground that read as continuous nature, remain mutable anywhere, and stay queryable and editable without baking game rules into the engine. Moria exists so multiple products can sit on one honest matter world instead of each rebuilding geology, mutability, and deep-Z from scratch. The substrate must stand alone with **no dependency on any game system or LLM**.

## Product boundary

**In product:** the reusable world substrate and its public interfaces—the engine-facing surface that lets consumers generate, stream, query, mutate, and present a geological natural world whose authoritative truth is material voxels. Derived presentation is a view of that truth, not authority.

**Adjacent, not identity:** a walkable-world executable, if present, is only a **validation harness**. It must consume the same public interfaces available to an external game, with no privileged or game-specific paths. Its controller, character, camera, authored region, route, presentation, workloads, and acceptance gates are not Moria’s product identity.

**Out of product:** the actual game; game rules; System/LLM, spell, gas, combat, AI, and building layers (compatibility seams only where substrate requirements demand them). Gameplay, UX, controllers, authored content, and game-specific policy remain consumer-owned.

## Future products and enabling implications

Described future consumers—a System-driven ARPG, DF-style fortress/colony play, Moria-style descent, and pure sandbox—are **not** current product. High-level enabling implications the substrate is meant to support, without owning their content or policy:

- Natural surface worlds over continuous deep-Z material truth.
- Everywhere mutability so dig, place, and underground play can be honest at the consumer layer.
- Clean separation so game rules, economy, agents, and presentation live above matter, queries, and mutation.

## Non-goals

- Shipping a full game, combat, stats, AI, spells, gas policy, weather/seasons as product features, or System/LLM integration inside Moria.
- Owning consumer controllers, cameras, characters, demo narratives, curated seed content, or acceptance scenarios as product identity.
- Implementing building, fortress, or semantic game layers here beyond optional seams the substrate itself needs.

## Confirmed vision constraints

- Product identity and repository boundary: reusable substrate crate(s); the actual game is a separate downstream consumer and is not this repository.
- Any in-repo walkable executable is a harness that uses only the public substrate interfaces available to an external game—no privileged access.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world executable a **required current deliverable** of this repository, or only a **permitted** adjacent validation consumer of the substrate?

- **Proposed safe answer:** Permitted, not mandatory. If built, it exercises public APIs only; its controls, content, presentation, route, and gates stay outside substrate identity and scope.
- **If different:** Making the harness mandatory keeps substrate identity but adds a required adjacent artifact to current delivery. Importing its player, world tour, or acceptance numbers into product scope would redefine Moria as a demo product rather than a reusable engine.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident voxel-world substrate crate and positions the walkable executable as a separate consumer/validation harness, not a game layer; delivery detail remains subordinate.
- **docs/seeds/project-boundary.md** — Binding correction: product is the reusable substrate; game is out of repo; harness may exist only via public APIs; System/LLM/spell/gas/combat/AI/building layers out of scope (seams only); packaging mechanics remain subordinate design input.
- **docs/seeds/product-one-seed.md** — Motivates a first walkable validation story and proof that the world is mutable material, not decorative terrain; controller, seed content, milestones, and numeric gates are harness/consumer input for downstream design, not fused product scope pending Q1.
- **docs/seeds/voxel-world-substrate.md** — Supplies long-horizon substrate purpose (natural look, everywhere-mutable, deep-Z, multi-game reuse, GPU-resident, substrate-not-game); mechanism inventories, open technical choices, and build-order detail remain subordinate design input.
