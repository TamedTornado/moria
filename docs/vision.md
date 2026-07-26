# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). It is the matter, generation, meshing, mutation, query, streaming, and persistence foundation for later games — not a game itself.

The first product-shaped deliverable is **Product One: the Walkable World** — one curated generated region and a third-person character who can traverse it — shipped as a **validation harness executable**. That harness consumes the substrate only through the same public interfaces an external game would use. It exists to prove substrate claims (look, mutability, depth, performance), not to be a game layer.

---

## Purpose

Make one claim undeniable and reusable:

> This is not a heightmap with props. It is a fully material voxel world that reads as a normal natural landscape, is mutable everywhere, supports continuous deep-Z play, and is packaged so a separate game can build on it.

Success means future products (ARPG, fortress/colony, descent/sandbox, or pure tech demos) start from a walkable, benchmarked substrate instead of a whiteboard — and that audiences and developers can verify that claim with numbers and a downloadable demo.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Voxel substrate crate(s) and public API | The actual game(s) that will consume Moria |
| Generation, matter representation, meshing/dressing, dig/place verbs, mirror queries | Game rules, progression, combat, stats, economy |
| Streaming, lazy materialization, delta persistence | System / LLM layer, spells, gas policy, pricing |
| Walkable-world executable as **public-API validation harness** | AI agents, multiplayer live service, building/fortress gameplay |
| Compatibility *seams* where substrate requirements demand them | Implementation of semantic/game layers above the script/API seam |

A Cargo workspace (or equivalent) must keep the reusable substrate and the harness separated. The precise crate split is a technical-design decision; the consumer boundary is not optional. Nothing above the matter layer may touch voxels directly — all mutation and inspection goes through verbs and queries.

---

## Required product-level outcomes

These are the outcomes Product One must make true of the substrate (capabilities, not a feature checklist):

1. **Natural surface over voxel truth.** Rolling terrain, forest, river/lake, cliffs, caves, and light surface dressing that read as a normal world — while every visible solid is backed by mutable voxel matter (smooth isosurface extraction; mesh is a view, not authority).

2. **Mutable everywhere as proof.** Dig and place (debug keys) carve real tunnels with cut faces that look like cut earth; remesh is incremental and hitch-free at demo scale. Without this, the demo is indistinguishable from decorative terrain.

3. **Continuous 3D / deep Z.** One continuous run from canopy/cliff height into a walkable cave (~−40 m class depth) against voxel collision (not render mesh), with underground light and camera behavior sufficient for the claim.

4. **Geology-first generation, lazily materialized.** Curated seed parameters produce strata, caves, ore/aquifer honesty when dug, and POI stamp path exercised once — without loading the whole region as dense voxels. Homogeneous/sparse storage is load-bearing for memory and bandwidth.

5. **Public substrate boundary from day one.** Dig/place and mirror queries exist as engine API; the harness and any future game share that surface. Format room for later sim (state nibble, aggregates, fluid tiers) may exist; Product One does not run the full cellular/fluid/integrity stack.

6. **Performance and persistence credibility.** Targets are part of the product promise: ~60 fps at mid-tier / M4-class targets at demo resolution; dig-to-remesh within a couple of frames; cold start to walkable in seconds; region memory under streaming discipline; single-slot seed + delta save/load that restores exactly. A scripted benchmark (flythrough + carve) with machine profile is a deliverable so later changes regress against Product One.

7. **Portable GPU path.** wgpu/WGSL as the load-bearing graphics/compute stack (no native-Metal fork in core layers); design that remains valid on discrete GPUs (command/mirror style), with Apple GPU constraints (e.g. no 64-bit buffer atomics) respected.

High-level capabilities later games will need — full fluid tiers, structural integrity, fire/granular CA, rich building/blueprint/mechanisms, entity nav classes, multiplayer-ready verb authority — are **design context** for seams and data layout. They are **not** Product One scope unless selected by the Product One seed.

---

## Non-goals

- Implementing a game: combat, stats, entities beyond the player, AI, quests, economy, or mode-specific UI.
- System / LLM, spells, gas metering, intent, or any game-layer policy object.
- Building gameplay (blueprints as product, mechanisms, work orders, room designation) — stamp/prefab may be exercised once for substrate proof only.
- Full fluid simulation, weather/seasons/growth sim, fire ecology, structural integrity, granular settle, or tree felling/rigid conversion (stretch at most; not required for Product One done).
- Fluids beyond static bodies (tier-1 lakes/river surface; no flow sim).
- Rich persistence (versioning, multi-slot, cross-mode fortress reclaim loops) beyond seed + deltas / single save slot.
- Owning privileged or game-specific paths inside the harness that an external consumer could not use.

---

## Unresolved human questions

None that change product **identity**, **purpose**, or **boundary**. The seeds agree:

- **Product** = substrate crate(s).
- **First deliverable** = Product One walkable harness on a curated region.
- **Games and System** = out of this repository.

Open items left to design/measurement (not vision blockers): voxel size final call (25 cm baseline vs 12.5 cm), distant LOD strategy, object-layer scaling, later fluid-pressure fidelity, and when multiplayer enters scope statements. Product Two direction (fortress toybox vs ARPG/System, etc.) is explicitly future and does not redefine Moria.

If human intent differs — e.g. Product One *is* the product rather than a harness, or a named game mode is in-repo — that should be stated before design proceeds.

---

## Seed contributions

| Source | Role in this vision |
|---|---|
| **README.md** | Names the product (Moria), positions it as GPU-resident substrate crate, and frames the walkable executable as consumer/harness — not game layer. |
| **docs/seeds/project-boundary.md** | Binding boundary: reusable crate(s), separate game consumer, harness-only executable, workspace separation, out-of-scope game/System/building layers; seams allowed, implementations not. |
| **docs/seeds/product-one-seed.md** | Binding first milestone: product statement, non-goals, dig/place as proof, seed-region proof points, player/traversal claims, performance/dev-platform targets, milestone arc, and what Product One “buys.” Demo content (specific biome list, materials palette, ruin, tree species) is harness evidence, not permanent product identity. |
| **docs/seeds/voxel-world-substrate.md** | Architecture reference for the substrate’s long-horizon capabilities (look strategy, bricks/columns, geology pipeline, vegetation model, fluid tiers, integrity, building verbs, streaming/persistence, layering). Only the slice selected by Product One is required now; the rest informs seams and non-goals so future games are not painted into a corner. |
| **docs/seeds/README.md** *(index)* | Confirms Product One as binding implementation+harness scope and substrate doc as reference with selective adoption; restates operator boundary. |

**Omitted from current scope on purpose:** full substrate build order beyond Product One; System attachment patterns as features; fortress/ARPG/Moria-descent gameplay; viral stretch goals (e.g. timber) as commitments; concrete crate graph and kernel designs (downstream design).

**Conflicts:** None material. Product One’s demo richness sits under the boundary’s “harness, not game” rule; the substrate doc’s broader layers are reference, not a mandate to implement everything in Product One.
