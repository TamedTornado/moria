# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical
design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**: a Rust crate
or small family of tightly scoped Rust crates that owns matter, generation,
meshing, mutation, streaming, and persistence of a continuous 3D material
world.

That substrate is the product of this repository. **Product One** (“the
walkable world”) is its first delivery slice: the generation and matter depth
needed to host one curated generated region, plus the public interfaces a
consumer needs to traverse, query, and mutate that world as proof. Product
One limits *what is built first*; it does not redefine the product as a demo
game.

The repository may ship a **walkable-world executable**. That binary is a
**separate consumer and validation harness**. It must use the same public
interfaces available to an external game; it is not a game layer and owns no
privileged or game-specific implementation paths. Third-person control,
camera, traversal presentation, and debug presentation live in that harness,
not as product-owned features.

Future games (ARPG, fortress/colony, descent/roguelike, pure sandbox) and
higher layers (System/LLM, spells, gas policy, combat, AI, building gameplay)
are **downstream consumers**, not this product.

---

## Purpose

Make one claim undeniable and reusable:

> This is not a heightmap with props — it is a fully material world that
> reads as a normal landscape, remains mutable everywhere, and is clean
> enough that many games can sit on the same crate stack.

The substrate exists so later products start from a walkable, diggable,
streamed world with measured performance — not from a whiteboard. Product
One proves that claim with a curated generated seed and a public-API
harness: dig and place are *proof of matter*, not game systems.

---

## Boundary

| In product (substrate) | Adjacent (not product-owned) | Out of product |
|---|---|---|
| Generation, matter storage, meshing/dressing, dig/place and related matter verbs, collision against voxel truth, streaming, delta persistence | Walkable-world **validation harness**: third-person controller, camera, traversal presentation, debug controls — consumes public APIs only | Game rules, combat, stats, AI, entities beyond harness needs |
| Public script/API surface (verbs + mirror queries): **nothing above the matter layer touches voxels directly** | First-slice validation consumer that exercises those interfaces | The System / LLM, spells, gas metering, pricing policy |
| Compatibility *seams* only where substrate requirements demand them | — | Building UI, blueprints-as-gameplay, work orders, mechanisms-as-play, room-assignment behavior |
| Substrate outcome families beyond the first slice (fluids tiers, integrity, granular response, ambient material state, interactive objects, etc.) as later deliveries of **this same product** | — | Implementing those game layers or importing their content here |

**Cargo workspace** separates reusable substrate crates from the validation
harness. The precise crate split is a technical-design decision; the
consumer boundary is not optional.

**Substrate vs. first slice.** The substrate owns a broad capability envelope
over the life of the product (see required outcomes). Product One explicitly
excludes from the *first* delivery: fluid flow beyond static bodies, CA
(fire, granular settle, integrity), felling/rigid conversion of voxel
objects, weather/seasons/growth sims, embedded scripting language, and rich
persistence. Format and layering may anticipate later substrate work without
shipping it in Product One. None of that imports System, game rules, AI, or
building-gameplay behavior across the project boundary.

---

## Required product-level outcomes

Outcomes the product must deliver — not a feature list. Outcomes marked
**later substrate** remain required of this product after the first slice;
they are not optional “support” via formats alone, nor are they Product One
must-ships.

### Substrate (enduring product)

1. **Substrate as crate(s).** A reusable Rust package boundary inside a
   **Cargo workspace**, with a public mutation and query surface. External
   consumers (including the in-repo harness) share that surface; no back
   doors into voxel storage. Nothing above the matter layer touches voxels
   directly.

2. **Universally mutable, movable, placeable matter.** Any voxel can be
   destroyed, moved, or placed; the mesh is a regenerated view, never
   authoritative truth. Dig/place (and incremental remesh) are first-class
   so cuts read as cut matter.

3. **Material world that reads as landscape.** Smooth isosurface terrain
   over voxel matter; surface dressing and voxel-backed objects so the world
   does not look like a cube grid while remaining diggable and continuous in
   3D, including underground depth.

4. **Interactive voxel-backed objects.** Trees, boulders, and similar matter
   that can burn, break, or block participate as objects in the material
   world (placement, registration, rendering in the first slice; fuller
   interaction such as felling/rigid conversion as later substrate work).

5. **Material physics and simulation families (later substrate).** Multi-tier
   fluid behavior; structural integrity and cave-in response; granular
   settle; ambient material state (weather/time/fire ecology on aggregates
   as designed). Product One ships static water only and does not run CA;
   these families remain substrate-owned outcomes for later slices.

6. **Geology-first generation with sparsity.** Lazy materialization from a
   seedable pipeline (columns, strata, caves, materials, POI metadata);
   homogeneous bricks so dense full-volume residency is not required.

7. **Mutation-safe queries, streaming, and durable change.** Consumers query
   and path against matter through the public surface under mutation;
   streaming rings keep active and cold regions bounded; truth is worldgen
   function plus edit deltas so scars restore exactly without full-volume
   dumps as the save format.

8. **Portable GPU residency and measured interactive performance.** Matter
   and meshing reside on a portable GPU substrate; interactive frame rate,
   bounded memory under streaming, responsive mutation and cold start, and
   exact save/load restoration are product outcomes. Credibility includes
   regression evidence with **machine profile** so results stay comparable
   across hardware — not a specific vendor API or device rule set as product
   identity.

### Product One — fused validation delivery (first slice)

9. **One curated generated seed as consumer-visible proof.** Not an authored
   world: generation parameters are curated so a single region reliably
   presents, together, the proof points that make the material-world claim
   undeniable:
   - credible surface landscape and vegetation at density;
   - carved static-water geography (river channel and lake body);
   - exposed and underground geology (strata, walkable cave depth);
   - ore and aquifer evidence underground;
   - voxel objects (trees, micro objects such as boulders);
   - a stamped ruin/POI that exercises the stamp/prefab path once;
   - a continuous surface-to-deep traversal route (canopy/cliff level to
     deep cave floor in one run).

   These are validation content for the first delivery, not permanent game
   identity or a component inventory of forever content.

10. **Public interfaces the harness validates.** Dig/place verbs, mirror
    queries, collision against voxel occupancy (not the render mesh),
    streaming and persistence surfaces — proven by the adjacent harness
    using only public APIs. The harness’s third-person character, camera,
    and debug presentation are how validation is performed; they are not
    themselves product-level outcomes.

---

## Non-goals

- Shipping a game, combat loop, progression, economy, or multiplayer session
- Implementing System/LLM authorship, spells, gas, or intent layers
- Building gameplay (blueprints, work orders, mechanisms, room assignment)
  as product features — a stamped POI validates generation/stamp paths, not
  fortress mode
- Treating the walkable executable as a player-facing product rather than a
  validation harness
- Product One behavior for full fluid simulation, fire CA, structural
  integrity, granular settle, or weather/growth (later substrate outcomes;
  formats and seams may anticipate them)
- Felling trees into rigid bodies and similar physics-coupled spectacles as
  first-slice identity (stretch validation at most)
- Rich persistence (multi-slot, versioning, cross-mode fortress reclaim)
  beyond single-slot seed + deltas for first-slice validation
- Game-specific content identity (named characters, campaign, authored
  quests, franchise lore)
- Platform- or machine-specific implementation gates (particular GPU APIs,
  atomic widths, host devices) as product identity — those belong in design
  and benchmarks

---

## Unresolved questions for humans

The seeds agree on product identity: the **reusable substrate** is the
product; Product One is the **first delivery slice**; the walkable
executable is a **validation harness**, not a game. No identity, purpose, or
boundary conflict requires a guess.

Open items that design may resolve later without changing *what product this
is* (voxel size final call, distant LOD strategy, object-layer scaling,
provisional discrete-GPU baselines, ordering of later substrate families
after Product One) are left to technical design and measurement. None
currently block vision approval.

---

## Seed contributions

| Source | What it contributed to this vision |
|---|---|
| **README.md** | Product name and one-line identity: GPU-resident voxel substrate crate; walkable executable as harness, not game. |
| **project-boundary.md** | Binding scope: substrate crate(s) vs. external game; harness must use public APIs; **Cargo workspace** consumer boundary (crate split is design); explicit exclusion of System/LLM/spell/gas/combat/AI/building layers. |
| **product-one-seed.md** | First delivery slice and “done” shape: material-world claim; dig/place as proof; generation/matter/API depth for the milestone; **fused seed validation** (landscape, vegetation density, static water, geology, ore/aquifer, voxel objects, stamped POI, continuous Z route); harness-owned controller/camera/debug; product-level performance and persistence outcomes (numbers and machine-profiled evidence, not device-specific rules as identity). |
| **voxel-world-substrate.md** | Enduring substrate outcome families and layering: smooth meshing vs. cube look; bricks/sparsity; geology pipeline; dressing vs. voxel objects; fluid tiers; integrity/granular/ambient sim; mutation-safe queries; streaming/deltas; **nothing above the matter layer touches voxels directly**. These are required later outcomes of the same product, filtered by Product One for the first slice — not imported game design. |
| **docs/seeds/README.md** *(manifest note)* | Authority order: Product One binds the first milestone; substrate doc is architecture reference filtered by that seed for the milestone; project boundary clarifies Moria is only the substrate (game/System/etc. out of scope). |

**Omitted from vision on purpose:** detailed material palettes, exact region
dimensions, milestone schedules, kernel/API shapes, CA rule tables, named
future games’ gameplay, System attachment recipes, and host-specific GPU
constraints. Those belong in design, benchmarks, or remain reference-only
until a later slice expands substrate delivery.
