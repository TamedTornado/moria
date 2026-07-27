# Moria product design

Standalone product design for the Moria voxel-world substrate. This document
turns the approved product vision into consumer-facing systems, interactions,
rules, states, content boundaries, behavior, failure behavior, and validation
experience. It does not choose implementation technology, storage formats,
algorithms, crate layout, APIs, deployment topology, or delivery milestones.

**Authority.** `docs/product-vision.md` is the complete product boundary. This
design preserves its requirements and non-goals in meaning. It does not reopen
seed documents or redefine product identity from harness content.

---

## 1. Overview

### 1.1 Product

**Moria** is reusable **voxel-world infrastructure**. Game and tool authors
install it and drive continuous three-dimensional **material volumes** through a
single public consumer contract.

The product is engine-shaped world infrastructure, not a shipped game. Its job
is to make hard world systems agree as **explicit, shareable contracts**: sparse
material truth, bounded inspection, mutation admission, streaming lifecycle,
collision against matter rather than presentation, persistence of matter plus
edits, GPU-resident representation of that matter, measurable presentation
derived from truth, and seams so a physics engine can plug in without privileged
access to voxel storage.

### 1.2 Core claim

A consumer can obtain a continuous three-dimensional material world—including
movable, damageable voxel volumes such as players and enemies—inspect and mutate
it only through supported interfaces, keep authoritative matter **GPU-resident**
for scale and gameplay-enabling work, and trust that what they see and collide
with is a view of the **same** authoritative matter.

Gravity, force, material strength, and related physical response are
**supportable through exposed bindings**. The substrate does not own a physics
simulator. How a game fills or seeds sparse material volumes is a consumer
concern that runs **on top of** the substrate.

### 1.3 What success looks like for consumers

| Need | Success |
| --- | --- |
| One material world | Occupancy, queries, collision truth, persistence, and (when present) physics plug-ins all read the same authoritative matter. |
| Contracted access only | Install the facade; inspect through public reads; mutate through admitted edits; never require privileged internal paths. |
| Scale without full residency | Untouched homogeneous volume stays cheap; the interesting shell and active edits pay detailed cost. |
| Everywhere mutation | Any exposed material cell can be destroyed or placed; cut faces and scars remain honest matter; presentation rebuilds from truth. |
| Genuine volumetric depth | Depth along the full third axis is real material content, not a painted underside of a heightmap floor. |
| Movable material volumes | Dynamic voxel volumes move and take damage under the same truth contracts as static geometry. |
| Physics without ownership | Strength, gravity parameters, force, and related fields are exposable so an external engine can attach. |
| Tractable scars | Edits and related world-state scars persist without dumping every cell of untouched volume. |
| GPU-scale work | Sparse representation and a command/query boundary keep matter GPU-resident and allow asynchronous GPU work without changing the consumer contract. |
| Measurable quality | Harnesses can evidence mutation response, streaming, GPU memory behavior, collision-truth honesty, and physics-binding readiness when exercised. |
| Content ownership | Consumers inject or drive world content through seams; no particular generator is substrate law. |
| Domain-appropriate look | Fully material volumes can present as coherent for the consumer’s domain; no single overworld aesthetic is mandated. |

### 1.4 Primary consumers

1. **Game and tool authors** who need continuous material volumes with honest
   mutation, inspection, collision truth, streaming, and persistence.
2. **Adjacent in-repo harnesses** (curation, benchmark, visual validation,
   optional walkable proof) that exercise the **same** public interfaces an
   external game would use. They prove contracts; they do not redefine product
   identity or “done.”

---

## 2. Product boundary (owned vs adjacent vs out)

### 2.1 This product owns

User-visible substrate systems and their public facade:

1. Sparse storage and lazy materialization of voxel truth.
2. GPU-resident sparse representation and a command/query boundary that can
   complete work asynchronously without changing the consumer contract.
3. Bounded world inspection and telemetry.
4. Mutation admission (dig, place, and related world-edit verbs).
5. Streaming and lifecycle so large regions need not keep full raw-voxel
   residency.
6. Collision and occupancy truth against voxel matter (not disposable meshes).
7. Physics plug-in bindings: material strength, gravity parameters, applied
   force, and related supportable fields—without baking in a physics engine.
8. Support for dynamic voxel volumes that move and can take damage.
9. Persistence of material truth plus edit deltas (and related world-state
   scars), without requiring a full-volume dump.
10. Presentation support that derives surfaces and surface dressing from
    material truth, without serializing derived geometry as authority.
11. Object and clutter registration hooks for matter-backed assemblies
    (vegetation, micro-objects, similar).
12. Content-injection seams so consumers drive how base volume is produced.
13. Volume-general contracts that do not encode planetary heightmap or
    gravity-aligned terrain as the only legal world shape.

### 2.2 Adjacent (not product identity)

- Curation, benchmark, and visual-validation executables and similar harnesses.
  Controllers, characters, cameras, authored demo routes, presentation polish,
  acceptance scenarios, and game-specific generation algorithms belong to those
  consumers.
- A walkable-world harness is a **permitted** adjacent artifact, not a mandatory
  definition of “done.”
- Any physics engine (hand-rolled or third-party) that integrates through
  bindings. A simple proof engine may demonstrate seams; shipping or owning one
  is not required. Gravity response, contact resolution as a simulation loop,
  and force-driven crumbling dynamics remain plug-in / consumer concerns.

### 2.3 Downstream (not this product)

Actual games and game layers, including but not limited to: player control,
characters, skeletal animation, game-specific presentation, combat rules, AI
behavior, economy, building policy, the System / LLM layer, spells, gas
pricing, agent labor, building UI / blueprints as gameplay, mechanisms as game
entities—and **which** generation pipeline and **which** physics engine a game
chooses.

Freeform ship and station games are **future-consumer examples**. They motivate
volume-general contracts; they are not current delivery or validation targets.

---

## 3. User-visible systems

These systems are the product’s surface for consumers. They are described by
what they guarantee and how they behave, not by implementation.

### 3.1 Material truth system

**Role.** Holds authoritative occupancy and material of continuous
three-dimensional volumes.

**Consumer-visible guarantees:**

- Continuous material volumes are representable without full raw-voxel residency
  of untouched regions.
- Homogeneous emptiness and solid remain cheap; detail cost concentrates on
  interesting shell (surfaces, voids, structures) and active edits.
- Deep volume along the full third axis is first-class material content (caves,
  strata, ore, aquifers as material bands, buried structure, and freeform
  interiors expressible as volume).
- Contracts remain volume-general: planetary heightmap and gravity-aligned
  terrain are not the only legal world shapes.

**Not owned:** any particular world generator, overworld palette, or ship/station
content pack.

### 3.2 Lazy materialization system

**Role.** Makes regions detailed on demand rather than all at once.

**Consumer-visible guarantees:**

- Approaching, inspecting, or mutating a location can cause required detail to
  appear for that work.
- Areas that remain uniform need not expand into full detail solely because they
  exist in the world bounds.
- Materialization does not invent a second occupancy model; expanded detail is
  still the same material truth.

### 3.3 Inspection system

**Role.** Bounded public reads of world state and substrate health.

**Consumer-visible guarantees:**

- Consumers inspect through public reads, queries, snapshots, telemetry, or
  events.
- Inspection never requires privileged internal storage paths.
- Bounds and result shapes are finite and contract-defined so large worlds remain
  queryable without dumping the entire volume.

**Typical inspection classes (product altitude):**

- Occupancy / material at a location or bounded region.
- Streaming / residency / lifecycle status of regions under contract.
- Telemetry useful for harnesses and games (work pending, memory-class
  observations, mutation and presentation readiness signals as defined by the
  public boundary).
- Collision / occupancy query results (see §3.6).

Exact query catalogs beyond these classes remain an implementation concern so
long as the public boundary is sufficient for harnesses and external games to
share one contract.

### 3.4 Mutation admission system

**Role.** Sole path by which material truth changes.

**Consumer-visible guarantees:**

- Dig, place, and related world-edit verbs are admitted only through the public
  contract.
- Any material cell the contract exposes can be destroyed or placed under the
  same rules (mutation is universal within the contract).
- Cut faces and scars remain honest matter; presentation and collision rebuild
  from the changed truth.
- Decorative-only solid geometry that cannot be edited under the same rules is
  not the product model for exposed material.

**Not owned:** building policy, construction UI, blueprints as gameplay, dig as
progression, or game-mode rules about who may edit what.

### 3.5 Streaming and lifecycle system

**Role.** Keep large worlds tractable by bounding what is actively resident in
detail.

**Consumer-visible guarantees:**

- Active and cold regions remain manageable; large worlds do not require
  permanent full raw-voxel residency of an entire region.
- Moving focus (camera, player proxy, harness focus, or consumer-driven interest)
  changes which regions are active without changing underlying material truth.
- Streaming lifecycle is observable enough for harnesses to evidence scale
  behavior without privileged internals.

### 3.6 Collision and occupancy truth system

**Role.** Provide contacts and occupancy answers against material authority.

**Consumer-visible guarantees:**

- Occupancy and collision truth read voxel matter, not disposable meshes.
- Consumers and physics plug-ins share one material world; they must not invent a
  parallel mesh world for contact.
- Newly mutated openings and fills affect occupancy truth so traversal and
  contact proofs remain honest after edits.

**Not owned:** character controllers, movement feel, full contact-resolution
simulation loops, or a baked-in physics engine.

### 3.7 Physics plug-in binding surface

**Role.** Expose material properties and world seams a physics engine needs.

**Consumer-visible guarantees:**

- Material strength, gravity parameters, applied force, and related supportable
  fields are exposable.
- An external engine (hand-rolled or third-party) can attach without privileged
  voxel storage access.
- Physics engines are guests: they consume truth and bindings; they do not own
  voxel storage.
- Runtime physics simulation is **not** a substrate deliverable. A simple
  engine is an acceptable **proof** of the seams when a harness or consumer
  chooses to exercise them.

**Not owned:** gravity response as a mandatory simulation, structural integrity /
cave-in simulation, span tables, force-driven crumbling as substrate product, or
shipping any particular physics engine.

### 3.8 Dynamic material volume system

**Role.** Support matter that moves and can take damage under the same contracts
as static world geometry.

**Consumer-visible guarantees:**

- Movable, damageable voxel volumes (for example combatants treated as material)
  share the same truth contracts as static world matter.
- Movement and damage to those volumes remain subject to inspection, mutation
  admission, collision truth, and presentation-from-truth rules as applicable.
- Dynamic volumes are not a disconnected overlay that only pretends to be matter.

**Not owned:** character animation, combat rules, AI, hit-point systems, or
game-specific entity models beyond the material-volume class.

### 3.9 Persistence system

**Role.** Save and restore material truth plus edit scars tractably.

**Consumer-visible guarantees:**

- Edit deltas and related world-state scars persist without a dump of every cell
  of untouched volume.
- Reload reconstructs the same authoritative matter for saved scars; derived
  presentation is rebuilt from restored truth, not loaded as authority.
- How the *base* volume is produced (generation function, authored fill, import)
  is the consumer’s concern; the substrate persists the material result and the
  scars, not a particular generator identity as product law.

**Not owned:** multi-slot save UX as a game feature, account systems, or
persistence of excluded game systems.

### 3.10 Derived presentation system

**Role.** Surfaces and surface dressing derived from material truth.

**Consumer-visible guarantees:**

- Visible surfaces rebuild from truth; meshes, dressing, and debug geometry are
  **derived and disposable**.
- Derived geometry is never serialized as world authority.
- Vegetation and clutter presentation stays derived from matter—not a
  disconnected prop layer that desyncs from material truth when supporting
  matter changes.
- Fully material volumes can present as coherent for the consumer’s domain
  (geology, masonry, interiors, and similar). No single natural-overworld
  aesthetic is mandated.
- Ship bulkheads and similar freeform-hull presentation remain future-consumer
  context, not current validation targets.

**Not owned:** a finished visual game engine claim, game art direction, polished
demo lighting as substrate requirement, or a mandated Minecraft-style cube look.

### 3.11 Object and clutter registration system

**Role.** Register matter-backed assemblies without baking them into a single
terrain slab.

**Consumer-visible guarantees:**

- Vegetation, micro-objects, and similar assemblies can register as matter-backed
  objects or clutter.
- Registration hooks support consumer content without requiring one fixed terrain
  palette.
- Presentation of registered objects remains consistent with material truth
  rules (derived views, honest mutation when those objects are exposed material).

**Not owned:** tree felling, rigid-body conversion of vegetation, growth systems,
fire, or other future consumer simulation concepts.

### 3.12 Content injection seams

**Role.** Let consumers inject or drive world content, including their own
generation algorithms.

**Consumer-visible guarantees:**

- Consumers own how base volume is produced.
- The substrate provides seams for fill, injection, and drive of material truth
  without embedding any particular generator as substrate law.
- Harness-side generation used only to exercise contracts remains a harness
  concern and does not reclassify generation as substrate product.

### 3.13 GPU-resident command / query boundary

**Role.** Keep authoritative matter GPU-resident and allow asynchronous GPU work
while preserving one consumer contract.

**Consumer-visible guarantees:**

- Authoritative matter can live GPU-resident for scale and gameplay-enabling
  work—a deliberate product distinction from CPU-driven voxel engines.
- Consumers interact through commands and queries (and related public events),
  not through direct ownership of internal buffers.
- GPU work may complete asynchronously; consumers must not depend on synchronous
  ownership of internal storage or privileged buffer access.
- Implementation ownership of work may move between CPU and GPU without changing
  the consumer contract.

Specific kernels and simulations remain design-selected later; residency and the
async-capable boundary do not.

---

## 4. Interactions

### 4.1 Install and drive the facade

1. A consumer installs the substrate facade into its application.
2. The consumer configures or injects world content through content seams (its
   own generation, authored fill, or harness parameters).
3. The consumer inspects through public reads and mutates through admitted
   edits only.
4. Adjacent harnesses follow the same path; no privileged install path exists
   for validation.

### 4.2 Inspect material truth

1. The consumer issues a bounded inspection request (point, region, snapshot, or
   telemetry class exposed by the contract).
2. The substrate returns answers from authoritative matter (or lifecycle /
   telemetry observations), not from disposable meshes as authority.
3. If detail is not yet resident for that work, lazy materialization and
   streaming lifecycle bring needed detail into play without inventing a second
   world model.

### 4.3 Mutate matter (dig / place / related verbs)

1. The consumer submits an admitted edit (destroy material, place material, or
   related world-edit verb) against cells the contract exposes.
2. Material truth updates for the affected volume.
3. Collision / occupancy truth reflects the new matter.
4. Derived presentation and dressing refresh from the changed truth for affected
   surfaces.
5. Edit scars become first-class candidates for persistence.
6. Consumers observe completion through the public command/query boundary,
   including when GPU work finishes asynchronously.

**Honesty test.** After dig or place, cut faces and openings are real matter and
real occupancy—not a painted hole in a mesh that still collides as solid, and
not solid scenery that cannot be edited.

### 4.4 Stream and focus lifecycle

1. Consumer focus (or harness focus) moves through the world.
2. Nearby or requested regions become active in detail as needed.
3. Distant or idle homogeneous regions remain compact / cold without changing
   truth.
4. Harnesses can evidence that large regions remain tractable under this
   lifecycle.

### 4.5 Query collision and occupancy

1. A consumer or physics plug-in asks occupancy or contact questions against
   material authority.
2. Answers come from voxel matter, not from presentation meshes.
3. After mutation, subsequent queries reflect the new truth so movement and
   contact proofs stay honest.

### 4.6 Attach a physics plug-in (optional for a given consumer)

1. The consumer binds an external physics engine through the plug-in surface.
2. The engine reads supportable fields (strength, gravity parameters, force, and
   related bindings) and material truth through public seams.
3. The engine never gains privileged voxel storage access.
4. Simulation results that should affect matter re-enter through admitted
   mutation or other public world verbs—not by writing private storage.

If no physics engine is attached, the substrate still provides collision /
occupancy truth; runtime force/gravity simulation simply does not run as a
substrate feature.

### 4.7 Move and damage dynamic material volumes

1. A consumer drives a dynamic voxel volume’s pose or damage under the same
   matter contracts as static geometry.
2. Occupancy, inspection, presentation-from-truth, and (when used) physics
   bindings treat that volume as material, not as a disconnected billboard.
3. Damage that removes or alters matter uses admitted mutation semantics so
   scars and presentation stay honest.

### 4.8 Persist and restore scars

1. The consumer requests persistence of material truth changes and related
   world-state scars.
2. The substrate records scars without dumping untouched homogeneous volume.
3. On restore, the consumer re-establishes base volume by its own strategy
   (regenerate, reload authored base, etc.) and the substrate reapplies scars so
   authoritative matter matches the saved edits.
4. Presentation rebuilds from restored truth; derived geometry is not the
   save format for authority.

### 4.9 Register objects and clutter

1. The consumer registers matter-backed assemblies (vegetation, micro-objects,
   stamps, similar) through registration hooks.
2. Presentation and, where exposed as material, mutation and collision remain
   consistent with truth-vs-view rules.
3. Supporting-surface changes remove or refresh derived dressing so props do not
   float as a desynced layer.

### 4.10 Present the world

1. The consumer (or harness) requests or enables presentation for active
   regions.
2. Surfaces and dressing derive from current material truth.
3. Domain look is consumer-driven; the substrate supports coherent material
   presentation rather than mandating one postcard aesthetic.
4. Debug or diagnostic geometry, when used, is explicitly disposable and never
   confusable with authority.

---

## 5. Rules and invariants

These must remain true for the product to be Moria as approved.

1. **Matter is authority; views are disposable.** Presentation, dressing, and
   debug geometry never become truth.
2. **One consumer contract.** Harnesses and external games use the same public
   facade. Privileged harness paths invalidate proof.
3. **No second world for collision.** Occupancy and collision truth are
   material.
4. **Mutation is universal within the contract.** Exposed material is editable
   under the same rules; decorative-only uneditable solid is not the model.
5. **Depth is volume, not paint.** Underground and freeform interiors are
   expressible as real material volume.
6. **Homogeneous emptiness and solid are cheap.** Scale depends on not paying
   full detail cost for untouched volume.
7. **Scars are first-class persistence.** Edits survive without full dumps.
8. **Residency does not break contracts.** GPU-resident work and async
   completion remain behind the public command/query boundary.
9. **Physics engines are guests.** They attach through bindings; they do not own
   voxel storage.
10. **Content algorithms are guests.** Generators and fill strategies run on
    seams; they are not substrate law.
11. **Volume-general contracts.** Substrate contracts must not assume
    gravity-aligned planetary terrain as the only legal world shape—even though
    ship/station content is not current delivery or validation.
12. **Dynamic voxel volumes are first-class matter.** Movable, damageable volumes
    share truth contracts with static geometry.
13. **Substrate first.** Moria is world infrastructure, not a finished visual
    game engine claimed before feasibility and visual-acceptance gates are met.

---

## 6. States

Product-level states consumers and harnesses can reason about. Names are
conceptual, not implementation enums.

### 6.1 World material states

| State | Meaning |
| --- | --- |
| **Homogeneous compact** | Untouched uniform empty or solid volume represented cheaply without full raw detail. |
| **Detailed active** | Region expanded as needed for inspection, presentation, mutation, or contact work. |
| **Scarred** | Material differs from the consumer’s base volume strategy because of admitted edits or related world-state scars. |
| **Pending presentation** | Material truth has changed (or newly activated) and derived surfaces/dressing are not yet ready for the current truth. |
| **Presentation current** | Derived views match current material truth for the active concern. |

Truth transitions (compact ↔ detailed, clean base ↔ scarred) do not create a
second occupancy model. Presentation readiness may lag truth when work is
asynchronous; consumers observe readiness through the public boundary rather than
by owning internal buffers.

### 6.2 Region lifecycle states

| State | Meaning |
| --- | --- |
| **Cold / inactive** | Not held in detailed active residency; truth remains reconstructible when needed. |
| **Activating** | Streaming / materialization work is bringing the region into detailed use. |
| **Active** | Available for the consumer’s current inspection, presentation, mutation, or contact needs. |
| **Deactivating** | Leaving detailed residency while preserving truth and scars as required by contract. |

### 6.3 Mutation work states

| State | Meaning |
| --- | --- |
| **Edit admitted** | A public mutation command has been accepted. |
| **Truth updated** | Authoritative matter reflects the edit. |
| **Occupancy current** | Collision / occupancy queries reflect the edit. |
| **Presentation refreshed** | Derived surfaces/dressing for affected volume match new truth. |
| **Scar durable** | Edit is included in persistence candidates / durable scar set as applicable. |

Asynchronous GPU completion may separate these observations in time. The product
guarantee is that consumers can discover readiness through the contract—not that
every side effect is synchronous.

### 6.4 Dynamic volume states

| State | Meaning |
| --- | --- |
| **Static material** | World matter not currently driven as a moving volume. |
| **Dynamic pose** | Material volume whose placement can move under consumer (or plug-in) drive. |
| **Damaged / altered** | Material of the volume has been reduced or changed through admitted damage or mutation semantics. |

### 6.5 Physics binding states

| State | Meaning |
| --- | --- |
| **Bindings available** | Supportable fields and seams are exposable regardless of whether a plug-in is attached. |
| **Plug-in attached** | An external engine is consuming bindings and material truth through public seams. |
| **No plug-in** | Substrate still provides material and occupancy truth; runtime physics simulation is simply not running as a guest. |

### 6.6 Persistence states

| State | Meaning |
| --- | --- |
| **Base only** | No durable scars beyond the consumer’s base volume strategy. |
| **Scar set present** | Edit deltas / related scars exist and can be saved. |
| **Restored** | Base strategy plus applied scars reconstruct authoritative matter; presentation rebuilds from that truth. |

---

## 7. Content boundaries

### 7.1 In content scope (substrate-supported)

- Continuous three-dimensional material volumes of any consumer domain the
  contracts can express (natural landscapes, underground geology, constructed
  interiors among them).
- Material cells that can be inspected and, when exposed, mutated.
- Deep volumetric features as real matter (voids, strata, bands, buried
  structure).
- Static or dynamic material volumes under the same truth contracts.
- Matter-backed object and clutter registration (placement and derived
  presentation; not vegetation simulation systems).
- Domain-coherent presentation support without a mandated single palette.
- Consumer-owned base content production via injection seams.
- Supportable material fields for physics guests (strength, gravity parameters,
  force, related).

### 7.2 Explicitly out of content / product scope

From the approved vision’s non-goals and exclusions (preserved in meaning):

- Shipping a game, game mode, progression loop, or game-rules stack.
- Treating validation-harness content, third-person controllers, demo routes,
  content palettes, or machine-specific demo targets as substrate requirements
  or as mandatory delivery for product completion.
- Baking deterministic or procedural world generation into the substrate as
  product identity.
- Baking in a hand-rolled or third-party physics engine as product identity.
- System / LLM, spells, gas / pricing policy, combat rules / AI behavior, agent
  labor, building UI / blueprints as gameplay, mechanisms as game entities.
- Delivering or specifically validating freeform ships, stations, or other
  multi-deck freeform hulls as current product work.
- Full fluid simulation and cellular automata (fire, wetness, growth) as current
  product requirements.
- Tree felling or rigid-body conversion of vegetation as current product
  requirements.
- Web / wasm as a Product One or substrate target platform.
- Claiming a released, finished visual engine before feasibility and
  visual-acceptance gates are met.
- Limiting identity to a Minecraft-style cube aesthetic, a single
  natural-overworld palette, static scenery without movable material volumes, or
  heightmap terrain that only pretends to have depth.
- Assuming gravity-aligned planetary terrain as the only legal world shape in
  contracts.
- Structural integrity / cave-in simulation, span tables, fortress engineering
  UI, navigation graphs, multiplayer services, weather simulation, seasons,
  growth systems, and fine splash / particle matter layers as current substrate
  product.

### 7.3 Future-consumer context only

Possible later products motivate reusable capabilities but do not import fiction,
UI, missions, or delivery into current scope:

- System-driven ARPG on a continuous natural world.
- Fortress / colony engineering and designation play.
- Descent-style roguelike through deep geology.
- Pure sandboxes.
- Games whose players and enemies are voxel volumes that move and take damage
  under the same matter contracts, with physics engines plugged in through
  bindings.
- Freeform ship and station volumes that motivate everywhere mutation, deep
  multi-deck volume, GPU-resident matter at combat/design scale, physics-ready
  bindings under force, and honest damage/salvage—without importing that fiction
  into Moria.

---

## 8. Behavior

### 8.1 Truth vs view

Occupancy, queries, collision truth, and persistence run against voxel matter.
Meshes, surface dressing, and debug geometry are derived and disposable. Physics
engines, when present, also consume material truth through public bindings rather
than a private mesh world. Derived geometry is never serialized as world
authority.

### 8.2 Contracted consumption

External consumers install the facade, inspect through public reads, mutate
through admitted edits, and never require privileged internal paths. Adjacent
harnesses and external game crates share the same public boundary. GPU work may
complete asynchronously; consumers must not depend on direct buffer access or
synchronous ownership of internal storage.

### 8.3 Sparse scale

Large regions remain tractable: untouched homogeneous volume stays cheap; only
the interesting shell and active edits pay detailed cost. Streaming and lifecycle
keep residency bounded without requiring full raw-voxel presence for an entire
region.

### 8.4 Mutable everywhere

Any material cell the contract exposes can be destroyed or placed. Cut faces and
scars remain honest matter; presentation rebuilds from truth. Mutation is a
first-class product proof—not optional scenery decoration outside the material
world.

### 8.5 Deep Z is first-class

Volume along the full depth axis is real content—genuine volumetric depth, not a
heightmap floor with painted underground. Contracts stay volume-general so
non-planetary freeform volumes remain expressible; ship and station interiors are
future-consumer motivation, not a required current deliverable shape.

### 8.6 Dynamic voxel volumes

The world is not static geometry alone. The substrate supports voxel volumes that
move and can take damage so games can treat combatants as matter under the same
truth contracts rather than as overlays disconnected from the world.

### 8.7 Physics-ready bindings, not a baked-in engine

The substrate exposes bindings and material data a physics engine needs—whether
the consumer hand-rolls an engine or adopts one. Runtime physics simulation is
not substrate-owned product. Collision and occupancy truth against voxel matter
remains a substrate concern so plug-ins and consumers share one material world.

### 8.8 Cheap scars over full dumps

Persistence keeps material edits and related scars tractable. A consumer may
choose a reproducible generation function as its base-world strategy; that choice
is game-dependent and is not a substrate deliverable.

### 8.9 GPU-resident architecture

Sparse representation and a command/query boundary keep world matter
GPU-resident and support asynchronous GPU work. This is a product distinction
from CPU-driven voxel engines. Specific kernels and simulations remain
later choices; residency and the async-capable boundary do not.

### 8.10 World-dependent presentation

How a world “looks natural” depends on the consumer’s world—landscape geology,
fortress masonry, and other material styles. The substrate supports fully
material volumes that read as coherent for their domain; it does not mandate a
single overworld aesthetic or a heightmap-with-props look.

### 8.11 Supporting consumption principles

- Consumers must not receive privileged access to internal voxel storage.
- Mutations enter through explicit public commands.
- Inspection uses bounded public interfaces.
- The same public boundary must serve validation harnesses and external game
  crates.
- Implementation ownership of work may move between CPU and GPU without changing
  the consumer contract.
- Vegetation and clutter presentation stays derived from matter—not a
  disconnected prop layer that desyncs from material truth.

---

## 9. Failure behavior

Product-level failure and degradation rules (what consumers should experience
when things go wrong or are incomplete). These are behavioral contracts, not
error-code designs.

### 9.1 Contract violations

- Attempts to inspect or mutate through non-public paths are not supported
  product behavior. Harnesses that require privileged access fail as product
  proofs even if they appear to “work.”
- Edits outside admitted verbs or against cells the contract does not expose
  must not silently rewrite internal storage.

### 9.2 Incomplete readiness (async / streaming)

- If truth has updated but presentation is not yet current, consumers must be
  able to observe that distinction through the public boundary rather than by
  reading internal buffers.
- Collision / occupancy answers must not permanently diverge into a mesh-only
  world while presentation catches up; honesty of matter remains the rule.
  Transient lag is acceptable only if the contract makes readiness observable
  and eventual consistency with truth is guaranteed.
- Requests against regions still activating may block, queue, or return
  explicitly incomplete results as defined by the public contract—but must not
  invent occupancy that contradicts material truth.

### 9.3 Scale pressure

- When regions exceed what full raw-voxel residency would reasonably allow, the
  product continues via sparsity and streaming rather than requiring the entire
  volume detailed at once.
- Failure to keep homogeneous volume cheap is a product quality failure for the
  sparse-scale claim, not a consumer problem to solve with private mesh worlds.

### 9.4 Persistence failures

- A successful restore must reapply scars so edited matter matches what was
  saved; partial silent drop of scars is a product failure.
- Derived meshes must not be required as the authority needed to restore a
  world. If presentation assets are missing, truth and scars still restore and
  views rebuild.

### 9.5 Physics guest failures

- If a physics plug-in misbehaves, crashes, or is absent, the substrate’s
  material truth, inspection, mutation, and occupancy truth remain well-defined.
- A missing or broken physics engine does not remove the requirement that
  bindings be exposable; it only means runtime simulation is not running.

### 9.6 Content guest failures

- A consumer generator failure is a consumer failure. The substrate does not
  substitute a baked-in world generator to “save” the session as product
  behavior.
- Invalid or empty injection still must not open privileged storage paths as a
  workaround.

### 9.7 Presentation desync

- If dressing or object presentation remains after its supporting matter is
  gone, or collides differently from occupancy truth, the product has failed the
  truth-vs-view claim for that case.
- Debug visualizations that are mistaken for authority in save/load or collision
  paths are product failures.

### 9.8 Premature finished-engine claims

- Marketing or acceptance language that claims a released, finished visual engine
  before feasibility and visual-acceptance gates are met is out of product
  posture. Validation may prove contracts without redefining identity as a
  finished game engine.

---

## 10. Validation experience

How the product proves itself without turning harness content into product
requirements.

### 10.1 Principles

- **Contracts over spectacle.** Evidence shows inspection, mutation, streaming,
  collision truth, persistence, GPU-resident behavior, and (when exercised)
  physics bindings hold—not that a particular forest postcard or character
  controller exists.
- **Same-interface proof.** Harnesses must use the public facade available to
  external games. Privileged internal paths invalidate the proof.
- **Mutation is the honesty test.** Dig and place (or equivalent admitted edits)
  must leave honest cut faces and rematerialized presentation from truth; a
  world that only looks good until edited has failed the material claim.
- **Collision against truth.** Movement and contact proofs read voxel matter, not
  disposable meshes.
- **Sparse scale is real.** Regions large enough that full raw-voxel residency is
  unreasonable must remain tractable under streaming and homogeneous cheap
  storage.
- **Deep volume is exercisable.** Depth content must be reachable as material
  volume (for example caves, strata, buried structure), not as skybox or painted
  floors.
- **Measurable substrate quality.** Benchmarks and evidence capture mutation
  response, streaming, GPU memory behavior, collision-truth honesty, and
  physics-binding readiness when exercised—with machine context so results are
  comparable. Specific performance numbers, demo routes, and acceptance scenes
  belong to harness/TDD design, not this product design’s identity.
- **Optional physics proof.** A simple physics plug-in may demonstrate bindings;
  shipping or owning that engine is not required for product completeness.
- **Optional walkable harness.** A walkable third-person proof may make claims
  undeniable to humans; its controls, characters, assets, curated routes, content
  palettes, and generation pipelines are harness particulars, not substrate
  requirements.
- **No premature finished-engine claim.** Do not claim a released, finished
  visual engine before feasibility and visual-acceptance gates are met.

### 10.2 Adjacent validation artifacts (permitted, not identity)

Consumers may include:

| Artifact | Purpose | Must not become |
| --- | --- | --- |
| Curation tooling | Parameters and content injection for exercises | Substrate generator identity |
| Benchmark harness | Comparable evidence for mutation, streaming, memory, honesty metrics | Game-mode definition of “done” |
| Visual validation | Human-reviewable presentation of material truth | Mandated overworld postcard as product law |
| Optional walkable harness | Make continuous material claims undeniable via traversal | Character controller / demo route as substrate requirement |
| Optional simple physics plug-in | Prove binding readiness | Baked-in physics engine as product |

All of the above, when present, use the same public consumer contract.

### 10.3 Evidence classes the product must be able to show

Without fixing numeric thresholds here:

1. **Mutation honesty** — admitted dig/place leaves real cut faces; presentation
   and occupancy follow truth.
2. **Streaming / sparse scale** — large regions remain tractable; homogeneous
   volume stays cheap.
3. **Deep volume** — reachable material depth, not painted underside.
4. **Collision-truth honesty** — contacts/occupancy against matter.
5. **Scar persistence** — edits restore without full-volume dumps; derived
   geometry not required as authority.
6. **GPU-resident / async boundary** — work and residency remain behind the
   public command/query contract.
7. **Physics-binding readiness (when exercised)** — a guest engine can attach
   through bindings without privileged storage.
8. **Dynamic volume class** — movable/damageable material volumes under the same
   contracts can be exercised.
9. **Same-interface discipline** — harnesses do not need privileged paths.

Machine context must accompany quantitative evidence so results remain
comparable across hardware.

### 10.4 What validation must not require

- A specific third-person character, camera feel, or demo route as substrate
  completion criteria.
- A fixed natural-overworld content palette or Minecraft-style cube aesthetic as
  product identity.
- Ship/station freeform hull delivery or validation as current work.
- Ownership of a physics engine or world generator as product completeness.
- Web / wasm platform targeting.

---

## 11. Non-goals

Explicitly out of product scope (preserved from the approved vision):

- Shipping a game, game mode, progression loop, or game-rules stack.
- Treating validation-harness content, third-person controllers, demo routes,
  content palettes, or machine-specific demo targets as substrate requirements
  or as mandatory delivery for product completion.
- Baking deterministic or procedural world generation into the substrate as
  product identity. Generation algorithms are game-dependent and run on top of
  the substrate; consumers own them.
- Baking in a hand-rolled or third-party physics engine as product identity.
  Material strength, gravity, force, and related fields must be *supportable*
  via plug-in bindings; the engine that consumes them is adjacent. A simple
  proof engine is optional demonstration, not required delivery.
- Implementing excluded layers here: System / LLM, spells, gas / pricing policy,
  combat rules / AI behavior, agent labor, building UI / blueprints as gameplay,
  mechanisms as game entities.
- Delivering or specifically validating freeform ships, stations, or other
  multi-deck freeform hulls as current product work—those remain future-consumer
  examples that motivate volume-general contracts.
- Full fluid simulation and cellular automata (fire, wetness, growth) as current
  product requirements—these may appear as future consumer concepts or format
  hooks unless later selected explicitly.
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

Any later design choice that would reclassify substrate vs consumer ownership
(for example promoting full fluid CA, tree felling, structural collapse
simulation, freeform-hull delivery, or a baked-in physics engine into substrate
product) requires an **explicit human scope decision**—not silent expansion.

---

## 12. Resolved product decisions

Closed by the approved vision. Do not reopen without a new human decision.

| Topic | Decision |
| --- | --- |
| Product identity | Reusable voxel-world substrate / public consumer contract—not a shipped game. |
| Walkable-world visual validation harness | Adjacent artifact. May exist for validation; does not define product identity or “done.” |
| Everywhere mutation | Binding. |
| First-class deep Z | Binding. Genuine volumetric depth, not heightmap terrain. |
| Natural-looking presentation | Depends on the consumer’s world—coherent material presentation for that domain, not a single natural-overworld mandate. |
| GPU-resident / async-capable architecture | Binding product direction and a core distinction from CPU-driven voxel engines. Specific simulations remain later choices. |
| Multi-world freeform volumes (ships / stations) | Contracts are volume-general. Delivering or specifically validating ships and stations is not current scope. |
| Dynamic (moving, damageable) voxel volumes | Yes. Substrate must support that class of matter. |
| Deterministic / procedural world generation | No as substrate product. Generation runs on top and is game-dependent. |
| Matter physics | Bindings yes; engine no. Collision/occupancy truth against voxel matter remains substrate concern. |
| Older “Product One — The Walkable World” identity language | Superseded on identity by the product vision. Harness content does not redefine the product. |

---

## 13. Open questions for the human

The approved vision currently records **no open product-boundary questions** that
would change identity, purpose, or boundary. Engineering and milestone
sequencing remain intentionally undecided there.

The following items cannot be fully resolved from the vision alone at product
design altitude. They are recorded as questions rather than guessed defaults.
They do **not** reopen closed identity decisions above.

1. **Minimum public material property set for physics bindings.** Which supportable
   fields beyond strength, gravity parameters, and applied force must be present
   for the first binding surface to count as “physics-ready,” and which may wait?
   (Vision requires the class of bindings; it does not rank the first property
   set.)

2. **Gravity parameterization for non-planetary volumes.** How should gravity
   parameters be expressed so volume-general contracts remain honest for
   freeform hulls later, without delivering ship/station content now?

3. **Dynamic volume damage semantics at the substrate boundary.** When a dynamic
   material volume “takes damage,” which outcomes are substrate mutation verbs
   versus consumer/combat policy (partial cell erosion, whole-volume removal,
   material substitution)?

4. **Presentation coherence bar without a mandated aesthetic.** What evidence is
   sufficient that “coherent for the consumer’s domain” is met when no single
   overworld postcard is required—purely contract/harness-defined scenes, or a
   small set of named domain samples?

5. **Async readiness visibility.** At product altitude, which readiness
   distinctions must consumers always observe (truth updated vs occupancy
   current vs presentation refreshed vs scar durable), and which may remain
   harness telemetry only?

If answering any of these would reclassify substrate vs consumer ownership, treat
that answer as a scope decision requiring explicit human approval.

---

## 14. Provenance note

This design is synthesized solely from the approved `docs/product-vision.md`.
Seed documents and superseded Product One walkable-world planning language were
not treated as authoritative product boundary. Implementation, crate structure,
algorithms, storage layouts, numeric acceptance thresholds, demo routes, and
task breakdowns remain outside this document’s scope.
