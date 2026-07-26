# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). It is the engine layer for natural-looking, fully mutable voxel matter—not a game.

A **walkable-world executable** ships in-repo only as a **validation harness**: it exercises generation, streaming, meshing, editing, collision, persistence, and performance through the same public interfaces an external game would use. It is also the **public/downloadable proof artifact** for the intended audience. It is not a privileged game layer and not a shippable game product.

---

## Purpose

Make one claim undeniable in playable form: **this is not a heightmap with props—it is a fully material world, and it looks good.**

Moria exists so future games (ARPG, fortress/colony, descent/sandbox, pure toybox) can sit on the same matter stack without reimplementing world truth. This repository builds that stack and proves it with a product-shaped demo slice; it does not build those games.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Reusable substrate crate(s) with a hard public API | The actual game(s) that consume Moria |
| Validation harness that only uses public substrate interfaces | Game rules, combat, stats, AI, entities beyond a demo player |
| Generation, matter representation, meshing/dressing, dig/place, streaming, persistence deltas | System / LLM, spells, gas policy, pricing, intent |
| Compatibility *seams* where substrate requirements demand them | Building UI, blueprints-as-gameplay, mechanisms, labor, economy |
| One curated generated region as the harness’s proof bed | Authored campaign content, characters, factions, quests |

**Cargo workspace separation** between substrate and harness is an immediate product rule: the consumer boundary is not optional. Precise crate split is a later technical decision.

**Layering rule (product-level):** nothing above the matter layer touches voxels directly. Mutation and queries go through verbs and mirror APIs—the sandbox, multiplayer-readiness, and reuse boundary in one constraint.

---

## Required product-level outcomes

These are the outcomes Product One must make true. Detail and implementation live downstream.

1. **Readable natural world** — Rolling terrain, forest, river/lake, cliffs, and underground space that read as a normal outdoor/underground world, not a cube aesthetic. Voxel grid is truth; smooth isosurface meshing is the view.

2. **Mutable material truth** — Anywhere you can reach, matter can be dug or placed. Dig/place on debug keys is in scope as *proof*, not gameplay. Collision and queries use voxel occupancy, not the render mesh.

3. **Deep Z is real** — Continuous vertical play from surface (and cliff tops) into walkable underground (caves, strata, ore/aquifer bands in cuts). Underground is content volume, not a painted floor.

4. **Geology-first generation** — Worldgen produces columns, strata, caves, materials, and lazy brick materialization—not a heightmap with rock painted underneath. One curated seed/region is enough for the harness; the generation layer remains the reusable asset.

5. **Sparse GPU-resident substrate** — Brick pool, homogeneous sentinels, and streaming so a region large enough that raw voxels do not all fit still runs. Meshing is incremental over dirty bricks; the mesh is never authoritative or saved.

6. **Dressing tied to matter** — Grass/clutter as data-driven instances; trees/boulders as voxel-backed objects for placement and rendering (felling/rigid conversion is stretch, not required). Static water bodies (tier-1 surfaces/channels) yes; flow simulation no for this slice.

7. **Harness proves the API** — Third-person walk/run/jump/swim controller, free-orbit camera, continuous demo route, and debug views (wireframe/bricks, raw voxels, streaming rings, time-of-day). Demo player only—no combat, AI, or game systems.

8. **Public proof artifact** — The playable harness is released as a public/downloadable proof for the intended audience (e.g. milestone posts and a downloadable demo). It remains a validation consumer of the substrate, not a shippable game product.

9. **Credibility numbers** — Deliverables include benchmarks (e.g. target framerate class, dig-to-remesh latency, cold start to walkable, memory with streaming, delta save size) plus a scripted scene and machine profile so later substrate work can regress against Product One.

10. **Persistence model** — Truth = worldgen function + edit deltas; reload same seed + deltas (single save slot is enough). No save versioning product for this slice.

11. **Portable GPU path** — wgpu/WGSL load-bearing path; design stays viable on unified-memory (e.g. Apple Silicon) constraints such as no 64-bit buffer atomics, with sparsity treated as load-bearing rather than deferred polish.

---

## Non-goals

Explicitly **not** current product:

- Combat, stats, AI, NPCs/entities beyond the harness player  
- System / LLM, spells, gas, pricing, intent  
- Building-as-gameplay (UI, work orders, mechanisms, rooms economy)  
- Fluids beyond static bodies; weather/season/growth sims (fixed time-of-day is enough)  
- Cellular automata (fire, granular settle, integrity/cave-ins) as running systems—format may reserve bits; nothing runs them in Product One  
- Tree felling / rigid-body conversion (stretch only)  
- Embedded scripting language  
- Multiplayer, cross-run fortress reclaim loops, or any full game mode  
- Implementing future game layers “while we’re here”

Longer substrate reference material (full fluid tiers, integrity, building verbs, weather ecology, nav/labor, System attachment) is **capability context** for seams and future milestones—not imported gameplay or scope for the current product.

---

## Unresolved human questions

None that change **product identity** (substrate + public-API harness as validation consumer and public proof) or **purpose** (material world proven in a walkable demo). Seeds agree: Moria is only the voxel-world substrate; Product One selects which reference capabilities ship now. Human review confirmed the harness is a public/downloadable proof artifact while remaining a validation consumer, not a game.

Open technical/design calls already flagged in seeds, for design—not vision blockers:

- Final voxel size (25 cm baseline vs 12.5 cm / adaptive)  
- Distant LOD strategy vs camera needs  
- Object-layer scaling at high vegetation counts  
- Whether later fluid tiers need a pressure solve  
- Discrete-GPU performance re-baseline when hardware is available  

If the operator intends Product One’s seed-world content as fixed product identity rather than a disposable proof bed, that would change this document—say so before design freezes boundary language.

---

## What each seed contributed

| Source | Role in this vision |
|---|---|
| **README.md** | Names Moria as reusable GPU-resident voxel substrate crate; walkable executable as consumer/harness; points at seeds. |
| **project-boundary.md** | Binding identity: crate(s) only; game is external; harness must use public APIs; Cargo workspace consumer boundary; game/System/LLM/spell/gas/combat/AI/building out of scope; seams allowed, layers not. |
| **product-one-seed.md** | Binding *first* product slice: walkable material-world demo outcomes, non-goals, dig/place as proof, region-scale proof bed, substrate layer slice (gen full / matter partial / API sliver), harness player/camera, performance credibility, public/downloadable proof artifact for the audience, milestones as delivery shape—not game content to ship as product IP. |
| **voxel-world-substrate.md** | Architecture reference for look (smooth mesh over voxel truth), sparsity, geology-first gen, dressing/objects, persistence/streaming, layering diagram, and long-horizon capabilities. Only Product-One-selected portions are current requirements; remainder frames why seams exist. |
| **docs/seeds/README.md** | Manifest order: Product One binds implementation + harness; substrate doc is reference under that selection; boundary doc is operator clarification. |

**Omitted from current scope (kept as context only):** System/LLM authoring model, full CA and fluid toybox, structural integrity, building/blueprints/mechanisms, weather ecology, multi-mode game layering, fortress/ARPG/Moria-descent fantasies, stretch timber clip, and detailed material/POI inventory except as harness proof points design may thin or substitute.

**No material conflict** among seeds: boundary sets identity; Product One sets the first vertical slice; substrate doc supplies depth without expanding Product One into a game.
