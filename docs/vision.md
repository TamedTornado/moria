# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical
design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**: a Rust crate
(or small family of tightly scoped crates) that owns matter, generation,
meshing, mutation, streaming, and persistence of a continuous 3D material
world.

The repository may ship a **walkable-world executable**. That binary is a
consumer and validation harness only. It must use the same public interfaces
an external game would use; it is not a game layer and owns no privileged
implementation paths.

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
streamed world with measured performance — not from a whiteboard. The
walkable-world harness is how that claim is proven: a curated generated
region you can run through in third person, with dig/place as *proof of
matter*, not as game systems.

---

## Boundary

| In product | Out of product |
|---|---|
| Generation, matter storage, meshing/dressing, dig/place and related matter verbs, collision against voxel truth, streaming, delta persistence | Game rules, combat, stats, AI, entities beyond a harness player |
| Public script/API surface (verbs + mirror queries) so nothing touches voxels directly | The System / LLM, spells, gas metering, pricing policy |
| Validation harness that exercises the public API | Building UI, blueprints-as-gameplay, work orders, mechanisms-as-play |
| Compatibility *seams* only where substrate requirements demand them | Fluids beyond static bodies (in the first proven slice), weather/seasons/growth sims, CA (fire, granular settle, integrity) as shipped behavior |

**Cargo workspace** (or equivalent) separates reusable substrate crates from
the harness. The precise crate split is a design choice; the consumer
boundary is not optional.

**Reference vs. current scope.** The full substrate design describes
capabilities many future games need (multi-tier fluids, structural integrity,
voxel-object felling, ambient weather, room/nav semantics, multiplayer-ready
command architecture). Those describe *what the substrate must eventually be
able to support*. The **current** product commitment is the Product One
slice: generation full enough to host a credible region; matter and meshing
as the headline; a thin verb/query API; static water; player traversal and
debug mutation in a harness. Format and layering may leave room for later
layers without implementing them.

---

## Required product-level outcomes

Outcomes the product must deliver — not a feature list:

1. **Substrate as crate(s).** A reusable Rust package boundary with a public
   mutation and query surface. External consumers (including the in-repo
   harness) share that surface; no back doors into voxel storage.

2. **Material world that reads as landscape.** Smooth isosurface terrain
   (mesh is a view, never truth) over voxel matter; surface dressing and
   voxel-backed objects so the world does not look like a cube grid while
   remaining diggable and continuous in 3D, including underground depth.

3. **Geology-first generation with sparsity.** Lazy materialization from a
   seedable pipeline (columns, strata, caves, materials, POI metadata);
   homogeneous bricks and streaming so a region large enough to exercise
   real load does not require the whole volume as dense voxels in memory.

4. **Mutation as proof.** Dig and place (and incremental remesh) work as
   first-class operations with latency that preserves the demo claim: cuts
   look like cut matter, not prop holes in a static mesh.

5. **Walkable validation.** A third-person character that collides with
   voxel occupancy (not the render mesh), can traverse surface and deep
   routes in one continuous run, and exposes debug views that make
   streaming, bricks, and raw matter legible.

6. **Credibility under numbers.** Performance and memory targets, plus
   regression-friendly benchmarks (scripted traversal and carve load with
   machine profile), so later substrate changes can be judged against a
   known product-one baseline. Dev platform constraints (e.g. wgpu
   portability, no load-bearing native Metal fork, 32-bit atomics where
   required) are part of that credibility surface.

7. **Persistence model.** Truth is worldgen function plus edit deltas;
   reload restores the same seed world and scars without treating full
   volume dumps as the save format.

---

## Non-goals

- Shipping a game, combat loop, progression, economy, or multiplayer session
- Implementing System/LLM authorship, spells, gas, or intent layers
- Full fluid simulation, fire CA, structural integrity, granular settle, or
  weather/growth as product-one behavior (formats and seams may anticipate
  them)
- Building gameplay (blueprints, work orders, mechanisms, room assignment)
  as product features — stamp/prefab paths may be exercised only as
  generation/validation, not as fortress mode
- Felling trees into rigid bodies and similar physics-coupled spectacles
  (stretch validation, not core identity)
- Rich persistence (multi-slot, versioning, cross-mode fortress reclaim)
  beyond single-slot seed + deltas for the harness
- Game-specific content identity (named characters, campaign, authored
  quests, franchise lore). Curated demo geography exists only to prove
  substrate claims.

---

## Unresolved questions for humans

The seeds agree on product identity (substrate + public-API harness) and on
which document binds the first milestone versus which is architecture
reference. No identity/purpose/boundary conflict requires a guess.

Open items that design may resolve later without changing *what product this
is* (voxel size final call, distant LOD strategy, object-layer scaling,
provisional discrete-GPU baselines) are left to technical design and
measurement. None currently block vision approval.

If operators later want the walkable demo treated as a *shipped player-facing
product* rather than a validation harness, that would change positioning and
should be stated explicitly; seeds currently reject that reading.

---

## Seed contributions

| Source | What it contributed to this vision |
|---|---|
| **README.md** | Product name and one-line identity: GPU-resident voxel substrate crate; walkable executable as harness, not game. |
| **project-boundary.md** | Binding scope: crate(s) vs. external game; harness must use public APIs; workspace consumer boundary; explicit exclusion of System/LLM/spell/gas/combat/AI/building layers. |
| **product-one-seed.md** | First proven slice and “done” shape: material-world claim, dig/place as proof, generation/matter/API depth for the milestone, walkable third-person harness, performance/persistence outcomes, and the milestone ladder that defines credibility. Demo world contents (specific biome features, material list, ruin stamp) are treated as validation context, not as permanent product content. |
| **voxel-world-substrate.md** | Capability envelope and layering model the substrate must support over time (smooth meshing vs. cube look, bricks/sparsity, geology pipeline, dressing vs. voxel objects, fluid tiers, integrity, building verbs, streaming/deltas, clean crate layers). Only the Product One–selected portions are current commitment; the rest is preserved as future-facing substrate responsibility, not as imported game design. |
| **docs/seeds/README.md** *(manifest note)* | Clarifies authority order: Product One binds the milestone; substrate doc is architecture reference filtered by that seed; project boundary is the operator clarification that Moria is only the substrate. |

**Omitted from vision on purpose:** detailed material palettes, exact region
dimensions, milestone schedules, kernel/API shapes, CA rule tables, named
future games’ gameplay, and System attachment recipes. Those belong in
design or remain reference-only until a later product expands the slice.
