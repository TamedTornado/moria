# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates.

It is an engine layer for natural-looking, fully material 3D worlds: continuous terrain and deep underground volume on one voxel truth, smooth rendering as a non-authoritative view, and mutation (dig/place and related matter verbs) through a public API. Game rules, content authorship, and playable game modes do not live here.

A **walkable-world executable** may ship in this repository only as a **validation harness and product-shaped demo**. It consumes the substrate through the same public interfaces an external game would use. It is not a game product and must not own privileged or game-specific implementation paths.

The first concrete delivery shape for that harness (and the substrate slice it exercises) is the “Product One / Walkable World” proof: one generated natural region you can traverse in third person, where dig/place demonstrates that everything visible is mutable matter—not a heightmap with props.

## Purpose

Moria exists so multiple future games—and pure sandbox or tool consumers—can share one matter foundation instead of each reinventing geology, meshing, streaming, mutation, and related world infrastructure.

At vision altitude, the product must make three claims true and reusable:

1. **Reads as a normal world** — rolling terrain, forests, water, cliffs, and similar surface reading, without a blocky “voxel aesthetic” as the primary look.
2. **Mutable everywhere, all the way down** — any material volume can be destroyed, moved, or placed; the underground is real content depth, not a decorative floor.
3. **Substrate, not game** — the same crate stack can underpin different games and modes; rules, economy, combat, AI, LLM/System behavior, and mode-specific UX live above the substrate.

The walkable harness exists to prove those claims with numbers and a short, undeniable demo path (traverse, carve, restore), for internal confidence and external credibility—not to ship a game.

## Product boundary

### In this product

| Belongs to Moria (substrate + harness) | Does not |
|---|---|
| Generation of geological/natural world structure (columns, strata, caves, ores, lazy materialization, POI metadata hooks) as reusable substrate capability | Authored campaign content, quests, or game modes |
| Matter representation: bricks, sparsity/homogeneous regions, material identity, density/occupancy for smooth meshing, state channel capacity | Game-defined material rules that require combat, economy, or LLM authorship as product features |
| GPU meshing of dirty regions; mesh as view only (physics/queries against voxel truth) | Authoritative render-mesh gameplay |
| Surface dressing and voxel-backed objects as substrate mechanisms (e.g. grass scatter, trees/boulders as objects) to the extent needed for a material world and API | Felling-as-gameplay fantasy, species design as game content, or ARPG presentation |
| Static water bodies (tier-1 surfaces/volumes) as substrate capability; higher fluid tiers only as future *substrate* work if retained under the same product identity—not as game hydrology UX | Full fortress hydrology toybox as a game feature set |
| Public mutation and query API (dig/place and mirror-style queries); nothing above matter touches voxels directly | Embedded scripting language, gas/pricing policy, spell packages, work orders |
| Streaming, edit-delta persistence, and performance characteristics appropriate to a large sparse region | Multi-slot campaign saves, cross-mode “reclaim fortress” game loops as product features (delta format may *enable* them later) |
| Walkable-world **harness**: third-person traversal, collision against voxel truth, debug dig/place and inspection tools, curated seed region that exercises proof points | Combat, stats, AI, entities beyond a single player avatar for validation, building UI, blueprints-as-gameplay, System/LLM |

**Repository boundary:** the actual game is a separate downstream consumer and is not part of this repository. Prefer a Cargo workspace split between reusable substrate crates and the validation harness; the exact crate graph is a technical-design decision, but the consumer boundary is not optional.

### Adjacent / downstream (not this product)

- Full games and modes (ARPG with System/LLM, fortress/colony, descent/roguelike, pure sandbox clients)
- Game rules, combat, AI agents, economy, gas/pricing policy, spells
- Building *layer* as gameplay (blueprints, mechanisms, rooms-as-designation, labor)
- Compatibility seams may be *designed* where substrate requirements demand them; those layers must not be *implemented* here

Downstream consumers may imply capabilities the substrate must enable. That implication does **not** pull consumer content, controls, presentation, characters, animation, workloads, or acceptance scenarios into current-product scope beyond what the harness needs to validate the substrate.

## Future products and enabling implications

Described future *consumers* of Moria (not current product identity):

| Future consumer | Role relative to Moria |
|---|---|
| System / LLM-driven ARPG | Game layer; System is a client of mirror + commands + content registries, not a substrate feature |
| Dwarf Fortress–style fortress / colony | Game layer on the same matter + API stack |
| Moria-style descent / adventure | Game layer emphasizing deep-Z and geology-gated progression |
| Pure sandbox / tool hosts | Consumers that may price mutation at zero or omit game systems |

**High-level enabling implications** (substrate should eventually make these *possible*; they do not import consumer design into current scope):

- **Matter + mutation API** so dig, place, and related verbs are shared across modes; gas/labor pricing is a policy plug-in above the substrate.
- **Geology-first generation and deep Z** so underground play and “honest dig-down” are real without per-game rewrites.
- **Smooth meshing with sharp cut faces** so player and structure edits read as cuts, not decorative mesh edits.
- **Column/index and sparse brick model** so large regions, Z-slice *views*, and lazy materialization are feasible for fortress-style and open-world consumers.
- **Object layer + dressing** so interactable vegetation/props and cheap ground clutter can stay consistent with matter truth.
- **Tiered fluids, integrity, granular settle, fire/ambient sim** (full substrate ambition in the design seed)—compatibility and data capacity may be planned; implementation depth beyond the Product One slice is substrate roadmap, not game-layer work, and is not committed as a near-term acceptance bar in this vision.
- **Delta persistence and multi-anchor streaming** so later modes can reuse edited worlds; cross-run “fortress as dungeon” is a consumer use of the format, not a current deliverable.
- **POI / metadata hooks** so higher layers (including a future System) can place and annotate without owning geology.

Do **not** treat future-game controllers (e.g. locked ARPG camera), characters, combat, animation, or mode-specific UX as Moria scope.

## Non-goals

- Shipping a game, campaign, or multiplayer service in this repository
- System / LLM integration as a product feature
- Combat, stats, AI, entities beyond harness-needed player avatar
- Gas, pricing, intent, spells
- Building UI, blueprints-as-gameplay, mechanisms, work orders, room designation gameplay
- Fluids beyond static bodies in the first proven slice (no flow simulation as current acceptance)
- Weather simulation, seasons, growth simulation as current acceptance (fixed time-of-day for the harness is enough)
- Persistence productization beyond seed + edit deltas suitable for harness validation (no multi-slot versioning product)
- Native Metal (or other backend) forks in load-bearing layers; portability via portable GPU abstraction is the intent
- Making the primary surface aesthetic “raw cubes” (raw voxel view remains a debug path)

## Confirmed vision constraints

Only constraints present in the seeds and appropriate at vision altitude:

1. **Product is the substrate**, exposed as Rust crate(s); the walkable executable is a consumer/harness, not the game.
2. **Harness uses only public substrate interfaces** (no privileged game-only paths inside the crate boundary).
3. **Voxel truth is authoritative** for interaction and queries; extracted mesh is a regenerated view, not saved truth.
4. **GPU-resident world substrate** with sparse brick-oriented storage and dirty-region meshing as the intended architecture class (exact layouts and algorithms are design-level).
5. **Generation is geology-capable and lazy**—world cost stays low until touched; deep underground is first-class.
6. **Dig and place remain in the first proof** (harness debug tools)—not as gameplay systems, as evidence of full material mutability.
7. **Game layers listed in the boundary seed stay out of implementation** (System/LLM, spell, gas, combat, AI, building layers), though seams may be designed.
8. **Portable GPU stack** (wgpu/WGSL-class approach stated in seeds)—no native Metal fork in load-bearing layers.
9. **Dev target class includes Apple silicon unified-memory constraints** that forbid relying on 64-bit buffer atomics; bandwidth-conscious sparsity is load-bearing for that class. Discrete-GPU targets remain provisional until re-baselined on other hardware.

Exact region dimensions, voxel edge length choice, brick size, frame-time/memory thresholds, material inventories, milestone calendars, and benchmark scene scripts are **downstream design/acceptance**, not vision constraints—even where seeds state numbers.

## Assumptions proposed for approval

1. **Primary identity:** “Moria” names the **substrate product** in this repository. Future games (including a possible descent game that reuses the Moria fantasy) are separate products that *consume* Moria.
2. **First delivery shape:** Product One (walkable region + third-person harness + dig/place proof + substrate slice in the product-one seed) is the **near-term definition of a complete first vertical proof**, not a second product. Broader substrate capabilities (multi-tier fluids, integrity, CA ecology, richer object behavior, full building verbs beyond dig/place, nav aggregates, etc.) remain **in-product substrate ambition** for later slices, not transfers of fortress/ARPG gameplay into this repo.
3. **Tree felling / rigid conversion** is stretch for the first proof, not required for product identity.
4. **Public demo / audience artifact** (downloadable harness build, milestone clips) is a desired outcome of the first proof, secondary to the reusable crate boundary.
5. **Open technical choices** in the substrate seed (final voxel size, distant LOD strategy, object-layer scaling, fluid pressure model, multiplayer timing) stay **unsettled at vision**; they do not fork product identity.

## Questions for human review

**Q1.** Is the near-term “done” bar for *this* product the Product One walkable proof (harness + substrate slice that makes the material-world claim undeniable), or crate/API maturity with the harness optional?

- **Proposed:** Product One walkable proof is the near-term done bar; the crate boundary and public API are mandatory from the start; the harness is required validation, not optional tooling.
- **If harness-optional:** Purpose and boundary stay substrate-first, but “What we are building now” drops playable third-person demo and curated seed-world proof points from current scope.

**Q2.** Should full substrate systems beyond the Product One slice (e.g. integrity/cave-ins, fluid flow tiers, fire/granular CA, stamp/blueprint APIs, nav aggregates) be treated as **same-product later work** under Moria, or as **explicitly out of current product vision** until a later vision revision?

- **Proposed:** Same-product later work—owned by the substrate identity, not by game consumers—with **no near-term acceptance commitment** in this vision document.
- **If out until revision:** Non-goals and boundary should list those systems as non-goals for Moria-as-now; enabling implications for fortress/ARPG shrink accordingly.

**Q3.** Confirm naming: keep **Moria** as the substrate/repository product name even though seeds also use “Moria” for a future descent-style *game*?

- **Proposed:** Yes—Moria = substrate product; future descent game is a separately named (or later-named) consumer.
- **If no:** Rename product or game in a follow-up so downstream docs do not collide.

## Seed synthesis

| Seed | Role in this fusion |
|---|---|
| **`README.md`** | Anchors current product identity: reusable GPU-resident voxel-world substrate as a Rust crate; walkable-world executable is consumer/validation harness, not a game layer. Points at `docs/seeds/` as preserved inputs. |
| **`docs/seeds/project-boundary.md`** | **Binding boundary authority.** Product = substrate crate(s); actual game out of repo; harness must use public interfaces; Cargo workspace split preferred; game rules and System/LLM/spell/gas/combat/AI/building layers out of scope (seams only). Overrides any reading that would make the walkable demo or a full game the product. |
| **`docs/seeds/product-one-seed.md`** | Defines the **first proof shape**: natural generated region, third-person traversal, dig/place as mutability proof, partial substrate slice (generation + matter partial + API sliver), explicit non-goals for game systems, and harness/debug affordances. Supplies purpose-level claims (“not a heightmap with props”) and performance *class* of ambition. Concrete dimensions, material lists, milestone order, and numeric targets are preserved only as design-seed context—not vision constraints or feature inventory. “Product two” pointers are future consumers, not current scope. |
| **`docs/seeds/voxel-world-substrate.md`** | Supplies **substrate purpose and long-horizon capability shape**: normal look vs voxel truth, mutability, deep Z, layering (generation / matter / script-API / semantic / game), GPU-resident sparse bricks, geology-first gen, objects vs dressing, fluid tiers, integrity, building verbs, persistence/streaming, and multi-game reuse. Used to fuse identity and enabling implications. Implementation detail, algorithms, bit layouts, build-order catalogs, and open engineering questions are **not** lifted into vision constraints; future game fantasies illustrate *why* seams exist, not what ships in Moria now. |

**Contradiction handling:** Product One reads like a small game; the boundary seed and README reclassify it as harness + first substrate slice. The full substrate seed describes fortress/ARPG/System attachment points; the boundary seed forbids implementing those layers here. This brief keeps **one product** (the substrate), one **near-term proof** (Product One harness), and **future games as consumers**, with Q1–Q3 offered only where delivery bar, later substrate depth, or naming could still change the brief.
