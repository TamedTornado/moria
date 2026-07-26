# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a **Rust crate** (or a small family of tightly scoped Rust crates). It generates, stores, streams, mutates, and exposes continuous volumetric worlds so external games can treat terrain, geology, and matter as one mutable truth—not a heightmap with decorative props.

It is not a game, not an ARPG shell, and not a fortress builder. Those are future or adjacent consumers of this substrate.

## Purpose

Game products that need deep underground play, freeform dig and place, and a natural-looking overworld share the same hard problem: a world that is fully material, continuous in three dimensions, and still readable as ordinary landscape. Moria exists so that problem is solved once, as a standalone engine layer with no dependency on LLM or game-rule systems, and so multiple games can sit on the same matter, query, and mutation foundation.

## Product boundary

**This product owns** the substrate: world generation as geology, sparse GPU-resident matter, smooth render views derived from voxel truth, mutation and query surfaces, surface dressing and voxel-backed world objects at the matter level, static fluid bodies where required for a truthful world, edit persistence, and streaming around activity.

**Adjacent, not identity:** a walkable-world executable may live in the same repository as a **validation harness**. If present, it must exercise the substrate only through the **same public interfaces** available to an external game—no privileged or game-specific implementation paths. The harness proves terrain generation, streaming, meshing, editing, collision against voxel truth, persistence, and performance; it does not define Moria’s product identity.

**Downstream / out of repo:** the actual game (or games). Game rules and the System, LLM, spell, gas, combat, AI, and building *game* layers are not implemented here. Compatibility seams may be designed where substrate requirements demand them; those layers remain consumer-owned.

**Consumer vs substrate:** gameplay, UX, character controllers, cameras, authored demo routes, presentation policy, pricing policy, and game-specific acceptance scenarios belong to consumers or the harness—not to the substrate’s identity—unless a later approved boundary moves them.

## Required product outcomes

A downstream design must make these product-level guarantees true:

- **Natural look, voxel truth.** The world reads as continuous natural terrain (hills, forest, water, cliffs, caves); the voxel field is authoritative for physics, queries, and mutation. The mesh (or other render view) is regenerated and never the save truth.
- **Mutable everywhere, deep Z first-class.** Any material volume can be destroyed or placed; underground strata, caves, and buried matter are real content, not a floor under a skybox.
- **Geology-first generation.** Worlds are produced as layered geology and voids (not painted rock under a heightmap), with lazy materialization so untouched volume stays cheap and large regions remain tractable under sparsity.
- **Public matter surface.** Consumers mutate and inspect the world only through substrate verbs and queries; nothing above the matter layer touches voxels directly. Adjacent harnesses and external games share that boundary.
- **Matter-consistent dressing and objects.** Surface clutter and interactable natural objects stay consistent with the material world (remove ground, remove grass; voxel-backed trees and rocks participate as matter, not as free-floating props).
- **Persist and stream as substrate.** Truth is generation plus edit deltas; activity-centered streaming keeps resident cost bounded while scars and edits remain reloadable.

## Future products and enabling implications

Described future consumers include a System-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style deep-descent experience, and pure sandbox modes. They are **not** current scope.

High-level enabling implications (not a roadmap commitment): richer fluid and fire simulation, structural integrity and granular settle, priced verb policy plugs, nav derived from matter, blueprint/stamp and mechanism seams, and multiplayer-ready command authority. These may be designed as substrate capabilities over time; their **gameplay, content, controllers, characters, and presentation stay consumer-owned**. A future consumer’s needs do not pull excluded game layers back into current Moria scope.

## Non-goals

- Shipping a playable commercial game, combat, stats, AI agents, or multiplayer product in this repository.
- Implementing System/LLM authorship, spells, gas economy, or building/fortress game systems here.
- Treating the validation harness’s character, camera, debug UX, curated postcard region, or marketing demo as the product itself.
- Making the substrate depend on an LLM to generate or run the world.
- Adopting a Minecraft cube aesthetic as the primary surface look.

## Confirmed vision constraints

- **Delivery form:** Rust crate (or small family of crates) for integration by Rust game consumers.
- **GPU-resident matter world** as a defining quality of the substrate.
- **Portable GPU stack intent:** load-bearing work stays on a portable graphics abstraction (wgpu/WGSL family), not a single-vendor native fork in core layers.
- **Strict consumer boundary:** harness and games use public interfaces only; no privileged substrate back doors.
- **Zero LLM dependency** for the substrate to stand alone.
- **Explicit exclusions:** game rules and System, spell, gas, combat, AI, and building layers are out of current product scope.

## Deferred design decisions

- Exact crate split and workspace layout (boundary is fixed; packaging is design).
- Voxel scale, LOD/impostor strategy, object-layer capacity limits, and fluid-simulation depth beyond static bodies.
- How much of the long-horizon matter suite (dynamic fluids, fire CA, integrity, granular settle, felling/rigid coupling) ships in the first design slice versus later substrate work.
- Harness-only concerns: demo region curation, controller feel, cameras, debug tools, scripted benchmarks, and numeric performance gates.
- Multiplayer deployment timing (architecture may stay server-ready without shipping netcode).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required deliverable** of this repository’s current product effort, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted and expected for proving the substrate, but **not** part of product identity; if required as delivery, the vision records a mandatory adjacent harness without absorbing its controls, content, presentation, or performance scenario into the substrate.
- **If answered differently:** “Required” adds an adjacent delivery obligation (still outside product identity). “Optional/absent” means design need not plan a walkable executable—only library-level validation—narrowing repository outcomes without changing the substrate itself.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident Rust voxel substrate and positions the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding boundary—substrate as product; game out of repo; optional harness on public APIs only; game/System/building layers out of scope.
- **`docs/seeds/product-one-seed.md`:** Motivates an early vertical proof (natural walkable region, dig/place honesty, partial matter slice) and supplies harness-shaped demo detail kept out of substrate identity pending Q1.
- **`docs/seeds/voxel-world-substrate.md`:** Full substrate intent—natural look over mutable voxels, deep Z, geology, matter/query layering, and multi-game reuse—used as outcome authority, not as a mechanism or roadmap catalog.
