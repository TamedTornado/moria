# Moria product vision

Standalone product vision for designers, planners, and reviewers. This document
is self-contained: it carries the product substance authorized by the approved
scope boundary. Seed material is synthesized here; it is not necessary to reopen
the seed set or decide which seed is authoritative.

**Authority.** Human-approved scope in `docs/vision.md` wins over every seed
implication. Material classified as adjacent or future may explain a capability
but is not a current deliverable. Material classified as excluded does not enter
this product. Engineering choices (crate layout, algorithms, data formats,
milestone order) belong to design and TDD work, not this vision.

---

## 1. Product identity

**Moria** is a reusable **voxel-world substrate**: engine-shaped world
infrastructure that downstream games and tools install and drive through a
public consumer contract.

It is a **Rust / Bevy library ecosystem** for crate consumers—not an
ecosystem-neutral engine abstract, and not a playable game product. It may be
one crate or a small family of tightly scoped crates; the consumer boundary is
mandatory, the exact package split is not product identity.

The substrate is **not** limited to a Minecraft-style cube aesthetic, a single
natural-overworld content palette, static scenery, or heightmap terrain that
only pretends to have depth. Substrate contracts are **volume-general**: they
must not assume gravity-aligned planetary terrain as the only legal world shape.
Delivering or specifically validating freeform ships, stations, or other
multi-deck freeform hulls is **not** current scope; those remain future-consumer
examples that the contracts should remain able to support.

**World generation is not part of the substrate identity.** How a game fills or
seeds sparse material volumes is a consumer- or game-dependent algorithm that
runs *on top of* the substrate. The substrate provides storage, query, mutation,
streaming, collision-truth seams, physics plug-in bindings, persistence seams,
and presentation derivation for material truth—not a baked-in procedural
generator and not a hand-rolled physics engine.

---

## 2. Purpose

Voxel worlds only work when several hard systems agree as **explicit contracts**:
sparse material truth, bounded inspection, mutation admission, streaming
lifecycle, collision against matter rather than presentation, persistence of
world matter plus edits, GPU-resident representation of that matter, measurable
presentation derived from truth, and seams so a physics engine can plug in
without privileged access to voxel storage.

Moria exists so material worlds can be consumed without each game rebuilding
those contracts, and without hiding them behind a single privileged demo.

**Product claim (infrastructural):** a consumer can obtain a continuous
three-dimensional material world (including movable, damageable voxel volumes
such as players and enemies), inspect and mutate it only through supported
interfaces, keep authoritative matter GPU-resident for scale and
gameplay-enabling work, and trust that what they see and collide with is a view
of the same authoritative matter—while gravity, force, material strength, and
related physical response are **supportable through exposed bindings**, not baked
into a substrate-owned physics engine.

---

## 3. Consumers and needs

### Primary consumers

- **Game and tool authors** who install the public facade and build worlds,
  tools, or validation experiences on top of contracted material truth.
- **Adjacent in-repo harnesses** (curation, benchmark, visual validation) that
  prove the same contracts external games must use—without privileged paths.

### What consumers need from the product

| Need | What the substrate must provide |
| --- | --- |
| Shared material truth | One authoritative sparse volume world that inspection, mutation, collision, persistence, and presentation all agree on |
| Safe consumption | Public reads, admitted edits, telemetry—no requirement for privileged internal storage access |
| Scale without full residency | Large regions stay tractable: homogeneous untouched volume stays cheap; interesting shells and active edits pay detailed cost |
| Honest mutation | Any exposed material cell can be destroyed or placed; cut faces and scars remain real matter; presentation rebuilds from truth |
| Genuine volume | Full depth-axis content (caves, strata, buried structure, freeform interiors as expressible volumes)—not a heightmap floor with painted underground |
| Dynamic matter | Movable, damageable voxel volumes under the same truth contracts as static geometry |
| Physics attachability | Material properties and world seams a physics engine needs, without owning or shipping that engine |
| Cheap lasting scars | Persistence of material truth plus edit deltas and related world-state scars—not dumps of every brick |
| GPU-scale residency | Sparse GPU-resident representation and an async-capable command/query boundary so scale and gameplay-enabling work do not force a CPU-only voxel engine model |
| Domain-coherent presentation | Surfaces and dressing derived from material truth so worlds can read as coherent for their domain; no single mandated overworld look |
| Content injection seams | Ways for consumers to inject or drive world content (including their own generation) without embedding a particular generator as substrate law |
| Object assemblies | Registration hooks so vegetation, micro-objects, and other matter-backed assemblies can participate without baking into a single terrain slab |
| Evidence without redefinition | Benchmarks and harnesses can measure contract quality without redefining Moria as a game |

---

## 4. Product boundary

### This product owns

The reusable substrate and its public facade:

1. **Sparse storage and lazy materialization** of voxel truth.
2. **GPU-resident sparse representation** and a **command/query boundary** that
   can support asynchronous GPU work without changing the consumer contract.
3. **Bounded world inspection and telemetry** for consumers.
4. **Mutation admission** (dig, place, and related world-edit verbs) so nothing
   touches voxels outside the contract.
5. **Streaming and lifecycle** so large regions do not require full raw-voxel
   residency.
6. **Collision and occupancy truth** against voxel matter, not against
   disposable meshes—queries and contacts read material authority so consumers
   and plug-ins do not invent a second world.
7. **Physics plug-in bindings:** material properties and world seams a physics
   engine needs—material strength, gravity parameters, applied force, and
   related supportable fields—exposed so a hand-rolled or third-party engine can
   attach without privileged voxel access. The substrate does **not** bake in a
   physics engine.
8. **Dynamic voxel volumes**—matter that moves and can take damage (players,
   enemies, and similar)—not only static world geometry.
9. **Persistence** of material truth plus edit deltas (and related world-state
   scars), without requiring a dump of every brick. How the *base* volume is
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
  and visually validate the substrate, but must consume the **same public
  interfaces** available to an external game.
- Controllers, characters, cameras, authored demo routes, presentation polish,
  acceptance scenarios, and **game-specific generation algorithms** belong to
  those consumers—not to substrate identity.
- A walkable-world harness is a **permitted adjacent artifact**, not a mandatory
  definition of “done” for Moria. In-repo generation used only to exercise or
  demonstrate contracts remains a harness concern, not a claim that generation
  is substrate product.
- **Any physics engine**—hand-rolled or third-party—that integrates through the
  substrate’s bindings. An acceptable *proof* of those bindings may be a simple
  physics engine in a harness or consumer, but Moria does not need to ship or
  own one. Gravity response, contact resolution as a simulation loop,
  force-driven crumbling dynamics, and similar runtime physics remain plug-in /
  consumer concerns; the substrate makes them **supportable**, not mandatory
  substrate deliverables.

### Downstream (not this product / repository)

Actual games and game layers, including:

- Player control, characters, skeletal animation, game-specific presentation
- Combat rules, AI behavior, economy, building policy
- The System / LLM layer, spells, gas pricing, and other gameplay rules
- **Which** procedural or authored generation pipeline a game uses
- **Which** physics engine a game chooses
- Freeform ship and station games (and their design/combat fiction)—**future-
  consumer examples**, not current delivery or validation targets

Compatibility seams may be designed where substrate requirements demand them;
those layers are not implemented here.

---

## 5. Product behavior and capabilities

At product altitude, the substrate must **enable** and **guarantee** the
following. How each is implemented is design work; that something is true under
the public contract is product substance.

### Material authority and contracted access

- Occupancy, queries, collision truth, and persistence run against **voxel
  matter**.
- Meshes, dressing, and debug geometry are **derived and disposable**—never
  serialized as world truth, never the sole basis for gameplay-critical
  occupancy or collision.
- External consumers install the facade, inspect through public reads, mutate
  through admitted edits, and never require privileged internal paths.
- The same public boundary must serve validation harnesses and external game
  crates.

### Sparse scale and lifecycle

- Large continuous volumes remain tractable under sparsity: untouched homogeneous
  volume stays cheap; surface, voids, structures, and player scars pay detailed
  cost.
- Streaming and lifecycle keep residency bounded so regions need not hold full
  raw-voxel detail everywhere at once.
- Lazy materialization of truth is in scope as substrate storage behavior; *what
  fills* a newly touched volume is driven by consumer content strategy, not by a
  substrate-owned generator product.

### Mutation and volumetric depth

- **Everywhere mutation:** any material cell the contract exposes can be
  destroyed or placed; cut faces and scars remain honest matter; presentation
  rebuilds from truth.
- **First-class deep Z:** volume along the full depth axis is real content—
  genuine volumetric depth, not a heightmap floor with painted underground.
  Caves, strata, ore, aquifers as material bands, and buried structure are
  material volume, not skybox scenery.
- Contracts stay volume-general so non-planetary freeform volumes remain
  expressible later; ship/station interiors are future-consumer motivation, not
  a required current deliverable shape.

### Dynamic volumes and physics readiness

- The world is not static geometry alone. The substrate supports **voxel volumes
  that move and can take damage** under the same truth contracts as terrain, so
  future games can treat combatants as matter rather than overlays disconnected
  from the world.
- The substrate exposes **plug-in bindings and supportable material data**
  (strength, gravity, force, and related fields) so a physics engine can attach.
  Runtime physics simulation is **not** a substrate-owned product; a simple
  engine is an acceptable proof of the seams, not required delivery.
- When a physics engine is present, it consumes material truth through public
  bindings rather than a private mesh world.

### Persistence

- Persistence keeps material edits and related scars tractable—**cheap scars over
  full dumps**.
- A consumer may choose a reproducible generation function (or authored content)
  as its base-world strategy; that choice is game-dependent and is **not** a
  substrate deliverable.

### GPU-resident architecture (product direction)

- Sparse brick-oriented storage and a command/query boundary keep world
  representation **GPU-resident** and support **asynchronous GPU work**.
- This is a deliberate product distinction from CPU-driven voxel engines:
  residency enables gameplay-scale mutation, meshing, and future simulation
  without abandoning the consumer contract.
- Specific kernels and simulations remain milestone-selected; residency and the
  async-capable boundary do not.
- Consumers must not depend on direct buffer access or synchronous readback of
  internal GPU storage; the public contract stays stable even if ownership of
  work moves between CPU and GPU.

### Presentation and objects

- Presentation derives surfaces and surface dressing from material truth.
- How a world “looks natural” depends on the consumer’s world (landscape
  geology, fortress masonry, and other material styles). The substrate must
  support fully material volumes that read as coherent for their domain; it does
  not mandate a single overworld aesthetic or a heightmap-with-props look.
- Object and clutter registration lets matter-backed assemblies (trees, rocks,
  micro-objects, and similar) participate as registered world matter rather than
  fake props that ignore the truth contracts. Downstream behaviors such as tree
  felling or rigid-body conversion of vegetation are **not** current product
  requirements.

### Measurable quality

- Benchmarks and harnesses can evidence mutation response, streaming, GPU memory
  behavior, collision-truth honesty, physics-binding readiness when exercised,
  and related contracts without redefining the product as a game.
- Harness-side generation or a proof physics plug-in used for tests or demos
  does not make generation or a physics engine a substrate requirement.
- The product does not claim a released, finished visual engine before
  feasibility and visual-acceptance gates are met.

---

## 6. Product-level outcomes

These are outcomes the substrate must enable—not a feature inventory or
implementation plan:

1. **Truth vs view.** Occupancy, queries, collision truth, and persistence run
   against voxel matter; meshes, dressing, and debug geometry are derived and
   disposable. Physics engines, when present, also consume material truth through
   public bindings rather than a private mesh world.
2. **Contracted consumption.** External consumers install the facade, inspect
   through public reads, mutate through admitted edits, and never require
   privileged internal paths.
3. **Sparse scale.** Large regions remain tractable: untouched homogeneous volume
   stays cheap; only the interesting shell and active edits pay detailed cost.
4. **Mutable everywhere.** Any material cell the contract exposes can be
   destroyed or placed; cut faces and scars remain honest matter, and
   presentation rebuilds from truth.
5. **Deep Z is first-class.** Volume along the full depth axis is real content—
   genuine volumetric depth, not a heightmap floor with painted underground.
   Contracts stay volume-general so non-planetary freeform volumes remain
   expressible; ship/station interiors are future-consumer motivation, not a
   required current deliverable shape.
6. **Dynamic voxel volumes.** The world is not static geometry alone. The
   substrate must support voxel volumes that move and can take damage so future
   games can treat combatants as matter under the same truth contracts.
7. **Physics-ready bindings, not a baked-in engine.** The substrate exposes
   whatever bindings and material data a physics engine needs to plug in—material
   strength, gravity, force, and related supportable fields—whether the consumer
   hand-rolls an engine or adopts one. Runtime physics simulation is not a
   substrate-owned product.
8. **Cheap scars over full dumps.** Persistence keeps material edits and related
   scars tractable. Base-world production strategy is game-dependent and not a
   substrate deliverable.
9. **GPU-resident architecture.** Sparse residency and an async-capable
   command/query boundary keep world representation GPU-resident for scale and
   gameplay-enabling work while preserving the consumer contract. Specific
   kernels and simulations remain milestone-selected.
10. **Measurable substrate quality.** Benchmarks and harnesses can evidence the
    contracts above without redefining the product as a game.

---

## 7. Invariants and constraints

These are binding product constraints (not implementation checklists):

| Invariant | Meaning |
| --- | --- |
| Substrate first | Moria is reusable world infrastructure; games are separate consumers |
| Public facade only | Adjacent harnesses and external games share the same interfaces; no privileged access path |
| Matter is authority | Derived presentation is never saved or treated as truth |
| Admitted mutation only | World edits enter through the public mutation contract |
| Collision reads matter | Occupancy and collision truth come from voxel authority, not disposable meshes |
| Volume-general contracts | Contracts must not assume gravity-aligned planetary terrain as the only world shape |
| Everywhere mutation | Exposed material is mutable; decoration-only “fake world” geometry is not the product model |
| Deep Z is real volume | Underground and full-depth content are material volume, not painted floors |
| Dynamic volumes in scope | Movable, damageable material volumes are a product capability class |
| Physics via bindings | Strength, gravity, force, and related fields are supportable; the engine is adjacent |
| Generation above substrate | Procedural/deterministic generation is not substrate product identity |
| GPU residency direction | GPU-resident sparse representation and async-capable boundary are binding product direction |
| Rust / Bevy consumers | Product form is a Rust/Bevy library ecosystem for crate consumers |
| No web/wasm target | Web / wasm is not a Product One or substrate target platform |
| Harness is not “done” | A walkable validation harness does not define product completion |
| Natural look is domain-dependent | Coherent material presentation for the consumer’s domain is required; a single natural-overworld mandate is not |

Engineering and milestone sequencing (which material properties land first, how
gravity is parameterized for non-planetary volumes, binding shape for force and
strength, exact sparse layouts, graphics backends, buffer formats) remain
design/TDD concerns, not vision-identity blockers—unless a later explicit human
decision elevates a specific engineering choice into product law.

---

## 8. Validation principles

Validation proves substrate contracts; it does not redefine the product.

- **Same interfaces.** Any walkable or visual validation harness must consume
  the public facade available to an external game crate.
- **Optional harness.** A walkable-world visual validation harness is a
  permitted adjacent artifact only—not mandatory current delivery that defines
  Moria’s completion or identity.
- **Proof over fiction.** Dig/place and collision-against-truth style exercises
  are valuable *as proofs* that matter is authoritative; they are not a mandate
  to ship a particular character, control scheme, curated postcard region, or
  content palette as substrate product.
- **Numbers without product redefinition.** Benchmarks may evidence mutation
  response, streaming, GPU residency behavior, collision-truth honesty, and
  physics-binding readiness when exercised. Machine-specific demo targets and
  example performance tables from seed material are **not** automatically
  substrate product law.
- **Proof engines optional.** A simple physics engine or harness-side generator
  may prove seams; shipping either is not required substrate delivery.
- **Feasibility before “finished engine” claims.** Do not claim a released,
  finished visual engine before feasibility and visual-acceptance gates are met.

Validation-harness content, third-person controllers, demo routes, and
machine-specific demo targets are **not** substrate requirements or mandatory
delivery for product completion.

---

## 9. Non-goals

Explicitly out of product identity and current substrate delivery:

- Shipping a game, game mode, progression loop, or game-rules stack in this
  product.
- Treating validation-harness content, third-person controllers, demo routes, or
  machine-specific demo targets as substrate requirements or as mandatory
  delivery for product completion.
- **Baking deterministic or procedural world generation into the substrate** as
  product identity. Generation algorithms are game-dependent and run on top of
  the substrate; consumers own them.
- **Baking in a hand-rolled or third-party physics engine** as product identity.
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
- Limiting the substrate identity to a Minecraft-style cube aesthetic, a single
  natural-overworld content palette, static scenery without movable material
  volumes, or heightmap terrain that only pretends to have depth.
- Assuming gravity-aligned planetary terrain as the only legal world shape in
  substrate contracts (volume-general contracts are required even though
  ship/station *content* is not current delivery).

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
  engines plugged in through substrate bindings without those engines living
  inside Moria.
- Freeform material ship/station volumes (including space-trading and combat
  fiction in that mold) as **nonbinding** motivation for everywhere mutation,
  first-class volumetric depth through multi-deck interiors, GPU-resident matter
  at combat and design scale, physics-ready material bindings under force, and
  truth-vs-view so damage and salvage stay honest—without importing fiction, UI,
  mission systems, freeform-hull *delivery*, or a substrate-owned physics stack
  into current Moria scope.

A “walkable world” proof shape (curated region, forest, ruin, dig-as-demo) may
help a validation consumer make substrate claims undeniable. That shape’s
content, controls, milestones, performance tables, and curated generation
pipeline remain **context for what the substrate might support**, not the
definition of the product itself.

---

## 11. Resolved human scope decisions

Preserve these decisions in meaning; do not reopen them in product design
without a new explicit human decision.

| ID | Question | Decision |
| --- | --- | --- |
| **Q1** | Is a walkable-world visual validation harness mandatory current delivery or only a permitted adjacent artifact? | **Adjacent artifact.** It may exist for validation; it does not define product identity or “done.” Moria is not limited to a Minecraft-style world. |
| **Q2** | Are natural-looking terrain, everywhere mutation, and first-class deep Z binding current outcomes? | **Everywhere mutation and first-class deep Z are binding.** Deep Z means genuine volumetric depth, not heightmap terrain. Natural-looking presentation **depends on the consumer’s world**—required as coherent material presentation for that domain, not as a single natural-overworld mandate. |
| **Q3** | Is GPU-resident / asynchronous-GPU-capable architecture binding current direction? | **Yes.** GPU residency is an important product feature: it enables many gameplay capabilities and is a core distinction between Moria and CPU-driven voxel engines. Specific simulations remain milestone-selected. |
| **Q4** | Multi-world freeform volumes: does current product identity include ship/station material volumes on the same contracts as natural geology? | **Contracts are volume-general** (must not assume gravity-aligned planetary terrain only). **Delivering or specifically validating ships and stations is not current scope**; they remain future-consumer examples. |
| **Q5** | Must the substrate support dynamic (moving, damageable) voxel volumes—not only static world geometry? | **Yes.** Players and enemies will be voxel volumes that move and can take damage; the engine must support that class of matter. |
| **Q6** | Is deterministic / procedural world generation part of the substrate product? | **No.** Generation is an algorithm that runs on top of the substrate and is game-dependent; it must not be baked into substrate identity. |
| **Q7** | Is matter physics (collision for moving entities, gravity, force for explosions/crumbling) in product scope? | **Bindings yes; engine no.** The substrate exposes plug-in bindings and supportable material data (strength, gravity, force, etc.) so a physics engine can attach—hand-rolled or third-party. A full or hand-rolled physics engine is **not** baked into the substrate; it is **100% adjacent**. An acceptable proof of the bindings is a simple physics engine, but shipping one is not required. Collision/occupancy **truth** against voxel matter remains a substrate concern so plug-ins and consumers share one material world. |

### Confirmed assumptions (approved with the scope boundary)

1. **Authority order stands.** Project boundary and the approved vision are
   binding for product identity; walkable-world and broad architecture
   references inform capabilities and seams only.
2. **Harnesses are adjacent.** Curation, bench, and demo executables may ship
   in-repo to prove contracts; they do not redefine Moria as a game or as “the
   walkable demo.”
3. **Volume-general contracts without ship/station delivery (affirmed).**
   “World” means continuous material volume in general—not only planetary
   terrain—so freeform volumes can share the same contracts later. Current
   product does not deliver or specifically validate ships and stations.
4. **Generation above the substrate (affirmed).** Any in-repo or example
   generator exists to exercise or demo contracts; it does not establish
   generation as a required substrate deliverable.
5. **Physics via bindings, engine adjacent (affirmed).** Material strength,
   gravity, force, and related fields are supportable through exposed substrate
   bindings. A physics engine is not baked into Moria; a simple proof engine is
   optional and adjacent.

---

## 12. Open product-boundary questions

None currently open that would change product identity, purpose, or boundary.

Remaining work is engineering and milestone sequencing (for example which
material properties land first, how gravity is parameterized for non-planetary
volumes, and the concrete binding shape for force and strength). Those are
design/TDD concerns, not vision-identity blockers.

Older planning language that titles the effort as “Product One — The Walkable
World” is superseded on identity by the boundary and this vision. Engineering
docs that still list deterministic generation under the public facade describe
implementation or harness practice; this vision treats generation as **not**
substrate product identity per Q6. Seed language that listed structural
integrity / cave-ins or rigid conversion as non-current remains context for
future consumers; this vision requires **physics-ready bindings** (Q7) without
importing a substrate-owned physics engine, fortress span tables, or fortress
UI. Full fluid CA, fire/growth CA, and tree-felling remain non-goals until
explicitly selected.

---

## 13. Provenance (not substitutes for the synthesis above)

| Source | Role in this synthesis |
| --- | --- |
| `docs/vision.md` | **Authoritative scope boundary.** Purpose, ownership, outcomes, non-goals, resolved decisions Q1–Q7, and confirmed constraints are preserved in meaning throughout this document. |
| `docs/seeds/README.md` | Authority order among seeds: project boundary first; GPU-resident note second as supporting principles (residency/async boundary elevated by human decision); broad voxel reference third; Product One seed last as validation example. Conflicts resolve toward the boundary. |
| `docs/seeds/project-boundary.md` | Binding product target: reusable substrate consumed through public Rust APIs; walkable executable only as validation harness on public interfaces; game/System/LLM/spell/gas/combat/AI/building layers out of scope. |
| `docs/seeds/gpu-resident-substrate.md` | Supporting architecture principles: sparse GPU-oriented residency, public command/query/event boundary, derived meshes not truth, no privileged storage access. Implementation details (atomic widths, named backends, buffer formats) stay design-level unless the scope boundary elevates them—which it does not beyond residency and the async-capable boundary. Optional CA/particles/scripting/LLM/gas/player systems remain excluded or nonbinding. |
| `docs/seeds/voxel-world-substrate.md` | Architecture reference motivating coherent domain presentation, everywhere mutation, deep Z, substrate-not-game layering, GPU-resident direction, object/clutter distinction, persistence-as-scars, and streaming. Generation pipelines, fluid tiers, weather, integrity simulation, building UI, nav/agent labor, and game examples inform motivation only where the scope boundary selects the underlying product capability. |
| `docs/seeds/product-one-seed.md` | Downstream / validation example of a fully material walkable proof shape. Supplies motivation for smooth material presentation, dig/place as proof, geology depth, sparse streaming, seed+delta-style scar persistence, collision against truth, and measurable quality. Does **not** import character, controls, curated region, content palette, performance tables, milestones, or generation pipeline into product identity. |
| `docs/seeds/system-substrate-pivot.md` | Excluded-source notice only. Contributes no product requirements. |

When seed documents conflict, the project boundary and approved vision win:
substrate product first; game examples and Product One detail are nonbinding
unless selected by that boundary or an explicit human decision.
