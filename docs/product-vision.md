# Moria product vision

This document is the standalone product vision for **Moria**. It states what the product is, who it serves, what it enables and guarantees, and what it deliberately does not do. A product designer or downstream planner should be able to work from this document alone without reopening seed material or choosing among competing sources.

Scope boundary authority: human-approved product identity, current vs adjacent vs future vs excluded, and all resolved scope decisions are preserved here. Material classified as adjacent or future may explain capability motivation; it is not a current deliverable. Material classified as excluded does not appear as product requirement.

---

## 1. Product identity

**Moria is a reusable voxel-world substrate**—engine-shaped world infrastructure that downstream games and tools install and drive through a public consumer contract. It is **not** a playable game product, a game mode, or a finished visual game engine released as entertainment.

It is delivered as a **Rust and Bevy library** (or small family of crates) for crate consumers. It is not an ecosystem-neutral engine abstract that pretends platform or stack neutrality beyond that identity.

The substrate is **not limited** to a Minecraft-style cube aesthetic, a single natural-overworld content palette, or static scenery. Its material contracts target **continuous three-dimensional volumes**—natural landscapes, underground geology, and constructed interiors among them—and those volumes are not only static scenery.

**Substrate contracts are volume-general.** They must not assume gravity-aligned planetary terrain as the only legal kind of world. Delivering or specifically validating freeform ship and station hulls is **not** current scope; those remain future-consumer examples that the contracts should remain able to support.

**World generation is not part of the substrate identity.** How a game fills or seeds sparse material volumes is a consumer- or game-dependent algorithm that runs *on top of* the substrate. The substrate provides storage, query, mutation, streaming, collision-truth seams, physics plug-in bindings, persistence seams, and presentation derivation for material truth—not a baked-in procedural generator and not a hand-rolled physics engine.

---

## 2. Purpose

Voxel worlds only work when several hard systems agree as **explicit contracts**:

- sparse material truth  
- bounded inspection  
- mutation admission  
- streaming lifecycle  
- collision against matter rather than presentation  
- persistence of world matter plus edits  
- GPU-resident representation of that matter  
- measurable presentation derived from truth  
- **seams so a physics engine can plug in** without privileged access to voxel storage  

Moria exists so material worlds can be consumed **without each game rebuilding those contracts**, and without hiding them behind a single privileged demo.

The product’s claim is infrastructural: a consumer can obtain a continuous three-dimensional material world (including movable, damageable voxel volumes such as players and enemies), inspect and mutate it only through supported interfaces, keep authoritative matter GPU-resident for scale and gameplay-enabling work, and trust that what they see and collide with is a view of the same authoritative matter—while gravity, force, material strength, and related physical response are **supportable through exposed bindings**, not baked into a substrate-owned physics engine.

---

## 3. Who the product serves

| Consumer | Relationship to Moria |
| --- | --- |
| **External game and tool crates** | Primary customers. They install the public facade, inspect through public reads, mutate through admitted edits, and never require privileged internal paths. |
| **In-repo validation consumers** | Curation, benchmark, and visual-validation executables may exist to exercise contracts and capture evidence. They are **adjacent**, not product identity. They must use the **same public interfaces** available to an external game. |
| **Physics engines (hand-rolled or third-party)** | **100% adjacent.** They integrate through substrate bindings. Moria does not need to ship or own one. |
| **Downstream games and game layers** | **Not this product.** Player control, characters, combat, AI, economy, System / LLM, spells, generation pipelines, and choice of physics engine live outside the substrate. |

Adjacent harnesses (including any walkable-world validation executable) have **no privileged access path** into the substrate. A walkable-world harness is a **permitted adjacent artifact**, not a mandatory definition of “done” for Moria.

---

## 4. What the product owns

The product owns the reusable substrate and its public facade. Consumers mutate and inspect the world only through that contract.

### 4.1 Material truth and scale

- **Sparse storage and lazy materialization** of voxel truth so large regions stay tractable: untouched homogeneous volume stays cheap; only the interesting shell and active edits pay detailed cost.
- **Streaming and lifecycle** so large regions do not require full raw-voxel residency in memory.
- **Volume-general contracts** that do not encode planetary heightmap assumptions or gravity-aligned terrain as the only legal world shape.

### 4.2 Consumer contract

- **Bounded world inspection and telemetry** for consumers.
- **Mutation admission** (dig, place, and related world-edit verbs) so nothing touches voxels outside the contract.
- **Seams to inject or drive world content** (including generation algorithms) without embedding any particular generator as substrate law.
- The same public boundary serves validation harnesses and external game crates.

### 4.3 Collision truth and physics readiness

- **Collision and occupancy truth against voxel matter**, not against disposable meshes. Queries and contacts read material authority so consumers and plug-ins do not invent a second world.
- **Physics plug-in bindings:** material properties and world seams a physics engine needs—material strength, gravity parameters, applied force, and related supportable fields—exposed so a hand-rolled or third-party engine can attach without privileged voxel access. The substrate does **not** bake in a physics engine.
- Gravity response, contact resolution as a simulation loop, force-driven crumbling dynamics, and similar runtime physics remain **plug-in / consumer concerns**; the substrate makes them supportable, not mandatory substrate deliverables.

### 4.4 Dynamic material volumes

- Support for **dynamic voxel volumes**—matter that moves and can take damage (players, enemies, and similar)—not only static world geometry.
- Future games can treat combatants as matter under the same truth contracts rather than as overlays disconnected from the world.

### 4.5 Persistence

- **Persistence of material truth plus edit deltas** (and related world-state scars), without requiring a dump of every brick.
- How the *base* volume is produced is the consumer’s concern. A consumer may choose a reproducible generation function as its base-world strategy; that choice is game-dependent and is not a substrate deliverable.
- **Cheap scars over full dumps:** persistence keeps material edits and related scars tractable.

### 4.6 Presentation derived from truth

- **Presentation support** that derives surfaces and surface dressing from material truth, without serializing derived geometry as authority.
- Meshes, dressing, and debug geometry are **derived and disposable**; occupancy, queries, collision truth, and persistence run against voxel matter.
- **Object and clutter registration hooks** so vegetation, micro-objects, and other matter-backed assemblies can register without baking into a single terrain slab.
- **World-dependent presentation:** how a world “looks natural” depends on the consumer’s world—landscape geology, fortress masonry, and other material styles. The substrate must support fully material volumes that read as coherent for their domain; it does not mandate a single overworld aesthetic or a heightmap-with-props look.

### 4.7 GPU-resident architecture

- **GPU-resident sparse representation** of authoritative matter and a **command/query boundary** that can support asynchronous GPU work without changing the consumer contract.
- This is a deliberate product distinction from CPU-driven voxel engines: residency enables gameplay-scale mutation, meshing, and future simulation without abandoning the consumer contract.
- Specific kernels and simulations remain milestone-selected; **residency and the async-capable boundary do not**.
- Consumers must not depend on direct privileged access to internal storage or on synchronous ownership of GPU work completion; inspection uses bounded public queries, snapshots, telemetry, or events.

---

## 5. Product-level outcomes

These are outcomes the substrate must **enable and prove**, not a feature inventory or implementation plan:

1. **Truth vs view.** Occupancy, queries, collision truth, and persistence run against voxel matter; meshes, dressing, and debug geometry are derived and disposable. Physics engines, when present, also consume material truth through public bindings rather than a private mesh world.

2. **Contracted consumption.** External consumers install the facade, inspect through public reads, mutate through admitted edits, and never require privileged internal paths.

3. **Sparse scale.** Large regions remain tractable: untouched homogeneous volume stays cheap; only the interesting shell and active edits pay detailed cost.

4. **Mutable everywhere.** Any material cell the contract exposes can be destroyed or placed; cut faces and scars remain honest matter, and presentation rebuilds from truth.

5. **Deep Z is first-class.** Volume along the full depth axis is real content—genuine volumetric depth, not a heightmap floor with painted underground. Caves, strata, ore, aquifers as material bands, and buried structure are material volume, not skybox scenery. Contracts stay volume-general so non-planetary freeform volumes remain expressible; ship/station interiors are future-consumer motivation, not a required current deliverable shape.

6. **Dynamic voxel volumes.** The world is not static geometry alone. The substrate must support voxel volumes that move and can take damage so future games can treat combatants as matter under the same truth contracts.

7. **Physics-ready bindings, not a baked-in engine.** The substrate exposes bindings and material data a physics engine needs to plug in—material strength, gravity, force, and related supportable fields—whether the consumer hand-rolls an engine or adopts one. Runtime physics simulation is not a substrate-owned product; a simple engine is an acceptable proof of the seams, not a required deliverable.

8. **Cheap scars over full dumps.** Persistence keeps material edits and related scars tractable—not a dump of every brick.

9. **GPU-resident architecture.** Sparse storage and a command/query boundary keep world representation GPU-resident and support asynchronous GPU work as a product distinction from CPU-driven voxel engines.

10. **Measurable substrate quality.** Benchmarks and harnesses can evidence mutation response, streaming, GPU memory behavior, collision-truth honesty, physics-binding readiness when exercised, and related contracts without redefining the product as a game. Harness-side generation or a proof physics plug-in used for tests or demos does not make generation or a physics engine a substrate requirement.

---

## 6. Capabilities consumers can rely on

At product altitude, a consumer of Moria can expect:

| Capability | Guarantee |
| --- | --- |
| **Install and drive** | Install the public facade and drive the world without privileged substrate internals. |
| **Inspect** | Bounded reads, snapshots, and telemetry of material truth. |
| **Mutate** | Admitted edit verbs (dig, place, and related world edits); no out-of-band voxel writes. |
| **Scale sparsely** | Large continuous volumes without full raw-voxel residency. |
| **Stream** | Lifecycle that keeps active regions available and cold regions cheap. |
| **Collide honestly** | Collision and occupancy against matter truth, not presentation meshes. |
| **Persist cheaply** | Material truth + edit scars without full-volume dumps. |
| **See derived presentation** | Surfaces and dressing derived from truth; derived geometry never becomes authority. |
| **Register matter-backed objects** | Hooks for vegetation, micro-objects, and similar assemblies. |
| **Inject content** | Seams to drive or inject world content (including consumer-owned generation). |
| **Attach physics** | Bindings and supportable material fields so a physics engine can plug in. |
| **Host dynamic volumes** | Movable, damageable voxel volumes under the same truth contracts as the world. |
| **Stay volume-general** | Contracts that do not force planetary heightmap-only world shape. |
| **Benefit from GPU residency** | Authoritative matter kept GPU-resident with an async-capable command/query boundary. |

---

## 7. Constraints and invariants

These are binding product constraints, not engineering preferences:

1. **Substrate, not game.** Product completion is not defined by shipping a playable game, character controller, combat loop, or curated demo route.

2. **Public-contract only.** Adjacent consumers—including any walkable validation harness—have no privileged access path into the substrate.

3. **Truth vs view invariant.** Occupancy, queries, collision truth, and persistence run against voxel matter. Derived meshes, dressing, and debug geometry are never serialized as authority.

4. **Everywhere mutation.** Any material cell the contract exposes can be destroyed or placed; presentation rebuilds from truth.

5. **First-class volumetric depth.** Deep Z is genuine material volume, not heightmap terrain that pretends to have depth.

6. **Volume-general contracts.** Substrate contracts must not assume gravity-aligned planetary terrain as the only legal world shape.

7. **Dynamic volumes in scope.** Movable, damageable voxel volumes are a binding capability class, not a future stretch.

8. **Physics adjacent.** Material strength, gravity, force, and related fields are supportable via plug-in bindings. Owning or shipping a physics engine is not substrate product.

9. **Generation above the substrate.** Procedural or deterministic world generation is game-dependent and must not be baked into substrate identity.

10. **GPU residency is product direction.** Sparse GPU-resident representation and an async-capable command/query boundary are binding product direction, not optional polish.

11. **Stack identity.** The product is a Rust / Bevy library ecosystem for crate consumers.

12. **No web/wasm target.** Web / wasm is not a Product One or substrate target platform.

13. **No premature “finished engine” claim.** The product does not claim a released, finished visual engine before feasibility and visual-acceptance gates are met.

14. **Presentation is domain-coherent, not one aesthetic.** Natural-looking presentation depends on the consumer’s world; the substrate requires coherent material presentation for that domain, not a single natural-overworld mandate.

---

## 8. Validation principles

Validation proves substrate contracts; it does not redefine the product as a game.

- **Same public interfaces.** Validation harnesses, benchmarks, and demos must consume the substrate through the same public contract available to an external game.

- **Evidence without identity shift.** Measurable quality may include mutation response, streaming behavior, GPU memory behavior, collision-truth honesty, physics-binding readiness when exercised, and related contract health. Those metrics do not make harness content, controllers, or demo routes into substrate requirements.

- **Optional proof artifacts.** A simple physics engine used only to prove bindings, or in-repo generation used only to exercise contracts, is acceptable adjacent proof. Neither becomes a required substrate deliverable.

- **Walkable harness is optional adjacent.** A walkable-world visual validation harness may exist; it is not mandatory current delivery and does not define product identity or “done.”

- **Feasibility before release claims.** Substrate quality and visual acceptance are evidenced through harnesses and gates; shipping claims wait on those gates rather than on game-like completeness.

Validation-harness content, third-person controllers, demo routes, machine-specific demo targets, curated regions, example materials palettes, performance tables from example demos, and harness-only generation pipelines are **not** product requirements.

---

## 9. Adjacent (not product identity)

The following may exist in or around the repository and may be essential to engineering confidence, but they are **not** Moria’s product identity:

- Curation, benchmark, and visual-validation executables and similar harnesses.
- Controllers, characters, cameras, authored demo routes, presentation polish, and acceptance scenarios used only to exercise contracts.
- **Any physics engine**—hand-rolled or third-party—including gravity response, contact simulation loops, and force-driven failure dynamics as runtime systems.
- Game-specific generation algorithms used in harnesses or demos.
- A walkable-world proof shape (curated region, forest, ruin, dig-as-demo): permitted adjacent context for what the substrate might support, not the definition of the product.

---

## 10. Downstream and future consumers (context only)

Actual games and game layers are **downstream, not this product**. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

Examples of excluded game layers: player control, characters, skeletal animation, game-specific presentation, combat rules, AI behavior, economy, building policy, the System / LLM layer, spells, gas pricing, agent labor, building UI / blueprints as gameplay, mechanisms as game entities, and **which** procedural or authored generation pipeline or physics engine a game chooses.

**Future-consumer motivation (nonbinding):** possible later products that motivate reusable material-world capabilities—System-driven ARPG on a continuous natural world; fortress / colony engineering play; descent-style geology roguelike; pure sandboxes; games whose players and enemies are voxel volumes under the same matter contracts; freeform ship and station games where hulls are material volumes. Their gameplay, characters, assets, content palettes, fiction, and presentation are **not** current Moria scope. Freeform ships and stations motivate volume-general contracts, everywhere mutation, deep interiors, GPU-resident matter, physics-ready bindings, and truth-vs-view honesty **without** importing hull delivery, mission systems, or a substrate-owned physics stack into current scope.

---

## 11. Non-goals

Explicitly out of product scope:

- Shipping a game, game mode, progression loop, or game-rules stack in this product.
- Treating validation-harness content, third-person controllers, demo routes, or machine-specific demo targets as substrate requirements or as mandatory delivery for product completion.
- **Baking deterministic or procedural world generation into the substrate** as product identity.
- **Baking in a hand-rolled or third-party physics engine** as product identity (bindings yes; engine no).
- Implementing excluded layers here: System / LLM, spells, gas / pricing policy, combat rules / AI behavior, agent labor, building UI / blueprints as gameplay, mechanisms as game entities.
- Delivering or specifically validating freeform ships, stations, or other multi-deck freeform hulls as current product work.
- Full fluid simulation and cellular automata (fire, wetness, growth) as current product requirements—these may appear as future consumer concepts or format hooks unless later selected explicitly.
- Tree felling or rigid-body conversion of vegetation as current product requirements (future consumer concepts unless later selected).
- Web / wasm as a Product One or substrate target platform.
- Claiming a released, finished visual engine before feasibility and visual-acceptance gates are met.
- Limiting the substrate identity to a Minecraft-style cube aesthetic, a single natural-overworld content palette, static scenery without movable material volumes, or heightmap terrain that only pretends to have depth.
- Assuming gravity-aligned planetary terrain as the only legal world shape in substrate contracts.

---

## 12. Resolved scope decisions

These human decisions are binding and are not reopened by seed implication:

| Topic | Decision |
| --- | --- |
| Walkable-world visual validation harness | **Adjacent artifact.** May exist for validation; does not define product identity or “done.” Moria is not limited to a Minecraft-style world. |
| Natural-looking terrain, everywhere mutation, deep Z | **Everywhere mutation and first-class deep Z are binding.** Deep Z means genuine volumetric depth, not heightmap terrain. Natural-looking presentation depends on the consumer’s world—coherent material presentation for that domain, not a single natural-overworld mandate. |
| GPU-resident / asynchronous-GPU-capable architecture | **Yes—binding current direction.** Enables gameplay-scale capabilities and distinguishes Moria from CPU-driven voxel engines. Specific simulations remain milestone-selected. |
| Multi-world freeform volumes (ships/stations) | **Contracts are volume-general** (must not assume gravity-aligned planetary terrain only). **Delivering or specifically validating ships and stations is not current scope**; they remain future-consumer examples. |
| Dynamic (moving, damageable) voxel volumes | **Yes.** Players and enemies will be voxel volumes that move and can take damage; the substrate must support that class of matter. |
| Deterministic / procedural world generation | **No.** Generation runs on top of the substrate and is game-dependent; it must not be baked into substrate identity. |
| Matter physics (collision, gravity, force) | **Bindings yes; engine no.** Substrate exposes plug-in bindings and supportable material data. A full or hand-rolled physics engine is **100% adjacent**. Collision/occupancy **truth** against voxel matter remains a substrate concern. A simple proof engine is optional demonstration, not required delivery. |

---

## 13. Open product-boundary questions

**None currently open** that would change product identity, purpose, or boundary.

Engineering and milestone sequencing—which material properties land first, how gravity is parameterized for non-planetary volumes, exact binding shape for force and strength—remain design and technical concerns, not vision-identity blockers. This vision does not resolve those choices and does not treat them as product-boundary gaps.

If a later decision reclassifies generation, physics ownership, freeform-hull delivery, fluid/CA simulation, or harness mandatoriness, that decision must be an **explicit human scope change**; seed documents alone cannot expand current deliverables.

---

## 14. Provenance (not a substitute for the vision above)

The sections above are the product vision. The following only records how source material contributed; designers should not reopen the seed set to reinterpret requirements.

| Source | Role in this synthesis |
| --- | --- |
| **`docs/vision.md`** | Human-approved scope boundary. Authoritative for current / adjacent / future / excluded, purpose, outcomes, non-goals, resolved decisions, and confirmed constraints. Substance above is carried forward from this boundary. |
| **`docs/seeds/README.md`** | Authority order among seeds: project boundary first; GPU-resident note as supporting principles; broad voxel reference third; Product One seed last as validation example. Conflicts resolve toward the boundary. |
| **`docs/seeds/project-boundary.md`** | Binding product target: reusable substrate via public Rust APIs; games separate; walkable executable is validation harness only; System / LLM / spell / gas / combat / AI / building layers out of scope. |
| **`docs/seeds/gpu-resident-substrate.md`** | Supporting architecture: sparse representation direction; command/query/event boundary; async GPU work without privileged consumer access; derived render data never world truth. Product-level residency and contract boundary retained; brick dimensions, allocator policy, atomic widths, and named backends not elevated to product requirements. Optional CA / integrity / particle extensions remain non-current. |
| **`docs/seeds/voxel-world-substrate.md`** | Architecture reference motivating truth-vs-view, everywhere mutation, deep Z, substrate-not-game, and GPU-resident direction where aligned with the boundary. Generation pipelines, fluid tiers, integrity sims, building verbs, nav, weather, and game examples retained only as context or future-consumer motivation—not current deliverables. |
| **`docs/seeds/product-one-seed.md`** | Downstream / validation example only. Motivates fully material world proof shape, dig/place as honesty proof, sparse streaming, seed+delta save shape, and measurable quality—without importing third-person character, curated region, content palette, performance tables, milestones, or generation pipeline into product identity. |
| **`docs/seeds/system-substrate-pivot.md`** | Excluded-source notice only. Contributes no product requirements. |

**Authority rule:** when seed documents conflict, the project boundary and the human-approved scope decisions win. Supporting material becomes binding only when that boundary or an explicit human decision selects a claim. Older planning titles that frame the effort as “Product One — The Walkable World” are superseded on product identity by this vision.
