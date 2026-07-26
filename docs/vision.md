# Moria — Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria is a reusable, GPU-resident voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). Downstream games consume it; they are not this repository.

The repository may ship a **walkable-world executable**. That binary is a **validation harness and product-shaped demo**, not a game. It must exercise the substrate only through the same public interfaces an external game would use—no privileged or game-specific paths inside the harness.

**Product One** is the first binding milestone of that product: substrate plus one generated region plus a character who can run through it, with dig/place as proof that the world is mutable matter. It is tech proven as a demo, not a content product.

---

## Purpose

Make one claim undeniable and reusable:

> This is not a heightmap with props. It is a fully material voxel world that *reads* as a normal natural surface, remains continuous and walkable in deep Z, and can be mutated anywhere—while the same crate stack stays clean enough for multiple game genres to sit on top.

The substrate exists so future games (ARPG with a System, fortress/colony, descent roguelike, pure sandbox) can share **matter, generation, queries, mutation, streaming, and persistence** without each reimplementing the world layer. Those games are consumers and context; they are not in scope here.

---

## Boundary

| In scope | Out of scope |
|---|---|
| Voxel matter representation (bricks, sparsity, lazy materialization) | Any full game (ARPG, fortress, Moria descent, sandbox rules) |
| Geology-first world generation and column-derived surface/underground | The System / LLM, spells, gas pricing, intent |
| Smooth isosurface meshing as a *view* of voxel truth | Combat, stats, AI, entities beyond the harness player |
| Dig/place and mirror queries through a public verb/API boundary | Building UI, blueprints-as-gameplay, mechanisms, room economy |
| Streaming, edit-delta persistence, performance credibility | Full CA (fire, integrity, granular settle), advanced fluid tiers as product goals |
| Validation harness: third-person run through one curated region | Weather/seasons/growth sims beyond a fixed time-of-day control |

**Layering rule (product identity):** nothing above the matter layer touches voxels directly. Everything goes through verbs and queries. Compatibility seams for later game layers may be designed where substrate requirements demand them; those layers must not be implemented here.

**Workspace rule:** Cargo (or equivalent) must separate reusable substrate crates from the harness so the consumer boundary is structural, not aspirational.

---

## Required product-level outcomes

These are outcomes the product must make true—not an implementation checklist.

1. **Material world, not scenery.** Terrain and diggable underground are the same matter system. Carved faces look like cuts; collision and queries use voxel truth, not the render mesh.
2. **Looks like a normal world.** Smooth extraction and surface dressing so the default experience is hills, forest, water, cliffs, caves—not a cube aesthetic (raw voxels remain a debug view).
3. **Deep Z is first-class.** Continuous vertical play from surface into walkable underground; geology (strata, caves, interesting subsurface bands) is real content, not a painted floor.
4. **Mutable everywhere (proof in Product One).** Dig and place are in the first milestone as the credibility proof—debug tools, not gameplay loops.
5. **Substrate, not game.** Public API + crate boundary so an external game is a peer consumer of the harness, not a fork of engine internals.
6. **Scales by sparsity.** Homogeneous sentinels, lazy materialization, and streaming so a region large enough to matter does not require the whole volume as dense voxels in memory.
7. **Credible performance and persistence.** Walkable cold-start, incremental remesh after mutation, budgeted GPU residency under streaming, and save/load as worldgen + edit deltas (exact restore of scars).
8. **Decision bed for open substrate choices.** Product One is the benchmark region where voxel size, LOD, object-layer cost, and similar open questions are answered with measurements—not guesses.

Product One’s curated “postcard” region (meadow, forest, river/lake, cliff, cave, micro objects, one stamped ruin, limited material palette) and third-person controller exist to **prove** outcomes 1–7 in one continuous demo route. Their specific content is harness design, not the long-term product identity.

---

## Non-goals

- Shipping a game, campaign, combat loop, or economy.
- Implementing System/LLM, spells, gas, AI agents, or building-as-gameplay.
- Full ambient simulation (weather ecology, seasons, growth, fire CA, structural integrity, granular settle) as Product One deliverables—even where the substrate format anticipates them.
- Fluids beyond static bodies (tier-1 surfaces/channels); no flow simulation as a current product claim.
- Multiplayer, cross-run fortress reclaim, or semantic fortress features (rooms, work orders, mechanisms).
- Replacing or embedding a scripting language in Product One (engine-internal dig/place API is enough to establish the boundary).
- Native Metal (or other backend) forks in load-bearing layers; portability via wgpu/WGSL is the point of the crate.

---

## Unresolved questions for humans

None that change **what product this is**. Seeds agree: Moria is the substrate; Product One is the binding first milestone and public-API harness; broader game material is reference for capability seams only.

Open technical choices (voxel size 25 cm vs 12.5 cm, distant LOD strategy, object-layer scaling, later fluid fidelity, multiplayer readiness as a scope statement) belong to design and measurement against Product One. They do not redefine product identity.

If operators later want the walkable demo to become a **shippable content product** with authored progression (rather than a harness), that would change identity and should be an explicit decision—not an assumption of these seeds.

---

## Seed contributions (traceability)

| Seed | What it contributed to this vision |
|---|---|
| **README.md** | One-line product identity: GPU-resident voxel substrate as a Rust crate; walkable executable as harness for gen, streaming, meshing, editing, collision, persistence, performance. |
| **project-boundary.md** | Binding operator boundary: substrate is the product; game is downstream; harness uses public APIs only; game/System/LLM/spell/gas/combat/AI/building layers out of scope; workspace split is mandatory. |
| **product-one-seed.md** | Binding first milestone: outcomes for “walkable material world,” dig/place as proof, harness player/camera, performance credibility targets as product language, non-goals for Product One, and that generation + matter slice + API sliver are what ships first. Specific seed-world features and milestone order are treated as harness/demo shape, not imported as game content scope. |
| **voxel-world-substrate.md** | Architecture reference for *why* the substrate must support smooth meshing, geology-first gen, deep Z, sparsity, streaming/deltas, object vs dressing separation, and clean layering so future games can attach. Only capabilities needed to support those futures (and selected by Product One) enter current outcomes; full CA, fluids tiers 2–3, integrity, building, nav/entities, weather ecology, and game-layer examples remain context—not current delivery. |
| **docs/seeds/README.md** | Manifest confirmation: Product One binds implementation + harness; substrate doc is reference sliced by Product One; project-boundary is the operator clarification. |

### Deliberately not imported into current scope

From the substrate reference and future-product language: ARPG/System hooks, fortress labor and designations, spell/gas policy, multiplayer servers, fire/integrity/granular CA, advanced hydrology, mechanism entities, room detection, tree felling as required delivery, weather/fire ecology, and all named gameplay fantasies (Moria descent difficulty, DF reclaim, etc.). High-level **capabilities** those depend on (mutable matter, geology, API boundary, sparsity, persistence) are preserved; their **gameplay and content** are not.

---

## Status

This document is a **proposal**. Human approval freezes product identity and boundary for downstream design. Implementation detail, crate splits, algorithms, and milestone scheduling belong in later design work—not here.
