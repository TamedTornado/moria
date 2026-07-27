# Moria product vision

This document is the standalone product vision for **Moria**. A downstream product designer should be able to use it without reopening seed material or adjudicating which seed is authoritative.

**Authority.** `docs/vision.md` is the human-approved scope boundary. It defines what is current, adjacent, future, excluded, and unresolved. This vision synthesizes product substance within that boundary. When any supporting idea conflicts with the scope boundary, the scope boundary wins.

---

## What Moria is

Moria is a **reusable voxel-world substrate**: engine-shaped world infrastructure that downstream games and tools install and drive through a public consumer contract. It is a Rust and Bevy library (or small family of crates) for crate consumers—not a playable game, not a finished visual product by itself, and not an ecosystem-neutral engine abstract.

The product’s claim is infrastructural. A consumer can obtain a continuous three-dimensional material world; inspect and mutate that world only through supported interfaces; keep authoritative matter GPU-resident for scale and gameplay-enabling work; and trust that presentation and collision read the same material authority—while gravity, force, material strength, and related physical response remain **supportable through exposed bindings**, not baked into a substrate-owned physics engine.

Moria is not limited to a Minecraft-style cube aesthetic or a single overworld content palette. Material contracts target continuous three-dimensional volumes—natural landscapes, underground geology, and constructed interiors among them. Those volumes are not only static scenery. **Substrate contracts are volume-general:** they must not assume gravity-aligned planetary terrain as the only legal world shape. Delivering or specifically validating freeform ship and station hulls is **not** current scope; those remain future-consumer examples that the contracts should remain able to support.

---

## Purpose

Voxel worlds only work when several hard systems agree as explicit contracts:

- sparse material truth
- bounded inspection
- mutation admission
- streaming lifecycle
- collision against matter rather than presentation
- persistence of world matter plus edits
- GPU-resident representation of that matter
- measurable presentation derived from truth
- seams so a physics engine can plug in without privileged access to voxel storage

Moria exists so material worlds can be consumed without each game rebuilding those contracts, and without hiding them behind a single privileged demo.

**World generation is not part of the substrate identity.** How a game fills or seeds sparse material volumes is a consumer- or game-dependent algorithm that runs *on top of* the substrate. The substrate provides storage, query, mutation, streaming, collision-truth seams, physics plug-in bindings, persistence seams, and presentation derivation for material truth—not a baked-in procedural generator and not a hand-rolled physics engine.

---

## Who consumes it and what they need

### Primary consumers

- **External game and tool crates** that install the public facade and own their own generation, rules, presentation polish, and physics engine choice.
- **In-repo adjacent harnesses** (curation, benchmarks, visual validation, optional walkable demos) that exercise the same public interfaces an external game would use. They prove contracts; they do not redefine the product as a game.

### Consumer needs the product must meet

| Need | What “good” means |
| --- | --- |
| Install and drive the world without privileged internals | External consumers inspect through public reads, mutate through admitted edits, and never require private storage paths. Validation harnesses share that same boundary. |
| One material authority | Occupancy, queries, collision truth, and persistence run against voxel matter. Meshes, dressing, and debug geometry are derived views and are never serialized as truth. |
| Scale without full raw-voxel residency | Large regions stay tractable: untouched homogeneous volume stays cheap; only the interesting shell and active edits pay detailed cost. |
| Mutate anywhere the contract exposes | Any material cell the contract exposes can be destroyed or placed; cut faces and scars remain honest matter; presentation rebuilds from truth. |
| Genuine volumetric depth | Volume along the full depth axis is real content—caves, strata, ore, aquifers as material bands, buried structure—not a heightmap floor with painted underground. |
| Movable, damageable matter | The world is not static geometry alone. Voxel volumes that move and can take damage (players, enemies, and similar) share the same truth contracts as terrain. |
| Physics without owning a physics stack | Material strength, gravity, force, and related supportable fields are exposed so a hand-rolled or third-party engine can attach. Runtime physics remains a plug-in / consumer concern. |
| Tractable persistence of change | Material edits and related scars stay cheap to keep—not a dump of every region of untouched volume. Base-volume production strategy is the consumer’s concern. |
| GPU-scale residency with a stable contract | Sparse representation stays GPU-resident; a command/query boundary supports asynchronous GPU work without changing the consumer contract or granting direct buffer access. |
| Room for domain-specific worlds | Contracts stay volume-general so freeform volumes remain expressible later; coherent material presentation depends on the consumer’s world, not on a single mandated overworld look. |

---

## Product boundary

### This product owns

The reusable substrate and its public facade:

1. **Sparse storage and lazy materialization** of voxel truth.
2. **GPU-resident sparse representation** and a command/query boundary that can support asynchronous GPU work without changing the consumer contract.
3. **Bounded world inspection and telemetry** for consumers.
4. **Mutation admission** (dig, place, and related world-edit verbs) so nothing touches voxels outside the contract.
5. **Streaming and lifecycle** so large regions do not require full raw-voxel residency.
6. **Collision and occupancy truth** against voxel matter, not against disposable meshes—queries and contacts read material authority so consumers and plug-ins do not invent a second world.
7. **Physics plug-in bindings:** material properties and world seams a physics engine needs—material strength, gravity parameters, applied force, and related supportable fields—exposed so a hand-rolled or third-party engine can attach without privileged voxel access. The substrate does **not** bake in a physics engine.
8. **Dynamic voxel volumes**—matter that moves and can take damage (players, enemies, and similar)—not only static world geometry.
9. **Persistence of material truth plus edit deltas** (and related world-state scars), without requiring a dump of every brick; how the *base* volume is produced is the consumer’s concern.
10. **Presentation support** that derives surfaces and surface dressing from material truth, without serializing derived geometry as authority.
11. **Object and clutter registration hooks** so vegetation, micro-objects, and other matter-backed assemblies can register without baking into a single terrain slab.
12. **Seams a consumer can use to inject or drive world content** (including generation algorithms), without embedding any particular generator as substrate law.
13. **Volume-general contracts** that do not encode planetary heightmap assumptions or gravity-aligned terrain as the only legal world shape.

### Adjacent (not product identity)

- **Curation, benchmark, and visual-validation executables** and similar harnesses. They may curate parameters, exercise contracts, capture evidence, and visually validate the substrate, but they must consume the same public interfaces available to an external game. Controllers, characters, cameras, authored demo routes, presentation polish, acceptance scenarios, and game-specific generation algorithms belong to those consumers—not to substrate identity. A walkable-world harness is a **permitted adjacent artifact**, not a mandatory definition of “done” for Moria. In-repo generation used only to exercise or demonstrate contracts remains a harness concern, not a claim that generation is substrate product.
- **Any physics engine**—hand-rolled or third-party—that integrates through the substrate’s bindings. An acceptable *proof* of those bindings may be a simple physics engine in a harness or consumer, but Moria does not need to ship or own one. Gravity response, contact resolution as a simulation loop, force-driven crumbling dynamics, and similar runtime physics remain plug-in / consumer concerns; the substrate makes them supportable, not mandatory substrate deliverables.

### Downstream / future (not this repository’s product)

Actual games and game layers are not Moria:

- player control, characters, skeletal animation, game-specific presentation
- combat rules, AI behavior, economy, building policy
- System / LLM layers, spells, gas pricing, and other gameplay rules
- which procedural or authored generation pipeline a game uses
- which physics engine a game chooses
- freeform ship and station games (and their design/combat fiction)—**future-consumer examples**, not current delivery or validation targets

Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

### Explicitly excluded from current product requirements

Unless a later explicit human decision selects them, the product does **not** require:

- shipping a game, game mode, progression loop, or game-rules stack
- treating validation-harness content, third-person controllers, demo routes, or machine-specific demo targets as substrate requirements or as mandatory delivery for product completion
- baking deterministic or procedural world generation into the substrate as product identity
- baking in a hand-rolled or third-party physics engine as product identity
- System / LLM, spells, gas / pricing policy, combat rules / AI behavior, agent labor, building UI / blueprints as gameplay, mechanisms as game entities
- delivering or specifically validating freeform ships, stations, or other multi-deck freeform hulls as current product work
- full fluid simulation and cellular automata (fire, wetness, growth) as current product requirements
- tree felling or rigid-body conversion of vegetation as current product requirements
- web / wasm as a substrate target platform
- claiming a released, finished visual engine before feasibility and visual-acceptance gates are met
- limiting substrate identity to a Minecraft-style cube aesthetic, a single natural-overworld content palette, static scenery without movable material volumes, or heightmap terrain that only pretends to have depth
- assuming gravity-aligned planetary terrain as the only legal world shape in substrate contracts

---

## Product-level outcomes

These are outcomes the substrate must enable, guarantee, or make provable—not a feature inventory or implementation plan.

1. **Truth vs view.** Occupancy, queries, collision truth, and persistence run against voxel matter; meshes, dressing, and debug geometry are derived and disposable. Physics engines, when present, also consume material truth through public bindings rather than a private mesh world.
2. **Contracted consumption.** External consumers install the facade, inspect through public reads, mutate through admitted edits, and never require privileged internal paths. Adjacent harnesses use the same contract.
3. **Sparse scale.** Large regions remain tractable: untouched homogeneous volume stays cheap; only the interesting shell and active edits pay detailed cost.
4. **Mutable everywhere.** Any material cell the contract exposes can be destroyed or placed; cut faces and scars remain honest matter; presentation rebuilds from truth.
5. **Deep Z is first-class.** Volume along the full depth axis is real content—genuine volumetric depth, not a heightmap floor with painted underground. Contracts stay volume-general so non-planetary freeform volumes remain expressible; ship/station interiors are future-consumer motivation, not a required current deliverable shape.
6. **Dynamic voxel volumes.** The substrate supports voxel volumes that move and can take damage so future games can treat combatants as matter under the same truth contracts rather than as overlays disconnected from the world.
7. **Physics-ready bindings, not a baked-in engine.** The substrate exposes bindings and material data a physics engine needs to plug in—material strength, gravity, force, and related supportable fields. Runtime physics simulation is **not** a substrate-owned product; a simple engine is an acceptable proof of the seams, not a required deliverable.
8. **Cheap scars over full dumps.** Persistence keeps material edits and related scars tractable. A consumer may choose a reproducible generation function as its base-world strategy; that choice is game-dependent and is not a substrate deliverable.
9. **GPU-resident architecture.** Sparse storage and a command/query boundary keep world representation GPU-resident and support asynchronous GPU work. This is a product distinction from CPU-driven voxel engines: residency enables gameplay-scale mutation, meshing, and future simulation without abandoning the consumer contract. Specific kernels and simulations remain milestone-selected; residency and the async-capable boundary do not.
10. **Measurable substrate quality.** Benchmarks and harnesses can evidence mutation response, streaming, GPU memory behavior, collision-truth honesty, physics-binding readiness when exercised, and related contracts without redefining the product as a game. Harness-side generation or a proof physics plug-in used for tests or demos does not make generation or a physics engine a substrate requirement.

**World-dependent presentation.** How a world “looks natural” depends on the consumer’s world—landscape geology, fortress masonry, and other material styles. The substrate must support fully material volumes that read as coherent for their domain; it does not mandate a single overworld aesthetic or a heightmap-with-props look. Freeform-hull presentation remains future-consumer context, not a current validation target.

---

## Capabilities the product must enable

Stated as product capabilities, not technical design:

| Capability | Guarantee / enablement |
| --- | --- |
| Sparse material world | Represent continuous 3D material volumes sparsely so idle or homogeneous regions do not force full detailed residency. |
| Public inspect | Bounded reads, snapshots, telemetry, or events sufficient for consumers and tools to understand world state without private storage access. |
| Public mutate | Explicit admitted edit verbs so dig, place, and related changes go through the contract only. |
| Stream and lifecycle | Bring regions into and out of active residency so large worlds remain workable. |
| Collision/occupancy truth | Queries and contacts against material authority, shared by consumers and physics plug-ins. |
| Physics plug-in seams | Supportable material strength, gravity, force, and related fields so an external engine can attach. |
| Dynamic material volumes | Movable, damageable voxel volumes under the same matter contracts as static geometry. |
| Persist matter + scars | Save authoritative matter changes without treating derived presentation as truth or dumping untouched volume wholesale. |
| Derive presentation | Surfaces and surface dressing from material truth; derived geometry never becomes authority. |
| Register matter-backed objects | Vegetation, micro-objects, and similar assemblies can register as matter-backed rather than fake props disconnected from mutation. |
| Inject consumer content | Seams for consumer-owned generation and content driving without baking a generator into substrate law. |
| Volume-general expression | Contracts remain usable for freeform volumes later; they do not hard-code planetary terrain as the only world. |

---

## Constraints and invariants

These are binding product constraints, not implementation tactics.

### Invariants

- **One authority.** Voxel matter is truth for occupancy, mutation, collision truth, and persistence. Derived presentation is disposable.
- **One public boundary.** Consumers and adjacent harnesses share the same public inspect/mutate/telemetry contract. No privileged access path into internal storage for demos or tools.
- **Async-capable residency without contract break.** GPU work may complete asynchronously; consumers must not depend on direct buffer access or synchronous ownership of internal storage. Moving work between CPU and GPU must not require changing the consumer contract.
- **Everywhere mutation (within the contract).** Material cells the contract exposes are mutable; presentation follows truth after edits.
- **First-class volumetric depth.** Deep volume is real content, not heightmap theater.
- **Volume-general contracts.** Substrate contracts must not assume gravity-aligned planetary terrain as the only legal world shape.
- **Dynamic matter class.** Movable, damageable voxel volumes are a first-class matter class, not a later bolt-on disconnected from world truth.
- **Physics is supportable, not owned.** Bindings and supportable material data are substrate; the simulation engine is adjacent.
- **Generation is above the substrate.** Filling or seeding volumes is consumer-owned algorithm territory.

### Confirmed vision constraints (resolved scope)

- Adjacent consumers, including any walkable validation harness, have no privileged access path into the substrate.
- The walkable-world harness is an adjacent artifact only—not a required product delivery that defines Moria’s completion.
- The product is a Rust / Bevy library ecosystem for crate consumers.
- Everywhere mutation and first-class deep Z are binding product outcomes; natural-looking presentation is world-dependent, not a single mandated overworld aesthetic.
- Substrate contracts are volume-general; ships and stations remain future-consumer examples, not current delivery or validation targets.
- Dynamic voxel volumes are a binding product capability.
- Physics is adjacent, not baked in; bindings and supportable material data are required.
- Procedural / deterministic world generation is not substrate product.
- GPU residency and an async-capable command/query boundary are binding product direction and a deliberate distinction from CPU-driven voxel engines.

---

## Validation principles

How success is proven without redefining the product as a game:

1. **Contracts over scenes.** Quality is measured against public inspect/mutate/streaming/collision-truth/persistence/GPU-residency/physics-binding outcomes—not against a particular character, route, or fiction.
2. **Same interfaces as external games.** Any in-repo harness that validates the substrate must use the public consumer contract. Privileged demo paths do not count as product proof.
3. **Evidence without product redefinition.** Benchmarks and harnesses may capture mutation response, streaming, GPU memory behavior, collision-truth honesty, and physics-binding readiness when exercised. Harness content, controllers, cameras, and acceptance set dressing are evidence tools, not requirements.
4. **Truth honesty checks.** Collision and occupancy must be showable against material authority rather than disposable meshes. Presentation must be showable as rebuilt from truth after mutation.
5. **Feasibility before “finished engine” claims.** The product does not claim a released, finished visual engine before feasibility and visual-acceptance gates are met.
6. **Optional proofs stay optional.** A walkable demo, a curated region, or a simple physics plug-in may prove seams; none of them is mandatory “done” for substrate identity. A proof physics engine does not make owning a physics engine a substrate requirement. Harness generation does not make generation a substrate requirement.

Machine-specific performance tables, demo routes, content palettes, and validation-harness particulars from seed material are **not** product requirements.

---

## Non-goals

- Shipping a game, game mode, progression loop, or game-rules stack in this product.
- Treating validation-harness content, third-person controllers, demo routes, or machine-specific demo targets as substrate requirements or as mandatory delivery for product completion.
- Baking deterministic or procedural world generation into the substrate as product identity.
- Baking in a hand-rolled or third-party physics engine as product identity. Material strength, gravity, force, and related fields must be *supportable* via plug-in bindings; the engine that consumes them is adjacent.
- Implementing excluded layers here: System / LLM, spells, gas / pricing policy, combat rules / AI behavior, agent labor, building UI / blueprints as gameplay, mechanisms as game entities.
- Delivering or specifically validating freeform ships, stations, or other multi-deck freeform hulls as current product work.
- Full fluid simulation and cellular automata (fire, wetness, growth) as current product requirements.
- Tree felling or rigid-body conversion of vegetation as current product requirements.
- Web / wasm as a substrate target platform.
- Claiming a released, finished visual engine before feasibility and visual-acceptance gates are met.
- Limiting substrate identity to a Minecraft-style cube aesthetic, a single natural-overworld content palette, static scenery without movable material volumes, or heightmap terrain that only pretends to have depth.
- Assuming gravity-aligned planetary terrain as the only legal world shape in substrate contracts.

---

## Future consumers (context only)

The following illustrate what the substrate must remain able to support. Their gameplay, characters, assets, content palettes, and presentation are **not** current Moria scope:

- A System-driven ARPG on a continuous natural world.
- A fortress / colony game with engineering and designation play.
- A descent-style roguelike through deep geology.
- Pure sandboxes.
- Games whose players and enemies are voxel volumes that move and take damage under the same matter contracts as terrain—using collision truth and physics engines plugged in through substrate bindings without those engines living inside Moria.
- Freeform material ships and stations (including multi-deck interiors, honest damage, and salvageable geometry) as **nonbinding** motivation for volume-general mutability, deep interiors, GPU-resident matter, physics-ready material bindings, and truth-vs-view—without importing their fiction, UI, mission systems, freeform-hull *delivery*, or a substrate-owned physics stack into current scope.

A “walkable world” proof shape (curated region, forest, ruin, dig-as-demo) is **context for what the substrate might support**, not the definition of the product itself.

---

## Resolved human scope decisions

These decisions are closed. Design and implementation must preserve their meaning.

| Question | Decision |
| --- | --- |
| Is a walkable-world visual validation harness mandatory current delivery or only a permitted adjacent artifact? | **Adjacent artifact.** It may exist for validation; it does not define product identity or “done.” Moria is not limited to a Minecraft-style world. |
| Are natural-looking terrain, everywhere mutation, and first-class deep Z binding current outcomes? | **Everywhere mutation and first-class deep Z are binding.** Deep Z means genuine volumetric depth, not heightmap terrain. Natural-looking presentation depends on the consumer’s world—coherent material presentation for that domain, not a single natural-overworld mandate. |
| Is GPU-resident / asynchronous-GPU-capable architecture binding current direction? | **Yes.** GPU residency is an important product feature and a core distinction from CPU-driven voxel engines. Specific simulations remain milestone-selected. |
| Does current product identity include ship/station material volumes on the same contracts as natural geology? | **Contracts are volume-general** (must not assume gravity-aligned planetary terrain only). **Delivering or specifically validating ships and stations is not current scope**; they remain future-consumer examples. |
| Must the substrate support dynamic (moving, damageable) voxel volumes—not only static world geometry? | **Yes.** Players and enemies will be voxel volumes that move and can take damage; the engine must support that class of matter. |
| Is deterministic / procedural world generation part of the substrate product? | **No.** Generation is an algorithm that runs on top of the substrate and is game-dependent; it must not be baked into substrate identity. |
| Is matter physics (collision for moving entities, gravity, force for explosions/crumbling) in product scope? | **Bindings yes; engine no.** The substrate exposes plug-in bindings and supportable material data so a physics engine can attach. A full or hand-rolled physics engine is **not** baked into the substrate; it is **100% adjacent**. An acceptable proof of the bindings is a simple physics engine, but shipping one is not required. Collision/occupancy **truth** against voxel matter remains a substrate concern so plug-ins and consumers share one material world. |

---

## Open product-boundary questions

**None currently open** that would change product identity, purpose, or boundary.

Engineering and sequencing concerns (which material properties land first, how gravity is parameterized for non-planetary volumes, binding shape for force and strength, storage layout, presentation algorithms, platform engineering limits) remain design and TDD concerns, not vision-identity blockers. This document does not reopen them as product-scope questions.

If later work proposes elevating an excluded capability (fluids CA, tree felling, ship/station delivery, a substrate-owned physics engine, baked-in generation, web/wasm, or a mandatory walkable demo as “done”) into current product law, that requires an explicit human scope decision—not silent expansion from seed language.

---

## Provenance (not a substitute for the synthesis above)

Source order used for this synthesis; substance lives in the sections above.

| Source | Role in synthesis |
| --- | --- |
| `docs/vision.md` | **Authoritative scope boundary.** Current vs adjacent vs future vs excluded; purpose; product ownership; outcomes; non-goals; resolved Q1–Q7; open-question status. |
| `docs/seeds/README.md` | Authority order among seeds; project boundary wins conflicts. |
| `docs/seeds/project-boundary.md` | Binding product target: reusable substrate crate(s); games separate; harness uses public interfaces only; excluded game/System layers. |
| `docs/seeds/gpu-resident-substrate.md` | Supporting principles for GPU-resident sparse matter and command/query boundary—elevated to product direction only where the scope boundary affirms residency and async-capable contracts. Implementation detail left to design. |
| `docs/seeds/voxel-world-substrate.md` | Broad architecture reference: mutable-everywhere, deep Z, substrate-not-game, presentation-as-view, object/clutter hooks, streaming/persistence ideas. Game examples and non-selected extensions treated as context only. |
| `docs/seeds/product-one-seed.md` | Downstream validation example only. Motivates fully material worlds, dig/place proof, collision against truth, measurable quality. Does **not** import character, route, palette, performance tables, or generation pipeline as product law. |
| `docs/seeds/system-substrate-pivot.md` | Excluded-source notice; contributes no product requirements. |

Older planning language that titles the effort as “Product One — The Walkable World” is superseded on identity by the scope boundary and this vision. Seed implementation particulars (storage formats, named graphics backends, atomic widths, platform engineering constraints, algorithms, crate splits, milestone schedules) are not product requirements unless the approved scope boundary explicitly makes an exact choice binding.
