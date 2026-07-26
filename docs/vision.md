# Project vision

## What we are building now

**Moria** — a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. Games and other consumers integrate it as an engine layer: matter, world generation, queries, mutation, and related world physics live here; game rules do not.

## Purpose

Moria exists so multiple games can share one material world foundation instead of each reimplementing voxel truth, diggable geology, streaming, and mutation. The substrate must stand alone: it needs no LLM, System, or game policy to operate. Downstream titles (sandbox, fortress, descent, ARPG) differ above the substrate; they share the same world engine underneath.

## Product boundary

**In product:** the reusable substrate and its public integration surface for external consumers.

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness. If present, it is a separate consumer of the substrate, not a game layer and not a privileged path into the engine. Whether that harness is a required current delivery remains open (see Q1). Its controller, character, demo route, authored seed content, presentation, platform gates, and benchmark workloads are harness-owned, not product scope.

**Downstream, not this product:** the actual game and repository-external titles that consume Moria.

**Explicitly outside this product:** game rules and the System, LLM, spell, gas, combat, AI, and building *game* layers. Compatibility seams may be designed where substrate needs demand them; those layers are not implemented here.

## Required product outcomes

- **Rust-integrated substrate.** Downstream consumers use Moria as a Rust crate (or small crate family) with a public API boundary: no consumer—including any validation harness—gets privileged or game-specific implementation paths into voxel truth.
- **Natural material worlds.** Generated regions read as continuous, natural surface and deep underground space; the voxel grid is authoritative matter, and rendered appearance is a regenerated view of that truth—not a separate decorative world.
- **Universal mutability.** Matter can be destroyed, moved, and placed throughout the volume; dig and place are first-class substrate operations that keep the world honest after change.
- **Deep vertical worlds.** Underground and deep-Z space are first-class content, not a thin floor under a heightmap.
- **Geology, scale, and memory of change.** Worlds generate as diggable geology; large regions stream and materialize without eager full-volume residency; edits and scars persist as the durable record of change.
- **Shared matter foundation.** The substrate owns reusable world capabilities—queries and mutation, structural response, fluids, interactable vegetation and objects, and related ambient matter behavior—so multiple game genres can sit above one engine without embedding game policy in the crate.

## Future products and enabling implications

Future consumers include an actual game outside this repository and genre variants motivated by the substrate seed (sandbox, fortress/colony, Moria-style descent, System ARPG). Moria enables those titles by providing shared matter, world, and mutation foundations. Their gameplay, UX, controllers, authored content, presentation, combat, economy, and policy remain consumer-owned. A first walkable demo slice may exercise a subset of substrate depth; that slice does not redefine product identity or drop substrate responsibilities into “later products.”

## Non-goals

- Shipping the actual game, its rules, or genre-specific gameplay in this product.
- Implementing System/LLM features, spells, gas/pricing policy, combat, or AI here.
- Implementing building *game* layers (work orders, fortress designation UX, economy) here; substrate dig/place and matter response stay in product.
- Treating harness-specific content, cameras, characters, demo routes, device targets, or performance thresholds as substrate promises.
- Making the substrate depend on an LLM or System to function.

## Confirmed vision constraints

- Product form is a Rust crate or small family of tightly scoped Rust crates.
- The world substrate is GPU-resident.
- The substrate stands alone with zero LLM/System dependency.
- Every consumer, including any validation harness, uses the same public interfaces; privileged harness-only engine paths are excluded.
- Game rules and System, LLM, spell, gas, combat, AI, and building game layers are out of repository scope (seams only where substrate requirements need them).

## Deferred design decisions

- Crate split, workspace layout, and internal module boundaries (beyond the consumer-boundary outcome).
- Delivery sequence and depth of first substrate slices (meshing strategy, generation pipeline stages, which matter systems ship when).
- Storage layouts, algorithms, voxel scale, streaming ring policy, persistence encoding, and API shape.
- Whether and how a validation harness is built, and all harness content, controls, platforms, and acceptance metrics.
- Multiplayer and other long-horizon integration postures beyond keeping the public command/query boundary clean.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current delivery** of this effort, or only a **permitted adjacent artifact** that design may use for validation?

*Proposed answer:* Permitted only—not a required delivery. Product identity stays the substrate; design may still plan a harness without a vision-level delivery mandate.

*If different:* A “required” answer commits the effort to ship an adjacent harness as current delivery while still keeping harness controls, content, and gates outside substrate identity. A “permitted only” answer leaves harness existence to design without a product delivery obligation.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world substrate consumed as a Rust crate, and separates the walkable-world executable as consumer/harness rather than game layer.
- **docs/seeds/project-boundary.md** — Fixes current product identity (Rust substrate crates), excludes the actual game and game/System layers from this repository, and binds the equal public-interface rule for any harness.
- **docs/seeds/product-one-seed.md** — Motivates a first walkable proof and dig/place honesty; its demo content, controller, platforms, and milestones are treated as adjacent-harness/first-slice detail, not product redefinition.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate-level outcomes (natural material worlds, deep mutability, geology, streaming, persistence, shared matter/physics foundation, zero LLM dependency) without importing mechanism inventory or game layers into this brief.
