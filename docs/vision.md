# Moria — Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). Downstream games consume it; this repository does not ship a game.

**Product One** is the first vertical slice that proves the product: a generated walkable world, exercised by a required validation harness, with dig/place as proof of materiality. Product One does **not** define the full product mandate. Capabilities deferred within Product One remain required outcomes of the current product (the substrate), delivered beyond that first slice.

The walkable-world executable is a **required validation consumer and outward-facing playable proof**, not an optional demo and not a game layer. It must consume the substrate through the same public interfaces available to an external game. Product One delivers a public artifact for its audience (including a downloadable playable proof); milestone posts and release sequencing are delivery process, not vision inventory.

---

## Purpose

Make one claim undeniable and reusable:

> This is not decorative geometry over a heightmap — it is a fully material, mutable voxel world that looks good, digs honestly, streams at scale, and behaves as matter — and it exists as a clean engine layer other products can build on.

The substrate exists so future games (ARPG with System/LLM, fortress/colony, descent roguelike, pure sandbox) start from a walkable material world rather than a whiteboard. Those games are **consumers**, not this product. Their gameplay, content, characters, and systems are out of scope; the substrate must still provide the matter, physics, generation, object, and persistence capabilities those consumers will need.

---

## Boundary

| In (current product: substrate) | Out (game-owned; not this repository) |
|---|---|
| Generation layer (geology, lazy materialization, POI metadata, material registry) | Game rules, combat, stats, entities beyond harness needs, AI |
| Matter layer (bricks, meshing, dressing, voxel objects, CA, fluids, fire/wetness, integrity, granular) | System / LLM layer, spells, gas policy, intent, pricing |
| Command-in / stale-mirror + events-out public surface for matter and physics; object-facing APIs | Building UI, blueprints, mechanisms, work orders, room economy as product features |
| Streaming rings; durable persistence of changed matter and object state | Game-authored weather, seasons, or growth content (substrate owns thin ambient drivers only) |
| Walkable validation harness and public playable proof that exercise the public API | Multiplayer as a shipped feature; native Metal fork of load-bearing layers |
| Compatibility *seams* where substrate requirements demand them | |

**Product One vs substrate (first slice vs product mandate):**

| Deferred or thinned in Product One | Still required of the substrate (beyond Product One) |
|---|---|
| Fluids beyond static bodies (tier 1) | Active fluid behavior (coarse flow and fine splash tiers as designed) |
| CA, fire, wetness rules not running | Fire/wetness CA and thin ambient-world drivers (time/seasons, weather effects, growth ticks, wetness and water-table changes, fire ecology) |
| Integrity and granular settle not running | Granular movement and structural failure |
| Object dynamics (felling / rigid conversion) stretch | Voxel-backed interactive objects and their dynamic lifecycle |
| Persistence as single-slot seed + deltas | Durable persistence of matter and object state across streaming and later runs |
| Script/API as dig/place + mirror queries only | Full commands-in / stale-mirror + events-out consumer boundary and object-facing surface |
| Weather, seasons, growth (fixed time-of-day enough for first slice) | Ambient drivers above as substrate outcomes, without game content authorship |

**Layering intent (product-level, not crate plan):** nothing above the matter layer touches voxels directly. Mutations enter through **commands**. Consumers observe a **deliberately stale/coarse mirror and events**, not direct synchronous voxel truth. That command/mirror lifecycle is part of the product architecture even on unified-memory development hardware. Product One ships the dig/place and mirror-query sliver first; the product mandate is the fuller matter-and-physics-facing surface, without importing gas, System, embedded scripting languages, mechanisms, or other game-owned behavior.

**Cargo workspace boundary** between reusable substrate and validation harness is expected; exact crate split is a later technical decision. The consumer boundary is not optional.

---

## Required product-level outcomes

Product success is judged by what an external consumer (and the harness) can rely on. Outcomes marked *first-slice* are required of Product One; the rest are required of the current product and may land after the first proof delivery.

### Core material world

1. **Natural-looking material world** — terrain that reads as a normal surface world (hills, forest, water, cliffs, caves); the voxel grid is truth, not the look. Smooth isosurface extraction; the mesh is a regenerated view, never authoritative.
2. **Mutable everywhere** — dig and place against voxel truth; cut faces read as cuts; collision against occupancy, not the render mesh. *(first-slice proof)*
3. **Deep Z is first-class** — continuous surface-to-underground traversal; geology (strata, caves, ore, aquifer bands) exists under the surface, not painted rock under a heightmap. *(first-slice proof)*
4. **Scales by sparsity and laziness** — homogeneous bricks, lazy materialization, streaming; a region large enough that raw voxels do not all fit in memory is the proof bed. *(first-slice proof)*

### Generation (full reusable asset, not one one-off region)

5. **Geological generation layer** — continent/climate-capable pipeline thinned only as needed for a curated first region; columns, strata, caves, ore, and related geology as designed. The generation layer is a reusable product asset.
6. **Lazy independent materialization** — bricks materialize on touch from seed + pipeline; idle world cost stays column/coarse maps + sentinels.
7. **POI metadata and authored placement** — generation emits placement metadata; the first slice exercises voxel-object placement and at least one authored-placement (stamp/prefab) path. Seed-world content checklists are evidence of the claim, not a content GDD imported into vision.

### Matter behavior and objects (substrate mandate; partial in first slice)

8. **Voxel-derived dressing synchronized with matter** — grass/clutter as pure functions of surface voxels and aggregates so dig, burn, and state changes never leave fake decoration behind. *(first-slice for scatter dressing)*
9. **Voxel-backed interactive objects** — trees, boulders, and similar interactables are objects with voxel identity (placement, registration, rendering in the first slice). **Dynamic lifecycle** (e.g. felling → rigid/debris → re-voxelize) is a substrate outcome, not globally optional; Product One may treat tree felling as stretch, but the product does not.
10. **Active material and fluid behavior** — static bodies first; active flow and fine splash as substrate fluid tiers beyond the first slice.
11. **Fire/wetness and thin ambient-world drivers** — post–Product One, the substrate provides thin ambient simulation so the surface world *behaves*: time/seasons, weather effects, growth ticks, wetness and water-table changes, and fire ecology — running on cheap aggregates and only concretizing to voxels in active range. Product One may use a fixed time-of-day convenience only. Game-authored weather, seasons, or growth *content* remains out of scope; the drivers and matter responses are substrate outcomes.
12. **Granular movement and structural failure** — granular settle and support/integrity failure as substrate outcomes beyond the first slice.

### Consumer surface, harness, durability, and proof

13. **Commands in; stale mirror and events out** — mutations enter only through commands; consumers observe a deliberately stale/coarse mirror plus events, never direct synchronous voxel truth. The GPU-resident substrate retains this command/mirror lifecycle even when developed on unified-memory hardware. Product One establishes the seam with dig/place and mirror queries; the product mandate is the fuller matter-and-physics-facing surface, without shipping game policy (gas, System, mechanisms).
14. **Required validation harness and public playable proof** — a generated walkable executable is both the validation harness and an outward-facing playable proof for Product One’s audience (including a downloadable artifact). It provides third-person traversal of a generated natural region against voxel truth, dig/place as materiality proof, and exercise of generation, objects, dressing, streaming, and persistence seams in scope for the slice.
15. **Responsive real-time operation as product outcome** — the substrate must support interactive frame rate under traversal and mutation, dig-to-remesh responsiveness without hitch on local carves, prompt cold-start into a walkable world, bounded resident memory under streaming, compact delta saves after heavy defacement, and exact restoration on load. Machine-specific numeric targets and workload scripts belong to design/measurement; the fused operational claim is product-level. Benchmarks that report numbers **with machine profile** remain part of the deliverable so substrate changes can regress against Product One and results stay comparable across hardware.
16. **Durable persistence** — truth as worldgen function + edit deltas; changed matter and object state survive streaming and later runs. Product One may ship a minimal single-slot form; cross-run durability of scars and object journals is a substrate outcome.
17. **Decision bed** — Product One answers open substrate choices (e.g. voxel size, LOD, object-layer scaling) with measurements, not guesses.

---

## Non-goals

- Shipping any game, campaign, characters, combat, AI agents, or economy.
- Implementing System/LLM, spell, gas, pricing, or intent systems in this repository.
- Building as gameplay in this repository (player building UI, blueprints as fortress features, mechanisms, work orders, room-ledger economy). Compatibility seams and stamp/prefab placement for world content remain in.
- Authoring a large hand-built world — curated generation parameters for a proof region are enough for Product One; the generation *layer* itself is not optional.
- Game-authored weather, seasons, or growth content (thin ambient drivers and matter responses are substrate outcomes beyond Product One; authored content is not).
- Multiplayer as a shipped feature (architecture may remain server-authoritative-ready).
- Native Metal fork of load-bearing layers; stay on portable GPU abstraction.
- Importing seed-world content checklists, character fantasy, milestone/release sequencing, or future game modes as current product scope.
- Encoding machine-specific performance numbers as vision commitments (the operational outcomes in §15 are in; the tables of thresholds are design/measurement).

**Not non-goals (clarification):** fluids beyond static bodies, fire/CA/wetness, integrity, granular settle, object dynamics, thin ambient-world drivers (time/seasons, weather effects, growth ticks, wetness/water-table, fire ecology), and durable object/matter persistence are **deferred within Product One**, not excluded from the product. Tree felling is stretch for the first slice only.

Future products and examples in the seeds (fortress toybox, System ARPG, Moria-style descent, DF hydrology, siege beasts, etc.) inform **why** the substrate must stay game-agnostic and material-first. Their gameplay, content, characters, and implementation are **not** current scope.

---

## Unresolved questions for humans

None that change product identity, purpose, or boundary. The seeds agree:

- **Moria** is the reusable substrate (project boundary + substrate design).
- **Product One** pins what is built first and how “done” looks for the first proof; it does not supersede the broader substrate mandate.
- Game-owned layers stay out; matter/physics/generation/object/persistence families stay in.

Open technical questions (voxel size 25 cm vs 12.5 cm, distant LOD strategy, object-layer capacity, fluid tier-2 fidelity, multiplayer scope statements) belong to design/measurement after this vision, not to vision approval.

---

## What each seed contributed

| Source | Contribution to this vision |
|---|---|
| **README.md** | Names the product (Moria), states crate + harness split, points at seeds. |
| **project-boundary.md** | Binding product identity: reusable substrate crates; game out of repo; harness must use public APIs; game/System/LLM/spell/gas/combat/AI/building layers out of scope except compatibility seams. |
| **product-one-seed.md** | Binding **first slice**: walkable generated proof, third-person harness, dig/place as materiality proof, full generation layer as reusable asset, partial matter layer, thin API sliver first, Product One-only non-goals (fluids/CA/integrity/object dynamics/weather-seasons-growth thinned for the slice). Authorizes performance outcomes as product spec (frame rate, dig-to-remesh, cold-start, bounded memory, compact deltas, exact restore) while machine numbers stay design/measurement; retains command/mirror architecture on unified-memory hardware; public artifact and downloadable playable proof for the audience. Seed-world content and numeric thresholds are *evidence of the claim*, not vision inventory. |
| **voxel-world-substrate.md** | Binding **substrate mandate**: design goals; geology-first gen; brick sparsity; smooth meshing; dressing synchronized with matter; voxel objects and dynamic lifecycle; fluid tiers; fire/wetness; thin ambient simulation (time/seasons, weather effects, growth ticks, wetness/water-table, fire ecology); granular and integrity; streaming and durable persistence; GPU-resident **commands in, stale mirror + events out** lifecycle; object-facing layering; substrate build order (for the substrate itself, not “later products only”). Future game examples filter **purpose**, not current scope of matter capabilities. |

Authority note: Product One pins sequencing and first-proof “done”; it does not demote the substrate specification to optional future reference. Both describe the current product at different altitudes (first slice vs full substrate).

---

## Status

This document is a **proposal**. Human approval freezes product identity, boundary, and required product-level outcomes for downstream design. Implementation detail, crate split, milestone engineering, benchmark thresholds, and seed-world content remain out of this document.
