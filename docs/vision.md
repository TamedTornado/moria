# Project vision

## What we are building now

**Moria** is a reusable voxel-world substrate: a Rust and Bevy library (or small family of crates) that downstream games and tools install and drive through a public consumer contract. It is engine-shaped world infrastructure, not a playable game product.

It is not limited to a Minecraft-style cube aesthetic or a single overworld content palette. The material contracts target continuous three-dimensional volumes—natural landscapes, underground geology, and constructed interiors among them. **Whether freeform ship and station hulls are in binding current scope** is a multi-world identity question still awaiting human approval (see Assumptions and Open questions below). Until that handoff is approved, ship/station support is **conditional context**, not a settled product claim.

## Purpose

Voxel worlds only work when several hard systems agree as explicit contracts: deterministic generation, sparse material truth, bounded inspection, mutation admission, streaming lifecycle, collision against matter rather than presentation, persistence of generated truth plus edits, GPU-resident representation of that matter, and measurable presentation derived from truth. Moria exists so material worlds can be consumed without each game rebuilding those contracts, and without hiding them behind a single privileged demo.

The product’s claim is infrastructural: a consumer can obtain a continuous three-dimensional material world, inspect and mutate it only through supported interfaces, keep authoritative matter GPU-resident for scale and gameplay-enabling work, and trust that what they see and collide with is a view of the same authoritative matter.

## Product boundary

**This product owns** the reusable substrate and its public facade:

- Deterministic world generation suitable for sparse material worlds.
- Sparse storage and lazy materialization of voxel truth.
- GPU-resident sparse representation and a command/query boundary that can support asynchronous GPU work without changing the consumer contract.
- Bounded world inspection and telemetry for consumers.
- Mutation admission (dig, place, and related world-edit verbs) so nothing touches voxels outside the contract.
- Streaming and lifecycle so large regions do not require full raw-voxel residency.
- Collision and occupancy truth against voxel matter, not against disposable meshes.
- Persistence model of reproducible generation plus edit deltas.
- Presentation support that derives surfaces and surface dressing from material truth, without serializing derived geometry as authority.
- Object and clutter registration hooks so vegetation, micro-objects, and other matter-backed assemblies can register without baking into a single terrain slab.

**Adjacent, not the product identity** are curation, benchmark, and visual-validation executables (`moria-curate`, `moria-bench`, `moria-demo` and similar). They may exist to curate content parameters, exercise contracts, capture evidence, and visually validate the substrate, but they must consume the same public interfaces available to an external game. Controllers, characters, cameras, authored demo routes, presentation polish, and acceptance scenarios belong to those consumers—not to substrate identity. A walkable-world harness is a **permitted adjacent artifact**, not a mandatory definition of “done” for Moria.

**Downstream, not this repository** are actual games and game layers: player control, characters, skeletal animation, game-specific presentation, combat, AI, economy, building policy, the System / LLM layer, spells, gas pricing, and other gameplay rules. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Product-level outcomes

These are outcomes the substrate must enable, not a feature inventory or implementation plan:

1. **Truth vs view.** Occupancy, queries, collision, and persistence run against voxel matter; meshes, dressing, and debug geometry are derived and disposable.
2. **Contracted consumption.** External consumers install the facade, inspect through public reads, mutate through admitted edits, and never require privileged internal paths.
3. **Sparse scale.** Large regions remain tractable: untouched homogeneous volume stays cheap; only the interesting shell and active edits pay detailed cost.
4. **Mutable everywhere.** Any material cell the contract exposes can be destroyed or placed; cut faces and scars remain honest matter, and presentation rebuilds from truth.
5. **Deep Z is first-class.** Volume along the full depth axis is real content—caves, strata, ore, aquifers as material bands, buried structure—not a painted floor under a skybox. Multi-deck ship or station interiors are in scope for this outcome **only if** multi-world freeform volumes are approved; until then they motivate the capability as consumer context, not a binding Deep Z deliverable shape.
6. **Reproducible base, cheap scars.** World identity is generation parameters (seed and curation) plus edit deltas—not a dump of every brick.
7. **GPU-resident architecture.** Sparse brick storage and a command/query boundary keep world representation GPU-resident and support asynchronous GPU work. This is a product distinction from CPU-driven voxel engines: residency enables gameplay-scale mutation, meshing, and future simulation without abandoning the consumer contract. Specific kernels and simulations remain milestone-selected; residency and the async-capable boundary do not.
8. **Measurable substrate quality.** Benchmarks and harnesses can evidence generation, mutation response, streaming, GPU memory behavior, and related contracts without redefining the product as a game.

**World-dependent presentation.** How a world “looks natural” depends on the consumer’s world—landscape geology, fortress masonry, and other material styles. The substrate must support fully material volumes that read as coherent for their domain; it does not mandate a single overworld aesthetic or a heightmap-with-props look. Ship bulkheads and similar freeform-hull presentation remain conditional on multi-world approval.

## Non-goals

- Shipping a game, game mode, progression loop, or game-rules stack in this product.
- Treating validation-harness content, third-person controllers, demo routes, or machine-specific demo targets as substrate requirements or as mandatory delivery for product completion.
- Implementing excluded layers here: System / LLM, spells, gas / pricing policy, combat, AI, agent labor, building UI / blueprints as gameplay, mechanisms as game entities.
- Full fluid simulation, cellular automata (fire, wetness, growth), granular settling, structural integrity / cave-ins, tree felling or rigid-body conversion—these may appear as future consumer concepts or format hooks; they are not current product scope unless later selected explicitly.
- Web / wasm as a Product One or substrate target platform.
- Claiming a released, finished visual engine before feasibility and visual-acceptance gates are met.
- Limiting the substrate identity to a Minecraft-style cube aesthetic or a single natural-overworld content palette.

## Future consumers (context only)

Reference material describes possible later products that motivate reusable material-world capabilities. Their gameplay, characters, assets, content palettes, and presentation are **not** current Moria scope. They illustrate what the substrate must remain able to support:

- A System-driven ARPG on a continuous natural world.
- A fortress / colony game with engineering and designation play.
- A descent-style roguelike through deep geology.
- Pure sandboxes.
- A single-captain space trading and combat game in the Escape Velocity mold—starmap, cargo runs, factions, missions—where the ship is a freeform material volume rather than a hull with abstract slots. Reactors, conduits, weapons, and life support occupy real space; power routes are visible geometry; a hit destroys a specific thing rather than a health number. Ship design is conversational with an engineer who proposes layouts and benches them under load; ships wear, fail under stress, and leave wrecks as real salvageable geometry. Debris from fights the player never saw still floats when they arrive. This paragraph is **explicitly nonbinding future-consumer context**: it motivates everywhere mutation, first-class depth through multi-deck interiors, GPU-resident matter at combat and design scale, and truth-vs-view so damage and salvage stay honest—without importing its fiction, UI, mission systems, or freeform-hull requirements into current Moria scope until multi-world is approved.

The “walkable world” seed describes a third-person proof shape (curated region, forest, ruin, dig-as-demo) that a validation consumer might use to make substrate claims undeniable. That seed’s content, controls, milestones, and performance tables remain **context for what the substrate might support**, not the definition of the product itself.

## Confirmed vision constraints

- Adjacent consumers, including any walkable validation harness, have no privileged access path into the substrate.
- The walkable-world harness is an adjacent artifact only—not a required product delivery that defines Moria’s completion.
- The product is a Rust / Bevy library ecosystem for crate consumers, not an ecosystem-neutral engine abstract.
- Everywhere mutation and first-class deep Z (geology and continuous depth as real content) are binding product outcomes; natural-looking presentation is world-dependent, not a single mandated overworld aesthetic.
- GPU residency and an async-capable command/query boundary are binding product direction and a deliberate distinction from CPU-driven voxel engines.
- When seed documents conflict, the project boundary wins: substrate product first; game examples and Product One detail are nonbinding unless selected by that boundary or an explicit human decision.
- Freeform ship/station volumes are **not** confirmed product identity; they remain under the multi-world approval handoff below.

## Assumptions proposed for approval

1. **Authority order stands.** `docs/seeds/project-boundary.md` is binding for product identity; the walkable-world seed and broad architecture reference inform capabilities and seams only.
2. **Harnesses are adjacent.** Curation, bench, and demo executables may ship in-repo to prove contracts; they do not redefine Moria as a game or as “the walkable demo.”
3. **Multi-world material volumes (pending).** Proposal for human approval: “world” means continuous material volume in general—not only planetary terrain—so ship interiors, stations, and other freeform volumes would share the same contracts. **This assumption is not yet approved.** Until the human accepts it, ship/station support stays conditional in the opening statement and Deep Z outcome, and must not be treated as settled substrate identity.

## Resolved human decisions

| Question | Decision |
| --- | --- |
| **Q1.** Is a walkable-world visual validation harness mandatory current delivery or only a permitted adjacent artifact? | **Adjacent artifact.** It may exist for validation; it does not define product identity or “done.” Moria is not limited to a Minecraft-style world. |
| **Q2.** Are natural-looking terrain, everywhere mutation, and first-class deep Z binding current outcomes? | **Everywhere mutation and first-class deep Z are binding.** Natural-looking presentation **depends on the consumer’s world**—required as coherent material presentation for that domain, not as a single natural-overworld mandate. |
| **Q3.** Is GPU-resident / asynchronous-GPU-capable architecture binding current direction? | **Yes.** GPU residency is an important product feature: it enables many gameplay capabilities and is a core distinction between Moria and CPU-driven voxel engines. Specific simulations remain milestone-selected. |

## Open questions (identity)

| Question | Why it matters |
| --- | --- |
| **Q4.** Multi-world freeform volumes: does current product identity include ship/station (and similar freeform) material volumes on the same contracts as natural geology, or do those remain nonbinding future-consumer motivation only? | Approving elevates freeform hulls into binding opening and Deep Z scope; rejecting keeps them as future-consumer context and leaves Deep Z framed around continuous geological/constructed depth without mandating multi-deck ship interiors. |

Older planning docs that still title the effort as “Product One — The Walkable World” should be read as superseded on identity by the boundary and this vision once approved, except where this vision still marks multi-world as pending.

## Seed synthesis

| Source | Contribution to this vision |
| --- | --- |
| **README.md** | Names Moria as a Rust/Bevy voxel-world substrate consumed as a crate; executables curate, exercise, benchmark, and visually validate; player controllers, characters, and game-specific presentation are outside the substrate boundary; status is active engineering, not a released engine. Workspace crate roles and evidence detail stay subordinate engineering input. |
| **docs/seeds/project-boundary.md** | **Binding product target:** reusable substrate (crate family); games are separate consumers; any walkable executable is a public-interface validation harness only; System / LLM / spell / gas / combat / AI / building layers are out of scope. Cargo workspace split is motivated but left as technical design. |
| **docs/seeds/product-one-seed.md** | **Downstream / validation example:** supplies a “fully material walkable world” proof shape and explicit non-goals (no combat, System, CA, building UI, dynamic fluids). Motivates smooth material truth, dig/place proof, geology, sparse streaming, seed+delta save, and measurable quality. Does **not** import third-person character, curated 1 km postcard, content palette, performance tables, or milestone schedule into product identity. GPU memory language in that seed aligns with the affirmed GPU-resident direction without importing demo numbers as substrate law. |
| **docs/seeds/voxel-world-substrate.md** | **Architecture reference:** long-horizon goals include reads as a coherent world for its domain; mutable everywhere; deep Z; substrate not game; and GPU-resident direction—now affirmed as product outcomes/direction where stated above. Layering diagram and many extensions (fluids tier 2+, integrity, weather, building verbs, nav, multiplayer readiness) remain context or future seams unless selected. Game examples (ARPG, fortress, Moria descent) motivate reusability only. Companion `gpu-resident-substrate.md` (supporting architecture per seeds README) details the GPU-resident claim without becoming a separate product target. |
| **Human review (prior pass)** | Resolved Q1–Q3 as tabulated above. Added a space-trading / freeform-ship consumer as **nonbinding** motivation for volume-general mutability, deep interiors, GPU-resident matter, and honest damage/salvage—without importing that game’s systems into current scope. |
| **Human review (this pass)** | Preserve multi-world as an approval handoff, not a settled claim. Ship/station support is conditional in the opening and Deep Z outcome until approved. The space-game paragraph remains explicitly nonbinding future-consumer context. The claim that no identity questions remain is withdrawn; Q4 stays open. |

Authority among seeds (from `docs/seeds/README.md`): project boundary first; GPU-resident architecture note second as supporting principles (now elevated to product direction by human decision where residency and async boundary are concerned); broad voxel reference third; Product One seed last as validation example. Conflicts resolve toward the boundary without silent expansion of scope.
