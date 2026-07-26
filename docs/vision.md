# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** for games and tools, exposed as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation—not a game, not a demo package, and not an LLM-dependent system.

## Purpose

Give downstream games a shared material world they can generate, stream, query, mutate, and persist: a natural-looking surface and deep underground whose **voxel matter is the truth**, while rendered geometry remains a disposable view. Game rules, presentation, and content policy live above this layer so the same substrate can underwrite different games without embedding any one of them.

## Product boundary

**In product**
- The reusable substrate: world generation, matter representation and simulation hooks, meshing as a view of voxel truth, public mutation and query interfaces, streaming, and persistence of world scars.
- Integration shape: a Rust crate (or small crate family) intended for consumption by external games and tools.

**Adjacent, not the product**
- A walkable-world executable **may** exist as a **validation harness** only. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific substrate paths. Whether that harness is a required current delivery is **unresolved (Q1)**; until then it is neither committed nor ruled out beyond “may exist.”
- Its controllers, character, camera, authored seed route, debug presentation, scripted benchmarks, and machine-specific performance gates are harness- or consumer-owned, not substrate identity.

**Downstream / out of repository product**
- The actual game (or games) that consume the crate.
- Game rules and future **System, LLM, spell, gas, combat, AI, and building** layers: not implemented here. Compatibility seams may be designed where substrate requirements demand them.

## Required product outcomes

1. **Natural material world.** Surface terrain, vegetation-scale content, water bodies, and geology read as a continuous natural world; the voxel grid is authoritative matter, not a cube aesthetic or decorative mesh with fake fill.
2. **Mutable everywhere.** Any volume in the playable field can be destroyed, reshaped, or filled through substrate mutation APIs; dig and place are first-class proofs of material truth, not optional polish.
3. **Deep Z is first-class.** Underground volume (strata, caves, ores, aquifers-scale structure) is real content continuous with the surface, not a thin crust over empty or painted depth.
4. **Geology-first generation with lazy cost.** Worlds are produced as geology and columns that materialize matter on touch so large regions need not reside fully as dense voxels until needed.
5. **Public verbs and queries only.** Consumers—including any validation harness—change and inspect the world through the substrate’s public interfaces; nothing above the matter core reaches voxels by a privileged side path.
6. **Long-lived, streamable worlds.** Persistence is generation plus edit deltas (and related object journals as designed later); streaming keeps active neighborhoods resident while cold volume stays cheap—enough for future games to scar, abandon, and reclaim space on the same foundation.

Collectively these outcomes imply substrate responsibility for **matter, physics-facing queries, and mutation**—including smooth terrain presentation of voxel truth, collision against matter rather than mesh authority, interactable voxel-backed objects versus pure dressing, and room for fluid, integrity, and placement capabilities games will need—without shipping those games’ UX, rules, or content.

## Future products and enabling implications

Future **consumers** (not current product) include a System/LLM ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style descent adventure, a pure sandbox, and later “product two” experiences that start from a walkable material world rather than a whiteboard.

Enabling implications only: the substrate should remain free of LLM dependency; gas/pricing and similar policies stay injectable above it; building, combat, AI, and spell layers remain consumers. A first public walkable demonstration may motivate validation of generation, streaming, meshing, editing, collision, persistence, and performance—without transferring that demonstration’s content or controls into the substrate product (see Q1).

## Non-goals

- Implementing the game, System/LLM features, spells, gas economy, combat, AI, or building-game layers in this product.
- Treating the walkable demo’s character, third-person fantasy presentation, curated postcard region, or milestone theater as substrate scope.
- Full per-voxel fluid CA at continental scale as a defining promise; fluid support is tiered substrate capability, not one sim model for everything.
- Making the primary surface look a raw Minecraft-style cube grid (debug raw views may still exist).
- Embedding privileged harness-only world paths that external games cannot use.

## Confirmed vision constraints

- **Rust crate consumption** is the intended integration ecosystem for the substrate.
- The substrate is **GPU-resident** for its world/matter workload posture.
- The substrate must **stand alone with zero LLM dependency**.
- Any in-repo validation executable, if built, is a **consumer of public interfaces**, not a second privileged implementation of the world.
- **Game layers listed under the project boundary are out of scope** for implementation here; seams only where the substrate itself requires them.

## Deferred design decisions

- Delivery depth and sequence of matter capabilities (which simulation tiers, object behaviors, and generation passes ship when).
- Crate split, internal layering detail, and concrete storage/meshing/sim mechanisms.
- Voxel scale, LOD strategy, object-layer capacity limits, and fluid-solver fidelity.
- Whether and how multiplayer-authoritative command flow is scoped beyond architectural readiness of verb/query separation.
- Harness-only choices: seed world contents, controller/camera, debug tools, benchmark scenes, and target machines or frame budgets.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1. Is an in-repo walkable-world validation harness a required current delivery, or only permitted?**

- **Proposed answer:** Only **permitted**. The current product is the substrate crate(s); a walkable harness may be added as an adjacent consumer through public APIs, but shipping it is not required for the product vision to hold.
- **If different:** Making the harness **mandatory** adds a current-repo delivery obligation (a public-API walkable validator) without changing substrate identity; harness content and controls still stay outside the product boundary. Forbidding it entirely would remove the project’s named validation adjacent artifact and push all proof of generation/streaming/meshing/editing/collision/persistence onto external consumers or later design.

## Seed synthesis

- **`README.md`** — Names Moria as a reusable GPU-resident voxel-world substrate Rust crate and separates the walkable-world executable as consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binds product identity to the substrate crate family, keeps the real game out of repo, permits a public-API-only harness, and excludes System/LLM/spell/gas/combat/AI/building layer implementation.
- **`docs/seeds/product-one-seed.md`** — Describes a first walkable proof slice and demo motivations; used only to clarify validation intent and non-transfer of consumer content, not to redefine product identity or import harness acceptance detail.
- **`docs/seeds/voxel-world-substrate.md`** — Supplies substrate design goals and outcome families (natural look vs voxel truth, full mutability, deep Z, geology generation, matter/physics/queries/mutation, streaming/persistence, reusable layering) translated here at vision altitude, not as a mechanism catalog.
