# Project vision

## What we are building now

**Moria** is a reusable voxel-world substrate: a Rust and Bevy library (or small family of crates) that downstream games and tools install and drive through a public consumer contract. It is engine-shaped world infrastructure, not a playable game product.

It is not limited to a Minecraft-style cube aesthetic or a single overworld content palette. The material contracts target continuous three-dimensional volumes—natural landscapes, underground geology, and constructed interiors among them—and those volumes are not only static scenery. **Substrate contracts are volume-general:** they must not assume gravity-aligned planetary terrain as the only kind of world. Delivering or specifically validating freeform ship and station hulls is **not** current scope; those remain future-consumer examples that the contracts should remain able to support.

## Purpose

Voxel worlds only work when several hard systems agree as explicit contracts: sparse material truth, bounded inspection, mutation admission, streaming lifecycle, collision against matter rather than presentation, persistence of world matter plus edits, GPU-resident representation of that matter, measurable presentation derived from truth, and **seams so a physics engine can plug in** without privileged access to voxel storage. Moria exists so material worlds can be consumed without each game rebuilding those contracts, and without hiding them behind a single privileged demo.

The product’s claim is infrastructural: a consumer can obtain a continuous three-dimensional material world (including movable, damageable voxel volumes such as players and enemies), inspect and mutate it only through supported interfaces, keep authoritative matter GPU-resident for scale and gameplay-enabling work, and trust that what they see and collide with is a view of the same authoritative matter—while gravity, force, material strength, and related physical response are **supportable through exposed bindings**, not baked into a substrate-owned physics engine.

**World generation is not part of the substrate identity.** How a game fills or seeds sparse material volumes is a consumer- or game-dependent algorithm that runs *on top of* the substrate. The substrate provides storage, query, mutation, streaming, collision-truth seams, physics plug-in bindings, persistence seams, and presentation derivation for material truth—not a baked-in procedural generator and not a hand-rolled physics engine.

## Product boundary

**This product owns** the reusable substrate and its public facade:

- Sparse storage and lazy materialization of voxel truth.
- GPU-resident sparse representation and a command/query boundary that can support asynchronous GPU work without changing the consumer contract.
- Bounded world inspection and telemetry for consumers.
- Mutation admission (dig, place, and related world-edit verbs) so nothing touches voxels outside the contract.
- Streaming and lifecycle so large regions do not require full raw-voxel residency.
- Collision and occupancy **truth** against voxel matter, not against disposable meshes—queries and contacts read material authority so consumers and plug-ins do not invent a second world.
- **Physics plug-in bindings:** material properties and world seams a physics engine needs—material strength, gravity parameters, applied force, and related supportable fields—exposed so a hand-rolled or third-party engine can attach without privileged voxel access. The substrate does **not** bake in a physics engine.
- Support for **dynamic voxel volumes**—matter that moves and can take damage (players, enemies, and similar)—not only static world geometry.
- Persistence of material truth plus edit deltas (and related world-state scars), without requiring a dump of every brick; how the *base* volume is produced is the consumer’s concern.
- Presentation support that derives surfaces and surface dressing from material truth, without serializing derived geometry as authority.
- Object and clutter registration hooks so vegetation, micro-objects, and other matter-backed assemblies can register without baking into a single terrain slab.
- Seams a consumer can use to inject or drive world content (including generation algorithms), without embedding any particular generator as substrate law.
- **Volume-general contracts** that do not encode planetary heightmap assumptions or gravity-aligned terrain as the only legal world shape.

**Adjacent, not the product identity** are curation, benchmark, and visual-validation executables (`moria-curate`, `moria-bench`, `moria-demo` and similar). They may exist to curate content parameters, exercise contracts, capture evidence, and visually validate the substrate, but they must consume the same public interfaces available to an external game. Controllers, characters, cameras, authored demo routes, presentation polish, acceptance scenarios, and **game-specific generation algorithms** belong to those consumers—not to substrate identity. A walkable-world harness is a **permitted adjacent artifact**, not a mandatory definition of “done” for Moria. In-repo generation used only to exercise or demonstrate contracts remains a harness concern, not a claim that generation is substrate product.

**Also adjacent (100% adjacent): any physics engine**—hand-rolled or third-party—that integrates through the substrate’s bindings. An acceptable *proof* of those bindings may be a simple physics engine in a harness or consumer, but Moria does not need to ship or own one. Gravity response, contact resolution as a simulation loop, force-driven crumbling dynamics, and similar runtime physics remain plug-in / consumer concerns; the substrate makes them supportable, not mandatory substrate deliverables.

**Downstream, not this repository** are actual games and game layers: player control, characters, skeletal animation, game-specific presentation, combat rules, AI behavior, economy, building policy, the System / LLM layer, spells, gas pricing, and other gameplay rules—including **which procedural or authored generation pipeline** a game uses and **which physics engine** it chooses. Freeform ship and station games (and their design/combat fiction) remain **future-consumer examples**, not current delivery or validation targets. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Product-level outcomes

These are outcomes the substrate must enable, not a feature inventory or implementation plan:

1. **Truth vs view.** Occupancy, queries, collision truth, and persistence run against voxel matter; meshes, dressing, and debug geometry are derived and disposable. Physics engines, when present, also consume material truth through public bindings rather than a private mesh world.
2. **Contracted consumption.** External consumers install the facade, inspect through public reads, mutate through admitted edits, and never require privileged internal paths.
3. **Sparse scale.** Large regions remain tractable: untouched homogeneous volume stays cheap; only the interesting shell and active edits pay detailed cost.
4. **Mutable everywhere.** Any material cell the contract exposes can be destroyed or placed; cut faces and scars remain honest matter, and presentation rebuilds from truth.
5. **Deep Z is first-class.** Volume along the full depth axis is real content—genuine volumetric depth, not a heightmap floor with painted underground. Caves, strata, ore, aquifers as material bands, and buried structure are material volume, not skybox scenery. Contracts stay volume-general so non-planetary freeform volumes remain expressible; ship/station interiors are future-consumer motivation, not a required current deliverable shape.
6. **Dynamic voxel volumes.** The world is not static geometry alone. The substrate must support voxel volumes that move and can take damage (e.g. players and enemies as material volumes), so future games can treat combatants as matter under the same truth contracts rather than as overlays disconnected from the world.
7. **Physics-ready bindings, not a baked-in engine.** The substrate exposes whatever bindings and material data a physics engine needs to plug in—material strength, gravity, force, and related supportable fields—whether the consumer hand-rolls an engine or adopts one. Runtime physics simulation is **not** a substrate-owned product; a simple engine is an acceptable proof of the seams, not a required deliverable.
8. **Cheap scars over full dumps.** Persistence keeps material edits and related scars tractable—not a dump of every brick. A consumer may choose a reproducible generation function as its base-world strategy; that choice is game-dependent and is not a substrate deliverable.
9. **GPU-resident architecture.** Sparse brick storage and a command/query boundary keep world representation GPU-resident and support asynchronous GPU work. This is a product distinction from CPU-driven voxel engines: residency enables gameplay-scale mutation, meshing, and future simulation without abandoning the consumer contract. Specific kernels and simulations remain milestone-selected; residency and the async-capable boundary do not.
10. **Measurable substrate quality.** Benchmarks and harnesses can evidence mutation response, streaming, GPU memory behavior, collision-truth honesty, physics-binding readiness when exercised, and related contracts without redefining the product as a game. Harness-side generation or a proof physics plug-in used for tests or demos does not make generation or a physics engine a substrate requirement.

**World-dependent presentation.** How a world “looks natural” depends on the consumer’s world—landscape geology, fortress masonry, and other material styles. The substrate must support fully material volumes that read as coherent for their domain; it does not mandate a single overworld aesthetic or a heightmap-with-props look. Ship bulkheads and similar freeform-hull presentation remain future-consumer context, not current validation targets.

## Non-goals

- Shipping a game, game mode, progression loop, or game-rules stack in this product.
- Treating validation-harness content, third-person controllers, demo routes, or machine-specific demo targets as substrate requirements or as mandatory delivery for product completion.
- **Baking deterministic or procedural world generation into the substrate** as product identity. Generation algorithms are game-dependent and run on top of the substrate; consumers own them.
- **Baking in a hand-rolled or third-party physics engine** as product identity. Material strength, gravity, force, and related fields must be *supportable* via plug-in bindings; the engine that consumes them is adjacent. A simple proof engine is optional demonstration, not required delivery.
- Implementing excluded layers here: System / LLM, spells, gas / pricing policy, combat rules / AI behavior, agent labor, building UI / blueprints as gameplay, mechanisms as game entities.
- Delivering or specifically validating freeform ships, stations, or other multi-deck freeform hulls as current product work—those remain future-consumer examples that motivate volume-general contracts.
- Full fluid simulation and cellular automata (fire, wetness, growth) as current product requirements—these may appear as future consumer concepts or format hooks unless later selected explicitly.
- Tree felling or rigid-body conversion of vegetation as current product requirements (future consumer concepts unless later selected).
- Web / wasm as a Product One or substrate target platform.
- Claiming a released, finished visual engine before feasibility and visual-acceptance gates are met.
- Limiting the substrate identity to a Minecraft-style cube aesthetic, a single natural-overworld content palette, static scenery without movable material volumes, or heightmap terrain that only pretends to have depth.
- Assuming gravity-aligned planetary terrain as the only legal world shape in substrate contracts (volume-general contracts are required even though ship/station *content* is not current delivery).

## Future consumers (context only)

Reference material describes possible later products that motivate reusable material-world capabilities. Their gameplay, characters, assets, content palettes, and presentation are **not** current Moria scope. They illustrate what the substrate must remain able to support:

- A System-driven ARPG on a continuous natural world.
- A fortress / colony game with engineering and designation play.
- A descent-style roguelike through deep geology.
- Pure sandboxes.
- Games whose players and enemies are voxel volumes that move and take damage under the same matter contracts as terrain—using collision truth and physics engines plugged in through substrate bindings (gravity, contact, force-driven failure) without those engines living inside Moria.
- A single-captain space trading and combat game in the Escape Velocity mold—starmap, cargo runs, factions, missions—where the ship is a freeform material volume rather than a hull with abstract slots. Reactors, conduits, weapons, and life support occupy real space; power routes are visible geometry; a hit destroys a specific thing rather than a health number. Ship design is conversational with an engineer who proposes layouts and benches them under load; ships wear, fail under stress, and leave wrecks as real salvageable geometry. Debris from fights the player never saw still floats when they arrive. This paragraph is **explicitly nonbinding future-consumer context**: it motivates everywhere mutation, first-class volumetric depth through multi-deck interiors, GPU-resident matter at combat and design scale, physics-ready material bindings under force, and truth-vs-view so damage and salvage stay honest—without importing its fiction, UI, mission systems, freeform-hull *delivery*, or a substrate-owned physics stack into current Moria scope.

The “walkable world” seed describes a third-person proof shape (curated region, forest, ruin, dig-as-demo) that a validation consumer might use to make substrate claims undeniable. That seed’s content, controls, milestones, performance tables, and **curated generation pipeline** remain **context for what the substrate might support**, not the definition of the product itself.

## Confirmed vision constraints

- Adjacent consumers, including any walkable validation harness, have no privileged access path into the substrate.
- The walkable-world harness is an adjacent artifact only—not a required product delivery that defines Moria’s completion.
- The product is a Rust / Bevy library ecosystem for crate consumers, not an ecosystem-neutral engine abstract.
- Everywhere mutation and first-class deep Z (genuine volumetric depth as real content, not heightmap terrain) are binding product outcomes; natural-looking presentation is world-dependent, not a single mandated overworld aesthetic.
- Substrate contracts are **volume-general** and must not assume gravity-aligned planetary terrain as the only world shape; ships and stations remain future-consumer examples, not current delivery or validation targets.
- Dynamic voxel volumes (movable, damageable matter such as players and enemies) are a binding product capability; the world is not static geometry alone.
- **Physics is adjacent, not baked in.** The substrate must expose plug-in bindings and supportable material data (strength, gravity, force, and related fields) so a physics engine—hand-rolled or not—can attach. Owning or shipping a physics engine is not substrate product; a simple engine is only an acceptable proof of the seams.
- Procedural / deterministic world generation is **not** substrate product; it is game-dependent and lives above the substrate.
- GPU residency and an async-capable command/query boundary are binding product direction and a deliberate distinction from CPU-driven voxel engines.
- When seed documents conflict, the project boundary wins: substrate product first; game examples and Product One detail are nonbinding unless selected by that boundary or an explicit human decision.

## Assumptions proposed for approval

1. **Authority order stands.** `docs/seeds/project-boundary.md` is binding for product identity; the walkable-world seed and broad architecture reference inform capabilities and seams only.
2. **Harnesses are adjacent.** Curation, bench, and demo executables may ship in-repo to prove contracts; they do not redefine Moria as a game or as “the walkable demo.”
3. **Volume-general contracts without ship/station delivery (affirmed).** “World” means continuous material volume in general—not only planetary terrain—so freeform volumes can share the same contracts later. Current product does **not** deliver or specifically validate ships and stations; those remain future-consumer examples.
4. **Generation above the substrate (affirmed).** Any in-repo or example generator exists to exercise or demo contracts; it does not establish generation as a required substrate deliverable.
5. **Physics via bindings, engine adjacent (affirmed).** Material strength, gravity, force, and related fields are supportable through exposed substrate bindings. A physics engine is not baked into Moria; a simple proof engine is optional and adjacent.

## Resolved human decisions

| Question | Decision |
| --- | --- |
| **Q1.** Is a walkable-world visual validation harness mandatory current delivery or only a permitted adjacent artifact? | **Adjacent artifact.** It may exist for validation; it does not define product identity or “done.” Moria is not limited to a Minecraft-style world. |
| **Q2.** Are natural-looking terrain, everywhere mutation, and first-class deep Z binding current outcomes? | **Everywhere mutation and first-class deep Z are binding.** Deep Z means genuine volumetric depth, not heightmap terrain. Natural-looking presentation **depends on the consumer’s world**—required as coherent material presentation for that domain, not as a single natural-overworld mandate. |
| **Q3.** Is GPU-resident / asynchronous-GPU-capable architecture binding current direction? | **Yes.** GPU residency is an important product feature: it enables many gameplay capabilities and is a core distinction between Moria and CPU-driven voxel engines. Specific simulations remain milestone-selected. |
| **Q4.** Multi-world freeform volumes: does current product identity include ship/station material volumes on the same contracts as natural geology? | **Contracts are volume-general** (must not assume gravity-aligned planetary terrain only). **Delivering or specifically validating ships and stations is not current scope**; they remain future-consumer examples. |
| **Q5.** Must the substrate support dynamic (moving, damageable) voxel volumes—not only static world geometry? | **Yes.** Players and enemies will be voxel volumes that move and can take damage; the engine must support that class of matter. |
| **Q6.** Is deterministic / procedural world generation part of the substrate product? | **No.** Generation is an algorithm that runs on top of the substrate and is game-dependent; it must not be baked into substrate identity. |
| **Q7.** Is matter physics (collision for moving entities, gravity, force for explosions/crumbling) in product scope? | **Bindings yes; engine no.** The substrate exposes plug-in bindings and supportable material data (strength, gravity, force, etc.) so a physics engine can attach—hand-rolled or third-party. A full or hand-rolled physics engine is **not** baked into the substrate; it is **100% adjacent**. An acceptable proof of the bindings is a simple physics engine, but shipping one is not required. Collision/occupancy **truth** against voxel matter remains a substrate concern so plug-ins and consumers share one material world. |

## Open questions (identity)

None currently open that would change product identity, purpose, or boundary. Engineering and milestone sequencing (which material properties land first, how gravity is parameterized for non-planetary volumes, binding shape for force and strength) remain design/TDD concerns, not vision-identity blockers.

Older planning docs that still title the effort as “Product One — The Walkable World” should be read as superseded on identity by the boundary and this vision once approved. Engineering docs that still list deterministic generation under the public facade describe current implementation or harness practice; this vision treats generation as **not** substrate product identity per Q6. Seed language that listed structural integrity / cave-ins or rigid conversion as non-current remains context for future consumers; this vision requires **physics-ready bindings** (Q7) without importing a substrate-owned physics engine, DF span tables, or fortress UI. Full fluid CA, fire/growth CA, and tree-felling remain non-goals until explicitly selected.

## Seed synthesis

| Source | Contribution to this vision |
| --- | --- |
| **README.md** | Names Moria as a Rust/Bevy voxel-world substrate consumed as a crate; executables curate, exercise, benchmark, and visually validate; player controllers, characters, and game-specific presentation are outside the substrate boundary; status is active engineering, not a released engine. Workspace crate roles and evidence detail stay subordinate engineering input. README generation claims are treated as engineering/harness practice, not as vision identity after human correction (Q6). |
| **docs/seeds/project-boundary.md** | **Binding product target:** reusable substrate (crate family); games are separate consumers; any walkable executable is a public-interface validation harness only; System / LLM / spell / gas / combat / AI / building layers are out of scope. Cargo workspace split is motivated but left as technical design. |
| **docs/seeds/product-one-seed.md** | **Downstream / validation example:** supplies a “fully material walkable world” proof shape and explicit non-goals (no combat, System, CA, building UI, dynamic fluids). Motivates smooth material truth, dig/place proof, geology, sparse streaming, seed+delta save, and measurable quality. Does **not** import third-person character, curated 1 km postcard, content palette, performance tables, milestone schedule, or generation pipeline into product identity. GPU memory language in that seed aligns with the affirmed GPU-resident direction without importing demo numbers as substrate law. Collision-against-truth in that seed aligns with substrate occupancy/collision truth without importing demo controller rules or a substrate physics engine as product law. |
| **docs/seeds/voxel-world-substrate.md** | **Architecture reference:** long-horizon goals include reads as a coherent world for its domain; mutable everywhere; deep Z; substrate not game; and GPU-resident direction—now affirmed as product outcomes/direction where stated above. Its generation-layer detail is **reference for consumers**, not a substrate mandate (per Q6). Layering diagram and many extensions (fluids tier 2+, weather, building verbs, nav, multiplayer readiness) remain context or future seams unless selected. Integrity / cave-in and other physics *simulations* stay consumer- or plug-in-facing; substrate **bindings** for material strength, gravity, and force are elevated by Q7 without baking an engine or importing DF span tables or fortress UI. Game examples (ARPG, fortress, Moria descent, freeform ships) motivate reusability only. Companion `gpu-resident-substrate.md` (supporting architecture per seeds README) details the GPU-resident claim without becoming a separate product target. |
| **Human review (prior passes)** | Resolved Q1–Q6 as tabulated above. Added a space-trading / freeform-ship consumer as **nonbinding** motivation for volume-general mutability, deep interiors, GPU-resident matter, and honest damage/salvage—without importing that game’s systems into current scope. An earlier pass treated matter physics as a baked-in substrate capability (prior Q7 wording); superseded by the correction below. |
| **Human review (this pass)** | **Q7 corrected:** do **not** bake in a hand-rolled physics engine. The substrate exposes bindings so a physics engine can plug in (hand-rolled or not). Material strength, gravity, force, and related fields must be supportable, not baked into a substrate-owned simulator. An acceptable proof is a simple physics engine; owning one is not required—physics is **100% adjacent**. |

Authority among seeds (from `docs/seeds/README.md`): project boundary first; GPU-resident architecture note second as supporting principles (now elevated to product direction by human decision where residency and async boundary are concerned); broad voxel reference third; Product One seed last as validation example. Conflicts resolve toward the boundary without silent expansion of scope. Explicit human decisions (including Q4–Q7 as corrected) override earlier vision wording and seed implication when they reclassify substrate vs consumer ownership.
