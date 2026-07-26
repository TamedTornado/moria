# Moria — Vision

*Proposal for human approval. Handoff to downstream design — not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped crates. The precise crate split is a design decision; the consumer boundary is not.

The **first build** is **Product One — "The Walkable World"**: substrate crates plus an adjacent walkable executable that validates and demonstrates them. That executable is a **separate consumer and validation harness** (and the public demo), not a game layer. It must use only the same public interfaces available to any external game.

Future games (ARPG with a System client, fortress/colony, descent/adventure, pure sandbox) are **downstream consumers outside this repository**. Their needs motivate the substrate; their gameplay, content, characters, assets, and rules do not enter current scope.

## Purpose

Ship an engine layer on which those games can sit: a continuous 3D world whose *truth* is mutable voxels, whose *look* is a normal natural landscape, and whose public surface exposes generation, matter, queries, and mutation without embedding game rules and without depending on an LLM.

Product One makes one claim undeniable for that layer: **this is a fully material world, and it looks good** — by shipping the reusable generation and matter slice, proving mutability under a player who can traverse surface and depth, and meeting interactive performance and persistence targets with comparable benchmark output.

## Boundary

### Substrate (repository product)

In scope for the substrate mandate — authorized outcome *families*, not a mechanism inventory:

| Outcome family | Intent |
|---|---|
| **Matter as truth** | Sparse brick-backed voxels are authoritative; the mesh is a regenerated view |
| **Geology-first generation** | Truthful surface and underground structure, lazy materialization, placement/POI metadata — shipped as the **reusable generation asset**, not a one-off demo script (a continent pass may be limited to a curated region without cheapening this layer) |
| **Smooth natural look** | Terrain and structures read as landscape, not default cube aesthetic |
| **Physics and responsive matter** | Dig/place and the wider mutation surface; matter that can respond (support, failure, rigid transitions when those systems are live) |
| **Voxel-backed interactable objects** | Trees, boulders, and similar objects participate as matter, not pure decoration |
| **Matter-derived dressing** | Grass and clutter driven by voxel state so look stays tied to truth |
| **Fluid behavior** | Bodies and, over substrate life, flow-capable fluid behavior |
| **Ambient / fire / growth behavior** | World that can simulate fire ecology, wetness, growth, and related ambient rules on the matter layer |
| **Granular and support failure** | Sand/gravel-style settle and structural integrity / cave-in class behavior |
| **Durable world and object change** | Streaming, delta persistence of scars and object journals, exact reload of what was changed |

The substrate must **stand alone** with zero LLM dependency. Compatibility *seams* may be designed where requirements demand them; game layers are not implemented here.

**Out of repository scope (game / product layers):** any shipped game; combat, stats, AI, agent labor; System/LLM authorship; spells, gas/pricing, intent; fortress designations, building as gameplay (blueprints, mechanisms, room economy as game features); multiplayer as a product.

### Product One (first delivery slice)

Product One **defers** many substrate families from *its* definition of done; that deferral is **not** a game-layer non-goal and does **not** strip them from the substrate mandate.

| Product One includes | Product One defers (still substrate-authorized later) |
|---|---|
| Generation layer as reusable asset (columns, strata, caves, ore, lazy gen, POI metadata; curated-region continent pass OK) | Full multi-region / unbounded continent productization beyond the seed |
| Brick pool, sparsity, lazy materialization, incremental smooth meshing | — |
| Grass/clutter dressing; static water bodies (tier-1 lakes/river surface) | Flow sim, weather/seasons, growth CA, fire ecology |
| Voxel objects placed and rendered (trees, boulders, etc.) | Rigid-body felling / physics coupling (**stretch only**, not definition of done) |
| Dig/place + mirror queries as public-facing engine API (debug-driven in the harness) | Full CA, integrity/granular settle, fluids beyond static bodies |
| Streaming rings, delta save/load of the seed world | Cross-run multi-mode fortress reclaim loops as a product feature |
| Walkable harness: third-person traversal, curated proof route, debug tools, **public/downloadable demo artifact**, benchmark scene | Other consumer surfaces (sequencing, not identity) |

Harness-only claims (do **not** treat as unqualified substrate functions): third-person controller and camera, the curated postcard route, debug dig/place and visualization keys, and the public demo/download. Those are **Product One delivery outcomes** that exercise the substrate.

## Required product-level outcomes

### Substrate outcomes (mandate)

1. **Reusable engine layer.** Generation + matter (+ thin verb/query surface) consumable by external games through public APIs only.
2. **Material world.** Continuous surface-to-depth voxel truth; mesh never authoritative.
3. **Geology-first generation as the reusable asset.** Truthful strata, caves, underground structure, lazy materialization, and placement metadata — designed for reuse across consumers, even when the first region is curated.
4. **Responsive matter families.** Over substrate life: mutation, interactable voxel objects, matter-derived dressing, fluids, ambient/fire/growth, granular/support failure, rigid transitions, and durable world/object change — as capability families, not a frozen feature list.
5. **Standalone and layerable.** No LLM in the substrate; game rules stay above; nothing above matter touches voxels except through the verb/query surface.

### Product One outcomes (first build)

1. **The claim is visible.** One natural region that reads as hills, forest, water, cliffs, and walkable cave — continuous in 3D — with dig/place proving full material mutability under interactive remesh.
2. **Harness as consumer.** Third-person traversal of a curated proof route; collision and mutation against voxel truth; debug interaction only through public substrate APIs.
3. **Interactive performance as product success.** The world is walkable at interactive frame rates, dig-to-remesh stays responsive, entry to a walkable world is quick, streamed GPU residency stays bounded for a demo-scale region, saves stay compact with exact restoration of scars, and a scripted benchmark scene produces **comparable** output across machines — so later work can regress against a living bar. Success is meeting that fused experience, not merely collecting metrics.
4. **Public artifact.** A downloadable / public demo is a required Product One outcome (communication and proof), not optional polish.
5. **Decision bed.** Product One answers open substrate knobs (e.g. voxel size, streaming/memory, meshing latency) with measurements from this region.

## Non-goals

- Shipping a game, campaign, combat loop, or multiplayer *product* in this repository.
- Implementing System/LLM, spell/gas economies, agent labor, or fortress designations.
- Treating Product One’s curated seed content (specific ruin, tree species, ore types, postcard route) as permanent game IP — it is validation and demonstration content.
- Making **tree felling / rigid fall** part of Product One’s definition of done (optional stretch only).
- Collapsing substrate scope to only what Product One ships first, or treating deferred matter families as permanently out of the substrate.
- Reducing Product One success to “we recorded baselines” without the qualitative performance outcomes above.
- Privileged harness paths that bypass the public crate API.

## Unresolved questions (for humans)

1. **Multiplayer readiness as substrate commitment.** The architecture reference notes that a verb/command surface is server-authoritative-ready by construction and asks whether that readiness should remain an **explicit substrate commitment** even though no multiplayer product is built. Should Moria’s substrate mandate require designing and preserving server-authoritative-ready command/query boundaries (no multiplayer product, no netcode game), or is multiplayer readiness explicitly out of substrate scope until a later product needs it?

No other identity/boundary ambiguity remains for vision: one crate or a small family is allowed (exact split is design); Product One is the first build; felling is stretch-only for Product One; the public/downloadable artifact is required; other consumer surfaces are delivery sequencing.

## Seed contributions (traceability)

| Source | What it contributed to this vision |
|---|---|
| **README.md** | Name and identity: GPU-resident voxel substrate as Rust crate; walkable executable as separate consumer/harness, not game. |
| **docs/seeds/project-boundary.md** | Binding boundary: reusable crates (one or small family); game external; harness uses public APIs; game/System/LLM/spell/gas/combat/AI/building out of scope; consumer split required, exact crate graph deferred to design. |
| **docs/seeds/product-one-seed.md** | First build: Walkable World; generation as reusable asset; matter slice vs deferred systems; dig/place as proof; harness traversal/demo; performance as product targets; public artifact; felling as stretch; explicit Product One non-goals without erasing substrate mandate. |
| **docs/seeds/voxel-world-substrate.md** | Long-horizon substrate mandate: Design Goal 4 and matter/world families (§§5–8, 11–12) — physics/responsive matter, objects, dressing, fluids, ambient/fire/growth, granular/support, rigid transitions, durable change; geology-first gen; layering; standalone engine. §14 open question on multiplayer readiness carried into vision. Future game modes and mechanism detail kept as capability context only. |

### Omitted or de-emphasized from seeds (intentionally)

- Full technical prescriptions (brick sizes, bit layouts, meshing algorithms, GPU architecture, kernel constraints, build-order code steps) — design/engineering.
- Named future games’ fantasy, progression, and content.
- Milestone calendars, viral-clip framing, and machine-specific numeric thresholds — design may restate targets; vision keeps the fused performance outcome.
- Exhaustive seed-route feature tables and material palettes as product identity — validation content for Product One, not the definition of Moria.

---

*If approved, downstream design refines the substrate mandate and Product One’s first slice against the architecture reference without expanding this repository into game layers, and without treating deferred substrate families as abandoned.*
