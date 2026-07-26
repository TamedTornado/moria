# Moria — Product Vision

*Proposal for human approval. Handoff to downstream design — not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). It is an engine layer: matter, generation, meshing, mutation, queries, streaming, and persistence for natural-looking, fully material 3D worlds.

The repository may ship a **walkable-world executable**. That executable is a **consumer and validation harness**, not the product and not a game. It must use the same public interfaces available to any external game; it owns no privileged or game-specific paths.

The actual game (or games) that will sit on this substrate live **outside this repository**.

---

## Purpose

Make one claim undeniable and reusable: a world that **reads as a normal outdoor landscape** (terrain, vegetation, water, cliffs, caves) is still **voxel truth all the way down** — walkable anywhere, continuous in 3D including deep underground, and mutable (dig/place) without becoming a heightmap-with-props or a cube aesthetic.

Product-level success is a **substrate other products can build on**, proven by a public, measurable walkable demo — not a finished game loop.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Substrate crates (generation, matter, public verb/query API) | Game rules, genres, and campaigns |
| Walkable-world harness consuming public APIs | The System / LLM layer |
| Seeded natural region for validation | Spells, gas/pricing, combat, AI, entities beyond a player avatar |
| Dig/place and debug views as *proof* of material world | Building UI, blueprints-as-gameplay, mechanisms, agent labor |
| Persistence of seed + edit deltas; streaming for scale | Full fluids sim, fire CA, integrity/cave-ins, weather/seasons as product features |
| Compatibility *seams* only where substrate design requires them | Implementing those higher layers here |

**Layering intent (preserved, not fully built now):** nothing above the matter layer touches voxels directly — mutation and inspection go through verbs and queries. That boundary is sandbox, multiplayer-readiness, and multi-game reuse in one rule. Gas, scripts, and game policies are injectors *above* the substrate, not features of it.

---

## Required product-level outcomes

What “this product works” means at the vision level — capabilities future games will need, without importing their content or rules:

1. **Readable world, material truth** — Smooth terrain meshing over voxel occupancy so the surface looks natural while collision, dig, and queries run against voxels, not the mesh.
2. **Mutable matter** — Dig and place (at least as harness/debug proof) with incremental remesh so cuts look like cuts and the world is not decorative geometry.
3. **Deep Z is first-class** — Continuous vertical play: surface, cliffs, and walkable underground (caves, strata, something true when you dig down), not a flat floor under a skybox.
4. **Geology-first generation** — Worlds generated as stratified, sparse material (columns → lazy bricks), not a heightmap with rock painted under it — so digging is honest.
5. **Scale via sparsity and streaming** — Homogeneous empty/solid regions stay cheap; active rings keep a large region walkable without loading everything as dense voxels.
6. **Persistence as seed + deltas** — Untouched world is regenerable; player scars and edits save compactly and reload faithfully.
7. **Consumer-safe public surface** — An external game (and the in-repo harness) share one API story; the harness does not become a second engine.
8. **Credible performance on the target path** — Walkable, dig-responsive, cold-start and memory numbers good enough to trust the crate as foundation (dev platform includes M4/unified memory constraints that shape portable GPU design).

The near-term **Product One** shape — one curated ~1 km region, third-person run-through, static water bodies, dressing (grass/trees as placed matter or scatter), single-save deltas, benchmarked flythrough — is how these outcomes are **proven**, not a separate product identity.

---

## Non-goals

- Shipping a game, combat loop, RPG systems, or AI agents in this repo.
- Implementing System/LLM authorship, spell/gas economies, or fortress/colony simulation.
- Full fluid dynamics, fire ecology, structural integrity, granular settle, weather/seasons/growth as delivered features of the current product (formats and seams may anticipate them; they do not define “done”).
- Tree felling / rigid-body conversion and similar stretch spectacle (nice proof clips, not product identity).
- Building-as-gameplay (blueprints, work orders, mechanisms, room semantics) beyond whatever the substrate API must eventually support for reuse.
- Native Metal (or other platform) forks in load-bearing layers — portability via wgpu/WGSL is intentional.
- Treating the walkable demo’s content (specific biomes, ruin stamp, material list, milestone marketing plan) as the long-term product surface.

---

## Unresolved human questions

Seeds agree on **what** Moria is. These remain for the human where they could still shift design emphasis (not implementation trivia):

1. **Voxel size commitment** — Seeds assume ~25 cm with Product One as the measurement bed for 25 cm vs 12.5 cm (or mixed). Is that decision deferred until harness numbers, or fixed now for API stability?
2. **Public artifact priority** — How hard should near-term delivery optimize for a downloadable walkable demo / audience milestones versus crate-only internal validation? (Does not change product identity; changes sequencing and packaging.)
3. **Downstream first consumer** — Fortress-style, ARPG-with-System, pure sandbox, or unspecified. Only matters for which compatibility seams are designed early; the substrate must not encode one game’s rules.

No seed conflict requires blocking approval: **substrate is the product; harness is validation; games are out of repo.**

---

## Seed contribution map

| Seed | What it contributed to this vision |
|---|---|
| `README.md` | Names the product (Moria), crate consumption model, and harness-as-validator stance. |
| `docs/seeds/project-boundary.md` | Binding boundary: reusable crate(s), no game in-repo, harness on public APIs, game/System/LLM/spell/gas/combat/AI/building layers out of scope; workspace split as consequence of boundary. |
| `docs/seeds/product-one-seed.md` | First shippable proof: product statement, non-goals, dig/place-as-proof, region/player/performance *outcomes* retained at product level; concrete seed world content, materials list, milestone schedule, and stretch timber treated as **context for the harness**, not vision scope. |
| `docs/seeds/voxel-world-substrate.md` | Capability north star for a multi-game substrate (look vs truth, sparsity, geology, dressing rules, fluid tiers, integrity, building verbs, entities/nav, persistence/streaming, layering). Only the outcomes required for a standalone, reusable foundation and for Product One’s selected slice are imported; full architecture, CA suites, building/fortress/ARPG examples, and open tech questions stay reference for later design. |
| `docs/seeds/README.md` | Manifest ordering: Product One binds the current milestone slice; substrate doc is architecture reference; boundary doc is operator clarification. |

**Deliberately not imported into current product scope:** named future games and fantasies (System ARPG, DF fortress, Moria descent roguelike), spell/gas/LLM mechanics, combat and AI, full hydrology and fire ecology, integrity and mechanisms, weather/seasons, entity labor, and any specific authored cast, items, or story.
