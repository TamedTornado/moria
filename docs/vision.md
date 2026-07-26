# Moria — Vision

*Proposal for human approval. Handoff to downstream design — not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**: a Rust crate (or small family of tightly scoped crates) that provides mutable volumetric matter, geology-first generation, smooth meshing as a non-authoritative view, streaming, and mutation through a public verb/query API.

The repository may ship a **walkable-world executable**, but that binary is only a **validation harness and product-shaped demo**. It must consume the substrate through the same public interfaces an external game would use. It is not a game layer and owns no privileged paths.

The first delivery target — **Product One, “The Walkable World”** — proves the substrate as a continuous, diggable natural region you can run through in third person. The claim to make undeniable: *this is not a heightmap with props; it is fully material world truth, and it looks good.*

---

## Purpose

Provide a standalone engine layer that future games (adventure, fortress/colony, sandbox, and any LLM-driven “System” layer) can sit on without reimplementing world matter. The substrate owns **matter, generation, physics-facing queries, and mutation**. Game rules, economy, combat, AI, and content authorship live above it — in other repositories or later layers.

Product One’s job is credibility and decision-making: enforce the consumer API boundary from the first commit, produce demoable milestones and measurable performance numbers, and answer open substrate questions (voxel size, LOD, object-layer scale) with measurements rather than guesses.

---

## Boundary

| In product | Out of product |
|---|---|
| Reusable substrate crates | Any full game |
| Public matter/generation/mesh/stream/persist APIs | Game rules, combat, stats, AI entities |
| Dig/place and mirror queries as engine verbs | System / LLM, spells, gas, intent, pricing policy |
| Walkable harness that only uses public APIs | Building UI, blueprints, mechanisms, work orders |
| Seeded region generation + lazy materialization | Weather/season/growth sims beyond a fixed time-of-day control |
| Smooth isosurface meshing + surface dressing | Fluids beyond static bodies (no flow sim in Product One) |
| Voxel objects for placement/render (trees, micro objects) | CA (fire, granular settle), structural integrity, tree felling as required scope |
| Delta persistence + streaming rings | Multiplayer; multi-save versioning; content pipelines for authored campaigns |

**Layering rule (product-level):** nothing above the matter layer touches voxels directly. Consumers mutate and query through verbs and mirrors. That boundary is the sandbox, multiplayer-readiness, and reuse seam — even when multiplayer and other games are not built here.

**Workspace rule:** Cargo (or equivalent) must separate reusable substrate from the harness. Crate graph details are design work; the consumer boundary is not optional.

---

## Required product-level outcomes

Outcomes, not a feature list. Product One succeeds when:

1. **Substrate exists as a consumable crate surface** with the API boundary enforced; the harness is an external-style client of that surface.
2. **Voxel truth, not decorative terrain.** Collision, dig, and place operate on volumetric occupancy/density; the render mesh is regenerated view, never authoritative save state.
3. **Reads as a normal world.** Continuous smooth terrain and surface dressing over material voxels — not a block aesthetic as the primary look.
4. **Mutable matter is demoable.** Debug dig/place proves mid-run carving with cut faces that look like cut earth; dirty regions remesh without hitching past stated latency.
5. **Deep Z is first-class.** Continuous traversal from surface features into underground volume (caves, strata visible when cut) is possible in one run; the underground is content space, not a skybox floor.
6. **Geology-first generation, lazily materialized.** Columns/strata/caves/ore-style pipeline materializes bricks on touch; homogeneous sparsity keeps idle volume cheap; the whole demo region must not fit in memory as raw voxels.
7. **Streaming and persistence are real.** Active rings around the player; truth = worldgen function + edit deltas; reload same seed + deltas restores the defaced world.
8. **Performance is part of the product.** Stated targets (frame rate, dig-to-remesh, cold start, memory envelope, delta save size) are validated by a scripted benchmark scene plus machine profile, so later substrate work can regress against Product One.
9. **Portable GPU stack.** wgpu/WGSL load-bearing path; design respects platforms without 64-bit buffer atomics (e.g. Apple GPUs); no native Metal fork in substrate layers.

Downstream games are out of scope, but the substrate must **leave seams** for them: priced verbs as a plug-in policy later, object and stamp paths, material registry, POI metadata, and aggregate mirrors suitable for nav/integrity/fluids when those layers are built elsewhere.

---

## Non-goals

- Implementing ARPG, fortress, Moria-descent, or sandbox *gameplay*.
- System/LLM integration, spell packages, gas metering, or combat.
- Full fluid CA, fire ecology, weather/season simulation, structural integrity, or granular physics as Product One deliverables (formats/seams may anticipate them).
- Tree felling / rigid conversion as a required outcome (stretch only if physics coupling is cheap).
- Embedded scripting language, building UX, blueprints-as-gameplay, agent labor.
- Authoring a large hand-made world; one curated *seed* of generation parameters is enough.
- Shipping multiple games from this repo; this repo ships substrate + harness only.

---

## Unresolved human questions

The seeds agree on product identity, purpose, and boundary. No question of human intent is open that would change *what product this is*.

Technical and slice questions (final voxel size 25 cm vs 12.5 cm, distant LOD strategy, object-layer capacity, later fluid pressure model, multiplayer timeline) are **design/measurement issues**, not vision ambiguities. Product One is explicitly the decision bed for several of them.

If leadership later wants the *walkable demo itself* to be the named product rather than a harness for the substrate crate, that would change packaging and success criteria — the seeds currently reject that reading (harness ≠ game; crate is the product). Affirm or correct that stance when approving this vision.

---

## Seed contributions (traceability)

| Seed | What it contributed to this vision |
|---|---|
| **README.md** | Names the product (Moria), crate-vs-harness split, and points at `docs/seeds/` as preserved inputs. |
| **docs/seeds/README.md** | **Precedence:** Product One is binding for this milestone; voxel-world-substrate is architecture reference (only selected portions required); project-boundary is operator clarification that game/System/LLM/building intent is out of scope. |
| **project-boundary.md** | Product = reusable substrate crates; game is a separate consumer; harness must use public APIs; Cargo workspace boundary; game rules and System/LLM/spell/gas/combat/AI/building layers out of scope (seams only). |
| **product-one-seed.md** | Product statement and non-goals for the first slice; dig/place as proof not gameplay; layer slice (generation full, matter partial, script/API sliver); performance targets and platform constraints; milestone intent; what Product One “buys.” **Not imported as current product content:** specific 1 km postcard composition, material list, ruin/POI set, character fantasy, trailer beats — those are harness *examples* proving the outcomes above. |
| **voxel-world-substrate.md** | Long-horizon substrate capabilities used as **context**: normal-looking mutable world, deep Z, GPU brick pool + sparsity, geology pipeline, smooth meshing, vegetation as objects vs dressing, fluid tiers, integrity, building verbs, nav/Z, persistence/streaming, crate layering. **Reference only for Product One** except where the Product One seed selects a subset. Future games (ARPG/System, DF fortress, Moria descent) illustrate reuse; their gameplay, characters, and assets are not in scope. |

---

## Approval

Approve this vision to lock product identity, purpose, boundary, outcomes, and non-goals for design and implementation planning. Corrections should be explicit human edits to intent — not silent expansion of scope from the architecture reference.
