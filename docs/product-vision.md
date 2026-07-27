# Moria product vision

This document is the standalone product vision for **Moria**. A downstream
product designer can use it without reopening seed documents or deciding which
source is authoritative. Substance lives here; provenance is recorded only to
show lineage.

**Authority.** Human-approved scope is `docs/vision.md`. That boundary defines
what is current, adjacent, future, excluded, and unresolved. Seed material
contributes purpose, needs, behavior, capabilities, constraints, invariants,
and validation principles only within that boundary. When any seed implication
conflicts with the scope boundary, the scope boundary wins.

---

## 1. Product identity

**Moria is a reusable voxel-world substrate:** engine-shaped world
infrastructure that downstream games and tools install and drive through a
public consumer contract. It is a Rust and Bevy library ecosystem (or a small
family of tightly scoped crates) for crate consumers—not an ecosystem-neutral
engine abstract, and not a playable game product.

It is not limited to a Minecraft-style cube aesthetic or a single overworld
content palette. Material contracts target continuous three-dimensional
volumes—natural landscapes, underground geology, and constructed interiors
among them. Those volumes are not only static scenery.

**Volume-general contracts** are required: substrate contracts must not assume
gravity-aligned planetary terrain as the only legal world shape. Delivering or
specifically validating freeform ship and station hulls is **not** current
scope; those remain future-consumer examples that the contracts should remain
able to support.

**World generation is not part of substrate identity.** How a game fills or
seeds sparse material volumes is a consumer- or game-dependent algorithm that
runs *on top of* the substrate. The substrate provides storage, query,
mutation, streaming, collision-truth seams, physics plug-in bindings,
persistence seams, and presentation derivation for material truth—not a
baked-in procedural generator and not a hand-rolled physics engine.

---

## 2. Purpose

Voxel worlds only work when several hard systems agree as explicit contracts:
sparse material truth, bounded inspection, mutation admission, streaming
lifecycle, collision against matter rather than presentation, persistence of
world matter plus edits, GPU-resident representation of that matter, measurable
presentation derived from truth, and seams so a physics engine can plug in
without privileged access to voxel storage.

Moria exists so material worlds can be consumed without each game rebuilding
those contracts, and without hiding them behind a single privileged demo.

The product’s claim is infrastructural: a consumer can obtain a continuous
three-dimensional material world (including movable, damageable voxel volumes
such as players and enemies), inspect and mutate it only through supported
interfaces, keep authoritative matter GPU-resident for scale and
gameplay-enabling work, and trust that what they see and collide with is a view
of the same authoritative matter—while gravity, force, material strength, and
related physical response are **supportable through exposed bindings**, not
baked into a substrate-owned physics engine.

---

## 3. Consumer needs

### Primary consumers

- **Game and tool authors** who need continuous material volumes with honest
  mutation, inspection, collision truth, streaming, and persistence, without
  rebuilding world infrastructure.
- **Validation and evidence harnesses** (curation, benchmark, visual demo) that
  must exercise the same public interfaces an external game would use—never a
  privileged internal path.

### Needs the product must meet

| Need | What success means for the consumer |
| --- | --- |
| One material world | Occupancy, queries, collision truth, and persistence agree on voxel matter; presentation is a derived view. |
| Contracted access only | Install the facade; inspect through public reads; mutate through admitted edits; never require privileged internal paths. |
| Scale without full residency | Large regions stay tractable; untouched homogeneous volume stays cheap; only the interesting shell and active edits pay detailed cost. |
| Everywhere mutation | Any material cell the contract exposes can be destroyed or placed; cut faces and scars remain honest matter; presentation rebuilds from truth. |
| Genuine volumetric depth | Volume along the full depth axis is real content—not a heightmap floor with painted underground. |
| Movable material volumes | Players, enemies, and similar can be voxel volumes that move and take damage under the same truth contracts as terrain. |
| Physics without owning physics | Material strength, gravity, force, and related fields are supportable so a hand-rolled or third-party physics engine can attach; the substrate does not ship the engine. |
| Tractable scars | Persistence keeps material edits and related scars cheap—not a dump of every brick. |
| GPU-resident scale | Sparse representation and a command/query boundary keep world matter GPU-resident and support asynchronous GPU work without breaking the consumer contract. |
| Evidence of quality | Benchmarks and harnesses can measure mutation response, streaming, GPU memory behavior, collision-truth honesty, and physics-binding readiness without redefining the product as a game. |

### Adjacent consumers (not product identity)

Curation, benchmark, and visual-validation executables may exist to curate
parameters, exercise contracts, capture evidence, and visually validate the
substrate. Controllers, characters, cameras, authored demo routes, presentation
polish, acceptance scenarios, and **game-specific generation algorithms** belong
to those consumers—not to substrate identity. A walkable-world harness is a
**permitted adjacent artifact**, not a mandatory definition of “done.”

**Any physics engine**—hand-rolled or third-party—that integrates through the
substrate’s bindings is **100% adjacent**. An acceptable *proof* of those
bindings may be a simple physics engine in a harness or consumer, but Moria
does not need to ship or own one.

### Downstream (not this product)

Actual games and game layers remain outside the product: player control,
characters, skeletal animation, game-specific presentation, combat rules, AI
behavior, economy, building policy, System / LLM layers, spells, gas pricing,
and other gameplay rules—including **which generation pipeline** a game uses
and **which physics engine** it chooses. Freeform ship and station games remain
**future-consumer examples**, not current delivery or validation targets.

---

## 4. Product behavior and capabilities

These are product-level capabilities the substrate must enable. They are not an
implementation inventory, crate split, algorithm choice, or milestone plan.

### What the product owns

1. **Sparse material truth** — Sparse storage and lazy materialization of voxel
   truth so large continuous volumes remain representable without full
   raw-voxel residency.
2. **GPU-resident representation** — Sparse GPU-resident world representation
   and a command/query boundary that can support asynchronous GPU work without
   changing the consumer contract. This is a deliberate product distinction
   from CPU-driven voxel engines: residency enables gameplay-scale mutation,
   meshing, and future simulation without abandoning contracted consumption.
   Specific kernels and simulations remain milestone-selected; residency and
   the async-capable boundary do not.
3. **Bounded inspection and telemetry** — Consumers inspect the world through
   public reads, bounded queries, snapshots, telemetry, or events—not direct
   buffer access or privileged storage paths.
4. **Mutation admission** — Dig, place, and related world-edit verbs so nothing
   touches voxels outside the contract. Mutations enter as explicit public
   commands.
5. **Streaming and lifecycle** — Large regions do not require full raw-voxel
   residency; streaming and lifecycle keep active and cold regions tractable.
6. **Collision and occupancy truth** — Queries and contacts read material
   authority so consumers and plug-ins do not invent a second world. Collision
   is against voxel matter, not against disposable meshes.
7. **Physics plug-in bindings** — Material properties and world seams a physics
   engine needs—material strength, gravity parameters, applied force, and
   related supportable fields—exposed so a hand-rolled or third-party engine
   can attach without privileged voxel access. Runtime physics simulation is
   **not** a substrate-owned product.
8. **Dynamic voxel volumes** — Support for matter that moves and can take
   damage (players, enemies, and similar), not only static world geometry.
9. **Persistence of material truth plus edit deltas** — Persist world matter
   scars without requiring a dump of every brick. How the *base* volume is
   produced is the consumer’s concern.
10. **Presentation derived from truth** — Surfaces and surface dressing derive
    from material truth; derived geometry is never serialized as authority.
11. **Object and clutter registration hooks** — Vegetation, micro-objects, and
    other matter-backed assemblies can register without baking into a single
    terrain slab. Repeated clutter shares presentation resources rather than
    inventing a disconnected prop layer.
12. **Content injection seams** — Consumers can inject or drive world content
    (including their own generation algorithms) without embedding any
    particular generator as substrate law.
13. **Volume-general contracts** — Contracts do not encode planetary heightmap
    assumptions or gravity-aligned terrain as the only legal world shape.

### Product-level outcomes

1. **Truth vs view.** Occupancy, queries, collision truth, and persistence run
   against voxel matter; meshes, dressing, and debug geometry are derived and
   disposable. Physics engines, when present, consume material truth through
   public bindings rather than a private mesh world.
2. **Contracted consumption.** External consumers and in-repo harnesses share
   the same public facade. No privileged internal path defines “done.”
3. **Sparse scale.** Untouched homogeneous volume stays cheap; only the
   interesting shell and active edits pay detailed cost.
4. **Mutable everywhere.** Any material cell the contract exposes can be
   destroyed or placed; cut faces and scars remain honest matter.
5. **Deep Z is first-class.** Genuine volumetric depth is real content—caves,
   strata, ore, aquifers as material bands, buried structure—not skybox
   scenery. Contracts stay volume-general so non-planetary freeform volumes
   remain expressible; ship/station interiors are future-consumer motivation,
   not a required current deliverable shape.
6. **Dynamic voxel volumes.** Future games can treat combatants as matter under
   the same truth contracts rather than as overlays disconnected from the
   world.
7. **Physics-ready bindings, not a baked-in engine.** Runtime gravity response,
   contact resolution as a simulation loop, force-driven crumbling dynamics,
   and similar remain plug-in / consumer concerns; the substrate makes them
   supportable.
8. **Cheap scars over full dumps.** Edit deltas and related scars stay
   tractable. A consumer may choose a reproducible generation function as its
   base-world strategy; that choice is game-dependent and is not a substrate
   deliverable.
9. **GPU-resident architecture.** Residency and an async-capable command/query
   boundary are binding product direction.
10. **Measurable substrate quality.** Harness-side generation or a proof
    physics plug-in used for tests or demos does not make generation or a
    physics engine a substrate requirement.

### World-dependent presentation

How a world “looks natural” depends on the consumer’s world—landscape geology,
fortress masonry, and other material styles. The substrate must support fully
material volumes that read as coherent for their domain; it does not mandate a
single overworld aesthetic or a heightmap-with-props look. Ship bulkheads and
similar freeform-hull presentation remain future-consumer context, not current
validation targets.

---

## 5. Constraints and invariants

These are product guarantees and hard constraints, not engineering preferences.

| Invariant | Meaning |
| --- | --- |
| **Public contract only** | Consumers—including any walkable validation harness—have no privileged access path into the substrate. |
| **Harness ≠ product** | A walkable-world or similar harness is an adjacent artifact only; it does not define product identity or completion. |
| **Truth over view** | Authoritative matter is voxel state and public substrate operations; rendered meshes and dressing are derived views and are never saved as world truth. |
| **Mutation is admitted** | Nothing mutates voxels outside explicit public commands. |
| **Inspection is bounded** | Consumers do not depend on direct buffer access or synchronous privileged readback of internal storage. |
| **Async-safe contract** | GPU work may complete asynchronously; the consumer contract remains stable when work is not synchronous. |
| **Collision honesty** | Collision and occupancy truth read material authority, not disposable presentation geometry. |
| **Physics is adjacent** | Material strength, gravity, force, and related fields are supportable through bindings; owning or shipping a physics engine is not substrate product. |
| **Generation is above** | Deterministic or procedural world generation is not substrate product identity; it is game-dependent. |
| **Volume-general** | Contracts must not assume gravity-aligned planetary terrain as the only world shape. |
| **Dynamic volumes in scope** | Movable, damageable voxel volumes are a binding capability; the world is not static geometry alone. |
| **Everywhere mutation** | Mutation of exposed material cells is a binding outcome. |
| **First-class deep Z** | Genuine volumetric depth is a binding outcome—not heightmap terrain that only pretends to have depth. |
| **GPU residency binding** | GPU-resident sparse representation and an async-capable command/query boundary are binding product direction. |
| **Rust / Bevy ecosystem** | The product is a Rust / Bevy library ecosystem for crate consumers. |
| **Native desktop targets** | Web / wasm is not a Product One or substrate target platform. |
| **Scope boundary wins** | When seed documents conflict, substrate product first; game examples and validation-demo detail are nonbinding unless selected by the approved scope boundary or an explicit human decision. |

Platform and feasibility notes that constrain product claims without becoming
game requirements: GPU-visible counters and allocation indices remain portable
across Metal, Vulkan, and Direct3D with 32-bit atomics; claiming a released,
finished visual engine before feasibility and visual-acceptance gates are met
is a non-goal.

---

## 6. Validation principles

Validation proves substrate contracts; it does not redefine the product as a
game or as “the walkable demo.”

1. **Same public interfaces.** Any in-repo harness, demo, or benchmark consumes
   the substrate through the same public interfaces available to an external
   game.
2. **Evidence of contracts, not content.** Measurable evidence may cover
   mutation response, streaming, GPU memory behavior, collision-truth honesty,
   physics-binding readiness when exercised, and related contracts.
3. **Harness content is nonbinding.** Controllers, characters, cameras, curated
   routes, content palettes, machine-specific demo performance tables, and
   generation pipelines used to exercise the substrate are **not** substrate
   requirements.
4. **Proof engines are optional.** A simple physics engine used only to prove
   plug-in bindings is acceptable demonstration, not required delivery.
5. **In-repo generation is harness practice.** Generation used only to exercise
   or demonstrate contracts remains a harness concern, not a claim that
   generation is substrate product.
6. **Visual coherence is domain-dependent.** The substrate must support fully
   material volumes that read as coherent for the consumer’s domain; it does
   not mandate a single natural-overworld postcard.
7. **Feasibility before “released engine” claims.** Product quality claims that
   depend on headed acceptance and measured feasibility must not outrun those
   gates.

---

## 7. Non-goals

The following are **out of product scope** unless a later explicit human
decision selects them:

- Shipping a game, game mode, progression loop, or game-rules stack in this
  product.
- Treating validation-harness content, third-person controllers, demo routes, or
  machine-specific demo targets as substrate requirements or as mandatory
  delivery for product completion.
- **Baking deterministic or procedural world generation into the substrate** as
  product identity.
- **Baking in a hand-rolled or third-party physics engine** as product
  identity. Supportable bindings are required; the engine that consumes them is
  adjacent.
- Implementing excluded layers here: System / LLM, spells, gas / pricing
  policy, combat rules / AI behavior, agent labor, building UI / blueprints as
  gameplay, mechanisms as game entities.
- Delivering or specifically validating freeform ships, stations, or other
  multi-deck freeform hulls as current product work.
- Full fluid simulation and cellular automata (fire, wetness, growth) as
  current product requirements.
- Tree felling or rigid-body conversion of vegetation as current product
  requirements.
- Web / wasm as a Product One or substrate target platform.
- Claiming a released, finished visual engine before feasibility and
  visual-acceptance gates are met.
- Limiting substrate identity to a Minecraft-style cube aesthetic, a single
  natural-overworld content palette, static scenery without movable material
  volumes, or heightmap terrain that only pretends to have depth.
- Assuming gravity-aligned planetary terrain as the only legal world shape in
  substrate contracts.

Compatibility seams may be designed where substrate requirements demand them;
excluded layers are not implemented here.

---

## 8. Adjacent and future context (nonbinding)

The following material **motivates** reusable capabilities. It is **not**
current Moria delivery, validation definition, or product identity.

### Adjacent artifacts

- Curation, benchmark, and visual-validation executables that prove contracts
  through public APIs.
- Optional simple physics engines used only as proof of plug-in bindings.
- In-repo or example generators used only to exercise or demonstrate contracts.

### Future consumer illustrations

Reference products that illustrate what the substrate must remain able to
support:

- A System-driven ARPG on a continuous natural world.
- A fortress / colony game with engineering and designation play.
- A descent-style roguelike through deep geology.
- Pure sandboxes.
- Games whose players and enemies are voxel volumes that move and take damage
  under the same matter contracts as terrain—using collision truth and physics
  engines plugged in through substrate bindings—without those engines living
  inside Moria.
- Freeform ship and station games (including multi-deck material volumes,
  damageable systems as real geometry, and salvageable wrecks) as **explicitly
  nonbinding** motivation for volume-general mutability, first-class volumetric
  depth, GPU-resident matter at combat/design scale, physics-ready material
  bindings under force, and truth-vs-view so damage and salvage stay honest—
  without importing fiction, UI, mission systems, freeform-hull *delivery*, or
  a substrate-owned physics stack into current scope.

A “walkable world” third-person proof shape (curated region, forest, ruin,
dig-as-demo) describes something a validation consumer *might* use to make
substrate claims undeniable. Its content, controls, milestones, performance
tables, and curated generation pipeline remain **context**, not the definition
of the product.

### Future extension concepts (not current requirements)

Unless later selected explicitly: full fluid CA, fire/wetness/growth cellular
automata, granular settling simulation, structural integrity / cave-in
simulation, tree felling and rigid conversion of vegetation, weather/season
simulation as substrate law, building UI and blueprint gameplay, multiplayer
product delivery, scripting runtimes, and LLM-authored kernels.

---

## 9. Resolved product decisions

These human decisions are binding for product identity. They are not open for
reinterpretation by seed implication.

| Topic | Decision |
| --- | --- |
| Walkable-world visual validation harness | **Adjacent artifact.** It may exist for validation; it does not define product identity or “done.” Moria is not limited to a Minecraft-style world. |
| Natural-looking terrain, everywhere mutation, deep Z | **Everywhere mutation and first-class deep Z are binding.** Deep Z means genuine volumetric depth, not heightmap terrain. Natural-looking presentation **depends on the consumer’s world**—coherent material presentation for that domain, not a single natural-overworld mandate. |
| GPU-resident / async-GPU-capable architecture | **Yes—binding current direction.** GPU residency enables gameplay-scale capabilities and distinguishes Moria from CPU-driven voxel engines. Specific simulations remain milestone-selected. |
| Ship / station material volumes | **Contracts are volume-general** (must not assume gravity-aligned planetary terrain only). **Delivering or specifically validating ships and stations is not current scope**; they remain future-consumer examples. |
| Dynamic (moving, damageable) voxel volumes | **Yes.** The substrate must support that class of matter—not only static world geometry. |
| Deterministic / procedural world generation | **No** as substrate product. Generation is an algorithm that runs on top of the substrate and is game-dependent. |
| Matter physics | **Bindings yes; engine no.** The substrate exposes plug-in bindings and supportable material data (strength, gravity, force, etc.). A full or hand-rolled physics engine is **not** baked into the substrate; it is **100% adjacent**. Collision/occupancy **truth** against voxel matter remains a substrate concern so plug-ins and consumers share one material world. A simple proof engine is acceptable demonstration, not required delivery. |

---

## 10. Open product-boundary questions

**None currently open** that would change product identity, purpose, or
boundary.

Engineering and milestone sequencing—which material properties land first, how
gravity is parameterized for non-planetary volumes, binding shape for force and
strength, voxel size, LOD strategy, and similar—remain design and TDD concerns,
not vision-identity blockers. This document deliberately does not settle those
technical-design choices.

If a later human decision reclassifies generation, physics ownership,
ship/station delivery, harness mandatory status, or any excluded layer, revise
this vision and the approved scope boundary together. Do not expand current
scope from seed implication alone.

---

## 11. Provenance

| Source | Contribution synthesized here |
| --- | --- |
| **`docs/vision.md`** | Authoritative scope boundary: current vs adjacent vs future vs excluded; resolved Q1–Q7; product outcomes; non-goals; confirmed constraints. All resolved human scope decisions are preserved in meaning. |
| **`docs/seeds/README.md`** | Authority order among seeds; exclusion of original System pivot product scope; GPU-resident note as supporting architecture only. |
| **`docs/seeds/project-boundary.md`** | Binding product target: reusable substrate as Rust crate family; games separate; walkable executable is public-interface validation only; System / LLM / spell / gas / combat / AI / building layers out of scope. |
| **`docs/seeds/gpu-resident-substrate.md`** | Substrate-only principles: sparse homogeneous-friendly representation; command/query/event boundary; derived render data never authoritative; async-capable consumer contract; portable 32-bit GPU-visible counters. Optional CA/particles/scripting remain non-current. |
| **`docs/seeds/voxel-world-substrate.md`** | Architecture reference informing outcomes: coherent material presentation for domain; mutable everywhere; deep Z; substrate not game; GPU-resident direction. Generation detail treated as consumer/harness reference, not substrate mandate. Integrity, fluids beyond static bodies, building verbs, nav, multiplayer readiness remain context or future unless selected. Game examples motivate reusability only. |
| **`docs/seeds/product-one-seed.md`** | Downstream validation example motivating dig/place proof of material truth, sparse streaming, seed+delta persistence pattern, collision against truth, and measurable quality—without importing character, route, palette, performance tables, milestones, or generation pipeline as product law. |
| **`docs/seeds/system-substrate-pivot.md`** | Excluded-source notice only; contributes no product requirements. |

Authority among seeds: project boundary first; GPU-resident architecture note
second as supporting principles (residency and async boundary elevated to
product direction by human decision); broad voxel reference third; Product One
seed last as validation example. Explicit human decisions override earlier
wording and seed implication when they reclassify substrate vs consumer
ownership.
