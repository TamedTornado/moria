# Moria — Vision

*Proposal for human approval. Handoff to downstream design — not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). Its first concrete product shape is **Product One — "The Walkable World"**: one generated natural region plus a third-person character who can traverse it, with dig/place as proof that the world is fully material matter — not a heightmap with props.

The walkable-world executable is a **validation harness and public demo**, not a game. It must consume the substrate through the same public interfaces available to any external game.

## Purpose

Prove and ship a substrate on which future games can sit: a continuous 3D world whose *truth* is mutable voxels, whose *look* is a normal natural landscape (smooth terrain, vegetation, water bodies, underground depth), and whose *API* exposes generation, matter, queries, and mutation without embedding game rules.

Product One makes one claim undeniable: **this is a fully material world, and it looks good.** It also establishes the crate boundary, performance baselines, and the decision bed for open substrate questions (voxel size, streaming/memory, meshing latency) with measurements rather than guesses.

## Boundary

| In scope | Out of scope |
|---|---|
| Reusable substrate crates (generation + matter + a thin dig/place / query API) | Any actual game as a product of this repository |
| Walkable-world harness that uses only public substrate interfaces | Privileged or game-specific paths inside the harness |
| One curated generated region; continuous surface-to-underground traversal | Combat, stats, AI, entities beyond the player |
| Smooth meshing, dressing, static water bodies, sparse brick storage, streaming, delta persistence | System / LLM, spells, gas/pricing, intent |
| Dig and place as debug proof of mutability | Building UI, blueprints, mechanisms, structural integrity as gameplay |
| Compatibility *seams* where substrate requirements demand them | Implementing game layers: combat, building systems, fortress/ARPG rules, fluid flow sim, weather/growth CA, fire, rigid-body felling (except as optional stretch) |

The substrate must **stand alone** with zero LLM dependency. Future products (ARPG with a System client, fortress/colony mode, descent/adventure modes) are **downstream consumers**. Their high-level needs — mutable matter everywhere, deep Z, geology-first generation, queryable world state, verb-based mutation, persistence of scars — define *why* the substrate exists; their gameplay, content, characters, assets, and rules do **not** enter current scope.

## Required product-level outcomes

1. **Material world, not scenery.** Terrain, caves, strata, and placed matter are voxel truth; the render mesh is a regenerated view. Collision and mutation operate on voxels, not on decorative geometry.
2. **Reads as a natural world.** Rolling terrain, forest, river/lake, cliffs with readable geology, and a walkable cave — continuous in 3D from surface into depth — without a default "cube world" aesthetic as the primary look.
3. **Mutable under the player.** Dig and place work as first-class substrate verbs (even if only driven by debug tools in Product One), with incremental remesh so cuts look like cut earth.
4. **Reusable crate boundary.** Substrate and harness are separated (e.g. Cargo workspace); nothing above the matter layer touches voxels except through the public verb/query surface the harness already uses.
5. **Tractable at scale.** Lazy materialization, sparse/homogeneous bricks, and streaming so a demo-scale region does not require raw full-voxel residency; cold start and memory stay within the Product One performance story (walkable quickly; GPU-resident budget under streaming).
6. **Credible persistence.** Untouched world from seed/generation; player scars as compact deltas that reload faithfully.
7. **Measurable.** Benchmarkable flythrough/carve scene and targets (frame rate, dig-to-remesh latency, cold start, memory, save size) so later substrate work can regress against Product One.

## Non-goals

- Shipping a game, campaign, combat loop, or multiplayer product in this repository.
- Implementing System/LLM authorship, spell/gas economies, agent labor, or fortress designations.
- Full fluid dynamics, weather/season sim, fire ecology, structural cave-ins, or tree felling physics as Product One requirements (formats/seams may anticipate them; behavior is later).
- Building as a player-facing feature set (placement API as engine surface is fine; blueprints, mechanisms, room semantics are not Product One).
- Treating the walkable demo’s seed content (specific ruin, tree species, ore types, postcard route) as permanent game IP — it is validation and communication of substrate capability.

## Unresolved questions (for humans)

These affect product identity, purpose, or boundary only if answered differently than the seeds imply; design may refine the rest.

1. **Confirm Product One as the sole near-term product shape.** Seeds agree Moria = substrate and Product One = first milestone. Is the walkable harness the *only* in-repo executable goal until substrate milestones complete, or should any other consumer surface appear earlier?
2. **Stretch goals vs. boundary.** Product One lists tree felling / rigid-body fall as stretch. Is stretch work allowed to pull physics coupling into this repo’s definition of done, or must “done” stay at milestone 6 (run + numbers) with felling deferred entirely?
3. **Crate family vs. single crate.** Boundary requires a consumer split; precise crate graph is technical design — unless humans want a hard product rule (e.g. “one published crate only” vs. “workspace of crates is fine”).
4. **Audience commitment.** Product One is framed as both engineering proof and public X/demo artifact. Is public demo/download a **required** outcome of this vision, or a nice-to-have communication channel?

No seed conflict requires guessing: `project-boundary.md` and `docs/seeds/README.md` bind Moria to the substrate; `product-one-seed.md` binds the first implementation slice; `voxel-world-substrate.md` is architecture reference for capabilities, not a second product.

## Seed contributions (traceability)

| Source | What it contributed to this vision |
|---|---|
| **README.md** | Product name and one-line identity: GPU-resident voxel substrate as Rust crate; walkable executable as harness, not game. |
| **docs/seeds/project-boundary.md** | Binding boundary: reusable crates only; game is external; harness uses public APIs; game/System/LLM/spell/gas/combat/AI/building layers out of scope; workspace-style consumer split is required in spirit. |
| **docs/seeds/product-one-seed.md** | Current product shape and purpose: Walkable World demo; region/traversal claim; dig/place as proof; explicit non-goals; performance and milestone outcomes at product level (not imported as a full feature list). |
| **docs/seeds/voxel-world-substrate.md** | Long-horizon substrate capabilities that Product One must not close off: matter-as-truth, smooth view, deep Z, geology-first gen, sparsity/streaming, verb/query layering, standalone engine layer. Future game modes and detailed systems (fluids tiers, integrity, CA, vegetation physics, mechanisms) retained only as capability context. |
| **docs/seeds/README.md** | Manifest authority: Product One is binding for the milestone; substrate doc is reference filtered by Product One; boundary doc is operator clarification. |

### Omitted or de-emphasized from seeds (intentionally)

- Full technical prescriptions (brick sizes, bit layouts, meshing algorithms, GPU architecture, build-order code steps) — belong in design/engineering, not vision.
- Named future games’ fantasy, progression, and content (Moria descent, DF fortress loops, ARPG System, spell packages).
- Milestone schedules, viral-clip framing detail, and machine-specific kernel constraints — useful later; not product identity.
- Exhaustive material palettes, seed-route feature tables, and stretch physics — validation content and optional work, not the product definition.

---

*If this vision is approved, downstream design should refine Product One’s substrate slice and harness against the architecture reference without expanding repository scope into game layers.*
