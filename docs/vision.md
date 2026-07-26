# Moria — Product Vision

*Proposal for human approval. Handoff to design, not a GDD, technical design, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**: a Rust crate (or small family of tightly scoped crates) that other products consume.

It is not a game. The repository may ship a **walkable-world executable** — a product-shaped demo and validation harness that exercises the substrate through the same public interfaces an external game would use. That executable is a consumer of Moria, not a privileged game layer inside it.

**Product One** (“the walkable world”) is the first milestone: one curated generated region plus a third-person character who can traverse it, with dig/place as proof that the world is fully material. It is the substrate proven in playable form, not a vertical slice of a future title.

---

## Purpose

Provide a durable world foundation that future games can build on without re-solving terrain, mutability, underground depth, streaming, or meshing.

The substrate must make one claim undeniable: **this is not a heightmap with props — it is a fully material world, and it looks good.** Geometry is a regenerated view of voxel truth. Digging, geology, caves, and continuous 3D traversal are first-class; decorative shells that cannot be mutated are not.

Downstream consumers (System ARPG, fortress/colony, descent roguelike, pure sandbox) are **context for capability**, not products of this repository. The substrate stands alone with zero LLM or game-rule dependency.

---

## Boundary

| In scope | Out of scope |
|---|---|
| Reusable substrate crates (generation, matter, a thin public verb/query API) | Any complete game, campaign, or “Moria” fantasy title as content |
| Walkable-world harness using only public APIs | Game rules, combat, stats, AI, entities beyond the harness player |
| One seed region that proves natural look + full material mutability | System / LLM, spells, gas, intent pricing |
| Dig/place as debug proof of material truth | Building UI, blueprints-as-gameplay, mechanisms, room economy |
| Persistence as seed + edit deltas; streaming rings | Cross-title hub loops, multiplayer implementation |
| Compatibility *seams* where substrate design requires them | Implementing semantic/game layers (rooms, work orders, gas policy, etc.) |

Workspace split between substrate and harness is required so the harness cannot grow privileged paths that an external game would not get.

---

## Required product-level outcomes

These are outcomes the current product must deliver so later games remain possible. They are not a feature checklist.

1. **Natural-looking material world.** Rolling terrain, forest, water, cliffs, and caves that read as a normal overworld — smooth isosurface for terrain, not a Minecraft cube aesthetic as the primary look — while remaining fully backed by mutable voxels.

2. **Mutable everywhere, deep Z first-class.** Any voxel can be destroyed or placed. Underground is content (caves, strata, ore, aquifers), not a painted floor under a heightmap. Collision and queries run against voxel truth, not the render mesh.

3. **Geology-first generation with lazy materialization.** Worldgen produces columns and strata as pure functions of seed and coordinates; bricks materialize on touch. Untouched world stays cheap via homogeneous sentinels and sparsity. Generation is the reusable asset; do not stub it into a disposable demo heightmap.

4. **Proof of substrate, not scenery.** A player can traverse a continuous route (surface → cliff → cave depth) and, via debug dig/place, carve real matter with cut faces that remesh as cuts. Without this, the demo is indistinguishable from any terrain scene.

5. **Clean consumer boundary.** Nothing above the matter layer touches voxels directly. Dig/place and mirror queries exist as engine-facing API from day one so the harness and future games share one contract. Gas, scripts languages, and full semantic layers are not required yet — only the seam that keeps them above the crate.

6. **Credible performance and persistence.** Sustained interactive frame rate on the stated mid-range / M4 class targets; incremental remesh after local edits without hitch; cold start into a walkable world under a few seconds; region memory bounded by streaming and sparsity; delta save/load that restores edits exactly. Scripted benchmarks with machine profile are part of the deliverable.

7. **Portable GPU path.** Load-bearing work stays on wgpu/WGSL (no native Metal fork in core). Dev constraints (e.g. no 64-bit buffer atomics on Apple GPUs; bandwidth-aware sparsity) shape design so discrete targets remain reachable.

---

## Non-goals

Explicitly **not** this product:

- Combat, progression, enemies, AI, multiplayer sessions  
- The System, LLM authoring, spells, gas metering, intent  
- Full fluid CA, fire ecology, weather/seasons sim, structural integrity / cave-ins, granular settle as running systems (format may reserve state; Product One does not run them)  
- Tree felling / rigid conversion as required (stretch only if cheap)  
- Building gameplay, blueprints UI, mechanisms, work orders, room detection  
- Fluids beyond static bodies (lakes / river channel with a surface)  
- Authored game content (characters, quests, factions, “Moria levels” as a title)

Long-form substrate reference material may describe those systems so the crate’s seams stay honest; implementing them here would cross the project boundary.

---

## Unresolved human questions

None of the seeds leave **current product identity** ambiguous: Moria is the substrate; the walkable world is the harness/demo; games are downstream.

These remain for humans to settle because they affect design tradeoffs (not whether this is a game vs. a crate):

1. **Voxel size final call** — 25 cm assumed for Product One; 12.5 cm is an open cost/fidelity question (possibly per-region later). Product One is the benchmark bed.  
2. **Distant-terrain LOD strategy** — chunked mesh LOD vs. column-index impostors (and how much a future locked camera would allow cheating).  
3. **Object-layer scaling** — when vegetation/object counts need their own spatial acceleration.  
4. **Discrete-GPU performance claims** — provisional until measured on non-Apple hardware; re-baseline rather than treat M4 numbers as final for all targets.  
5. **Product Two direction** — fortress toybox vs. System ARPG is explicitly deferred; it should not pull scope into Product One.

If any of the above should be treated as fixed product identity rather than open engineering questions, say so; otherwise design may decide them with measurements.

---

## Seed contributions (traceability)

| Seed | What it contributed to this vision |
|---|---|
| **README.md** | Names the product (Moria), crate consumption model, and harness-as-consumer role. |
| **project-boundary.md** | Binding operator boundary: substrate only; game/System/LLM/spell/gas/combat/AI/building out; public-API harness; Cargo workspace discipline. |
| **docs/seeds/README.md** | Authority order: Product One binds the implementation milestone; substrate doc is architecture reference, only the selected slice required; broader game intent deliberately absent. |
| **product-one-seed.md** | First shippable proof: region scale and character of the demo world, dig/place as non-negotiable proof, player/controller expectations, performance targets, milestone arc, explicit non-goals, and what Product One “buys” for later work. **Gameplay, asset lists, and milestone choreography are treated as demo constraints, not as a game design to import.** |
| **voxel-world-substrate.md** | High-level capabilities the crate must eventually support (natural look vs. voxel truth, deep Z, sparsity, geology-first gen, layering rules, GPU-resident matter). Future modes (ARPG, fortress, descent), CA/fluids/integrity/building/entity systems, and System attachment points are **reference only** — preserved as capability pressure on seams and storage, not as current scope. |

---

## Approval

This vision is ready for human review. Approve, amend identity/boundary language, or resolve the open questions above before treating design and implementation plans as authoritative.
