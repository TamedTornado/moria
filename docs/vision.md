# Moria — Vision

*Proposal for human approval and handoff to downstream design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). It is engine-layer infrastructure: matter, generation, meshing, mutation, queries, streaming, and persistence for continuous 3D voxel worlds.

The repository may ship a **walkable-world executable**. That binary is a **validation harness and product-shaped demo**, not a game. It must consume the substrate only through the same public interfaces an external game would use.

The first product slice is **Product One — “The Walkable World”**: one curated generated region, a third-person character who can traverse it, and dig/place as proof that the surface is mutable voxel truth—not a heightmap with props.

---

## Purpose

Make one claim undeniable and reusable:

> This is a fully material world that reads as a normal natural landscape, supports continuous deep-Z play, and can be mutated anywhere—and it is packaged so future games sit on top of it rather than inside it.

Product One proves that claim with a runnable artifact and measurable performance numbers. The substrate exists so later products (ARPG, fortress/colony, descent roguelike, pure sandbox) start from a walkable world instead of a whiteboard.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Generation, matter storage, meshing/dressing, dig/place verbs, collision against voxel truth, streaming, delta persistence | Any full game title or game rules layer |
| Public substrate API and the validation harness that exercises it | System / LLM, spells, gas/pricing policy, combat, AI agents, building gameplay UI |
| Compatibility *seams* only where substrate requirements demand them | Fluids beyond static bodies, CA (fire/wetness/granular settle), structural integrity, weather/seasons/growth sims |
| Seed-curated one-region world as demo content | Multi-region continents, multiplayer, scripted content pipelines beyond what’s needed to demo the substrate |

The consumer boundary is non-optional: a Cargo workspace (or equivalent) must separate reusable crates from the harness. Game-specific implementation paths must not live under privileged substrate internals.

Future games and systems described in the substrate architecture seed are **context for capability ambition**, not current scope. Preserve the high-level outcomes those products would need (mutable matter, deep Z, clean layering, GPU residency, public verbs/queries). Do **not** import their gameplay, content, characters, assets, or implementation into Moria now.

---

## Required product-level outcomes

Product One is “done” when these outcomes hold—not when every architecture idea is implemented.

1. **Material world, not painted terrain.** A generated natural region (hills, forest, river/lake, cliffs, caves, sparse micro-objects, one stamped ruin) is walkable end-to-end; what you see is backed by voxel truth.
2. **Smooth look, voxel authority.** Surface meshing presents organic terrain and honest cut faces; physics and queries run against voxels, not the render mesh.
3. **Mutable everywhere (proof).** Debug dig/place can carve and fill mid-traversal; remesh stays interactive; cut earth reads as cut earth.
4. **Deep Z is first-class.** Continuous traversal from surface features into walkable underground (cave route, geology visible when cut) without a skybox floor.
5. **Substrate, not demo code.** Generation, brick pool / sparsity, lazy materialization, meshing, dressing, and dig/place live behind a public API the harness uses like any external consumer.
6. **Credible performance & persistence.** Targets in the Product One seed (frame rate, dig-to-remesh latency, cold start, GPU memory under streaming, delta save/load) are demonstrated by a scripted benchmark with machine profile—so later substrate changes can regress against Product One.
7. **Decision bed.** Open substrate choices (e.g. voxel size, LOD, object-layer scale) can be answered with measurements from this region rather than speculation.

---

## Non-goals

Explicitly out of current product scope:

- Combat, stats, AI, NPCs/entities beyond the player controller
- The System, LLMs, spells, gas, intent/pricing economies
- Building UI, blueprints-as-gameplay, mechanisms, work orders, rooms/economy
- Full fluid simulation (flow, pressure); only static tier-1 water bodies
- CA-driven fire, wetness, granular settle, structural integrity / cave-ins
- Weather, seasons, growth simulation (a fixed time-of-day control is enough)
- Rich multi-slot versioned saves; only seed + deltas / single-slot restore
- Tree felling / rigid-body conversion (stretch only; not required for “done”)
- Implementing future game modes or shipping game content as the product

---

## Unresolved questions for humans

Seeds align on **what** the product is (substrate + Product One harness). These remain open for human or measurement-driven call and would affect identity or hard constraints if answered differently:

1. **Voxel size final call** — 25 cm is the working assumption; 12.5 cm is a cost/fidelity trade. Product One’s region is the benchmark bed. Confirm whether “done” freezes one size or allows a measured choice after the demo.
2. **Crate packaging** — one crate vs. a small family: consumer boundary is required; exact split is deferred to technical design. Confirm no preference that would change the public product shape.
3. **Discrete-GPU performance targets** — stated as provisional until a Linux/discrete baseline is available; M4/unified is the verified dev constraint. Confirm whether public “done” claims may rest on M4 + provisional discrete numbers.
4. **Stretch milestone (timber/felling)** — explicitly optional. Confirm it stays non-blocking for Product One acceptance.

No seed conflict requires guessing which product is current: the operator boundary and Product One seed bind implementation; the full substrate design is reference architecture, not the current milestone backlog.

---

## What each seed contributed

| Source | Role in this vision |
|---|---|
| **README.md** | Names the product (Moria), crate-first framing, and harness-not-game stance. |
| **docs/seeds/project-boundary.md** | Binding boundary: substrate is the product; game layers out of scope; harness uses public APIs; workspace split required. |
| **docs/seeds/product-one-seed.md** | Binding first slice: walkable demo claim, non-goals, seed-world proof points, player/traversal expectations, performance outcomes, milestones, what Product One “buys.” Detailed feature lists and milestone schedules are summarized as outcomes, not imported as a plan. |
| **docs/seeds/voxel-world-substrate.md** | Architecture reference and long-horizon capability surface (geology-first gen, brick sparsity, smooth meshing, object vs. dressing, fluid tiers, integrity, building, streaming/persistence layering). Only capabilities needed for Product One and for *supporting* future consumers at the product level are reflected above; game-mode examples (ARPG, DF fortress, Moria descent), CA/systems depth, and full layer stack are treated as context. |
| **docs/seeds/README.md** *(index)* | Confirms the same binding order: Product One selects the milestone; substrate doc is reference; broader game/System intent is deliberately absent. |

### Omitted or de-scoped source material (visible on purpose)

From the substrate architecture seed, **not** current vision scope except as future-facing seams: weather/fire ecology, multi-tier fluids, structural integrity, building/mechanisms/rooms, entity labor and pathfinding classes, multiplayer, System-authored content, full CA rule tables, and complete multi-game layer diagrams.

From Product One: **concrete** material palettes, exact region dimensions, keybind lists, milestone week estimates, and per-metric numeric tables belong in design/spec follow-ons; this vision keeps only the product-level claims those details serve.

---

*End of vision proposal. Approve, amend, or resolve the open questions before treating this as the handoff baseline for design.*
