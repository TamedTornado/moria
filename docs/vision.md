# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates).

Its first shippable form is **Product One — “The Walkable World”**: the substrate slice needed for one curated generated region, plus a third-person walkable-world **executable that is only a validation harness**. That harness must consume the substrate through the same public interfaces an external game would use. It is not a game layer and must not own privileged or game-specific paths.

The actual game (or games) that will sit on this substrate are **downstream consumers outside this repository**.

---

## Purpose

Make one claim undeniable and reusable:

> This is not a heightmap with props — it is a fully material voxel world that reads as a normal natural world, and it is engineered as a clean substrate for other products.

Moria exists so future games can share one matter foundation: continuous 3D terrain and geology, smooth non-cubic presentation over voxel truth, mutation (dig/place) everywhere, deep underground as real content, streaming and persistence under sparsity, and a public API boundary that keeps game rules out of the world engine.

Product One’s job is narrower and concrete: prove that foundation as a product-shaped demo — walk it, cut it, reload it, and measure it — so substrate choices are answered with evidence rather than guesses, and so later products start from a walkable world instead of a whiteboard.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Generation, matter, meshing/dressing, and the public mutation/query surface needed for Product One | Game rules, combat, stats, AI, economy |
| Walkable-world executable as **public-API validation harness** only | The System / LLM, spells, gas pricing, intent layers |
| Compatibility *seams* where substrate requirements demand them | Building gameplay (UI, blueprints, mechanisms, work orders) — not implemented here |
| Cargo workspace separation between reusable crates and the harness | Full ARPG, fortress/colony, or descent-roguelike products |

**Layering intent (product-level, not a crate map):** generation and matter are substrate; script/API verbs and mirror queries exist so nothing above the matter layer touches voxels directly; semantic and game layers belong to consumers. Gas, if ever present, is a policy injected by a consumer — not a Moria feature.

**Dev-platform constraints that shape the product identity of the crate:** wgpu/WGSL portability (no native Metal fork in load-bearing layers); 32-bit GPU atomics only (Apple GPU constraint); bandwidth-aware sparsity (homogeneous sentinels and lazy materialization are load-bearing, not deferred polish).

---

## Required product-level outcomes

Product One succeeds when all of the following are true at the product level (not as an exhaustive feature list):

1. **Material world, not scenery.** A generated natural region (hills, forest, river/lake channel, cliffs with readable strata, walkable cave depth, sparse micro-objects, one stamped ruin as proof of the prefab path) is continuous and walkable; what you see is backed by mutable voxel truth.
2. **Presentation is a view.** Smooth isosurface meshing (surface nets / dual contouring over dirty bricks) plus material-aware rendering makes the world read as terrain, not a cube grid; collision and queries run against voxels, not the render mesh.
3. **Mutation is the proof.** Dig and place exist on a debug path (not as gameplay systems) so a hillside tunnel mid-run looks like cut earth — the demo differentiator from “any Unity terrain scene.”
4. **Deep Z is real.** A continuous route from surface/canopy-relevant height into underground (~−40m class cave play in the seed region) exercises continuous 3D, underground rendering/light, and geology honesty (strata, aquifer band, ore).
5. **Harness = consumer.** Third-person traversal (run, sprint, jump, surface swim) and debug views exercise the substrate only through public dig/place and query APIs; the harness does not bypass the crate boundary.
6. **Engine-shaped performance and persistence.** Targets that define “done” for this milestone: interactive frame rate on the stated mid-class / M4-class machines; dig-to-remesh without hitch on modest carves; cold start to walkable under lazy materialization; GPU-resident memory under streaming for a region that must not fit as raw voxels; delta save/load that restores edit truth. Scripted benchmark scenes with machine profiles are part of the deliverable so regressions stay comparable.
7. **Reusable asset, not a one-off demo.** The generation pipeline and matter substrate ship as designed for reuse; Product One may stub continent-scale breadth to one curated region, but must not cheapen the geology-first, lazy, sparse architecture into a throwaway scene.

**Substrate capabilities Product One must establish** (so later games can exist without rewriting the world layer) include: brick-pool storage with homogeneous sentinels; per-voxel material/density (and reserved state); column-aware geology generation with lazy brick materialization; GPU dirty-brick meshing; static (tier-1) water bodies; grass/clutter as voxel-derived dressing; voxel objects placed and rendered (trees/boulders) without requiring felling/rigid conversion; streaming rings; worldgen-function-plus-edit-delta persistence; and an internal verb/query boundary that future consumers will share.

---

## Non-goals

Explicitly **not** in current scope:

- Combat, RPG stats, entities beyond the player avatar, or any AI
- The System, LLMs, spells, gas metering, or intent/pricing policy
- Building as a product: placement UI, blueprints, mechanisms, rooms/work orders
- Fluids beyond static bodies (no flow sim, no pressure solve)
- Weather simulation, seasons, growth, fire CA, granular settle, structural integrity / cave-ins
- Voxel-object felling and rigid-body conversion (stretch only; not required for Product One done)
- Embedded scripting languages for content authors
- Multiplayer, multi-save versioning, or cross-mode fortress/ARPG loops
- Implementing any full game mode (ARPG, fortress, descent roguelike) in this repo

Reference material in the substrate seed describes those systems so the **substrate can remain compatible**; Product One does not implement them.

---

## Unresolved questions for humans

These would materially affect identity, purpose, or boundary if answered differently. Technical open questions that Product One is meant to *measure* (e.g. 25 cm vs 12.5 cm voxels, distant LOD strategy, object-registry scaling) are deliberately left to design/benchmarks, not vision.

1. **Product naming in public artifacts.** Repo and crate story use “Moria”; Product One is branded “The Walkable World.” Should public demos, crate names, and audience posts lead with Moria (substrate), Walkable World (demo), or a fixed pairing?
2. **Harness richness.** Is the third-person character controller permanent product surface of the validation executable, or a temporary demo shell that may be replaced by thinner camera/flythrough tools once benchmarks exist?
3. **Stretch “Timber” (felled tree + rigid body).** Confirm it remains optional / out of the Product One acceptance bar, given it couples physics beyond the core matter proof.
4. **Discrete-GPU performance claims.** Product One pins numbers on M4-class and provisional 3060-class targets. Should published “done” claims wait for a discrete-GPU re-baseline, or is M4 + provisional discrete language acceptable for the first public credibility post?

---

## Seed contributions (traceability)

| Seed | Role in this vision |
|---|---|
| **`README.md`** | Names the product (Moria), states substrate-as-crate and harness-as-consumer in one paragraph, and points at the seed set. |
| **`docs/seeds/project-boundary.md`** | **Binding identity and boundary:** reusable Rust substrate; game is out of repo; harness must use public APIs; System/LLM/spell/gas/combat/AI/building layers out of scope; workspace split required. |
| **`docs/seeds/product-one-seed.md`** | **Binding first milestone:** Product One statement, seed-world proof points, which substrate layers ship now vs later, player/harness role, performance outcomes, non-goals, and what Product One buys for later work. |
| **`docs/seeds/voxel-world-substrate.md`** | **Architecture reference and capability horizon:** look problem (smooth mesh over voxel truth), bricks/sparsity, geology-first gen, dressing vs voxel objects, fluid tiers, integrity, building, entities/nav, streaming/persistence, layering diagram, and future game examples (ARPG, fortress, Moria-style descent, System hooks). Used only to retain high-level substrate capabilities and layering rules; gameplay, content, characters, and full implementation of later layers are **not** imported into current scope. |
| **`docs/seeds/README.md`** (manifest note) | Clarifies authority order: Product One selects the required substrate slice; the long substrate doc is reference; project-boundary is the operator clarification that Moria is only the voxel-world substrate. |

**No blocking ambiguity** was found about which product is current: all seeds agree Moria is the substrate, Product One is the first deliverable, and game/System material is context for capability seams only. The questions above are refinements of public framing and acceptance edges, not disputes over product identity.
