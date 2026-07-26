# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world **substrate**, exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for matter, world generation, mutation, queries, and related world services—not a game and not a gameplay product.

## Purpose

Moria exists so multiple future games can share one stand-alone world foundation: a natural-looking surface over fully mutable voxel truth, with deep underground as first-class space, without coupling the world stack to any one game’s rules, content, System/LLM features, or presentation. The substrate must stand alone with zero LLM dependency.

## Product boundary

**This product owns:** the reusable voxel-world substrate and its public consumer surface—world matter as authoritative truth, geology-oriented generation with on-demand materialization, mutation and query verbs that are the only path to touch voxels, non-authoritative visual meshing of that truth, and the substrate-side capabilities that make streaming, edit persistence, collision against matter, and performance exercise possible for integrators.

**Adjacent, not identity:** a walkable-world executable *may* exist in-repo as a validation harness. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific implementation paths. Whether that harness is a required current delivery is open (**Q1**); until answered, treat it only as a permitted adjacent artifact, not as the product.

**Downstream / other products own:** the actual game(s); game rules; System/LLM behavior; spells, gas policy, combat, and AI; building *game* layers (player building UX, blueprint/work-order gameplay, mechanism entities as game systems, room/economy semantics); and any harness- or demo-owned controller, character, camera, authored route, dressing content, debug presentation, workload, or acceptance scenario.

**Explicit exclusion still binding:** game rules and the future System, LLM, spell, gas, combat, AI, and building layers are out of scope to implement here. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented in this product.

## Required product outcomes

- **Rust-integrable substrate.** Downstream games and tools consume Moria as crate(s) through a public API surface; adjacent consumers have no privileged access to matter.
- **Normal world, voxel truth.** The world reads as ordinary natural terrain (rolling ground, vegetation presence, static water bodies as material volumes) while remaining fully material—not a heightmap skin with non-matter props pretending to be the world.
- **Mutable everywhere.** Any voxel in play can be destroyed, placed, or otherwise changed through substrate verbs; dig/place is a substrate proof of matter, not a game feature set.
- **Deep Z first-class.** Underground volume, strata, and continuous vertical play space are real substrate content dimensions, not a decorative floor under a surface shell.
- **Generation as diggable geology.** Worldgen produces layered, dig-honest structure and materializes bricks on touch so large regions stay sparse until needed.
- **View vs truth.** Rendered surface geometry is a regenerated view of voxels; physical interaction and world queries run against matter, not against a saved or authoritative mesh.

## Future products and enabling implications

Described consumers—the System ARPG, a fortress/colony game, a Moria-style descent roguelike, and pure sandbox modes—are **future or external games**, not this product. They motivate reuse and clean layering; they do not import gameplay, content, controllers, characters, animation, or game policy into Moria.

High-level enabling implications supported by the substrate seeds (delivery depth and sequence are design, not vision commitments): richer fluid behavior beyond still bodies; structural integrity and granular settle; fire and ambient surface simulation; vegetation and clutter as matter-backed or matter-driven dressing; object-layer interactables; priced verb/policy injection (e.g. gas as a game-supplied policy object); and metadata/POI hooks so a future System can author placement and materials without generating geology. Multiplayer-readiness of a command/verb boundary is architectural intent only if design retains it—not a ship commitment here.

## Non-goals

- Shipping a playable game, ARPG, fortress mode, or descent roguelike in this product.
- Implementing System/LLM, spells, gas economy, combat, AI, or building game layers in this repository.
- Treating demo character control, free-orbit camera, authored “postcard” routes, or milestone marketing clips as product scope.
- Making the substrate depend on an LLM or game-layer policy to function.
- Expanding first-consumer slice limits into a permanent reduction of substrate identity (slice depth is design).

## Confirmed vision constraints

- Delivery form is a **Rust** crate or small family of tightly scoped Rust crates.
- The world stack is **GPU-resident** at the product level (matter and related world work live in that residency model).
- Any in-repo harness or external game uses the **same public interfaces**; no privileged harness paths.
- Substrate **stands alone** without LLM dependency.
- **Game and building layers are not implemented here**; seams only where substrate needs them.

## Deferred design decisions

- Exact crate split and workspace packaging (boundary intent is fixed; layout is design).
- Voxel resolution, brick layout, meshing algorithm choice, LOD, and object-layer scaling.
- Which substrate capabilities ship in the first validated vertical slice versus later substrate increments.
- Fluid tiers beyond still bodies, integrity, CA rules, weather, and related sim depth.
- Persistence encoding details, streaming ring policy, and benchmark/harness scenario design.
- Graphics/portability stack and any target-machine performance gates (not fixed by product identity seeds).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** of this repository, or only a **permitted** adjacent validation artifact?

- **Proposed safe answer:** Permitted only—product identity and success are the reusable substrate crates; a harness may be added to exercise public APIs but is not mandatory for the product to be complete.
- **If answered differently:** If required, repository delivery must include a harness that validates substrate capabilities through public interfaces only; product identity still stays the substrate (not the harness’s controller, content, or acceptance numbers). If permitted-only, crates alone can satisfy the product without shipping a walkable demo.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world substrate consumed as a Rust crate, and separates the walkable-world executable as consumer/validation, not a game layer.
- **docs/seeds/project-boundary.md** — Binds current product to the substrate crate boundary, forbids privileged harness paths, and excludes game/System/LLM/spell/gas/combat/AI/building layers from this repo.
- **docs/seeds/product-one-seed.md** — Describes a first walkable validation slice and demo proofs that motivate substrate capabilities; its controller, seed content, milestones, and device targets are adjacent-consumer detail, not substrate identity.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate purpose and outcome-level responsibilities (natural look over mutable voxels, deep Z, geology generation, matter/API layering, stand-alone reuse) that future games consume without becoming this product.
