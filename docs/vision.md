# Project vision

## What we are building now

**Authority claim: C-001.**

**Moria** is a reusable voxel-world substrate: a Rust and Bevy library (or small family of crates) that downstream games and tools install and drive through a public consumer contract. It is engine-shaped world infrastructure, not a playable game product.

**Authority claim: C-002.**

It is not limited to a Minecraft-style cube aesthetic or a single overworld content palette. The material contracts target continuous three-dimensional volumes—natural landscapes, underground geology, and constructed interiors among them—and those volumes are not only static scenery. **Substrate contracts are volume-general:** they must not assume gravity-aligned planetary terrain as the only kind of world. Delivering or specifically validating freeform ship and station hulls is **not** current scope; those remain future-consumer examples that the contracts should remain able to support.

## Purpose

**Authority claims: C-003, C-004, C-005, C-006, C-007, C-008, C-009, C-010, C-011, C-015, C-016.**

Voxel worlds only work when several hard systems agree as explicit contracts: sparse material truth, bounded inspection, mutation admission, streaming lifecycle, collision against matter rather than presentation, persistence of world matter plus edits, GPU-resident representation of that matter, measurable presentation derived from truth, and **generic seams so external behavior can plug in** without privileged access to voxel storage. Moria exists so material worlds can be consumed without each game rebuilding those contracts, and without hiding them behind a single privileged demo.

The product’s claim is infrastructural: a consumer can obtain a continuous three-dimensional material world (including movable voxel volumes), inspect and mutate it only through supported interfaces, keep authoritative matter GPU-resident for scale and gameplay-enabling work, and trust that what they see and collide with is a view of the same authoritative matter. External plug-ins may implement physics, damage, or other behavior by observing that truth and requesting admitted changes, but the substrate does not prescribe their concepts, state, rules, or outcomes.

**Authority claim: C-012. World generation is not part of the substrate identity.** How a game fills or seeds sparse material volumes is a consumer- or game-dependent algorithm that runs *on top of* the substrate. The substrate provides storage, query, mutation, streaming, collision-truth seams, generic behavior-extension seams, persistence seams, and presentation derivation for material truth—not a baked-in procedural generator, physics engine, or damage system.

## Product boundary

**Authority claims: C-001, C-002, C-003, C-004, C-005, C-006, C-007, C-008, C-009, C-010, C-011, C-012, C-013, C-014, C-015, C-016.**

**This product owns** the reusable substrate and its public facade:

- Sparse storage and lazy materialization of voxel truth.
- GPU-resident sparse representation and a command/query boundary that can support asynchronous GPU work without changing the consumer contract.
- Bounded world inspection and telemetry for consumers.
- Mutation admission (dig, place, and related world-edit verbs) so nothing touches voxels outside the contract.
- Streaming and lifecycle so large regions do not require full raw-voxel residency.
- Collision and occupancy **truth** against voxel matter, not against disposable meshes—queries and contacts read material authority so consumers and plug-ins do not invent a second world.
- **Generic behavior-extension seams:** external systems can observe authoritative matter and lifecycle changes and request admitted mutations without privileged voxel access. Physics, damage, fracture, health, gravity, force, and similar semantics and state belong entirely to those external systems.
- Support for **dynamic voxel volumes**—material volumes that can move and be mutated through the same truth contracts as static world geometry. What movement or mutation *means* is consumer behavior, not substrate policy.
- Persistence of material truth plus edit deltas (and related world-state scars), without requiring a dump of every brick; how the *base* volume is produced is the consumer’s concern.
- Presentation support that derives surfaces and surface dressing from material truth, without serializing derived geometry as authority.
- Object and clutter registration hooks so vegetation, micro-objects, and other matter-backed assemblies can register without baking into a single terrain slab.
- Seams a consumer can use to inject or drive world content (including generation algorithms), without embedding any particular generator as substrate law.
- **Volume-general contracts** that do not encode planetary heightmap assumptions or gravity-aligned terrain as the only legal world shape.

**Authority claim: C-013. Adjacent, not the product identity** are curation, benchmark, and visual-validation executables (`moria-curate`, `moria-bench`, `moria-demo` and similar). They may exist to curate content parameters, exercise contracts, capture evidence, and visually validate the substrate, but they must consume the same public interfaces available to an external game. Controllers, characters, cameras, authored demo routes, presentation polish, acceptance scenarios, and **game-specific generation algorithms** belong to those consumers—not to substrate identity. A walkable-world harness is a **permitted adjacent artifact**, not a mandatory definition of “done” for Moria. In-repo generation used only to exercise or demonstrate contracts remains a harness concern, not a claim that generation is substrate product.

**Also adjacent (100% adjacent): behavior systems such as physics and damage.** A hand-rolled or third-party system may integrate through the substrate’s generic extension seams, but Moria does not ship or own its behavioral vocabulary, state model, or rules. Gravity, force, contact response, health, resistance, damage types, fracture, crumbling, and similar behavior remain plug-in / consumer concerns. An optional proof may demonstrate that a plug-in can observe truth and submit admitted changes; it does not make any particular behavior model a substrate deliverable.

**Authority claim: C-014. Downstream, not this repository** are actual games and game layers: player control, characters, skeletal animation, game-specific presentation, combat rules, AI behavior, economy, building policy, the System / LLM layer, spells, gas pricing, and other gameplay rules—including **which procedural or authored generation pipeline** a game uses and **which physics engine** it chooses. Freeform ship and station games (and their design/combat fiction) remain **future-consumer examples**, not current delivery or validation targets. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Product-level outcomes

These are outcomes the substrate must enable, not a feature inventory or implementation plan:

1. **Truth vs view.** Occupancy, queries, collision truth, and persistence run against voxel matter; meshes, dressing, and debug geometry are derived and disposable. External behavior systems also consume material truth through public seams rather than inventing a private second world.
2. **Contracted consumption.** External consumers install the facade, inspect through public reads, mutate through admitted edits, and never require privileged internal paths.
3. **Sparse scale.** Large regions remain tractable: untouched homogeneous volume stays cheap; only the interesting shell and active edits pay detailed cost.
4. **Mutable everywhere.** Any material cell the contract exposes can be destroyed or placed; cut faces and scars remain honest matter, and presentation rebuilds from truth.
5. **Deep Z is first-class.** Volume along the full depth axis is real content—genuine volumetric depth, not a heightmap floor with painted underground. Caves, strata, ore, aquifers as material bands, and buried structure are material volume, not skybox scenery. Contracts stay volume-general so non-planetary freeform volumes remain expressible; ship/station interiors are future-consumer motivation, not a required current deliverable shape.
6. **Dynamic voxel volumes.** The world is not static geometry alone. The substrate must support material volumes that move and can be mutated, so future games can represent moving matter under the same truth contracts rather than as overlays disconnected from the world. Damage and movement rules remain external behavior.
7. **Behavior-extension seams, not behavior policy.** External systems can observe authoritative matter and submit admitted changes through stable public seams. The substrate does not define physics or damage concepts, canonical behavioral fields, simulation state, or response rules. A proof plug-in may demonstrate the seam without becoming a required engine or behavior model.
8. **Cheap scars over full dumps.** Persistence keeps material edits and related scars tractable—not a dump of every brick. A consumer may choose a reproducible generation function as its base-world strategy; that choice is game-dependent and is not a substrate deliverable.
9. **GPU-resident architecture.** Sparse brick storage and a command/query boundary keep world representation GPU-resident and support asynchronous GPU work. This is a product distinction from CPU-driven voxel engines: residency enables gameplay-scale mutation, meshing, and future simulation without abandoning the consumer contract. Specific kernels and simulations remain milestone-selected; residency and the async-capable boundary do not.
10. **Measurable substrate quality.** Benchmarks and harnesses can evidence mutation response, streaming, GPU memory behavior, collision-truth honesty, and behavior-extension readiness when exercised without redefining the product as a game. Harness-side generation or a proof behavior plug-in used for tests or demos does not make generation, physics, or damage behavior a substrate requirement.

**World-dependent presentation.** How a world “looks natural” depends on the consumer’s world—landscape geology, fortress masonry, and other material styles. The substrate must support fully material volumes that read as coherent for their domain; it does not mandate a single overworld aesthetic or a heightmap-with-props look. Ship bulkheads and similar freeform-hull presentation remain future-consumer context, not current validation targets.

## Non-goals

- Shipping a game, game mode, progression loop, or game-rules stack in this product.
- Treating validation-harness content, third-person controllers, demo routes, or machine-specific demo targets as substrate requirements or as mandatory delivery for product completion.
- **Baking deterministic or procedural world generation into the substrate** as product identity. Generation algorithms are game-dependent and run on top of the substrate; consumers own them.
- **Baking in physics, damage, or another behavior model** as product identity. The substrate exposes generic extension seams, not canonical strength, gravity, force, health, resistance, fracture, damage, or solver fields and rules. A proof plug-in is optional demonstration, not required delivery.
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
- Games whose players and enemies are voxel volumes that move and are changed under the same matter contracts as terrain, while external physics and damage systems own every behavioral rule and submit their results through admitted mutations.
- A single-captain space trading and combat game in the Escape Velocity mold—starmap, cargo runs, factions, missions—where the ship is a freeform material volume rather than a hull with abstract slots. Reactors, conduits, weapons, and life support occupy real space; power routes are visible geometry; a hit destroys a specific thing rather than a health number. Ship design is conversational with an engineer who proposes layouts and benches them under load; ships wear, fail under stress, and leave wrecks as real salvageable geometry. Debris from fights the player never saw still floats when they arrive. This paragraph is **explicitly nonbinding future-consumer context**: it motivates everywhere mutation, first-class volumetric depth through multi-deck interiors, GPU-resident matter at combat and design scale, generic behavior-extension seams, and truth-vs-view so externally defined damage and salvage stay honest—without importing its fiction, UI, mission systems, freeform-hull *delivery*, or any behavior model into current Moria scope.

The “walkable world” seed describes a third-person proof shape (curated region, forest, ruin, dig-as-demo) that a validation consumer might use to make substrate claims undeniable. That seed’s content, controls, milestones, performance tables, and **curated generation pipeline** remain **context for what the substrate might support**, not the definition of the product itself.

## Confirmed vision constraints

- Adjacent consumers, including any walkable validation harness, have no privileged access path into the substrate.
- The walkable-world harness is an adjacent artifact only—not a required product delivery that defines Moria’s completion.
- The product is a Rust / Bevy library ecosystem for crate consumers, not an ecosystem-neutral engine abstract.
- Everywhere mutation and first-class deep Z (genuine volumetric depth as real content, not heightmap terrain) are binding product outcomes; natural-looking presentation is world-dependent, not a single mandated overworld aesthetic.
- Substrate contracts are **volume-general** and must not assume gravity-aligned planetary terrain as the only world shape; ships and stations remain future-consumer examples, not current delivery or validation targets.
- Dynamic voxel volumes (movable, externally mutable matter) are a binding product capability; the world is not static geometry alone.
- **Physics and damage are adjacent behavior, not baked in.** The substrate exposes generic observation and mutation seams. External systems own their concepts, state, and rules; Moria does not define canonical strength, gravity, force, health, resistance, fracture, damage, or solver semantics.
- Procedural / deterministic world generation is **not** substrate product; it is game-dependent and lives above the substrate.
- GPU residency and an async-capable command/query boundary are binding product direction and a deliberate distinction from CPU-driven voxel engines.
- When seed documents conflict, the project boundary wins: substrate product first; game examples and Product One detail are nonbinding unless selected by that boundary or an explicit human decision.

## Assumptions proposed for approval

1. **Authority order stands.** `docs/seeds/project-boundary.md` is binding for product identity; the walkable-world seed and broad architecture reference inform capabilities and seams only.
2. **Harnesses are adjacent.** Curation, bench, and demo executables may ship in-repo to prove contracts; they do not redefine Moria as a game or as “the walkable demo.”
3. **Volume-general contracts without ship/station delivery (affirmed).** “World” means continuous material volume in general—not only planetary terrain—so freeform volumes can share the same contracts later. Current product does **not** deliver or specifically validate ships and stations; those remain future-consumer examples.
4. **Generation above the substrate (affirmed).** Any in-repo or example generator exists to exercise or demo contracts; it does not establish generation as a required substrate deliverable.
5. **Behavior via generic seams (affirmed).** Physics, damage, and related behavior attach through generic observation and admitted-mutation seams. Their concepts, state, and rules remain external. A proof plug-in is optional and adjacent.

## Resolved human decisions

| Question | Decision |
| --- | --- |
| **AD-001 / Q1.** Is a walkable-world visual validation harness mandatory current delivery or only a permitted adjacent artifact? | **Adjacent artifact.** It may exist for validation; it does not define product identity or “done.” Moria is not limited to a Minecraft-style world. |
| **AD-002 / Q2.** Are natural-looking terrain, everywhere mutation, and first-class deep Z binding current outcomes? | **Everywhere mutation and first-class deep Z are binding.** Deep Z means genuine volumetric depth, not heightmap terrain. Natural-looking presentation **depends on the consumer’s world**—required as coherent material presentation for that domain, not as a single natural-overworld mandate. |
| **AD-003 / Q3.** Is GPU-resident / asynchronous-GPU-capable architecture binding current direction? | **Yes.** GPU residency is an important product feature: it enables many gameplay capabilities and is a core distinction between Moria and CPU-driven voxel engines. Specific simulations remain milestone-selected. |
| **AD-004 / Q4.** Multi-world freeform volumes: does current product identity include ship/station material volumes on the same contracts as natural geology? | **Contracts are volume-general** (must not assume gravity-aligned planetary terrain only). **Delivering or specifically validating ships and stations is not current scope**; they remain future-consumer examples. |
| **AD-005 / Q5.** Must the substrate support dynamic voxel volumes—not only static world geometry? | **Yes.** The substrate supports material volumes that move and can be mutated under the same truth contracts as static matter. Physics, damage, and other behavior that causes those changes remains external. |
| **AD-006 / Q6.** Is deterministic / procedural world generation part of the substrate product? | **No.** Generation is an algorithm that runs on top of the substrate and is game-dependent; it must not be baked into substrate identity. |
| **AD-007 / Q7.** Are physics or damage behavior in product scope? | **Generic extension seams yes; behavior policy no.** External systems observe authoritative matter and request admitted changes. They own gravity, force, contact response, health, resistance, damage, fracture, and all related state and rules. Collision/occupancy **truth** against voxel matter remains a substrate concern so plug-ins and consumers share one material world. A proof plug-in is optional; no particular engine or behavior vocabulary is required. |

## Open questions (identity)

None currently open that would change product identity, purpose, or boundary. Engineering and milestone sequencing—including the generic extension contract’s technical shape—remain design/TDD concerns, not vision-identity blockers.

Older planning docs that still title the effort as “Product One — The Walkable World” should be read as superseded on identity by the boundary and this vision once approved. Engineering docs that still list deterministic generation under the public facade describe historical implementation or harness practice; this vision treats generation as **not** substrate product identity per Q6. Seed language that listed structural integrity, cave-ins, rigid conversion, damage, physics fields, or simulation rules remains context for future consumers; this vision requires only **generic behavior-extension seams** (Q7), without importing any behavior model, DF span tables, or fortress UI. Full fluid CA, fire/growth CA, and tree-felling remain non-goals until explicitly selected.

## Seed synthesis

| Source | Contribution to this vision |
| --- | --- |
| **README.md** | Names Moria as a Rust/Bevy voxel-world substrate consumed as a crate; executables curate, exercise, benchmark, and visually validate; player controllers, characters, and game-specific presentation are outside the substrate boundary; status is active engineering, not a released engine. Workspace crate roles and evidence detail stay subordinate engineering input. README generation claims are treated as engineering/harness practice, not as vision identity after human correction (Q6). |
| **docs/seeds/project-boundary.md** | **Binding product target:** reusable substrate (crate family); games are separate consumers; any walkable executable is a public-interface validation harness only; System / LLM / spell / gas / combat / AI / building layers are out of scope. Cargo workspace split is motivated but left as technical design. |
| **docs/seeds/product-one-seed.md** | **Downstream / validation example:** supplies a “fully material walkable world” proof shape and explicit non-goals (no combat, System, CA, building UI, dynamic fluids). Motivates smooth material truth, dig/place proof, geology, sparse streaming, seed+delta save, and measurable quality. Does **not** import third-person character, curated 1 km postcard, content palette, performance tables, milestone schedule, or generation pipeline into product identity. GPU memory language in that seed aligns with the affirmed GPU-resident direction without importing demo numbers as substrate law. Collision-against-truth in that seed aligns with substrate occupancy/collision truth without importing demo controller rules or a substrate physics engine as product law. |
| **docs/seeds/voxel-world-substrate.md** | **Architecture reference:** long-horizon goals include reads as a coherent world for its domain; mutable everywhere; deep Z; substrate not game; and GPU-resident direction—now affirmed as product outcomes/direction where stated above. Its generation-layer detail is **reference for consumers**, not a substrate mandate (per Q6). Layering diagram and many extensions (fluids tier 2+, weather, building verbs, nav, multiplayer readiness) remain context or future seams unless selected. Integrity, cave-ins, physics, damage, and related simulations and fields stay consumer- or plug-in-owned; Q7 selects only generic behavior-extension seams without importing their concepts, DF span tables, or fortress UI. Game examples (ARPG, fortress, Moria descent, freeform ships) motivate reusability only. Companion `gpu-resident-substrate.md` (supporting architecture per seeds README) details the GPU-resident claim without becoming a separate product target. |
| **Human review (prior passes)** | Resolved Q1–Q6 as tabulated above. Added a space-trading / freeform-ship consumer as **nonbinding** motivation for volume-general mutability, deep interiors, GPU-resident matter, and honest damage/salvage—without importing that game’s systems into current scope. An earlier pass treated matter physics as a baked-in substrate capability (prior Q7 wording); superseded by the correction below. |
| **Human review (latest clarification)** | **Q7 corrected:** do not bake in a physics engine, damage system, or their vocabulary. The substrate exposes generic observation and admitted-mutation seams; external behavior owns gravity, force, contact response, health, resistance, damage, fracture, and all related state and rules. An optional proof plug-in demonstrates the seam only. |

Authority among seeds (from `docs/seeds/README.md`): project boundary first; GPU-resident architecture note second as supporting principles (now elevated to product direction by human decision where residency and async boundary are concerned); broad voxel reference third; Product One seed last as validation example. Conflicts resolve toward the boundary without silent expansion of scope. Explicit human decisions (including Q4–Q7 as corrected) override earlier vision wording and seed implication when they reclassify substrate vs consumer ownership.
