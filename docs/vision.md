# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is the matter-and-world foundation for later games: sparse voxel storage, geology-oriented generation with lazy materialization, smooth GPU meshing of mutable voxel truth, public mutation and query APIs (including dig/place), and collision against that voxel truth.

It is **not** a game. The in-repo walkable-world executable is a **validation harness** — an adjacent consumer that must exercise the substrate only through the same public interfaces an external game would use. Character control, camera, demo content, presentation, and acceptance scenarios belong to that harness (or to later games), not to the product identity of Moria.

Whether **streaming** and **persistence** are part of the first committed substrate envelope is open (see Q3); the README lists them among harness validation concerns, but Product One schedules them late and this brief does not treat them as settled “now” until review answers.

## Purpose

Moria exists so that multiple future games — adventure, fortress/colony, sandbox, and related modes — can share one honest material world: terrain and underground that read as a normal landscape, remain fully mutable, treat depth as first-class content, and stay free of any LLM or game-rule dependency in the engine layer.

The substrate’s job is to make one claim trustworthy for consumers and for validation: **the world is continuous voxel matter, not a heightmap with props; the mesh is a view; gameplay-relevant physics and edits run against the voxels.**

## Product boundary

| In this product (Moria) | Adjacent / not this product |
|---|---|
| Reusable voxel-world substrate (Rust crate or small crate family) | The actual game(s) — separate downstream consumers, not this repository’s product |
| Public APIs for generation, matter access, meshing-backed presentation data, dig/place and related verbs, queries, and events suitable for an external game | Game rules, combat, stats, AI, economy, quests, spells, gas/pricing policy, LLM/System behavior |
| GPU-resident brick/matter representation, sparsity, lazy materialization, smooth surface meshing from voxel data | Character controllers, cameras, HUD, demo routes, authored “seed world” scenery inventories, trailer presentation |
| Validation harness **as a consumer** of those public interfaces (workspace-separated) | Privileged or game-specific paths inside the substrate that only the harness may use |
| Compatibility **seams** where substrate requirements need them (hooks, registries, extension points) | Implementing System, spell, gas, combat, AI, or full building/fortress **layers** here |

**Workspace rule (product boundary, not crate topology):** keep a Cargo workspace boundary between reusable substrate and validation harness. Exact crate split is a later technical decision; the consumer boundary is not optional.

**Harness rule:** any walkable-world executable is a validation harness that consumes public interfaces only. Seeds list terrain generation, streaming, meshing, editing, collision, persistence, and performance-relevant behavior as harness concerns; which of those are **current substrate product** versus later or harness-only is constrained by the unambiguous core above and by Q1–Q4. Harness-specific controller, character, content, presentation, route, workload, platform gates, and performance thresholds are **not** substrate product scope.

## Future products and enabling implications

Described consumers and long-horizon modes that **are not current Moria product scope**:

- **System / LLM ARPG** — game layer client of the substrate; spells, gas policy, and System authorship sit above.
- **DF-style fortress / colony** — building, labor, designations, Z-slice UX, mechanisms as gameplay.
- **Moria-style descent / adventure** — danger-at-depth fantasy as game content on the same matter world.
- **Pure sandbox** — zero-priced verbs; still a consumer policy, not substrate identity.
- **“Product two”** after a walkable-world proof (fortress toybox or ARPG) — starts from a proven substrate, not from a whiteboard.

**Enabling implications** (high level only; not a committed roadmap or feature inventory):

- Keep the substrate **game-agnostic**: priced verbs and policies inject above matter; nothing above matter touches voxels except through public verbs/queries.
- Prefer representations that later support **deep dig, natural geology, surface dressing vs voxel objects, fluid bodies, structural integrity, and building stamps** without baking ARPG or fortress rules into the core.
- Design **metadata / POI / material registry seams** so a future System or hand content pipeline can place and extend without owning geology.
- If persistence ships, prefer **generation function + edit deltas** (and object/event journals where needed) so later modes can reclaim or reuse world scars.
- Streaming/sim **rings** and mirror/command style coupling remain architectural implications for GPU-resident multi-consumer use when those capabilities are in scope; exact schemes are design work.

Do **not** import future consumers’ gameplay, content, presentation, controllers, characters, animation, combat, or acceptance scenarios into current Moria scope.

## Non-goals

- Shipping a game, ARPG loop, fortress mode, or downloadable “game product” as Moria itself.
- Implementing System/LLM integration, spells, gas metering/policy, combat, stats, AI agents, or full building/blueprint/mechanism **gameplay layers** in this repository’s product.
- Treating the validation harness’s character, camera, demo route, seed-region scenery list, or benchmark theater as substrate features.
- Fluids beyond what the approved current slice needs; full per-voxel fluid CA; weather/seasons/growth sims; fire CA; structural integrity sim; granular settle — as **committed current product** (format or seams may anticipate them; running those systems is not assumed here).
- Making the surface aesthetic a raw cube-Minecraft look as the primary product look (debug raw-voxel views are fine; the intended read is a normal world over voxel truth).
- Embedding LLM dependency in the substrate (the substrate must stand alone).
- Expanding this vision into a GDD, full technical design, crate map, asset list, milestone catalog, or acceptance spreadsheet (downstream of vision approval).

## Confirmed vision constraints

Only constraints explicit in the seeds and appropriate at vision altitude:

1. **Product identity:** Moria is the reusable voxel-world substrate, not the game.
2. **Repository boundary:** the actual game is a separate downstream consumer and is not part of this product.
3. **Harness is not a game layer:** any walkable-world executable is a validation harness and must use public substrate interfaces only — no privileged game-only paths in the core.
4. **Workspace separation:** Cargo workspace boundary between substrate and harness is required; precise crate graph is not fixed here.
5. **GPU-resident substrate:** brick/matter path is GPU-resident in the sense established by the seeds (commands in, mirror/events out style coupling is an implied architecture family, not a named library mandate).
6. **Voxel truth, mesh as view:** physics, queries, and mutation authority live on voxels; extracted meshes are regenerated views, not saved authority.
7. **Mutable continuous 3D matter:** dig/build-anywhere and deep-Z underground are first-class substrate aims, not decorative floors under a heightmap.
8. **Natural-looking surface over voxel grid:** the voxel grid is the truth, not the required cube aesthetic; smooth extraction for terrain/structures is the intended primary look direction.
9. **Zero LLM dependency in the substrate:** System/LLM may be future clients; they are not substrate features.
10. **Out of implemented scope here:** game rules and the future System, LLM, spell, gas, combat, AI, and building **layers** (seams only where requirements demand them).
11. **Portable GPU stack intent:** load-bearing GPU work stays on a portable path (wgpu/WGSL called out in seeds); no native Metal-only fork in load-bearing layers.

## Assumptions proposed for approval

1. **Name and identity are settled:** “Moria” refers to this substrate project; renaming or treating the harness as the product would contradict the explicit boundary seed and is not proposed.
2. **README harness list is a validation concern list, not an automatic “all current” mandate:** generation, meshing, editing, and collision against voxel truth are treated as core; streaming and persistence wait on Q3; performance numerics wait on Q4.
3. **“Editing” at vision altitude includes dig and place** through the public API, because both the boundary/README validation list and the product-one proof claim depend on mutation, not view-only terrain (building *gameplay* remains out — Q5).
4. **One curated region is enough for first validation** of generation (and of streaming *if* streaming is in the approved envelope); infinite world or multi-region shipping is not assumed now.
5. **Static fluid *bodies* (lakes/river surface as volumes)** are not committed in “What we are building now”; they ride with the Product One slice under Q1-A if approved.
6. **Surface dressing and voxel objects** are not committed in “What we are building now”; they are part of the Product One slice proposed under Q1-A, not smuggled into confirmed scope.
7. **Dev machine anecdotes** (e.g. M4 Mac Mini limits, specific FPS/resolution tables) inform later design and harness benchmarks; they are not vision-level product identity.

## Questions for human review

**Q1 — First deliverable substrate depth**

Product One describes a first *substrate slice* (generation for a curated region; brick pool + lazy materialization + GPU dirty-brick meshing; static water bodies; grass/clutter dressing; voxel-object placement/render without felling; dig/place API sliver; no CA/fire/flowing fluids/integrity/granular settle). The full substrate seed describes a much larger matter stack.

Which is the approved **current-product** capability envelope?

- **A (proposed):** Product One’s substrate slice (above), as the committed “now,” with deeper matter systems only as format/seams or later work.
- **B:** Thinner core only (bricks, meshing, dig/place, minimal gen) — dressing and voxel objects deferred.
- **C:** Broader than Product One toward the full substrate seed (name which systems become current).

*How answers change the brief:* A keeps generation + meshing + mutation + light surface/object support in “now.” B narrows “now” and moves dressing/objects to future. C expands confirmed scope and shrinks non-goals.

**Q2 — Validation harness shape**

Seeds require a validation harness that consumes public APIs and may be a walkable-world executable. Product One specifies third-person character, free-orbit camera, swim, cave route, and debug keys.

Is a **walkable third-person demo** a required in-repo deliverable form for validation, or is **any** harness that exercises the public APIs (including non-character benchmarks/tools) sufficient?

- **A (proposed):** A walkable-world harness is the intended primary validation vehicle, but its controller, camera, character, and route remain **harness-owned** and outside substrate product scope.
- **B:** No walkable character requirement — headless or tool harnesses are enough if APIs and performance paths are covered.
- **C:** Walkable demo presentation is in scope for the *repository deliverable* even though it is not the substrate crate’s API surface (document it as a required adjacent artifact).

*How answers change the brief:* A/C keep a playable harness in the delivery story without turning Moria into a game; B removes walkability as a project expectation.

**Q3 — Persistence and streaming in “now”**

README lists streaming and persistence among harness validation concerns; Product One includes delta save/load and streaming rings in its later milestones; the substrate seed makes seed+deltas and rings foundational.

Are **streaming** and **persistence (seed + edit deltas)** required capabilities of the **current** substrate product?

- **A (proposed):** Yes — both are current substrate responsibilities at vision altitude (mechanics and thresholds remain design/harness work).
- **B:** Meshing/edit/gen first; streaming/persistence may follow in a near subsequent slice without blocking “substrate exists.”
- **C:** Persistence only / streaming only (specify).

*How answers change the brief:* A keeps both in “What we are building now.” B moves one or both to near-term follow-on and adjusts purpose/boundary wording.

**Q4 — Performance claims at vision altitude**

Product One states numeric targets (frame rate, dig-to-remesh latency, cold start, memory, save size) and scripted benchmark scenes. These are not identity-defining the same way “GPU-resident substrate” is.

Should the vision treat **quantitative performance targets** as:

- **A (proposed):** Out of vision — owned by harness/benchmarks and later design; vision only requires that the substrate be built so performance is measurable and regression-testable through public use.
- **B:** In vision as soft goals (cite categories only, no numbers).
- **C:** In vision with Product One’s numbers as product requirements.

*How answers change the brief:* A/B keep numbers out of confirmed constraints; C promotes them into constraints and acceptance ownership.

**Q5 — Dig/place as proof vs building as product**

Product One keeps dig/place on debug keys as *proof* of mutability, not as building gameplay. The substrate seed describes first-class placement, stamps, blueprints, and mechanisms for future fortress modes.

Confirm current product stance:

- **A (proposed):** Dig/place (and related mutation/query APIs) are in substrate scope; **building gameplay** (blueprints, work orders, mechanisms as game entities, room detection) is out of current product and listed only as future enabling seams if needed.
- **B:** Broader placement/stamp **API** is current; still no fortress UX/game rules.
- **C:** Something else (specify).

*How answers change the brief:* Clarifies non-goals vs script/API surface without pulling fortress mode into Moria-now.

## Seed synthesis

| Seed | Role in this fusion | What it contributed | What it did **not** dictate |
|---|---|---|---|
| `README.md` | Top-level product one-liner | Name **Moria**; reusable GPU-resident voxel-world substrate as Rust crate; walkable-world executable is separate **consumer/validation harness** for generation, streaming, meshing, editing, collision, persistence, performance — not a game layer; seeds live under `docs/seeds/` | Harness implementation, content, or numeric gates |
| `docs/seeds/project-boundary.md` | **Authoritative boundary correction** | Product = substrate crate(s); game is downstream and not this repo’s product; harness must use public APIs; Cargo workspace boundary required; game rules and System/LLM/spell/gas/combat/AI/building **layers** out of scope; seams allowed, implementations of those layers not | Crate graph details; harness feature list; full matter-stack roadmap |
| `docs/seeds/product-one-seed.md` | First vertical proof and proposed substrate *slice*; also describes harness/demo shape | Non-goals that reinforce “no game”; dig/place as mutability proof; proposed first substrate depth; curated-region validation idea; portable wgpu intent; milestone *spirit* (prove hill → carve → geology → dress → traverse → numbers) | Did **not** redefine product identity as “the walkable game”; character/camera/seed inventory/performance tables/milestone schedule were **not** lifted into committed current-product scope — held as harness/design or Q1–Q4 |
| `docs/seeds/voxel-world-substrate.md` | Long-horizon substrate design and multi-game rationale | Purpose (normal-looking mutable world, deep Z, substrate-not-game); layering model (generation / matter / script-API / semantic / game); voxel truth vs mesh view; sparsity/lazy materialization; future consumers (ARPG, fortress, sandbox); enabling implications (fluids tiers, integrity, building verbs, nav, persistence model); zero LLM dependency | Exact voxel size, brick dimensions, algorithms, bit layouts, CA rules, benchmark thresholds, and build-order checklists stay **downstream design**; full stack is not silently “all current” |

**Contradiction handling:** Product One’s “product-shaped demo” language is subordinated to the explicit boundary and README: the **current product** is the substrate; the walkable world is the **harness/consumer**, not a second product identity. Where Product One and the full substrate seed disagree on depth, this brief keeps only the unambiguous substrate identity in committed sections and places first-slice depth, harness shape, streaming/persistence timing, and performance numerics in **Questions for human review**.

**Gap handling:** Seeds do not fully settle first deliverable depth, whether walkability is mandatory for the harness, whether streaming/persistence are day-one substrate requirements, or whether performance numbers belong in vision. Those are Q1–Q5 with proposed answers so the brief can be approved or corrected claim-by-claim.

---

*This document is the proposed canonical vision for downstream design. Until human review approves it, disputed expansions remain only in the questions section above.*
