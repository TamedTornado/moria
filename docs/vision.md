# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a
Rust crate or a small family of tightly scoped Rust crates.

The substrate is the product. It owns the material world: generation, storage,
meshing, mutation verbs, streaming, persistence of world truth, and the public
API surface through which any external game must interact with voxels.

This repository may also ship a **walkable-world executable**. That executable
is a **validation harness and product-shaped demo**, not a game. It consumes
the substrate only through the same public interfaces available to an external
consumer. It exists to prove, measure, and demo the substrate—not to host game
rules, progression, or content systems.

The first concrete delivery slice is **Product One ("The Walkable World")**: one
generated natural region plus a third-person character that can traverse it,
with dig/place as debug proof that the world is fully material. Product One
selects which substrate layers ship first; it does not redefine the product as a
game.

## Purpose

Moria exists so future games can stand on a single world foundation where:

1. The surface **reads as a normal natural world** (terrain, forest, water,
   cliffs)—the voxel grid is truth, not the aesthetic.
2. **Matter is mutable everywhere**, surface to deep underground—no decorative
   geometry outside the material world.
3. **Deep Z is first-class**—caves, strata, ore, and continuous vertical play
   are substrate capabilities, not skybox floors.
4. The same crate stack can underpin multiple game genres without rewriting the
   world layer.

The substrate must stand alone with **zero dependency on LLM/System features**.
Those are future game-layer clients.

Product One’s job is to make one claim undeniable in a short demo: this is not a
heightmap with props—it is a fully material world, and it looks good. Dig and
place on a debug key are the proof, not building gameplay.

## Product boundary

| In this product (Moria) | Outside this product |
|---|---|
| Reusable voxel-world substrate crate(s) | The actual game(s) as products or repos |
| Generation, matter, meshing, streaming, persistence of world truth | Game rules, combat, stats, AI, economy |
| Public mutation/query API (verbs + mirror) | System / LLM orchestration and content authorship |
| Compatibility seams the substrate itself requires | Spells, gas policy, pricing of player power |
| Walkable-world executable as public-API validation harness | Building UI, blueprints-as-gameplay, mechanisms-as-gameplay |
| Cargo workspace boundary: substrate vs harness | Character fantasy, ARPG/fortress/roguelike loops |

**Rules that define the boundary:**

- The game is a **separate downstream consumer** and is not part of this
  repository’s product identity.
- The harness must not own privileged or game-specific implementation paths
  into voxels. Nothing above the matter layer touches voxels except through
  verbs and queries.
- Game rules and future System, LLM, spell, gas, combat, AI, and building
  *layers* are out of scope to implement here. Seams may be designed where
  substrate requirements demand them; those layers are not built in Moria.

**Product One is the first vertical slice of the substrate**, not a second
product: full generation layer (continent pass may be stubbed to one curated
region), partial matter layer (brick pool, lazy materialization, GPU
dirty-brick meshing, grass/clutter dressing, voxel-object placement/render,
static tier-1 water), and a thin script/API sliver (dig/place + mirror queries
for the harness). CA, fire, fluids beyond static bodies, integrity, granular
settle, and tree felling/rigid conversion are deferred substrate work, not
game work.

## Future products and enabling implications

These are **consumers of Moria**, not current deliverables. Their gameplay,
controllers (beyond the harness), characters, animation, presentation, and
content remain consumer-owned.

| Future consumer (directional) | What it needs from the substrate (high-level only) |
|---|---|
| System / LLM-driven ARPG | Mutable material world, mirror queries, command/event coupling, material/POI metadata the game layer can read and author against |
| DF-style fortress / colony | Deep-Z geology, column/Z-slice-friendly derived data, dig/build verbs, later integrity/fluids/support as substrate capabilities |
| Moria-style descent / adventure | Continuous 3D world with honest underground, strata/ore/caves as generated truth |
| Pure sandbox | Same verb/query surface with game-specific pricing policy set outside the crate |

**Enabling implications retained at vision altitude (not design commitments for
Product One):**

- **GPU-resident brick world** with homogeneous sentinels and lazy
  materialization so large regions stay tractable.
- **Mesh is a view**; physics and queries run against voxel truth.
- **Truth = worldgen function + edit deltas** for persistence and streaming.
- **FleX-pattern coupling** (commands in; mirror + events out) so discrete GPUs,
  sandboxing, and eventual multiplayer-authoritative shapes remain viable.
- **Layering** so gas/pricing, System authorship, and game rules inject above
  the substrate rather than inside it.
- Deferred matter capabilities (fluids tiers, integrity, CA, granular, richer
  object physics) stay **in the substrate product roadmap** after Product One;
  they are not reassigned to game repos by default.

Do not import consumer-specific mechanics (spells, labor, siege AI, town
economy, Diablo camera lock, etc.) into Moria’s scope.

## Non-goals

For the current product and for Product One explicitly:

- Combat, stats, AI, and entities beyond a single harness player avatar
- The System / any LLM dependency or feature in the substrate
- Gas, pricing, intent, and spell systems
- Building UI, blueprints-as-gameplay, mechanisms, work orders
- Fluids beyond static bodies (tier-1 lakes/river surface only in Product One)
- Weather simulation, seasons, growth simulation (fixed time-of-day control is
  enough for the demo)
- Persistence beyond reload of the same seed + deltas (single save slot, no
  versioning scheme as a product feature)
- Implementing full fortress, ARPG, or descent game loops in this repository
- Treating the walkable harness as the shipped game or as a privileged second
  world stack
- Expanding this vision into a GDD, full architecture, or feature catalog
  (downstream of vision approval)

## Confirmed vision constraints

Only constraints stated in the seeds:

1. **Product identity**: Moria is the reusable voxel-world substrate (Rust
   crate / small crate family), not the game.
2. **Harness rule**: Any walkable-world executable consumes the substrate
   through the same public APIs as an external game; no privileged game paths.
3. **Workspace rule**: Cargo workspace (or equivalent) separates reusable
   substrate from the validation harness; the consumer boundary is mandatory.
   Precise crate split is a later technical decision.
4. **No LLM/System in substrate**: Substrate stands alone; System is a future
   game-layer client.
5. **Out of scope layers**: Game rules; System/LLM; spell; gas; combat; AI;
   building layers—not implemented here.
6. **Product One region scale (first slice)**: Curated ~1 km × 1 km × 256 m
   generated region; whole region must not fit in memory as raw voxels
   (sparsity/streaming are real).
7. **Product One voxel/brick baseline for the benchmark bed**: 25 cm voxels,
   16³ bricks (final voxel size may be measured against this bed).
8. **Product One proof of mutability**: Dig and place remain in scope on a
   debug path; collision is against voxel truth, not the render mesh.
9. **Product One rendering direction**: Smooth isosurface extraction (surface
   nets / dual contouring) for terrain; mesh is non-authoritative.
10. **Dev-platform constraints (Product One)**: M4 Mac Mini / unified memory
    development; **no 64-bit buffer atomics** (32-bit counters/allocators);
    bandwidth-conscious design (sparsity and homogeneous sentinels are
    load-bearing); **wgpu/WGSL only** in load-bearing layers—no native Metal
    fork; discrete-GPU targets are provisional until re-baselined on discrete
    hardware.
11. **Product One performance targets** (harness + substrate slice as the
    measured product): 60 fps at stated resolutions on mid GPU / M4 class;
    dig-to-remesh within ~2 frames for modest carves; cold-start to walkable
    under ~5 s; full region under ~2 GB GPU resident with streaming;
    delta save size and exact restore after defacement as stated in the
    Product One seed.
12. **Seed authority after approval**: This vision supersedes raw seeds for
    downstream design; seeds remain historical source material.

## Assumptions proposed for approval

**A1.** “What we are building now” is the **substrate through Product One**.
Later substrate milestones (CA, fire, fluid tiers 2+, integrity, granular,
felling/rigid objects, richer ambient sim) remain in-repo substrate work after
Product One unless human review reassigns them. They are not “the game.”

**A2.** Product One’s seed world composition (meadow, forest, river/lake,
cliffs/strata, karst cave, aquifer/ore, micro objects, one stamped ruin),
materials palette sketch, third-person free-orbit harness controller, and
debug dig/place tools define the **first acceptance story** for the substrate
slice—not optional flavor text.

**A3.** The full `voxel-world-substrate` document is a **directional
architecture reference**. Only portions selected by Product One (and by this
vision’s boundary) are required for the current milestone. Unselected chapters
inform enabling implications and later substrate work; they are not silent
scope inflation for Product One.

**A4.** Project name **Moria** refers to this substrate product. Future game
products may use different names; “Moria-style descent” in the substrate seed
describes a consumer fantasy, not a second in-repo product.

**A5.** Multiplayer is **not** a Product One deliverable. The verb/command
style of the API may remain multiplayer-ready by construction, without
committing to a multiplayer product in this vision.

**A6.** Tree felling / rigid-body timber is a **stretch goal** for Product One,
not a blocking vision requirement.

## Questions for human review

No material identity, purpose, or boundary questions remain open: the operator
boundary seed and seed index already resolve the substrate-vs-game and
Product-One-as-first-slice questions. Remaining open items (exact voxel size,
LOD strategy, object-layer scaling, fluid pressure model) are technical
decisions for downstream design after vision approval, not vision forks.

*(If any assumption A1–A6 is wrong, reject or annotate that assumption rather
than treating silence as a second product.)*

## Seed synthesis

| Seed | Role in this brief |
|---|---|
| `README.md` | Names the product **Moria** as a reusable GPU-resident voxel-world substrate (Rust crate). States that the walkable-world executable is a separate consumer/validation harness, not a game layer. Points at `docs/seeds/` as preserved substrate inputs. |
| `docs/seeds/README.md` | Operator framing of the seed set: Product One is the **binding** first implementation + harness; `voxel-world-substrate.md` is architecture reference with only Product One–selected portions required; `project-boundary.md` is the binding boundary clarification. Restates that broader game/System/LLM/spell/gas/combat/AI/building intent is deliberately out of scope. |
| `docs/seeds/project-boundary.md` | **Authoritative boundary**: product = substrate crate(s); game is external; harness uses public APIs only; Cargo workspace consumer split is mandatory; game rules and System/LLM/spell/gas/combat/AI/building layers out of scope (seams only where required). |
| `docs/seeds/product-one-seed.md` | **First delivery slice**: product statement (material natural world + dig proof); explicit non-goals; seed region and composition; materials palette sketch; which substrate layers are full/partial/absent; harness player/camera/debug tools; performance and M4/wgpu constraints; milestone order; what Product One “buys” for later games. Used for purpose, non-goals, confirmed constraints, and first-slice scope—not as a redefinition of Moria into a game. |
| `docs/seeds/voxel-world-substrate.md` | **Directional substrate vision**: design goals (normal look, mutable matter, deep Z, substrate-not-game, GPU-resident); look strategy (smooth extraction + hybrid vegetation); storage/generation/layering principles; long-horizon matter capabilities (fluids tiers, integrity, building verbs, nav, streaming). Contributed purpose, enabling implications for future consumers, and deferred substrate roadmap. **Not** treated as a commitment to implement every section in Product One. Future ARPG/fortress/descent/sandbox games recorded only as consumers. |

**Contradiction handling:** Product One is written as a “product-shaped demo”
while the boundary insists the product is the substrate. Resolution (already
stated by the operator seed index): the **product is the substrate**; Product
One is the **first binding vertical slice and harness proof**, not a competing
game product. The harness includes a character controller solely to validate
traversal, collision-against-truth, and demoability.

**Gaps left to design (not vision questions):** crate graph, exact algorithms,
LOD, final voxel size after benchmarks, full material rules, and post–Product
One milestone scheduling.
