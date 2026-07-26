# Project vision

## What we are building now

**Moria** is a reusable voxel-world substrate: a Rust and Bevy library (or small family of crates) that downstream games and tools install and drive through a public consumer contract. It is engine-shaped world infrastructure, not a playable game product.

## Purpose

Voxel worlds only work when several hard systems agree as explicit contracts: deterministic generation, sparse material truth, bounded inspection, mutation admission, streaming lifecycle, collision against matter rather than presentation, persistence of generated truth plus edits, and measurable presentation derived from that truth. Moria exists so natural, fully material worlds can be consumed without each game rebuilding those contracts, and without hiding them behind a single privileged demo.

The product’s claim is infrastructural: a consumer can obtain a continuous three-dimensional material world—surface and deep underground—inspect and mutate it only through supported interfaces, and trust that what they see and collide with is a view of the same authoritative matter.

## Product boundary

**This product owns** the reusable substrate and its public facade:

- Deterministic world generation suitable for natural geology and sparse material worlds.
- Sparse storage and lazy materialization of voxel truth.
- Bounded world inspection and telemetry for consumers.
- Mutation admission (dig, place, and related world-edit verbs) so nothing touches voxels outside the contract.
- Streaming and lifecycle so large regions do not require full raw-voxel residency.
- Collision and occupancy truth against voxel matter, not against disposable meshes.
- Persistence model of reproducible generation plus edit deltas.
- Presentation support that derives smooth (or appropriately sharp) surfaces and surface dressing from material truth, without serializing derived geometry as authority.
- Object and clutter registration hooks so vegetation and micro-objects can be matter-backed without baking them into terrain.

**Adjacent, not the product identity** are curation, benchmark, and visual-validation executables (`moria-curate`, `moria-bench`, `moria-demo` and similar). They may exist to curate content parameters, exercise contracts, capture evidence, and visually validate the substrate, but they must consume the same public interfaces available to an external game. Controllers, characters, cameras, authored demo routes, presentation polish, and acceptance scenarios belong to those consumers—not to substrate identity.

**Downstream, not this repository** are actual games and game layers: player control, characters, skeletal animation, game-specific presentation, combat, AI, economy, building policy, the System / LLM layer, spells, gas pricing, and other gameplay rules. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product-level outcomes

These are outcomes the substrate must enable, not a feature inventory or implementation plan:

1. **Material natural world.** Generated terrain can read as ordinary landscape (hills, strata, water bodies, caves, vegetation-scale density) while remaining fully material—not a heightmap with disconnected props.
2. **Mutable everywhere.** Any material cell the contract exposes can be destroyed or placed; cut faces and scars remain honest matter, and presentation rebuilds from truth.
3. **Deep Z is first-class.** Underground content (caves, strata, ore, aquifers as material bands) is real three-dimensional world, not a painted floor under a skybox.
4. **Truth vs view.** Occupancy, queries, collision, and persistence run against voxel matter; meshes, dressing, and debug geometry are derived and disposable.
5. **Contracted consumption.** External consumers install the facade, inspect through public reads, mutate through admitted edits, and never require privileged internal paths.
6. **Sparse scale.** Large regions remain tractable: untouched homogeneous volume stays cheap; only the interesting shell and active edits pay detailed cost.
7. **Reproducible base, cheap scars.** World identity is generation parameters (seed and curation) plus edit deltas—not a dump of every brick.
8. **Measurable substrate quality.** Benchmarks and harnesses can evidence generation, mutation response, streaming, memory behavior, and related contracts without redefining the product as a game.

## Non-goals

- Shipping a game, game mode, progression loop, or game-rules stack in this product.
- Treating validation-harness content, third-person controllers, demo routes, or machine-specific demo targets as substrate requirements.
- Implementing excluded layers here: System / LLM, spells, gas / pricing policy, combat, AI, agent labor, building UI / blueprints as gameplay, mechanisms as game entities.
- Full fluid simulation, cellular automata (fire, wetness, growth), granular settling, structural integrity / cave-ins, tree felling or rigid-body conversion—these may appear as future consumer concepts or format hooks; they are not current product scope unless later selected explicitly.
- Web / wasm as a Product One or substrate target platform.
- Claiming a released, finished visual engine before feasibility and visual-acceptance gates are met.

## Future consumers (context only)

Reference material describes possible later products: a System-driven ARPG, a fortress / colony game, a descent-style roguelike, and pure sandboxes. Those motivate the substrate outcomes above—normal-looking natural worlds, mutability all the way down, first-class deep underground, and a clean matter / query / mutation boundary. Their gameplay, characters, assets, content palette, and presentation are **not** current Moria scope.

The “walkable world” seed describes a third-person proof shape (curated region, forest, ruin, dig-as-demo) that a validation consumer might use to make substrate claims undeniable. That seed’s content, controls, milestones, and performance tables are **context for what the substrate must support**, not the definition of the product itself.

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

No other seed conflicts change product identity: seeds README, project boundary, and repository README agree that Moria is the substrate and that Product One’s character, route, and demo targets are not Moria requirements. Older planning docs that still title the effort as “Product One — The Walkable World” should be read as superseded on identity by the boundary and this vision once approved.

## Seed synthesis

| Source | Contribution to this vision |
| --- | --- |
| **README.md** | Names Moria as a Rust/Bevy voxel-world substrate consumed as a crate; executables curate, exercise, benchmark, and visually validate; player controllers, characters, and game-specific presentation are outside the substrate boundary; status is active engineering, not a released engine. Workspace crate roles and evidence detail stay subordinate engineering input. |
| **docs/seeds/project-boundary.md** | **Binding product target:** reusable substrate (crate family); games are separate consumers; any walkable executable is a public-interface validation harness only; System / LLM / spell / gas / combat / AI / building layers are out of scope. Cargo workspace split is motivated but left as technical design. |
| **docs/seeds/product-one-seed.md** | **Downstream / validation example:** supplies the “fully material walkable world” proof shape and explicit non-goals (no combat, System, CA, building UI, dynamic fluids). Motivates substrate outcomes (smooth material truth, dig/place proof, geology, sparse streaming, seed+delta save, measurable quality). Does **not** import third-person character, curated 1 km postcard, content palette, performance tables, or milestone schedule into product identity. |
| **docs/seeds/voxel-world-substrate.md** | **Architecture reference:** long-horizon goals (reads as a normal world; mutable everywhere; deep Z; substrate not game; GPU-resident direction). Layering diagram and many extensions (fluids tier 2+, integrity, weather, building verbs, nav, multiplayer readiness) remain context or future seams unless selected. Game examples (ARPG, fortress, Moria descent) motivate reusability only. |

Authority among seeds (from `docs/seeds/README.md`): project boundary first; supporting architecture second; broad voxel reference third; Product One seed last as validation example. Conflicts resolve toward the boundary without silent expansion of scope.
