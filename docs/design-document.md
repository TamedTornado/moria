# Moria product design

## 1. Product definition

Moria is reusable voxel-world infrastructure for Rust and Bevy consumers. A
game or tool installs Moria, supplies material content, and interacts with that
content through one public consumer contract. Moria owns material truth,
bounded access to it, admitted changes, lifecycle, persistence seams,
collision/occupancy truth, and presentation derived from that truth.

The product is a substrate, not a game. It does not supply a player, camera,
world-generation recipe, physics engine, damage model, progression system, or
game-specific content. Validation executables may accompany the substrate, but
they are ordinary consumers of the same contract and do not define the
product's identity or completion.

Moria's central promise is:

> A consumer can host sparse, continuous three-dimensional material volumes;
> inspect, move, and edit them without privileged storage access; and rely on
> queries, collision, persistence, and presentation to refer to the same
> authoritative matter.

Authoritative matter is designed to remain GPU-resident at scale. Consumer
interaction therefore uses an asynchronous-capable command, query, observation,
and telemetry boundary rather than direct storage or buffer ownership.
Residency is a performance direction: it must let Moria's owned work, and
downstream behavior engines where feasible, operate without making a full CPU
mirror or synchronous readback the normal route to world truth. It does not
make those behavior engines or their policies part of Moria.

## 2. Design principles

### 2.1 Matter is authority

**Requirement: REQ-001. Authority: C-003, C-016.**

A material volume is the source of truth. Surface meshes, dressing, collision
debug shapes, slice views, and other visualizations are disposable projections
of a known matter revision. They may be rebuilt or absent without changing
what exists.

Collision and occupancy never infer truth from presentation. Persistence never
saves derived presentation as truth. A visual result that cannot survive an
edit and honest rebuild does not satisfy the product.

### 2.2 One contract for every consumer

**Requirement: REQ-002. Authority: C-003, C-013.**

An external game, editor, benchmark, and in-repository demo all:

- configure the same public facade;
- inject base material content through the same content seam;
- request bounded readiness and inspection;
- submit the same admitted commands;
- observe the same lifecycle and change notifications; and
- receive the same errors and telemetry.

No validation mode may bypass admission, inspect internal storage, mutate a
private CPU mirror, or treat a generated mesh as the world.

### 2.3 Volume, not terrain

**Requirement: REQ-003. Authority: C-002, C-006, C-007, AD-004, AD-005.**

The basic addressable object is a material volume, not a heightmap or a
gravity-aligned landscape. A volume has stable identity, its own local
three-dimensional address space, an extent or addressable domain, and a
placement in a containing world. Static landscape is one use of this model.
Movable material bodies and constructed interiors use the same model.

Nothing in the consumer contract assigns a privileged "up" axis, surface
height, sea level, ground plane, or planetary meaning. Consumers may add those
interpretations.

### 2.4 Sparse cost follows interest and change

**Requirement: REQ-004. Authority: C-004.**

Empty space and untouched homogeneous matter remain cheap. Detailed cost is
paid for materially interesting boundaries, voids, registered assemblies, and
scars, and only while consumer interest or unfinished work requires them.

The product makes this behavior observable through lifecycle and telemetry. It
does not promise an unbounded world with hidden unbounded residency.

### 2.5 Asynchrony is explicit

**Requirement: REQ-005. Authority: C-010, AD-003.**

Submitting work is distinct from completing work. A consumer never has to
assume that a GPU operation completed synchronously or that a readback is
immediately available. Commands, queries, derived presentation, and
persistence checkpoints expose their progress, completion, revision, and
failure rather than hiding delay behind direct storage access.

When an external behavior engine already operates on the GPU, this boundary
must support a bounded, authorized path for observing matter and requesting
effects without making a full CPU mirror or synchronous CPU readback the normal
coordination route. GPU compatibility does not grant direct ownership of
Moria's storage: bounds, admission, revisions, completion, and failure remain
the same public contract regardless of where the participating work runs.

### 2.6 Behavior remains external

**Requirement: REQ-006. Authority: C-008, AD-007.**

External systems may observe matter and lifecycle, perform any computation they
own, and request admitted changes. Moria does not interpret those changes as
gravity, force, fracture, damage, health, growth, fire, fluid flow, or another
behavior vocabulary. The substrate supplies truth and controlled effects, not
the rule that decides an effect should occur.

### 2.7 Correct first, then optimize continuously

**Requirement: REQ-007. Authority: C-015.**

Correct material truth, atomic mutation, honest readiness, and explicit failure
come before performance claims. Optimization may change where or how work runs,
but it may not weaken those public semantics, introduce a second world, or make
unknown matter appear resolved.

Within that constraint, Moria is designed for relentless, measured
optimization. GPU residency exists to make large-scale world work fast, not as
an architectural ornament. Generic extension seams must support the intended
performance shape in which downstream physics and other behavior engines run
primarily on the GPU when feasible, observe Moria truth without privileged
storage ownership, and return requested effects through normal admission. A
consumer may still choose a CPU-oriented engine; behavior placement and policy
remain external.

## 3. Consumer mental model

Moria exposes the following product concepts. Their precise Rust types and
internal representations belong to technical design.

### World

A world is the consumer-facing container for registered materials, material
volumes, content sources, lifecycle, scars, and observation. It provides the
scope in which identities and revisions are meaningful.

### Material

A material definition gives matter stable identity and the information Moria
needs for its owned responsibilities: whether and how it contributes to
occupancy and surface presentation, plus domain-appropriate presentation
inputs. Consumers may associate their own opaque metadata with a material, but
Moria does not standardize behavioral fields such as hardness, resistance,
wetness, temperature, flammability, health, or damage.

Empty space is distinguishable from occupied material. Material truth also
contains enough shape or coverage information to derive honest boundaries and
edited cut faces without requiring cube rendering as the product look. The
encoding and precision of that shape information are technical choices.

### Material volume

A material volume is a stable, addressable body of material truth. Static and
dynamic volumes have the same inspection, mutation, persistence, and
presentation guarantees.

A dynamic volume additionally has an admitted placement that may change over
time. Moving a volume changes where its matter is encountered; it does not
rewrite the volume's local material content. Movement has no built-in cause or
response. An external behavior system may decide where a volume moves and
request the placement change.

When volumes overlap, inspection preserves the identities of all encountered
volumes rather than silently merging them or choosing game-specific contact
behavior. A consumer or behavior plug-in decides what overlap means.

### Base content source

A base content source supplies matter for bounded parts of a volume when Moria
needs to materialize them. It may read authored content, import content, run a
consumer-owned generator, or combine those approaches. Moria does not know or
prescribe the algorithm.

Every base source declares a stable content lineage that can be compared with
persisted scars. For matter a checkpoint does not carry, the source must be
able to reconstruct the same base content for that lineage, or the consumer
must durably supply that base content alongside the scars. A changed or
non-reconstructable base requires a new lineage and explicit migration or
rebase. This prevents edits made against one base world from being silently
replayed onto a different one while still allowing untouched homogeneous
volume to remain cheap.

### Scar

A scar is a durable change relative to consumer-supplied base matter. It
includes admitted matter edits and the minimum substrate-owned world state
needed to reconstruct changed material volumes and their placements.
Consumer-owned simulation or gameplay state remains the consumer's persistence
responsibility.

### Revision

A revision identifies a committed state of a volume. Successful mutations and
placement changes advance the relevant revision. Query results, observations,
presentation status, and persistence checkpoints identify the revision they
describe, allowing consumers to reason about freshness without seeing storage.

Revisions express causality and freshness; their concrete representation and
cross-volume coordination are technical concerns.

### Interest

Interest is a bounded consumer request to make a portion of one or more volumes
ready for stated uses, such as inspection, collision, or presentation. An
interest source may represent a camera, editor selection, agent, benchmark
route, or background tool. Moria does not assume that interest is camera-shaped
or tied to a single player.

### Receipt and observation

A receipt lets a consumer follow admitted asynchronous work to completion. An
observation reports a committed change or lifecycle transition through a
bounded public channel. Observations are facts about Moria-owned state, not
instructions to an external behavior system.

## 4. Consumer journey

### 4.1 Configure

**Requirement: REQ-008. Authority: C-001, C-003.**

Before a world becomes usable, the consumer:

1. installs the Moria facade in its Bevy application or tool;
2. registers material definitions and presentation inputs;
3. registers one or more static or dynamic material volumes;
4. supplies a base content source for each volume that needs one;
5. selects persistence, presentation, resource-budget, and telemetry policies;
   and
6. starts the world and observes whether configuration becomes ready or fails.

Missing material identity, invalid volume domains, incompatible content
lineage, or absent required collaborators fail configuration explicitly. Moria
does not substitute a default overworld, hidden generator, or all-empty world.

### 4.2 Declare interest

**Requirement: REQ-009. Authority: C-004.**

The consumer declares bounded regions and the capabilities needed there.
Moria acknowledges the request, begins or prioritizes materialization, and
reports lifecycle changes. Readiness may arrive later and may differ between
authoritative matter and its derived presentation.

The consumer can change or withdraw interest. Withdrawal makes a region
eligible to become cold after in-flight work and persistence obligations are
safe; it does not immediately invalidate already issued results or discard
unsaved scars.

### 4.3 Inspect

**Requirement: REQ-010. Authority: C-003, C-016.**

The consumer submits a bounded query against a stated world or volume scope.
The result identifies:

- the bounds and volumes actually inspected;
- the committed revision or revisions observed;
- material and occupancy facts requested by the query;
- whether the result is complete, partial by explicit request, pending, or
  unavailable; and
- any lifecycle or budget condition preventing completion.

Moria never reports unloaded, failed, or unknown matter as empty. A query whose
bounds exceed the supported request contract is rejected rather than silently
clipped, unless the consumer explicitly asked for a partial result.

The product supports the inspection intents needed to consume material truth:

- sample matter at an addressed location;
- inspect a bounded three-dimensional region;
- determine occupancy in a point, region, or consumer collision shape;
- trace through matter to find ordered material encounters; and
- test overlap or swept movement against authoritative matter.

The exact primitive set, precision, and acceleration methods are selected in
technical design, but they must cover these intents without routing through a
render mesh.

### 4.4 Mutate matter

**Requirement: REQ-011. Authority: C-005, AD-002.**

All edits enter through public commands. The core design includes:

- remove or erode matter from a bounded target;
- place or replace material in a bounded target;
- apply a consumer-supplied bounded material patch or stamp;
- create or retire a material volume; and
- change the placement of a dynamic volume.

These are substrate effects, not game verbs. Moria does not price them, check
player inventory, infer damage, animate tools, or decide whether a game should
allow them.

Admission validates that the target and material identities exist, the request
is bounded and structurally valid, required truth can be made available, and
any supplied revision precondition still holds. The immediate outcome is
either:

- **rejected**, with a stable reason and no effect; or
- **admitted**, with a receipt for pending work.

An admitted command later reaches one terminal outcome:

- **applied**, with affected bounds and the new committed revision; or
- **failed**, with no committed effect and a reason the consumer can act on.

A single bounded matter-mutation command is one atomic public operation,
including when it affects multiple cells. All of its targeted changes become
visible together at one committed revision, or none of them do. Admission and
internal work may be staged, but queries, collision, observations,
persistence, and presentation never observe a partially committed command. A
consumer that wants independent success or failure submits separate commands.

Pending edits are not visible as committed truth. Once applied, public queries,
collision truth, scars, observations, and eventual presentation all converge on
the same new revision. Consumers can attach correlation metadata so an
external system can match an observation to the request that caused it.

Revision preconditions let editors and behavior systems detect stale decisions
instead of overwriting newer matter. Moria reports a conflict; it does not
silently retry a command whose meaning may have changed.

### 4.5 Observe and react

**Requirement: REQ-012. Authority: C-003.**

Consumers and external behavior plug-ins can subscribe to bounded observations
for:

- committed matter changes and their affected bounds;
- volume creation, retirement, and placement changes;
- material-region lifecycle transitions;
- presentation becoming current or failing; and
- persistence checkpoint completion or failure.

Observation delivery is finite and backpressure-aware. If a consumer falls
behind far enough that events cannot be retained, Moria reports an explicit
gap and the last trustworthy revision. The consumer then obtains a bounded
snapshot and resumes. Silent event loss is forbidden.

An observing plug-in receives no privileged voxel ownership. If it wants to
change matter or move a volume, it submits an ordinary admitted command and
receives the same validation and failure behavior as any other consumer.

A GPU-oriented plug-in follows those same rules. Its bounded observation and
effect handoff may remain GPU-oriented where feasible, but admission and
terminal outcomes stay visible to the owning consumer through receipts,
revisions, observations, and errors. A faster path may not bypass validation,
make pending work appear committed, or introduce a second copy of world truth.

### 4.6 Present

**Requirement: REQ-013. Authority: C-011.**

Moria derives visible surfaces and optional dressing from committed matter.
Presentation supports both coherent organic volumes and crisp constructed
forms without making one material domain the universal aesthetic. Material or
volume presentation inputs may express different surface character; the
substrate is not limited to raw cubes, nor does it mandate smooth geology
everywhere.

Presentation has its own observable status:

- **absent** — no view is requested or available;
- **building** — a view for a committed matter revision is in progress;
- **current** — the view represents the reported current revision;
- **stale** — a valid older view exists while newer matter awaits rebuilding;
  or
- **failed** — view derivation failed while material truth remains intact.

A stale view may remain visible to avoid a needless visual hole, but it is
never used for collision or reported as current. Consumers can choose whether
their experience displays stale presentation, a diagnostic fallback, or
nothing. A mutation validation capture is complete only after presentation
reports the mutation's revision as current.

The product distinguishes:

- **matter-backed assemblies**, such as vegetation, rocks, or constructed
  pieces that a consumer wants to query and mutate as material; and
- **derived dressing**, such as grass blades or surface scatter with no
  independent material identity.

Matter-backed assemblies register through the material-volume/object seam and
obey ordinary truth and mutation rules. Derived dressing remains anchored to a
specific matter revision and surface. When its supporting material changes,
the dressing is regenerated or removed; it cannot survive as a disconnected
prop that claims occupancy.

### 4.7 Persist and restore

**Requirement: REQ-014. Authority: C-009.**

The consumer requests a checkpoint at known committed revisions. A successful
checkpoint records substrate-owned scars and reconstruction state without
serializing derived geometry or requiring a raw dump of untouched homogeneous
volume. Its completion identifies exactly which committed revisions are
durable. Mutations committed after those revisions remain dirty for a later
checkpoint; they are not silently included or lost.

Restore combines the consumer's compatible base content lineage with the saved
scars. It restores static and dynamic material edits, persistent volume
identity, and volume placement needed to reconstruct the same material truth.
Moria reports the restored revision context before dependent consumers resume.
If exact base reconstruction cannot be established, restore fails rather than
claiming that lineage compatibility alone proves equality.

The following are explicit restore failures, never implicit empty or best-effort
success:

- corrupt or incomplete scar data;
- an unsupported saved product contract;
- missing material identities;
- incompatible base content lineage; or
- a required volume or content source that cannot be reconstructed.

Migration, rebasing scars onto changed base content, and persistence of
external behavior state require explicit consumer action. Moria does not guess.

### 4.8 Shut down

**Requirement: REQ-015. Authority: C-003.**

Shutdown stops new admissions, allows the consumer to observe or cancel
outstanding noncommitted work according to the public contract, and reports
whether required persistence completed. Dirty material is not silently
discarded because a region became cold or the application began shutdown.

## 5. Lifecycle and consistency

### 5.1 Material-region lifecycle

**Requirement: REQ-016. Authority: C-004.**

For a bounded region of a volume, authoritative matter moves through these
consumer-visible states:

| State | Meaning |
| --- | --- |
| **cold** | No detailed authoritative representation is currently ready; base lineage and durable scars remain known. |
| **requested** | Consumer interest or pending work requires the region. |
| **materializing** | Base content and scars are being combined into authoritative matter. |
| **ready** | Queries, collision, and admitted mutation can use the reported revision. |
| **retiring** | Interest has ended and in-flight, observation, or persistence obligations are being resolved. |
| **failed** | Authoritative matter could not be made ready; the cause and retryability are reported. |

Lifecycle is not a promise about a particular storage tier. "Ready" describes
what the consumer may safely do, while CPU/GPU ownership and allocation remain
internal.

Presentation readiness is reported separately. A region can have ready truth
and building or failed presentation. It must never have presentation treated as
truth when authoritative matter is unavailable.

### 5.2 Revision rules

**Requirement: REQ-017. Authority: C-003, C-005.**

- Queries describe committed revisions only.
- A mutation completion identifies the one revision at which all effects of
  that command commit atomically.
- An observation for a change is emitted only after that change commits.
- Collision results identify or are correlated with the committed matter
  revision used.
- Presentation and persistence identify the revisions they cover.
- A consumer may require a revision precondition for a command or a minimum
  revision for a query.
- No ordering is implied between independent worlds or volumes unless the
  public operation explicitly establishes it.

These rules let a consumer build responsive experiences without treating
asynchronous completion as nondeterministic truth.

### 5.3 Residency and resource pressure

**Requirement: REQ-018. Authority: C-004, C-010.**

Consumers express bounded interest, priority, and needed capability; Moria
decides how to meet them within configured resource budgets. Under pressure it
may delay lower-priority materialization, retire eligible regions, or reject
new work. It must report which action occurred and why.

Moria may not:

- evict matter still required by an admitted operation;
- discard an unpersisted scar without explicit consumer authorization;
- return unknown matter as empty to satisfy a deadline; or
- make a derived view current by relabeling an older revision.

Telemetry lets a consumer understand active interest, lifecycle distribution,
authoritative residency, derived-view cost, queue pressure, and failed work
without exposing internal storage.

## 6. Collision and dynamic-volume behavior

**Requirement: REQ-019. Authority: C-007, C-016, AD-005.**

Moria owns collision and occupancy **truth**, not motion policy.

Collision inspection:

- tests authoritative occupied matter for all relevant static and dynamic
  volumes;
- preserves volume identity, material identity, location, and surface/contact
  facts needed by a consumer;
- uses each volume's admitted placement for the tested revision;
- distinguishes no hit from unavailable truth; and
- remains valid when presentation is absent, stale, or rebuilt differently.

Moria does not apply gravity, integrate velocity, separate overlapping bodies,
choose friction or restitution, deal damage, fracture matter, or convert matter
to debris. An external system may use collision facts to decide on motion or
edits, then request a volume placement or matter mutation through the normal
contract.

Movement and material editing are independent. A dynamic volume can move
without resampling its local matter, and it can be edited while retaining its
identity. Conflicting movement and edit requests use revisions and explicit
completion rather than a hidden privileged order.

## 7. Content and presentation boundaries

**Requirement: REQ-020. Authority: C-011, C-012, C-013.**

Moria supports consumer-owned content without owning a content recipe.

The content seam must allow a consumer to provide:

- homogeneous empty and solid regions cheaply;
- arbitrary genuine three-dimensional matter, including voids and internal
  structure at any depth;
- material boundaries suitable for organic or constructed presentation;
- static and dynamic volumes;
- matter-backed assemblies; and
- authored, imported, stamped, or generated bounded content.

Natural geology—strata, caves, ore, aquifer bands, and buried structures—is a
useful validation content family because it proves deep volume and honest cut
faces. It is not a built-in generation pipeline or mandatory palette. A
fortress wall, sculpted object, or other consumer domain may demonstrate the
same substrate outcomes.

Moria may offer neutral registration and injection examples, but no example
becomes a default world law. In particular, there is no required continent,
biome, river, climate, vegetation, ruin-placement, or deterministic seed
algorithm.

## 8. Failure behavior

**Requirement: REQ-021. Authority: C-003, C-015.**

Failures are part of the public experience and must preserve truth.

| Condition | Required product behavior |
| --- | --- |
| Invalid configuration | World startup fails with actionable validation; no partial hidden world starts. |
| Content source is temporarily unavailable | The affected region remains pending or becomes failed according to the declared retry condition; it is not treated as empty. |
| Content source returns invalid content | The affected region becomes failed with the invalid scope identified; invalid matter is not admitted as truth. |
| Query is too large or outside the addressable domain | Reject with supported bounds; do not silently clip unless partial results were requested. |
| Query needs cold matter | Return pending/availability status or materialize under declared interest; never fabricate a result. |
| Mutation uses missing material or volume identity | Reject with no effect. |
| Mutation precondition is stale | Reject as conflict and report current revision context. |
| Admitted mutation work cannot complete | Finish the receipt as failed with no portion of the command committed. |
| Other admitted work cannot complete | Finish the receipt as failed, identify whether any committed revision changed, and never leave success unreported. |
| Resource budget is exhausted | Defer, retire eligible work, or reject according to policy and expose pressure in telemetry. |
| Derived presentation fails | Keep authoritative matter usable, report failed presentation, and permit retry or diagnostic fallback. |
| Observation history is lost | Report an explicit gap and require a bounded resnapshot. |
| Persistence fails | Retain dirty truth where possible, report that it is not durably checkpointed, and block silent discard. |
| Save/base lineage is incompatible | Fail restore pending explicit migration or rebase; never replay scars speculatively. |
| External behavior plug-in fails | Moria truth remains valid; only commands already admitted through the normal contract may affect it. |

Errors identify their scope, retryability, and whether any committed revision
changed. Human-readable diagnostics and machine-actionable categories describe
the same condition.

## 9. Telemetry and diagnostics

**Requirement: REQ-022. Authority: C-015.**

Public telemetry makes scale and honesty reviewable without becoming a storage
escape hatch. It covers:

- world and volume readiness;
- interest and material-region lifecycle counts;
- authoritative and derived residency at a consumer-meaningful level;
- command admission, queueing, completion, conflict, and failure;
- query bounds, readiness, completion, and failure;
- current, stale, building, and failed presentation;
- revision lag between truth and presentation;
- checkpoint progress, durable revision coverage, and restore failure;
- observation backlog and gaps;
- behavior-extension boundary activity and consumer-meaningful transfer or
  readback pressure when that boundary is exercised; and
- resource-pressure decisions.

Diagnostics may add raw-voxel, volume-boundary, lifecycle, revision, and
streaming visualizations. They are derived consumers of the public contract.
They do not gain mutation or storage privileges and are not product authority.

Measurements include machine and configuration context so evidence from
different systems can be compared without turning one machine's target into a
universal product requirement.

## 10. Validation experience

**Requirement: REQ-023. Authority: C-013, C-015.**

Moria is ready for review when it can prove its contracts through ordinary
consumers. The validation suite is scenario-based and fail-closed: missing
evidence is reported as not demonstrated, never inferred from a visually
plausible scene.

### Public-boundary proof

An external-style consumer configures a world, supplies content, declares
interest, queries, mutates, observes, checkpoints, and restores without an
internal path. The same scenario is reusable by in-repository harnesses.

### Truth-versus-view proof

With presentation present, absent, deliberately stale, and rebuilt, occupancy
and collision return the same matter facts for the same revision. Derived
geometry is discarded and regenerated without changing world truth.

### Mutation honesty proof

The consumer removes and places matter at exposed locations, including across
material boundaries and deep inside a volume. Completion advances revisions;
queries and collision observe the edit; presentation becomes current with
honest cut and placed surfaces; derived dressing updates with its support.
Multi-cell commands are also forced to fail after admission in validation: no
targeted cell changes, no mutation revision commits, and no observer sees an
intermediate subset.

### Deep-volume proof

Consumer-supplied content contains reachable voids, bands, and structures
through substantial three-dimensional depth. Inspection and edits operate
throughout that depth. A heightmap with decorated underground surfaces cannot
pass.

### Sparse-scale and lifecycle proof

A content domain large enough that raw detailed residency would be
unreasonable remains bounded under interest changes. Homogeneous untouched
regions stay cheap, cold regions materialize on demand, and scars survive
retirement. Evidence reports resource use and lifecycle behavior.

### Persistence proof

A consumer checkpoints an edited static volume and an edited, moved dynamic
volume without saving derived presentation or dumping untouched homogeneous
matter. Checkpoint completion identifies its durable revisions while a later
mutation remains explicitly dirty. Restore against the reconstructable base
lineage reproduces the same material, volume identities, and placements.
Missing, incompatible, or non-reconstructable base content fails restore
without speculative scar replay.

### Dynamic-volume proof

A material volume is queried and collided with, moved through an admitted
placement change, edited in local matter, checkpointed, and restored. It keeps
stable identity; no physics or damage model is required to demonstrate the
contract.

### Behavior-extension proof

An optional minimal external plug-in observes a committed change, inspects
bounded truth, and submits an ordinary mutation or movement request. It owns
the reason and all behavior state. Removing the plug-in removes the behavior
without removing or changing Moria's material vocabulary.

When this proof exercises a GPU-oriented behavior engine, it also shows that
bounded observation and effect handoff can remain GPU-oriented where feasible
without changing admission, atomic completion, revision, or failure semantics.
A CPU-oriented proof can establish semantic compatibility, but by itself does
not evidence the intended GPU-performance path.

### Failure proof

Validation deliberately exercises cold queries, invalid bounds, stale
preconditions, content-source failure, presentation failure, observation gaps,
resource pressure, and incompatible persistence lineage. Each fails in the
documented state without becoming empty matter, losing a scar, or exposing
storage.

### Quality evidence

Benchmarks record mutation-to-commit time, commit-to-current-presentation time,
query responsiveness, lifecycle transitions, authoritative and derived
residency, checkpoint/restore behavior, and collision-truth agreement with
machine context. When the behavior-extension boundary is exercised, evidence
also reports its handoff latency and consumer-meaningful transfer or readback
pressure. Correctness scenarios must pass before such measurements support an
optimization claim. Target thresholds and curated routes belong to the
validation plan and technical design; this product design requires the
measures and honest status, not seed-specific numbers.

A walkable third-person scene may make several proofs easy for a human to see,
but its character, controls, camera, route, palette, assets, generator, and
performance target remain harness content. No walkable harness is required to
declare the substrate itself complete.

## 11. Scope reconciliation

**Requirement: REQ-024. Authority: C-012, C-014, AD-001, AD-004, AD-006, AD-007.**

The subordinate design sources contain useful ideas at mixed levels of
authority. This design resolves them as follows.

### Selected into the product design

- Cheap representation of homogeneous empty and solid volume, sparse detail,
  and interest-driven materialization are required outcomes.
- A command/query/observation boundary hides GPU-resident storage and
  asynchronous completion from consumers without hiding state.
- GPU residency is selected for performance. The generic extension experience
  supports downstream behavior engines that keep their work primarily on the
  GPU when feasible, without giving them voxel ownership or importing their
  behavior into Moria.
- Correctness and explicit failure precede optimization; measured optimization
  continues behind invariant public semantics.
- Surface presentation, dressing, debug views, and collision visualizations are
  derived from matter revisions and are never authority.
- Presentation can express smooth organic surfaces and sharp constructed
  surfaces; raw voxel display remains a valid diagnostic, not the mandated
  look.
- Deep three-dimensional content and everywhere mutation are validated with
  honest cuts, not heightmap illusion.
- Collision and occupancy read material truth; motion and contact response are
  external.
- Interest may come from cameras, agents, tools, or background work, and may
  keep non-camera regions active.
- Base content plus sparse scars is the persistence experience; derived
  geometry is rebuilt.
- Matter-backed assemblies and derived surface dressing are separate,
  truth-preserving choices.
- Comparable benchmarks and human-visible proofs report missing evidence
  honestly.

### Excluded by the approved boundary

- Any built-in deterministic or procedural geology, biome, river, cave, ore,
  aquifer, vegetation, ruin, or other world-generation pipeline.
- The Product One region, material palette, third-person controller, camera,
  debug keys, curated route, milestones, machine targets, and release/demo
  positioning as Moria requirements.
- Physics, damage, fracture, strength, gravity, force, health, resistance,
  contact response, rigid conversion, re-voxelization, and debris rules.
- Cellular automata for fire, wetness, growth, granular settling, fluid
  simulation, weather, seasons, or structural collapse.
- Building gameplay, blueprints as player UI, mechanisms, rooms, navigation,
  work orders, economy, agents, combat, AI, spells, gas pricing, System/LLM
  behavior, multiplayer services, and game scripting policy.
- Current delivery or specific validation of ships, stations, multi-deck
  freeform hulls, or their game fiction. They motivate volume-general
  contracts only.
- Web/wasm as a current target and any claim of a finished visual engine before
  feasibility and visual acceptance are demonstrated.

### Deliberately deferred to technical design

The product outcomes do not select:

- crate count or source layout;
- spatial indexing, brick dimensions, voxel resolution, allocation, palette,
  payload, aggregate, or coordinate encodings;
- CPU/GPU work ownership, kernels, atomics, buffers, synchronization, or
  readback mechanisms;
- direct rendering, surface nets, dual contouring, another surface technique,
  or a measured hybrid;
- LOD, distant presentation, dressing generation, or object acceleration
  technique;
- exact collision primitives or collision-search algorithms, provided the
  inspection intents and truth guarantees above are met;
- cache, streaming-ring, eviction, compression, journal, or persistence
  formats;
- graphics backend details and machine-specific optimization; or
- acceptance thresholds, benchmark scenes, milestone order, and task
  decomposition.

Technical design should retain alternatives long enough to measure them
against GPU residency, portability, bounded access, sparse scale, presentation
quality, and mutation latency. No technical selection may add a privileged
consumer path or elevate a game-specific behavior into substrate policy.

## 12. Resolved human design decisions

**Requirement: REQ-025. Authority: C-005, D-001.**

**Multi-target mutation completion is atomic.** One bounded matter-mutation
command commits all targeted cells together at one revision or commits none of
them. Partial application is not a supported public outcome. This selects the
consumer-visible semantic only; the staging and coordination that realize it
remain technical-design concerns.

No consumer-visible human decision remains open in this design.

## 13. Completion criteria

The product design is realized when a Rust/Bevy consumer can, through the
public facade alone:

1. supply non-heightmap three-dimensional content for static and dynamic
   volumes;
2. keep large homogeneous regions cheap and materialize bounded interest;
3. inspect committed truth with explicit bounds, readiness, and revision;
4. remove, place, and patch matter through admitted asynchronous commands that
   commit atomically per command;
5. move and edit dynamic volumes without importing motion policy;
6. collide against material truth while presentation is independently rebuilt;
7. observe changes and recover explicitly from observation gaps;
8. derive coherent organic and constructed presentation plus truth-anchored
   dressing;
9. checkpoint and restore scars against compatible, reconstructable
   consumer-owned base content with explicit durable revision coverage;
10. attach an external behavior plug-in without privileged storage or a
    substrate-owned behavior vocabulary, while permitting a bounded
    GPU-oriented observation and effect path where feasible; and
11. measure and fail-close each claim, including adverse lifecycle and
    persistence cases.

Passing one curated demo or meeting one machine's performance table is not a
substitute for these contract outcomes.
