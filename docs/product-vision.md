# Moria product vision

Standalone product vision for the Moria voxel-world substrate. A downstream
product designer can design consumer contracts, acceptance criteria, and the
public facade from this document alone—without reopening seed material or
deciding which source is authoritative.

**Authority.** The human-approved project scope boundary is authoritative for
what is current, adjacent, future, excluded, and unresolved. Every claim below
is within that boundary. Adjacent or future material may explain why a
capability exists; it is not a current deliverable. Excluded material does not
appear as product requirement.

---

## 1. Product identity

**Moria** is a reusable **voxel-world substrate**: engine-shaped world
infrastructure that downstream games and tools install and drive through a
public consumer contract.

It is:

- A **Rust and Bevy library** (or small family of tightly scoped crates) for
  crate consumers—not an ecosystem-neutral engine abstract and not a shipped
  game.
- Infrastructure for continuous three-dimensional **material volumes**—natural
  landscapes, underground geology, and constructed interiors among them—without
  limiting identity to a Minecraft-style cube aesthetic or a single overworld
  content palette.
- **Volume-general:** substrate contracts must not assume gravity-aligned
  planetary terrain as the only legal world shape. Freeform hulls (ships,
  stations, multi-deck interiors) remain **future-consumer motivation**, not
  current delivery or validation targets; contracts must stay able to express
  them later.

It is **not**:

- A playable game, game mode, progression loop, or game-rules stack.
- A baked-in procedural or deterministic world generator.
- A baked-in physics engine (hand-rolled or third-party).
- Defined by any single validation harness, demo route, character controller,
  content postcard, or machine-specific demo target.

---

## 2. Purpose

Voxel worlds only work when several hard systems agree as **explicit, shareable
contracts**: sparse material truth, bounded inspection, mutation admission,
streaming lifecycle, collision against matter rather than presentation,
persistence of world matter plus edits, GPU-resident representation of that
matter, measurable presentation derived from truth, and seams so a physics
engine can plug in without privileged access to voxel storage.

Moria exists so material worlds can be consumed without each game rebuilding
those contracts, and without hiding them behind a single privileged demo.

**The product’s infrastructural claim:** a consumer can obtain a continuous
three-dimensional material world (including movable, damageable voxel volumes
such as players and enemies), inspect and mutate it only through supported
interfaces, keep authoritative matter **GPU-resident** for scale and
gameplay-enabling work, and trust that what they see and collide with is a view
of the **same** authoritative matter—while gravity, force, material strength,
and related physical response are **supportable through exposed bindings**, not
owned as a substrate physics simulator.

**World generation is not part of the substrate identity.** How a game fills or
seeds sparse material volumes is a consumer- or game-dependent algorithm that
runs *on top of* the substrate. The substrate provides storage, query,
mutation, streaming, collision-truth seams, physics plug-in bindings,
persistence seams, presentation derivation, and content-injection seams for
material truth—not a particular generator and not a particular physics engine.

---

## 3. Who it serves

### Primary consumers

- **Game and tool authors** who need continuous material volumes with honest
  mutation, inspection, collision truth, streaming, and persistence, without
  rebuilding world infrastructure for each title.
- **Adjacent in-repo harnesses** (curation, benchmark, visual validation,
  optional walkable proof) that exercise the same public interfaces an external
  game would use. They prove contracts; they do not redefine product identity
  or “done.”

### Needs the product must meet

| Consumer need | What success means |
| --- | --- |
| One material world | Occupancy, queries, collision truth, persistence, and (when present) physics plug-ins all read the same authoritative matter—not a private mesh world or a second invented occupancy model. |
| Contracted access only | Install the facade; inspect through public reads; mutate through admitted edits; never require privileged internal paths for harnesses or external crates. |
| Scale without full residency | Large regions stay tractable: untouched homogeneous volume stays cheap; only the interesting shell and active edits pay detailed cost. |
| Everywhere mutation | Any material cell the contract exposes can be destroyed or placed; cut faces and scars remain honest matter; presentation rebuilds from truth. |
| Genuine volumetric depth | Volume along the full third axis is real content—caves, strata, ore, aquifers as material bands, buried structure—not a heightmap floor with painted underground. |
| Movable material volumes | Dynamic voxel volumes (e.g. combatants as material) can move and take damage under the same truth contracts as static geometry. |
| Physics without ownership | Material strength, gravity parameters, applied force, and related supportable fields are exposable so a hand-rolled or third-party engine can attach without privileged voxel access. |
| Tractable scars | Material edits and related world-state scars persist without dumping every cell of an untouched volume. |
| GPU-scale work | Sparse representation and a command/query boundary keep world matter GPU-resident and support asynchronous GPU work without changing the consumer contract. |
| Measurable quality | Benchmarks and harnesses can evidence mutation response, streaming, GPU memory behavior, collision-truth honesty, and physics-binding readiness when exercised—without redefining the product as a game. |
| Content ownership | Consumers inject or drive world content (including their own generation algorithms) through seams; no particular generator is substrate law. |
| Domain-appropriate look | Fully material volumes can present as coherent for the consumer’s domain (geology, masonry, interiors, etc.); no single overworld aesthetic is mandated. |

---

## 4. Product boundary

### This product owns

The reusable substrate and its public facade:

1. **Sparse storage and lazy materialization** of voxel truth.
2. **GPU-resident sparse representation** and a **command/query boundary** that
   can support asynchronous GPU work without changing the consumer contract.
3. **Bounded world inspection** and **telemetry** for consumers.
4. **Mutation admission** (dig, place, and related world-edit verbs) so nothing
   touches voxels outside the contract.
5. **Streaming and lifecycle** so large regions do not require full raw-voxel
   residency.
6. **Collision and occupancy truth** against voxel matter, not against
   disposable meshes—queries and contacts read material authority so consumers
   and plug-ins do not invent a second world.
7. **Physics plug-in bindings:** material properties and world seams a physics
   engine needs—material strength, gravity parameters, applied force, and
   related supportable fields—exposed so a hand-rolled or third-party engine
   can attach without privileged voxel access. The substrate does **not** bake
   in a physics engine.
8. Support for **dynamic voxel volumes**—matter that moves and can take damage
   (players, enemies, and similar)—not only static world geometry.
9. **Persistence** of material truth plus edit deltas (and related world-state
   scars), without requiring a dump of every cell; how the *base* volume is
   produced is the consumer’s concern.
10. **Presentation support** that derives surfaces and surface dressing from
    material truth, without serializing derived geometry as authority.
11. **Object and clutter registration hooks** so vegetation, micro-objects, and
    other matter-backed assemblies can register without baking into a single
    terrain slab.
12. **Seams** a consumer can use to inject or drive world content (including
    generation algorithms), without embedding any particular generator as
    substrate law.
13. **Volume-general contracts** that do not encode planetary heightmap
    assumptions or gravity-aligned terrain as the only legal world shape.

### Adjacent (not product identity)

- **Curation, benchmark, and visual-validation executables** and similar
  harnesses. They may curate parameters, exercise contracts, capture evidence,
  and visually validate the substrate, but only through the same public
  interfaces available to an external game. Controllers, characters, cameras,
  authored demo routes, presentation polish, acceptance scenarios, and
  **game-specific generation algorithms** belong to those consumers—not to
  substrate identity.
- A **walkable-world harness** is a *permitted* adjacent artifact, not a
  mandatory definition of “done.” In-repo generation used only to exercise or
  demonstrate contracts remains a harness concern, not a claim that generation
  is substrate product.
- **Any physics engine**—hand-rolled or third-party—that integrates through the
  substrate’s bindings. An acceptable *proof* of those bindings may be a simple
  physics engine in a harness or consumer; Moria does not need to ship or own
  one. Gravity response, contact resolution as a simulation loop, force-driven
  crumbling dynamics, and similar runtime physics remain plug-in / consumer
  concerns; the substrate makes them supportable, not mandatory substrate
  deliverables.

### Downstream (not this product)

Actual games and game layers, including but not limited to: player control,
characters, skeletal animation, game-specific presentation, combat rules, AI
behavior, economy, building policy, the System / LLM layer, spells, gas
pricing, agent labor, building UI / blueprints as gameplay, mechanisms as game
entities—and **which** procedural or authored generation pipeline a game uses
and **which** physics engine it chooses.

Freeform ship and station games (and their design/combat fiction) are
**future-consumer examples**, not current delivery or validation targets.
Compatibility seams may be designed where substrate requirements demand them;
those layers are not implemented as Moria product.

---

## 5. Product-level outcomes

These are outcomes the substrate must enable—not a feature inventory or
implementation plan.

### 5.1 Truth vs view

Occupancy, queries, collision truth, and persistence run against voxel matter.
Meshes, surface dressing, and debug geometry are **derived and disposable**.
Physics engines, when present, also consume material truth through public
bindings rather than a private mesh world. Derived geometry is never
serialized as world authority.

### 5.2 Contracted consumption

External consumers install the facade, inspect through public reads, mutate
through admitted edits, and never require privileged internal paths. Adjacent
harnesses and external game crates share the **same** public boundary. GPU work
may complete asynchronously; consumers must not depend on direct buffer access
or synchronous ownership of internal storage.

### 5.3 Sparse scale

Large regions remain tractable: untouched homogeneous volume stays cheap; only
the interesting shell (surfaces, voids, structures, player scars) and active
edits pay detailed cost. Streaming and lifecycle keep residency bounded without
requiring full raw-voxel presence for an entire region.

### 5.4 Mutable everywhere

Any material cell the contract exposes can be destroyed or placed. Cut faces
and scars remain honest matter; presentation rebuilds from truth. Mutation is a
first-class product proof—not optional scenery decoration sitting outside the
material world.

### 5.5 Deep Z is first-class

Volume along the full depth axis is real content—genuine volumetric depth, not
a heightmap floor with painted underground. Caves, strata, ore, aquifers as
material bands, and buried structure are material volume, not skybox scenery.
Contracts stay volume-general so non-planetary freeform volumes remain
expressible; ship and station interiors are future-consumer motivation, not a
required current deliverable shape.

### 5.6 Dynamic voxel volumes

The world is not static geometry alone. The substrate must support voxel
volumes that move and can take damage (for example, players and enemies as
material volumes), so games can treat combatants as matter under the same truth
contracts rather than as overlays disconnected from the world.

### 5.7 Physics-ready bindings, not a baked-in engine

The substrate exposes bindings and material data a physics engine needs to plug
in—material strength, gravity, force, and related supportable fields—whether
the consumer hand-rolls an engine or adopts one. Runtime physics simulation is
**not** a substrate-owned product; a simple engine is an acceptable proof of the
seams, not a required deliverable. Collision and occupancy **truth** against
voxel matter remains a substrate concern so plug-ins and consumers share one
material world.

### 5.8 Cheap scars over full dumps

Persistence keeps material edits and related scars tractable—not a dump of
every cell. A consumer may choose a reproducible generation function as its
base-world strategy; that choice is game-dependent and is not a substrate
deliverable.

### 5.9 GPU-resident architecture

Sparse representation and a command/query boundary keep world matter
GPU-resident and support asynchronous GPU work. This is a product distinction
from CPU-driven voxel engines: residency enables gameplay-scale mutation,
meshing, and future simulation without abandoning the consumer contract.
Specific kernels and simulations remain design-selected later; residency and
the async-capable boundary do not.

### 5.10 Measurable substrate quality

Benchmarks and harnesses can evidence mutation response, streaming, GPU memory
behavior, collision-truth honesty, physics-binding readiness when exercised,
and related contracts without redefining the product as a game. Harness-side
generation or a proof physics plug-in used for tests or demos does not make
generation or a physics engine a substrate requirement.

### 5.11 World-dependent presentation

How a world “looks natural” depends on the consumer’s world—landscape geology,
fortress masonry, and other material styles. The substrate must support fully
material volumes that read as coherent for their domain; it does not mandate a
single overworld aesthetic or a heightmap-with-props look. Ship bulkheads and
similar freeform-hull presentation remain future-consumer context, not current
validation targets.

---

## 6. Capabilities (product altitude)

What the product enables and guarantees, without choosing algorithms, layouts,
crate splits, or milestones.

| Capability | Guarantee |
| --- | --- |
| Sparse material truth | Continuous volumes are representable without full raw-voxel residency of untouched regions. |
| Lazy materialization | Regions become detailed on demand—not all at once. |
| Public mutation verbs | Dig, place, and related world edits are admitted only through the contract. |
| Public inspection | Bounded reads, queries, snapshots, telemetry, or events—never privileged storage paths. |
| Streaming lifecycle | Active and cold regions remain tractable; large worlds do not require permanent full residency. |
| Collision / occupancy truth | Contacts and occupancy queries read material authority, not disposable meshes. |
| Physics plug-in surface | Strength, gravity, force, and related supportable fields are exposable for external engines. |
| Dynamic material volumes | Movable, damageable voxel volumes share the same truth contracts as static world matter. |
| Persistence of scars | Edit deltas and related world-state scars persist without full-volume dumps. |
| Derived presentation | Surfaces and surface dressing rebuild from truth; derived geometry is not authority. |
| Object / clutter registration | Matter-backed assemblies (vegetation, micro-objects, similar) can register without baking into one terrain slab. |
| Content injection | Consumers drive how base volume is produced; the substrate does not own a generator. |
| Volume-general shape | Contracts do not force planetary heightmap or gravity-aligned terrain as the only legal world. |
| GPU-resident + async boundary | Authoritative matter can live GPU-resident; async GPU work does not break the consumer contract. |

Supporting principles that constrain *how* the product is consumed (not
implementation checklists):

- Consumers must not receive privileged access to internal voxel storage.
- Mutations enter through explicit public commands.
- Inspection uses bounded public interfaces.
- The same public boundary must serve validation harnesses and external game
  crates.
- Implementation ownership of work may move between CPU and GPU without
  changing the consumer contract.
- Vegetation and clutter presentation stays derived from matter—not a
  disconnected prop layer that desyncs from material truth.

---

## 7. Constraints and invariants

### Binding product constraints

- **Substrate first.** Moria is world infrastructure consumed as a Rust / Bevy
  crate ecosystem, not a finished visual game engine claimed before feasibility
  and visual-acceptance gates are met.
- **No privileged harness path.** Adjacent consumers—including any walkable
  validation harness—have no privileged access into the substrate.
- **Walkable harness optional for “done.”** A walkable-world harness may exist
  for validation; it does not define product identity or completion.
- **Everywhere mutation and first-class deep Z** are binding product outcomes.
  Deep Z means genuine volumetric depth, not heightmap terrain.
  Natural-looking presentation depends on the consumer’s world—coherent
  material presentation for that domain, not a single natural-overworld
  mandate.
- **Volume-general contracts** are required even though ship/station *content*
  is not current delivery or validation.
- **Dynamic voxel volumes** are a binding product capability.
- **Physics is adjacent, not baked in.** Bindings and supportable material data
  are required; owning or shipping a physics engine is not.
- **Generation is not substrate product.** Procedural / deterministic world
  generation is game-dependent and lives above the substrate.
- **GPU residency and an async-capable command/query boundary** are binding
  product direction and a deliberate distinction from CPU-driven voxel engines.

### Invariants (must remain true)

1. **Matter is authority; views are disposable.** Presentation, dressing, and
   debug geometry never become truth.
2. **One consumer contract.** Harness and external game use the same public
   facade.
3. **No second world for collision.** Occupancy and collision truth are
   material, so plug-ins and consumers cannot invent a parallel mesh world.
4. **Mutation is universal within the contract.** Decorative-only solid
   geometry that cannot be edited under the same rules is not the product
   model for exposed material.
5. **Depth is volume, not paint.** Underground and freeform interiors are
   expressible as real material volume.
6. **Homogeneous emptiness and solid are cheap.** Scale depends on not paying
   full detail cost for untouched volume.
7. **Scars are first-class persistence.** Edits survive without full dumps.
8. **Residency does not break contracts.** GPU-resident work and async
   completion remain behind the public command/query boundary.
9. **Physics engines are guests.** They attach through bindings; they do not
   own voxel storage.
10. **Content algorithms are guests.** Generators and fill strategies run on
    seams; they are not substrate law.

---

## 8. Validation principles

How the product proves itself—without turning harness content into product
requirements.

- **Contracts over spectacle.** Evidence shows that inspection, mutation,
  streaming, collision truth, persistence, GPU-resident behavior, and (when
  exercised) physics bindings hold—not that a particular forest postcard or
  character controller exists.
- **Same-interface proof.** Harnesses must use the public facade available to
  external games. Privileged internal paths invalidate the proof.
- **Mutation is the honesty test.** Dig and place (or equivalent admitted
  edits) must leave honest cut faces and rematerialized presentation from
  truth; a world that only looks good until edited has failed the material
  claim.
- **Collision against truth.** Movement and contact proofs read voxel matter,
  not disposable meshes.
- **Sparse scale is real.** Regions large enough that full raw-voxel residency
  is unreasonable must remain tractable under streaming and homogeneous cheap
  storage.
- **Deep volume is exercisable.** Depth content must be reachable as material
  volume (for example caves, strata, buried structure), not as skybox or
  painted floors.
- **Measurable substrate quality.** Benchmarks and evidence capture mutation
  response, streaming, GPU memory behavior, collision-truth honesty, and
  physics-binding readiness when exercised—with machine context so results are
  comparable. Specific performance numbers, demo routes, and acceptance scenes
  belong to harness/TDD design, not this vision.
- **Optional physics proof.** A simple physics plug-in may demonstrate
  bindings; shipping or owning that engine is not required for product
  completeness.
- **Optional walkable harness.** A walkable third-person proof may make
  claims undeniable to humans; its controls, characters, assets, curated
  routes, content palettes, and generation pipelines are harness particulars,
  not substrate requirements.
- **No premature “finished engine” claim.** Do not claim a released, finished
  visual engine before feasibility and visual-acceptance gates are met.

---

## 9. Non-goals

Explicitly out of product scope:

- Shipping a game, game mode, progression loop, or game-rules stack.
- Treating validation-harness content, third-person controllers, demo routes,
  content palettes, or machine-specific demo targets as substrate requirements
  or as mandatory delivery for product completion.
- **Baking deterministic or procedural world generation into the substrate** as
  product identity. Generation algorithms are game-dependent and run on top of
  the substrate; consumers own them.
- **Baking in a hand-rolled or third-party physics engine** as product
  identity. Material strength, gravity, force, and related fields must be
  *supportable* via plug-in bindings; the engine that consumes them is
  adjacent. A simple proof engine is optional demonstration, not required
  delivery.
- Implementing excluded layers here: System / LLM, spells, gas / pricing
  policy, combat rules / AI behavior, agent labor, building UI / blueprints as
  gameplay, mechanisms as game entities.
- Delivering or specifically validating freeform ships, stations, or other
  multi-deck freeform hulls as current product work—those remain
  future-consumer examples that motivate volume-general contracts.
- Full fluid simulation and cellular automata (fire, wetness, growth) as
  current product requirements—these may appear as future consumer concepts or
  format hooks unless later selected explicitly.
- Tree felling or rigid-body conversion of vegetation as current product
  requirements (future consumer concepts unless later selected).
- Web / wasm as a Product One or substrate target platform.
- Claiming a released, finished visual engine before feasibility and
  visual-acceptance gates are met.
- Limiting substrate identity to a Minecraft-style cube aesthetic, a single
  natural-overworld content palette, static scenery without movable material
  volumes, or heightmap terrain that only pretends to have depth.
- Assuming gravity-aligned planetary terrain as the only legal world shape in
  substrate contracts.
- Structural integrity / cave-in simulation, span tables, fortress engineering
  UI, navigation graphs, multiplayer services, weather simulation, seasons,
  growth systems, and fine splash / particle matter layers as current substrate
  product (possible future consumer or plug-in concerns unless later selected).

---

## 10. Future consumers (context only)

Reference material describes possible later products that motivate reusable
material-world capabilities. Their gameplay, characters, assets, content
palettes, and presentation are **not** current Moria scope. They illustrate
what the substrate must remain able to support:

- A System-driven ARPG on a continuous natural world.
- A fortress / colony game with engineering and designation play.
- A descent-style roguelike through deep geology.
- Pure sandboxes.
- Games whose players and enemies are voxel volumes that move and take damage
  under the same matter contracts as terrain—using collision truth and physics
  engines plugged in through substrate bindings (gravity, contact, force-driven
  failure) without those engines living inside Moria.
- Freeform ship and station volumes (for example a single-captain trading and
  combat game where the hull is real material space rather than abstract slots).
  That fiction motivates everywhere mutation, first-class volumetric depth
  through multi-deck interiors, GPU-resident matter at combat and design scale,
  physics-ready material bindings under force, and truth-vs-view so damage and
  salvage stay honest—**without** importing its fiction, UI, mission systems,
  freeform-hull *delivery*, or a substrate-owned physics stack into current
  Moria scope.

A “walkable world” proof shape (curated region, forest, ruin, dig-as-demo) may
be used by a validation consumer to make substrate claims undeniable. That
shape’s content, controls, milestones, performance tables, and curated
generation pipeline remain **context for what the substrate might support**,
not the definition of the product itself.

---

## 11. Resolved scope decisions

These human decisions are closed. Do not reopen them in product design without
an explicit new human decision.

| Topic | Decision |
| --- | --- |
| Walkable-world visual validation harness | **Adjacent artifact.** May exist for validation; does not define product identity or “done.” Moria is not limited to a Minecraft-style world. |
| Natural-looking terrain, everywhere mutation, first-class deep Z | **Everywhere mutation and first-class deep Z are binding.** Deep Z means genuine volumetric depth, not heightmap terrain. Natural-looking presentation **depends on the consumer’s world**—coherent material presentation for that domain, not a single natural-overworld mandate. |
| GPU-resident / asynchronous-GPU-capable architecture | **Yes, binding.** GPU residency is an important product feature: it enables many gameplay capabilities and is a core distinction between Moria and CPU-driven voxel engines. Specific simulations remain design-selected later. |
| Multi-world freeform volumes (ships / stations) | **Contracts are volume-general** (must not assume gravity-aligned planetary terrain only). **Delivering or specifically validating ships and stations is not current scope**; they remain future-consumer examples. |
| Dynamic (moving, damageable) voxel volumes | **Yes.** Players and enemies will be voxel volumes that move and can take damage; the substrate must support that class of matter. |
| Deterministic / procedural world generation | **No.** Generation is an algorithm that runs on top of the substrate and is game-dependent; it must not be baked into substrate identity. |
| Matter physics (collision for moving entities, gravity, force) | **Bindings yes; engine no.** The substrate exposes plug-in bindings and supportable material data (strength, gravity, force, etc.) so a physics engine can attach—hand-rolled or third-party. A full or hand-rolled physics engine is **not** baked into the substrate; it is **100% adjacent**. An acceptable proof of the bindings is a simple physics engine, but shipping one is not required. Collision/occupancy **truth** against voxel matter remains a substrate concern so plug-ins and consumers share one material world. |

Older planning language that still titles the effort as “Product One — The
Walkable World” is superseded on identity by this product vision. Engineering
docs that list deterministic generation under the public facade describe
implementation or harness practice; generation is **not** substrate product
identity. Seed language that listed structural integrity, cave-ins, or rigid
conversion as non-current remains context for future consumers; this vision
requires **physics-ready bindings** without importing a substrate-owned physics
engine, fortress span tables, or fortress UI.

---

## 12. Open product-boundary questions

None currently open that would change product identity, purpose, or boundary.

Engineering and milestone sequencing—which material properties land first, how
gravity is parameterized for non-planetary volumes, binding shape for force and
strength, presentation technique choices, storage layouts, and platform
engineering constraints—remain design and technical concerns, not
vision-identity blockers. This document intentionally does not decide them.

If a later design choice would reclassify substrate vs consumer ownership
(for example promoting full fluid CA, tree felling, structural collapse
simulation, freeform-hull delivery, or a baked-in physics engine into substrate
product), that requires an explicit human scope decision—not silent expansion
from seed implication.

---

## 13. Provenance (not a substitute for the vision above)

Source materials informed this synthesis. They are not required reading for a
designer once this document is approved. Authority among seeds: project
boundary first; GPU-resident architecture note as supporting principles
(residency and async boundary elevated to product direction by human decision);
broad voxel reference third; Product One seed last as validation example.
Conflicts resolve toward the boundary without silent expansion of scope.
Explicit human decisions override earlier vision wording and seed implication
when they reclassify substrate vs consumer ownership.

| Source | Contribution |
| --- | --- |
| Human-approved scope boundary (`docs/vision.md`) | Binding product identity, purpose, boundary, outcomes, non-goals, resolved Q1–Q7, and validation posture. |
| Project boundary seed | Reusable substrate as product; games separate; harnesses public-interface only; System / LLM / spell / gas / combat / AI / building layers out of scope. |
| GPU-resident substrate seed | Supporting principles: sparse residency direction, command/query/event boundary, no privileged storage access, derived meshes never truth; optional CA/integrity/particle extensions not current requirements. |
| Voxel-world substrate seed | Architecture reference motivating coherent domain presentation, everywhere mutation, deep Z, substrate-not-game layering, and GPU-resident direction—without importing generation pipelines, fluid tiers, integrity span tables, building verbs, nav, multiplayer, or game examples as requirements. |
| Product One seed | Downstream / validation example motivating fully material walkable proof, dig/place honesty, collision against truth, sparse streaming, scar persistence, and measurable quality—without importing third-person character, curated region postcard, content palette, performance tables, milestones, or generation pipeline into product identity. |
| System-substrate pivot notice | Excluded source; contributes no product requirements. Useful substrate-only principles live in the GPU-resident note. |

**Not elevated into product requirements:** crate structure, algorithms, brick
dimensions, voxel size, buffer or index formats, atomic widths, named graphics
backends, platform-specific engineering constraints, portability mechanisms,
task breakdowns, implementation milestones, example-game fiction, characters,
controls, assets, or validation-harness particulars—unless a later explicit
human decision makes a specific choice binding at product altitude.
