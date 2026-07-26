# Project vision

## What we are building now

**Moria** is a reusable voxel-world substrate: a Rust and Bevy library (or small family of crates) that downstream games and tools install and drive through a public consumer contract. It is engine-shaped world infrastructure, not a playable game product.

## Purpose

Voxel worlds only work when several hard systems agree as explicit contracts: deterministic generation, sparse material truth, bounded inspection, mutation admission, streaming lifecycle, collision against matter rather than presentation, persistence of generated truth plus edits, and measurable presentation derived from that truth. Moria exists so material worlds can be consumed without each game rebuilding those contracts, and without hiding them behind a single privileged demo.

The product’s claim is infrastructural: a consumer can obtain a continuous three-dimensional material world, inspect and mutate it only through supported interfaces, and trust that what they see and collide with is a view of the same authoritative matter.

## Product boundary

**This product owns** the reusable substrate and its public facade:

- Deterministic world generation suitable for sparse material worlds.
- Sparse storage and lazy materialization of voxel truth.
- Bounded world inspection and telemetry for consumers.
- Mutation admission (dig, place, and related world-edit verbs) so nothing touches voxels outside the contract.
- Streaming and lifecycle so large regions do not require full raw-voxel residency.
- Collision and occupancy truth against voxel matter, not against disposable meshes.
- Persistence model of reproducible generation plus edit deltas.
- Presentation support that derives surfaces and surface dressing from material truth, without serializing derived geometry as authority.
- Object and clutter registration hooks so vegetation and micro-objects can be matter-backed without baking them into terrain.

**Adjacent, not the product identity** are curation, benchmark, and visual-validation executables (`moria-curate`, `moria-bench`, `moria-demo` and similar). They may exist to curate content parameters, exercise contracts, capture evidence, and visually validate the substrate, but they must consume the same public interfaces available to an external game. Controllers, characters, cameras, authored demo routes, presentation polish, and acceptance scenarios belong to those consumers—not to substrate identity.

**Downstream, not this repository** are actual games and game layers: player control, characters, skeletal animation, game-specific presentation, combat, AI, economy, building policy, the System / LLM layer, spells, gas pricing, and other gameplay rules. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Product-level outcomes

### Required outcomes

These are outcomes the substrate must enable, not a feature inventory or implementation plan:

1. **Truth vs view.** Occupancy, queries, collision, and persistence run against voxel matter; meshes, dressing, and debug geometry are derived and disposable.
2. **Contracted consumption.** External consumers install the facade, inspect through public reads, mutate through admitted edits, and never require privileged internal paths.
3. **Sparse scale.** Large regions remain tractable: untouched homogeneous volume stays cheap; only the interesting shell and active edits pay detailed cost.
4. **Reproducible base, cheap scars.** World identity is generation parameters (seed and curation) plus edit deltas—not a dump of every brick.
5. **Measurable substrate quality.** Benchmarks and harnesses can evidence generation, mutation response, streaming, memory behavior, and related contracts without redefining the product as a game.

### Proposed outcomes (pending human decision)

#### Pending Q2 — natural-world capability set

The following outcome set is drawn from the walkable-world seed and the voxel-world architecture reference. It is **proposed, not required**, until **Q2** is answered. Until then, design and delivery must not treat these as binding current Moria obligations:

1. **Natural-looking material terrain.** Generated terrain can read as ordinary landscape (hills, strata, water bodies, caves, vegetation-scale density) while remaining fully material—not a heightmap with disconnected props.
2. **Mutable everywhere.** Any material cell the contract exposes can be destroyed or placed; cut faces and scars remain honest matter, and presentation rebuilds from truth.
3. **Deep Z is first-class.** Underground content (caves, strata, ore, aquifers as material bands) is real three-dimensional world, not a painted floor under a skybox.

If Q2 affirms them as binding, they become required product-level outcomes. If not, they remain motivating context for future consumers or optional capability direction, not current product definition.

#### Pending Q3 — GPU-resident / async-GPU-capable direction

The following outcome is drawn from the voxel-world architecture reference and its companion GPU-resident architecture note. It is **proposed, not required**, until **Q3** is answered. Until then, design and delivery must not treat GPU-resident storage, GPU-primary simulation/meshing, or asynchronous GPU work as binding current Moria obligations:

1. **GPU-resident, asynchronous-GPU-capable architecture.** Sparse brick storage and a command/query boundary that can keep world representation GPU-resident and support asynchronous GPU work without changing the consumer contract.

If Q3 affirms this as binding, it becomes a required product-level direction (specific kernels and simulations still selected by milestone). If not, it remains nonbinding architectural context: portable presentation and measurable GPU memory behavior may still matter as engineering constraints without requiring a GPU-resident substrate identity.

Required outcomes above (truth vs view, contracted APIs, sparsity, seed+delta identity, measurable quality) do **not** by themselves mandate GPU residency; they remain required regardless of Q3.

## Non-goals

- Shipping a game, game mode, progression loop, or game-rules stack in this product.
- Treating validation-harness content, third-person controllers, demo routes, or machine-specific demo targets as substrate requirements.
- Implementing excluded layers here: System / LLM, spells, gas / pricing policy, combat, AI, agent labor, building UI / blueprints as gameplay, mechanisms as game entities.
- Full fluid simulation, cellular automata (fire, wetness, growth), granular settling, structural integrity / cave-ins, tree felling or rigid-body conversion—these may appear as future consumer concepts or format hooks; they are not current product scope unless later selected explicitly.
- Web / wasm as a Product One or substrate target platform.
- Claiming a released, finished visual engine before feasibility and visual-acceptance gates are met.
- Treating the proposed natural-world outcome set (natural look, everywhere mutation, first-class deep Z) as binding until Q2 is resolved.
- Treating GPU-resident storage, asynchronous GPU execution, or GPU-primary meshing/simulation as binding current product direction until Q3 is resolved.

## Future consumers (context only)

Reference material describes possible later products: a System-driven ARPG, a fortress / colony game, a descent-style roguelike, and pure sandboxes. Those motivate reusable material-world capabilities—normal-looking natural worlds, mutability all the way down, first-class deep underground, and a clean matter / query / mutation boundary. Their gameplay, characters, assets, content palette, and presentation are **not** current Moria scope.

The “walkable world” seed describes a third-person proof shape (curated region, forest, ruin, dig-as-demo) that a validation consumer might use to make substrate claims undeniable. That seed’s content, controls, milestones, and performance tables are **context for what the substrate might support**, not the definition of the product itself. Whether its core world-shape claims bind Moria now is **Q2**.

GPU-resident sparse storage and an async-capable command/query boundary appear in architecture seeds as a long-horizon direction. Whether that direction binds current Moria is **Q3**, independent of which natural-world outcomes Q2 selects.

## Confirmed vision constraints

- Adjacent consumers, including any walkable validation harness, have no privileged access path into the substrate.
- The product is a Rust / Bevy library ecosystem for crate consumers, not an ecosystem-neutral engine abstract.
- When seed documents conflict, the project boundary wins: substrate product first; game examples and Product One detail are nonbinding unless selected by that boundary or an explicit human decision.

## Assumptions proposed for approval

1. **Authority order stands.** `docs/seeds/project-boundary.md` is binding for product identity; the walkable-world seed and broad architecture reference inform capabilities and seams only.
2. **Harnesses are adjacent.** Curation, bench, and demo executables may ship in-repo to prove contracts; they do not redefine Moria as a game or as “the walkable demo.”

## Questions for human review

**Q1.** Is a walkable-world visual validation harness a **mandatory current delivery** of this repository, or only a **permitted adjacent artifact**?

- **Proposed answer:** Permitted adjacent artifact. Product identity and “done” for Moria are the reusable substrate and its public contracts; a walkable harness may be built to validate them but is not required to define the product.
- **If answered differently:** Making the harness mandatory keeps it outside product identity but adds a required adjacent deliverable (still without importing its controls, content palette, or acceptance numbers into the substrate). Treating it as optional leaves harness work non-blocking for substrate completion.

**Q2.** Are **natural-looking, fully material terrain**, **everywhere mutation**, and **first-class deep Z** binding **current Moria product outcomes**, or only motivating context from the walkable-world seed and architecture reference?

- **Proposed answer:** Binding current outcomes. Project boundary names a reusable voxel-world substrate; the architecture reference’s design goals and Product One’s proof claims describe what that substrate is for—without importing demo content, characters, or machine targets. Leaving them optional would leave “what worlds Moria must enable” underspecified for design handoff.
- **If answered differently:** Affirming them as nonbinding keeps Moria as contract infrastructure only (storage, generation hooks, mutation admission, streaming, persistence, presentation seams) without requiring natural-landscape quality, universal mutability proof, or underground-first content as current product success criteria. Affirming a subset (for example mutation and truth/view without natural look or deep Z) would require restating the required outcomes explicitly.

**Q3.** Is a **GPU-resident / asynchronous-GPU-capable architecture** a binding **current Moria product direction**, or only **nonbinding architectural context** from the voxel-world and GPU-resident architecture seeds?

- **Proposed answer:** Nonbinding context for current product identity. Project boundary and repository README define Moria as a reusable Rust/Bevy substrate with public contracts, sparsity, and measurable presentation—not as a GPU-resident engine. Seeds README ranks the GPU-resident note as supporting architecture that does not add features to the current milestone; specific simulations remain nonbinding until selected. Portable Bevy/wgpu presentation and honest GPU-memory measurement can still be engineering constraints without making GPU residency a product outcome.
- **If answered differently:** Affirming it as binding elevates sparse GPU-resident storage and an async-capable command/query boundary to required product direction (kernels and simulations still milestone-selected). That would reframe “presentation support” and measurable quality around GPU ownership of matter or mesh work, and would need explicit reconciliation with any current CPU-authoritative / upload-based implementation path. Affirming a narrower subset (for example async-safe consumer boundary without GPU-resident truth) would require restating the outcome explicitly.

No other seed conflicts change product identity: seeds README, project boundary, and repository README agree that Moria is the substrate and that Product One’s character, route, and demo targets are not Moria requirements. Older planning docs that still title the effort as “Product One — The Walkable World” should be read as superseded on identity by the boundary and this vision once approved. Remaining open identity-adjacent questions are **Q2** (whether the natural-world capability set is required of the substrate now) and **Q3** (whether GPU-resident / async-GPU-capable architecture is current direction or nonbinding context). **Q1** remains open for delivery obligation of the validation harness without changing product identity.

## Seed synthesis

| Source | Contribution to this vision |
| --- | --- |
| **README.md** | Names Moria as a Rust/Bevy voxel-world substrate consumed as a crate; executables curate, exercise, benchmark, and visually validate; player controllers, characters, and game-specific presentation are outside the substrate boundary; status is active engineering, not a released engine. Workspace crate roles and evidence detail stay subordinate engineering input. |
| **docs/seeds/project-boundary.md** | **Binding product target:** reusable substrate (crate family); games are separate consumers; any walkable executable is a public-interface validation harness only; System / LLM / spell / gas / combat / AI / building layers are out of scope. Cargo workspace split is motivated but left as technical design. |
| **docs/seeds/product-one-seed.md** | **Downstream / validation example:** supplies the “fully material walkable world” proof shape and explicit non-goals (no combat, System, CA, building UI, dynamic fluids). Motivates the proposed natural-world outcome set (smooth material truth, dig/place proof, geology, sparse streaming, seed+delta save, measurable quality). Does **not** import third-person character, curated 1 km postcard, content palette, performance tables, or milestone schedule into product identity. Whether its world-shape claims bind Moria is **Q2**. GPU memory numbers and GPU meshing language in that seed inform **Q3** but do not settle it. |
| **docs/seeds/voxel-world-substrate.md** | **Architecture reference:** long-horizon goals include reads as a normal world; mutable everywhere; deep Z; substrate not game; and GPU-resident direction. Natural-world goals are proposed pending **Q2**; GPU-resident / async-GPU-capable direction is proposed pending **Q3**—neither is silently required. Layering diagram and many extensions (fluids tier 2+, integrity, weather, building verbs, nav, multiplayer readiness) remain context or future seams unless selected. Game examples (ARPG, fortress, Moria descent) motivate reusability only. Companion `gpu-resident-substrate.md` (supporting architecture per seeds README) is the detailed source for the Q3 claim, not a separate product target. |

Authority among seeds (from `docs/seeds/README.md`): project boundary first; GPU-resident architecture note second as supporting principles without automatic milestone features; broad voxel reference third; Product One seed last as validation example. Conflicts resolve toward the boundary without silent expansion of scope.
