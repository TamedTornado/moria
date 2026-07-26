# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** for games and tools, exposed as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation—not a game, not a demo package, and not LLM-dependent.

## Purpose

Give downstream games a shared material world they can generate, stream, query, mutate, simulate, navigate, present as voxel truth, and persist—a natural-looking surface and deep underground whose **voxel matter is the authority**. Game rules, game-specific presentation, and content policy live above so the same substrate underwrites different games without embedding any one of them.

## Product boundary

**In product**
- The reusable substrate: geology-first generation; authoritative voxel matter; material and environmental simulation; voxel-backed dynamic objects and matter-synchronized dressing; substrate presentation of voxel truth as a natural-looking world; mutation-safe spatial and navigation data and queries; public command/verb, query, and event interaction with a stale-mirror observation path; streaming and persistence of world scars.
- Integration shape: a Rust crate (or small crate family) for external games and tools.

**Adjacent, not the product**
- A walkable-world executable **may** exist as a **validation harness** only. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific substrate paths. Whether that harness is a required current delivery is **unresolved (Q1)**; until then it is neither committed nor ruled out beyond “may exist.”
- Controllers, characters, cameras, authored seed routes, debug UI, scripted benchmarks, machine-specific performance gates, and **game-specific** presentation remain consumer-owned.

**Downstream / out of repository product**
- The actual game(s) that consume the crate.
- Game rules and future **System, LLM, spell, gas, combat, AI, and building** layers: not implemented here. Seams may be designed where substrate requirements demand them. Substrate spatial data, matter simulation, and world presentation of voxel truth are not those layers.

## Required product outcomes

1. **Natural material world with substrate presentation.** Surface terrain, geology, water, and surface content read as a continuous natural world. The voxel grid is authoritative matter. The substrate presents that truth as a smooth, material-derived world and keeps non-material dressing derived from and synchronized with voxel state. Game-specific presentation stays above.
2. **Mutable deep volume, geology-first and lazy.** Any playable volume can be destroyed, reshaped, or filled through substrate mutation APIs. Underground (strata, caves, ores, aquifer-scale structure) is continuous with the surface. Worlds are generated as geology and columns that materialize matter on touch so large regions need not sit fully as dense voxels until needed.
3. **Material and environmental simulation.** Consumer-visible substrate behaviors include granular settle and pour; tiered mutable fluids (still bodies, coarser active flow, fine boundary splash as designed later); ambient weather, time/season influence, and fire ecology; and material-dependent structural integrity with collapse when support fails. Mechanisms, fidelity, and delivery order remain design decisions.
4. **Dynamic voxel-backed objects and coherent dressing.** Anything that can burn, break, or block is voxel-backed. Trees and similar objects support falling and reintegration into the voxel world. Pure dressing has no independent identity and stays synchronized with underlying matter state.
5. **Mutation-safe spatial semantics.** The substrate maintains derived navigation and spatial data invalidated after mutation, supports continuous-Z traversal, and exposes multiple 3D movement classes (walk, climb, fly, burrow, swim and kin) as query capability. Agents, labor, and game behavior remain consumer-owned.
6. **Commands, queries, events, and durable streamable worlds.** Higher layers interact through commands/verbs in and queries plus events against a stale mirror out—no privileged direct voxel access. Persistence is generation plus edit deltas (and related object journals as designed later); streaming keeps active neighborhoods resident while cold volume stays cheap.

## Future products and enabling implications

Future **consumers** (not current product) include a System/LLM ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style descent adventure, a pure sandbox, and later experiences that start from a walkable material world.

Enabling implications only: zero LLM dependency in the substrate; gas/pricing injectable above it; building, combat, AI, and spell layers remain consumers. A first walkable demonstration may motivate validation without transferring its content or controls into the substrate (see Q1). Early consumer slices may exercise only part of the outcome set without removing unexercised substrate responsibilities.

## Non-goals

- Implementing the game, System/LLM features, spells, gas economy, combat, AI, or building-game layers here.
- Treating the walkable demo’s character, framing, curated region, or milestone theater as substrate scope.
- Full per-voxel fluid CA at continental scale as a single fluid model; fluids are tiered.
- Making the primary surface look a raw Minecraft-style cube grid (debug raw views may still exist).
- Embedding privileged harness-only world paths that external games cannot use.
- Owning agents, game rules, or game-specific presentation while providing substrate spatial data and world presentation of voxel truth.

## Confirmed vision constraints

- **Rust crate consumption** is the intended integration ecosystem.
- The substrate is **GPU-resident** for its world/matter workload posture.
- **Interaction contract:** higher layers drive the world with commands/verbs and observe through queries and events on a stale mirror; nothing above the matter core reaches voxels by a privileged side path.
- The substrate must **stand alone with zero LLM dependency**.
- Any in-repo validation executable, if built, is a **consumer of public interfaces**, not a second privileged world implementation.
- **Game layers listed under the project boundary are out of scope** for implementation here; seams only where the substrate itself requires them.

## Deferred design decisions

- Mechanisms, fidelity, and delivery order for simulation tiers, object lifecycles, generation passes, and presentation detail.
- Crate split, internal layering, and concrete storage, meshing, and sim implementations.
- Voxel scale, LOD strategy, object-layer capacity, and fluid-solver detail.
- Whether multiplayer-authoritative command flow is scoped beyond verb/query readiness.
- Harness-only choices: seed world, controller/camera, debug tools, benchmark scenes, and target machines or frame budgets.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1. Is an in-repo walkable-world validation harness a required current delivery, or only permitted?**

- **Proposed answer:** Only **permitted**. The current product is the substrate crate(s); a walkable harness may be added as an adjacent public-API consumer, but shipping it is not required for the product vision to hold.
- **If different:** Making the harness **mandatory** adds a current-repo delivery obligation without changing substrate identity; harness content and controls stay outside the product boundary. Forbidding it entirely removes the named validation adjacent artifact and pushes proof onto external consumers or later design.

## Seed synthesis

- **`README.md`** — Names Moria as a reusable GPU-resident voxel-world substrate Rust crate and separates the walkable-world executable as consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binds product identity to the substrate crate family, keeps the real game out of repo, permits a public-API-only harness, and excludes System/LLM/spell/gas/combat/AI/building layer implementation.
- **`docs/seeds/product-one-seed.md`** — Describes a first walkable proof slice; used only for validation intent and non-transfer of consumer content—not to redefine identity, strip substrate outcomes, or import harness acceptance detail.
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes substrate outcome families at vision altitude: natural look vs voxel truth with substrate presentation; mutability and deep Z; geology-first lazy generation; granular/fluid/weather/fire/integrity simulation; voxel-backed objects and synchronized dressing; mutation-safe spatial semantics; command/mirror/event interaction; streaming and persistence—not as a mechanism inventory.
