# Moria — Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). Downstream games consume it; this repository does not ship a game.

A walkable-world executable may ship with the substrate, but only as a **validation harness and product-shaped demo**. It must use the same public interfaces available to an external consumer — never privileged or game-specific paths.

The first deliverable that proves the product (**Product One**) is: one curated generated region, smooth voxel terrain that reads as a natural world, a third-person character who can traverse surface and deep underground against voxel truth, and dig/place as proof that the world is fully material matter — not a heightmap with props.

---

## Purpose

Make one claim undeniable and reusable:

> This is not decorative geometry over a heightmap — it is a fully material, mutable voxel world that looks good, digs honestly, and streams at scale — and it exists as a clean engine layer other products can build on.

The substrate exists so future games (ARPG with System/LLM, fortress/colony, descent roguelike, pure sandbox) start from a walkable material world rather than a whiteboard. Those games are **consumers**, not this product.

---

## Boundary

| In | Out |
|---|---|
| Generation, matter (bricks, meshing, dressing, static water bodies), and a thin mutation/query API | Game rules, combat, stats, AI, economy |
| Walkable harness that exercises the public API | System / LLM layer, spells, gas policy, intent |
| Persistence as seed + edit deltas; streaming rings | Building UI, blueprints, mechanisms, work orders (as product features) |
| Compatibility *seams* where substrate requirements demand them | Fluids beyond static bodies; fire/CA/integrity/granular settle as running systems |
| | Weather/seasons/growth sims; multiplayer as a shipped feature |

**Layering intent (product-level, not crate plan):** nothing above matter touches voxels directly; mutation and queries go through a verb/query boundary. That boundary is the sandbox, multiplayer-readiness, and reuse seam — even when only dig/place and mirror queries exist at first.

**Cargo workspace boundary** between reusable substrate and validation harness is expected; exact crate split is a later technical decision. The consumer boundary is not optional.

---

## Required product-level outcomes

Product success is judged by what an external consumer (and the harness) can rely on:

1. **Natural-looking material world** — rolling terrain, forest, water, cliffs, caves; the voxel grid is truth, not the look (smooth isosurface extraction; mesh is a regenerated view).
2. **Mutable everywhere** — dig and place work against voxel truth; cut faces read as cuts; collision is against occupancy, not the render mesh.
3. **Deep Z is first-class** — continuous surface-to-underground traversal; geology (strata, caves, ore, aquifer bands) exists under the surface, not painted rock under a heightmap.
4. **Scales by sparsity and laziness** — homogeneous bricks, lazy materialization, streaming; a region large enough that raw voxels do not all fit in memory is the proof bed, not a stretch goal.
5. **Reusable public surface** — generation and matter capabilities exposed so a separate game can consume the crate without forking substrate internals; the harness validates that claim.
6. **Credible performance and persistence** — walkable frame targets on the stated dev class of hardware; incremental remesh after carve without hitch; cold start and memory under sparsity design; save/load as seed + deltas that restore exactly.
7. **Decision bed** — Product One answers open substrate choices (e.g. voxel size, LOD, object-layer scaling) with measurements, not guesses.

---

## Non-goals

- Shipping any game, campaign, characters, combat, or AI agents.
- Implementing System/LLM, spell, gas, pricing, or intent systems.
- Full fluid simulation, fire ecology, structural integrity, granular settle, or weather/growth as running Product One systems (format and seams may anticipate them; they do not run).
- Tree felling / rigid conversion as a required outcome (stretch only if cheap).
- Building as gameplay (blueprints, mechanisms, room detection, labor).
- Authoring a large hand-built world — one curated *seed* that generates a proof region is enough.
- Persistence beyond single-slot seed + deltas; multiplayer shipping; native Metal fork of load-bearing layers.

Future products and examples in the seeds (fortress toybox, System ARPG, Moria-style descent, DF hydrology, siege beasts, etc.) inform **why** the substrate must stay game-agnostic and material-first. Their gameplay, content, characters, and implementation are **not** current scope.

---

## Unresolved questions for humans

None that change product identity, purpose, or boundary. The seeds agree: Moria is the substrate; games are downstream; Product One is the binding first slice; the broader substrate doc is architecture reference for later milestones.

Open technical questions (voxel size 25 cm vs 12.5 cm, distant LOD strategy, object-layer capacity, fluid tier-2 fidelity, multiplayer scope statements) belong to design/measurement after this vision, not to vision approval.

---

## What each seed contributed

| Source | Contribution to this vision |
|---|---|
| **README.md** | Names the product (Moria), states crate + harness split, points at seeds. |
| **docs/seeds/README.md** | Priority among seeds: Product One is binding for the first milestone; substrate architecture doc is reference filtered by Product One; boundary doc is operator clarification. |
| **project-boundary.md** | Binding product identity: reusable substrate crates; game out of repo; harness must use public APIs; game/System/LLM/spell/gas/combat/AI/building layers out of scope except compatibility seams. |
| **product-one-seed.md** | Binding first product slice: walkable generated region, third-person proof, dig/place as proof of materiality, performance/persistence credibility, explicit non-goals, and which generation/matter/API layers ship first. Seed-world content (specific biome checklist, material list, milestone order) is *evidence of the claim*, not a content GDD imported wholesale into vision. |
| **voxel-world-substrate.md** | Architecture and long-horizon capabilities (geology-first gen, brick sparsity, smooth meshing, object dressing, fluid tiers, integrity, building verbs, streaming/persistence, crate layering). Used here for **purpose and required capabilities**; full build order, CA/fluids/integrity/building, and future game modes remain reference for later products, not current product identity. |

---

## Status

This document is a **proposal**. Human approval freezes product identity and boundary for downstream design. Implementation detail, crate split, and milestone engineering remain out of this document.
