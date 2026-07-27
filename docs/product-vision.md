# Moria product vision

Standalone product vision for the Moria voxel-world substrate. A downstream product designer should be able to design consumer contracts, acceptance criteria, and UX of the public facade from this document alone—without reopening seed material or adjudicating which source is authoritative.

**Authority.** The human-approved project scope boundary (`docs/vision.md` and the binding product target in the project-boundary seed) wins over every seed implication. Material classified as adjacent or future may explain why a capability exists; it is not a current deliverable. Material classified as excluded does not appear as product requirement here.

---

## 1. Product identity

**Moria** is a reusable **voxel-world substrate**: engine-shaped world infrastructure that downstream games and tools install and drive through a public consumer contract.

It is:

- A **Rust and Bevy library** (or small family of tightly scoped crates) for crate consumers—not an ecosystem-neutral engine abstract and not a shipped game.
- A set of **explicit contracts** for continuous three-dimensional **material volumes**—natural landscapes, underground geology, and constructed interiors among them—without limiting identity to a Minecraft-style cube aesthetic or a single overworld content palette.
- **Volume-general:** substrate contracts must not assume gravity-aligned planetary terrain as the only legal world shape. Freeform hulls (ships, stations, multi-deck interiors) remain **future-consumer motivation**, not current delivery or validation targets; contracts must stay able to express them later.

It is **not**:

- A playable game, game mode, progression loop, or game-rules stack.
- A baked-in procedural or deterministic world generator.
- A baked-in physics engine (hand-rolled or third-party).
- Defined by any single validation harness, demo route, character controller, or content postcard.

---

## 2. Purpose

Voxel worlds only work when several hard systems agree as **explicit, shareable contracts**. Moria exists so material worlds can be consumed without each game rebuilding those contracts, and without hiding them behind a single privileged demo.

The product’s infrastructural claim:

A consumer can obtain a continuous three-dimensional material world (including movable, damageable voxel volumes such as players and enemies), inspect and mutate it only through supported interfaces, keep authoritative matter **GPU-resident** for scale and gameplay-enabling work, and trust that what they see and collide with is a view of the **same** authoritative matter—while gravity, force, material strength, and related physical response are **supportable through exposed bindings**, not owned as a substrate physics simulator.

**World generation is not part of the substrate identity.** How a game fills or seeds sparse material volumes is a consumer- or game-dependent algorithm that runs *on top of* the substrate. The substrate provides storage, query, mutation, streaming, collision-truth seams, physics plug-in bindings, persistence seams, presentation derivation, and content-injection seams for material truth—not a particular generator and not a particular physics engine.

---

## 3. Who it serves

### Primary consumers

- **Game and tool authors** who need a material world under a public facade: install the substrate, drive it with their own content pipeline, input, presentation policy, and (optionally) physics plug-in.
- **Adjacent in-repo harnesses** (curation, benchmark, visual validation, optional walkable proof) that exercise the same public interfaces an external game would use. They prove contracts; they do not redefine product identity or “done.”

### Needs the product must meet

| Consumer need | What the substrate must enable |
| --- | --- |
| One material world | Occupancy, queries, collision truth, persistence, and (when present) physics plug-ins all read the same authoritative matter—not a private mesh world or a second invented occupancy model. |
| Safe, portable access | Inspect and mutate only through public reads and admitted edits; no privileged internal paths for harnesses or external crates. |
| Scale without full residency | Large regions stay tractable: untouched homogeneous volume stays cheap; only the interesting shell and active edits pay detailed cost. |
| Honest mutation | Any exposed material cell can be destroyed or placed; cut faces and scars remain real matter; presentation rebuilds from truth. |
| Genuine volume | Depth along the full third axis is real content (caves, strata, ore, aquifers as material bands, buried structure)—not a heightmap floor with painted underground. |
| Movable matter | Dynamic voxel volumes (e.g. combatants as material) can move and take damage under the same truth contracts as static geometry. |
| Physics without ownership | Material strength, gravity parameters, applied force, and related supportable fields are exposable so a hand-rolled or third-party engine can attach without privileged voxel access. |
| Tractable saves | Material edits and related world-state scars persist without dumping every cell of an untouched volume. |
| GPU-scale work | Sparse representation and a command/query boundary keep world matter GPU-resident and support asynchronous GPU work without changing the consumer contract. |
| Measurable quality | Benchmarks and harnesses can evidence mutation response, streaming, GPU memory behavior, collision-truth honesty, and physics-binding readiness when exercised—without redefining the product as a game. |
| Content ownership | Consumers inject or drive world content (including their own generation algorithms) through seams; no particular generator is substrate law. |
| Domain-appropriate look | Fully material volumes can present as coherent for the consumer’s domain (geology, masonry, interiors, etc.); no single overworld aesthetic is mandated. |

---

## 4. Product boundary

### This product owns

The reusable substrate and its public facade:

1. **Sparse storage and lazy materialization** of voxel truth.
2. **GPU-resident sparse representation** and a **command/query boundary** that can support asynchronous GPU work without changing the consumer contract.
3. **Bounded world inspection** and **telemetry** for consumers.
4. **Mutation admission** (dig, place, and related world-edit verbs) so nothing touches voxels outside the contract.
5. **Streaming and lifecycle** so large regions do not require full raw-voxel residency.
6. **Collision and occupancy truth** against voxel matter, not against disposable meshes—queries and contacts read material authority so consumers and plug-ins do not invent a second world.
7. **Physics plug-in bindings:** material properties and world seams a physics engine needs (material strength, gravity parameters, applied force, and related supportable fields), exposed so an external engine can attach without privileged voxel access. The substrate does **not** bake in a physics engine.
8. Support for **dynamic voxel volumes**—matter that moves and can take damage—not only static world geometry.
9. **Persistence** of material truth plus edit deltas (and related world-state scars), without requiring a dump of every cell; how the *base* volume is produced is the consumer’s concern.
10. **Presentation support** that derives surfaces and surface dressing from material truth, without serializing derived geometry as authority.
11. **Object and clutter registration hooks** so vegetation, micro-objects, and other matter-backed assemblies can register without baking into a single terrain slab.
12. **Seams** a consumer can use to inject or drive world content (including generation algorithms), without embedding any particular generator as substrate law.
13. **Volume-general contracts** that do not encode planetary heightmap assumptions or gravity-aligned terrain as the only legal world shape.

### Adjacent (not product identity)

- **Curation, benchmark, and visual-validation executables** and similar harnesses. They may curate parameters, exercise contracts, capture evidence, and visually validate the substrate, but only through the same public interfaces available to an external game. Controllers, characters, cameras, authored demo routes, presentation polish, acceptance scenarios, and **game-specific generation algorithms** belong to those consumers—not to substrate identity.
- A **walkable-world harness** is a *permitted* adjacent artifact, not a mandatory definition of “done.”
- **Any physics engine**—hand-rolled or third-party—that integrates through the substrate’s bindings. An acceptable *proof* of those bindings may be a simple physics engine in a harness or consumer; Moria does not need to ship or own one. Gravity response, contact resolution as a simulation loop, force-driven crumbling dynamics, and similar runtime physics remain plug-in / consumer concerns.

### Downstream (not this product)

Actual games and game layers, including but not limited to: player control, characters, skeletal animation, game-specific presentation, combat rules, AI behavior, economy, building policy, the System / LLM layer, spells, gas pricing, agent labor, building UI / blueprints as gameplay, mechanisms as game entities—and **which** procedural or authored generation pipeline a game uses and **which** physics engine it chooses.

Freeform ship and station games (and their design/combat fiction) are **future-consumer examples**, not current delivery or validation targets. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented as Moria product.

---

## 5. Product-level outcomes

These are outcomes the substrate must enable—not a feature inventory or implementation plan.

### 5.1 Truth vs view

Occupancy, queries, collision truth, and persistence run against voxel matter. Meshes, surface dressing, and debug geometry are **derived and disposable**. Physics engines, when present, also consume material truth through public bindings rather than a private mesh world. Derived render data is never saved as world truth.

### 5.2 Contracted consumption

External consumers install the facade, inspect through public reads (bounded queries, snapshots, telemetry, or events as the contract provides), mutate through admitted edits, and never require privileged internal paths. The same public boundary must serve validation harnesses and external game crates. GPU work may complete asynchronously; consumers must not depend on direct buffer access or synchronous readback as the contract.

### 5.3 Sparse scale

Large regions remain tractable: untouched homogeneous volume stays cheap; only the interesting shell (surfaces, voids, structures, player scars) and active edits pay detailed cost. Streaming lifecycle keeps cold volume off the hot path.

### 5.4 Mutable everywhere

Any material cell the contract exposes can be destroyed or placed. Cut faces and scars remain honest matter. Presentation rebuilds from truth after mutation; visual “looks carved” follows material authority, not a separate art layer.

### 5.5 Deep Z is first-class

Volume along the full depth axis is real content—genuine volumetric depth, not a heightmap floor with painted underground. Caves, strata, ore, aquifers as material bands, and buried structure are material volume. Contracts stay volume-general so non-planetary freeform volumes remain expressible later; ship/station interiors are future-consumer motivation, not a required current deliverable shape.

### 5.6 Dynamic voxel volumes

The world is not static geometry alone. The substrate must support voxel volumes that move and can take damage so future games can treat combatants (and similar) as matter under the same truth contracts rather than as overlays disconnected from the world.

### 5.7 Physics-ready bindings, not a baked-in engine

The substrate exposes bindings and material data a physics engine needs to plug in—material strength, gravity, force, and related supportable fields—whether the consumer hand-rolls an engine or adopts one. Runtime physics simulation is **not** a substrate-owned product. A simple engine is an acceptable proof of the seams, not a required deliverable. Collision/occupancy **truth** against voxel matter remains substrate concern so plug-ins and consumers share one material world.

### 5.8 Cheap scars over full dumps

Persistence keeps material edits and related scars tractable—not a dump of every brick or cell of an untouched volume. A consumer may choose a reproducible generation function as its base-world strategy; that choice is game-dependent and is not a substrate deliverable. Substrate persistence owns the truth of matter plus deltas/scars, not the authorship of the base fill.

### 5.9 GPU-resident architecture

Sparse representation and a command/query boundary keep world representation GPU-resident and support asynchronous GPU work. This is a product distinction from CPU-driven voxel engines: residency enables gameplay-scale mutation, meshing, and future simulation without abandoning the consumer contract. Specific kernels and simulations remain milestone-selected; **residency and the async-capable boundary do not**.

### 5.10 Measurable substrate quality

Benchmarks and harnesses can evidence mutation response, streaming, GPU memory behavior, collision-truth honesty, physics-binding readiness when exercised, and related contracts without redefining the product as a game. Harness-side generation or a proof physics plug-in used for tests or demos does not make generation or a physics engine a substrate requirement.

### 5.11 World-dependent presentation

How a world “looks natural” or coherent depends on the consumer’s world—landscape geology, fortress masonry, constructed interiors, and other material styles. The substrate must support fully material volumes that read as coherent for their domain. It does not mandate a single overworld aesthetic, a heightmap-with-props look, or a cube-only visual identity. Ship bulkheads and similar freeform-hull presentation remain future-consumer context, not current validation targets.

---

## 6. Capabilities (product altitude)

What the product enables at the capability layer—without prescribing algorithms, layouts, or crate splits.

| Capability | Product meaning |
| --- | --- |
| Material truth | Authoritative sparse voxel matter is the source of occupancy, mutation, collision truth, and persistence. |
| Bounded inspection | Consumers read the world through supported, bounded interfaces and telemetry—not unbounded internal dumps. |
| Admitted mutation | Dig, place, and related world-edit verbs are the only path to change voxels; admission is part of the contract. |
| Streaming lifecycle | Regions enter and leave residency according to lifecycle rules so scale does not require full raw-voxel residency. |
| Collision truth | Contacts and occupancy queries are against matter, not against disposable presentation geometry. |
| Physics attachment | Supportable material and world fields (strength, gravity, force, and related) are available so a physics engine can plug in. |
| Dynamic volumes | Movable, damageable material volumes participate under the same matter contracts as static geometry. |
| Persistence of scars | Edits and related world-state scars persist tractably; base volume authorship stays consumer-side. |
| Derived presentation | Surfaces and surface dressing derive from material truth and remain non-authoritative. |
| Registered assemblies | Objects and clutter (vegetation, micro-objects, matter-backed assemblies) can register without collapsing into a single terrain slab. |
| Content injection | Consumers drive or inject world content—including their own generators—through public seams. |
| Volume generality | Contracts express continuous 3D material volume without requiring gravity-aligned planetary terrain as the only shape. |
| GPU residency | Authoritative sparse matter is GPU-resident under a command/query boundary that tolerates asynchronous completion. |

---

## 7. Invariants and guarantees

These are product-level invariants consumers and validators may rely on.

1. **Single authority.** Voxel material state and public substrate operations are authoritative; derived geometry is a view.
2. **No privileged consumers.** Adjacent harnesses and external games share one public contract; validation does not invent a back door.
3. **Mutation only by admission.** Nothing mutates voxels outside admitted edit paths.
4. **Collision honesty.** Collision and occupancy truth track material authority, not mesh presentation.
5. **Presentation non-authority.** Derived meshes, dressing, and debug geometry are never serialized as world truth.
6. **Async-safe contract.** Asynchronous GPU completion does not force consumers into privileged buffer access or a broken public contract.
7. **Everywhere mutability (where exposed).** Cells exposed by the contract are placeable and destroyable; scars remain material.
8. **Volumetric depth.** Deep volume is first-class content, not decorative underpainting of a heightmap.
9. **Dynamic matter class.** Movable, damageable volumes are a supported class of matter, not an unsupported overlay.
10. **Physics adjacency.** Physics simulation ownership stays outside the substrate; bindings and collision truth keep one shared material world.
11. **Generation adjacency.** World-fill algorithms are consumer-owned; substrate seams enable them without baking them in.
12. **Volume-general shape law.** Contracts must not encode “planetary heightfield only” as the legal world model.
13. **Measurability.** Quality claims about mutation, streaming, residency, collision truth, and binding readiness are exercisable without becoming a game product.

---

## 8. Constraints

### Platform and ecosystem

- Target consumers are **Rust / Bevy crate consumers**. Web / wasm is **not** a Product One or substrate target platform.
- GPU-visible counters, allocation indices, and related portable GPU constraints remain within a **32-bit atomic** discipline where that is a product-portability requirement (no assumption of 64-bit buffer atomics).
- Portability of coordinates and GPU-visible indices across Metal, Vulkan, and Direct3D-class backends is a product intent of the residency architecture; fork-to-one-API is not the product shape.

### Feasibility and release posture

- Do not claim a released, finished visual engine before feasibility and visual-acceptance gates are met.
- Engineering may use adjacent harnesses for evidence; harness content, machine-specific demo targets, and demo performance tables are **not** substrate product requirements.

### Scope discipline

- When historical or seed language conflicts with this vision, **substrate product first**; game examples and walkable-demo detail are nonbinding unless selected by the approved scope boundary or an explicit human decision.
- Specific simulation kernels, material-property rollout order, and binding shapes for force/strength/gravity parameterization (including non-planetary volumes) are **design / TDD concerns**, not invitations to expand product identity here.

---

## 9. Validation principles

Validation proves substrate contracts. It does not redefine the product as a game.

1. **Same door for everyone.** Harnesses and external consumers use the same public inspect/mutate/telemetry surface.
2. **Prove truth, not costume.** Dig/place and related mutations must show that presentation and collision follow material authority—not that a particular postcard forest or character controller exists.
3. **Prove scale and residency.** Streaming, sparse cost of homogeneous volume, and GPU-resident behavior are legitimate evidence surfaces.
4. **Prove depth and mutability.** First-class volumetric depth and everywhere mutation (on exposed cells) are binding outcomes to evidence.
5. **Prove binding readiness when exercised.** Physics-binding readiness may be shown with an optional simple proof engine; shipping that engine is not required.
6. **Do not import harness particulars as law.** Controllers, cameras, curated routes, content palettes, acceptance scenery, generation pipelines used only to exercise contracts, and machine-specific performance numbers belong to harnesses or engineering acceptance—not to substrate identity.
7. **Walkable harness is optional adjacency.** A walkable-world executable may exist; its existence is not mandatory product completion.

---

## 10. Non-goals

The following are **out of product scope** for current Moria (unless a later explicit human decision reclassifies them):

- Shipping a game, game mode, progression loop, or game-rules stack in this product.
- Treating validation-harness content, third-person controllers, demo routes, or machine-specific demo targets as substrate requirements or as mandatory delivery for product completion.
- **Baking deterministic or procedural world generation** into the substrate as product identity. Generation algorithms are game-dependent and run on top of the substrate.
- **Baking in a hand-rolled or third-party physics engine** as product identity. Material strength, gravity, force, and related fields must be *supportable* via plug-in bindings; the engine that consumes them is adjacent.
- Implementing excluded layers here: System / LLM, spells, gas / pricing policy, combat rules / AI behavior, agent labor, building UI / blueprints as gameplay, mechanisms as game entities.
- Delivering or specifically validating freeform ships, stations, or other multi-deck freeform hulls as current product work.
- Full fluid simulation and cellular automata (fire, wetness, growth) as current product requirements—may appear later as consumer concepts or format hooks unless explicitly selected.
- Tree felling or rigid-body conversion of vegetation as current product requirements.
- Web / wasm as a Product One or substrate target platform.
- Claiming a released, finished visual engine before feasibility and visual-acceptance gates are met.
- Limiting substrate identity to a Minecraft-style cube aesthetic, a single natural-overworld content palette, static scenery without movable material volumes, or heightmap terrain that only pretends to have depth.
- Assuming gravity-aligned planetary terrain as the only legal world shape in substrate contracts.
- Scripting runtimes, LLM-authored kernels, multiplayer product delivery, navigation/AI layers, weather/season simulation, structural-collapse simulation as substrate-owned physics, and related game systems sketched in broad architecture references—unless later selected explicitly.

---

## 11. Future consumers (context only)

Reference shapes that motivate reusable material-world capabilities. Their gameplay, characters, assets, content palettes, and presentation are **not** current Moria scope. They illustrate what the substrate must remain able to support:

- A System-driven ARPG on a continuous natural world.
- A fortress / colony game with engineering and designation play.
- A descent-style roguelike through deep geology.
- Pure sandboxes.
- Games whose players and enemies are voxel volumes that move and take damage under the same matter contracts as terrain—using collision truth and physics engines plugged in through substrate bindings—without those engines living inside Moria.
- Freeform material hulls (e.g. ships and stations) where damage, salvage, and multi-deck interiors stay honest matter—motivating everywhere mutation, volumetric depth, GPU-resident matter at combat/design scale, physics-ready bindings, and truth-vs-view—**without** importing that fiction, UI, mission systems, freeform-hull *delivery*, or a substrate-owned physics stack into current scope.

A “walkable world” proof shape (curated region, dig-as-demo, third-person traversal) may be used by a validation consumer to make substrate claims undeniable. That shape’s content, controls, milestones, performance tables, and curated generation pipeline remain **context for what the substrate might support**, not the definition of the product itself.

---

## 12. Resolved human scope decisions

These decisions are binding product identity; synthesis must not reopen them.

| Topic | Decision |
| --- | --- |
| Walkable-world visual validation harness | **Adjacent artifact only.** May exist for validation; does not define product identity or “done.” Moria is not limited to a Minecraft-style world. |
| Natural-looking terrain, everywhere mutation, first-class deep Z | **Everywhere mutation and first-class deep Z are binding.** Deep Z means genuine volumetric depth, not heightmap terrain. Natural-looking presentation **depends on the consumer’s world**—coherent material presentation for that domain, not a single natural-overworld mandate. |
| GPU-resident / asynchronous-GPU-capable architecture | **Binding current direction.** GPU residency enables gameplay-scale capabilities and distinguishes Moria from CPU-driven voxel engines. Specific simulations remain milestone-selected. |
| Multi-world freeform volumes (ships/stations) | **Contracts are volume-general** (must not assume gravity-aligned planetary terrain only). **Delivering or specifically validating ships and stations is not current scope.** |
| Dynamic (moving, damageable) voxel volumes | **Yes.** The substrate must support that class of matter (e.g. players and enemies as material volumes). |
| Deterministic / procedural world generation | **Not substrate product.** Generation runs on top of the substrate and is game-dependent. |
| Matter physics (collision for movers, gravity, force for explosions/crumbling) | **Bindings yes; engine no.** Expose plug-in bindings and supportable material data. Full/hand-rolled physics engines are **100% adjacent**. Collision/occupancy truth against voxel matter remains substrate concern. Simple proof engine optional, not required delivery. |

---

## 13. Open product-boundary questions

**None currently open** that would change product identity, purpose, or boundary.

Engineering and milestone sequencing—which material properties land first, how gravity is parameterized for non-planetary volumes, binding shape for force and strength, exact presentation strategies, and similar—are design/TDD concerns, not vision-identity blockers. This document does not invent answers to those questions.

If a later design choice would reclassify generation, physics ownership, ship/station delivery, harness mandatoriness, or other items above, that requires an **explicit human scope decision**—not silent expansion from seed language.

---

## 14. Provenance (not a substitute for the vision above)

| Source | Role in this synthesis |
| --- | --- |
| `docs/vision.md` | Human-approved scope boundary; authoritative for current / adjacent / future / excluded / unresolved. |
| `docs/seeds/README.md` | Seed authority order and conflict rule (project boundary first). |
| `docs/seeds/project-boundary.md` | Binding product target: reusable substrate; games separate; harnesses public-interface only; excluded game layers. |
| `docs/seeds/gpu-resident-substrate.md` | Supporting principles elevated where scope affirms GPU residency, async-capable boundary, derived-vs-truth, and no privileged access. Optional extensions (CA, collapse, particles, scripting, economy) remain non-goals unless selected. |
| `docs/seeds/voxel-world-substrate.md` | Architecture reference motivating mutability, deep Z, substrate-not-game, presentation-from-truth, streaming/persistence ideas, object/clutter registration, and multi-game reusability—filtered to product outcomes; generation pipelines, integrity simulation, fluids CA, building gameplay, nav/AI, and crate layering diagrams are not imported as product law. |
| `docs/seeds/product-one-seed.md` | Downstream / validation example only. Motivates fully material proof, dig/place as evidence, collision-against-truth, sparse streaming, and measurable quality without importing character, region postcard, content palette, performance tables, milestones, or generation pipeline into product identity. |
| `docs/seeds/system-substrate-pivot.md` | Excluded source notice only; contributes no product requirements. |

When seeds conflict, the approved scope boundary wins. Explicit human decisions in §12 override earlier seed implication.
