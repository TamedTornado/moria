# Moria product design

## 1. Design intent

Moria is a reusable voxel-world substrate for Rust and Bevy consumers. It gives
a game or tool one contracted material world: continuous three-dimensional
volumes that can be sparse, streamed, inspected, changed, persisted, presented,
and used for collision without any consumer gaining private access to the
underlying voxel storage.

The central experience is not a particular landscape or demo. It is the
confidence that every supported view of the world agrees:

- an inspection result describes the same matter that mutations change;
- collision and occupancy consult that matter rather than a render mesh;
- a save records that matter and its scars rather than disposable geometry;
- surfaces and dressing are rebuilt from that matter;
- a physics plug-in can consume the same public truth without owning it; and
- static and moving voxel volumes follow the same material rules.

Moria is engine-shaped infrastructure, not a game. It supplies the behavior and
evidence described here through a public consumer facade. Controllers, cameras,
world-generation algorithms, game rules, physics simulation, combat, AI, and
authored demo content remain consumers of that facade.

## 2. Product boundary

### 2.1 Moria owns

Moria owns the following user-visible substrate systems:

1. authoritative material volumes, including sparse and lazily materialized
   regions;
2. registration of material meaning needed by inspection, presentation,
   collision truth, persistence, and physics bindings;
3. consumer-driven base-content injection;
4. bounded inspection and explicit result completeness;
5. admitted material mutations such as removal, addition, and replacement;
6. streaming and residency lifecycle;
7. collision and occupancy truth over material, including moving material
   volumes;
8. persistence of material truth, consumer-supplied base identity, edits, and
   related scars without requiring a full untouched-volume dump;
9. derived surfaces, material appearance, and dressing that remain subordinate
   to matter;
10. registration of matter-backed objects and clutter;
11. a plug-in surface for external physics engines;
12. GPU-resident operation behind an asynchronous-capable command and query
    boundary; and
13. telemetry and evidence interfaces that make the above behavior reviewable.

### 2.2 Consumers own

Consumers decide what a world contains and what it means as a game. They own:

- procedural, deterministic, authored, or mixed generation algorithms;
- player and camera control;
- game-specific placement policy, building interfaces, blueprints, labor, and
  resource costs;
- game entities and rules, including combat, AI, progression, economy, the
  System / LLM layer, spells, and gas pricing;
- runtime physics simulation, including gravity response, contact resolution,
  collapse, debris, and force propagation;
- navigation, weather, seasons, growth, fire, and fluid simulation;
- the content palette and art direction for a particular world; and
- selection of validation scenes, routes, characters, and target machines.

A consumer may implement these systems using Moria's public truth, commands,
events, and bindings. Their usefulness does not make them substrate features.

### 2.3 Adjacent proof artifacts

Benchmarks, curation tools, visual checks, and an optional walkable-world
harness may live beside Moria. They have exactly the same access as an external
crate consumer. A proof is invalid if it reads private storage, changes matter
without mutation admission, collides with a private mesh world, or relies on a
generator or physics engine hidden inside the substrate.

No harness, including a polished third-person natural world, defines product
completion. A forest, river, cave, ruin, or cliff-to-cave route is one useful
proof composition, not required Moria content.

## 3. Design principles

### 3.1 Matter is authority

A material cell's occupancy and material association are authoritative.
Density or other shape information may refine how a cell contributes to a
surface, but presentation never replaces occupancy truth. Render meshes,
scatter points, debug geometry, acceleration views, and other derived products
may be discarded and rebuilt.

Every authoritative change produces a new observable world revision. Queries,
mutation outcomes, collision observations, persistence checkpoints, and view
rebuild status identify the revision they describe, allowing a consumer to
distinguish current truth from an older derived view.

### 3.2 Unknown is not empty

A region that is not resident, has not yet been supplied by its content source,
or cannot currently be inspected must never be reported as empty merely for
convenience. Inspection and collision results distinguish confirmed matter,
confirmed empty space, incomplete knowledge, pending work, and failure.

This is essential at streaming boundaries: a consumer can choose to wait, limit
movement, request residency, or apply its own policy, but Moria does not create
false voids.

### 3.3 One public contract

All consumers submit mutations and request inspection through the same facade.
There is no debug-only storage handle and no synchronous buffer ownership
available only to an in-repo executable. Implementation work may move between
CPU and GPU without changing the meaning of a consumer operation.

### 3.4 Volume, not terrain

A material volume has a three-dimensional domain and a frame of reference. It
does not imply a heightmap, a privileged up axis, a surface-at-one-height
model, or a single global gravity direction. Natural geology is a valid
consumer use, but deep caves, vertical strata, constructed interiors, and
other freeform arrangements remain real volume rather than special cases.

Ships and stations motivate this generality but are not current delivery or
validation targets.

### 3.5 Sparse cost follows interest

Known homogeneous solid and known empty space remain cheap. Detailed cost is
paid for boundaries, voids, heterogeneous content, active moving volumes, and
edits. Loading a large domain must not imply materializing every cell in it,
and saving it must not imply serializing every untouched cell.

### 3.6 Interaction significance determines representation

Content presented as solid, collidable, independently mutable, or physically
meaningful must be backed by authoritative matter. Matter-backed assemblies
such as a tree or boulder may have their own volume and lifecycle rather than
being baked into terrain.

Purely cosmetic dressing may be derived from material surfaces. It has no
independent occupancy or persistence claim and disappears or changes when its
supporting matter changes. This preserves the seed concept that grass and
micro-clutter can be inexpensive without becoming a disconnected second world.

## 4. Consumer experience

### 4.1 Establish a world

A consumer establishes a world by providing:

- the domains and material volumes it wants Moria to manage;
- material definitions and presentation associations used in that world;
- one or more content providers capable of supplying base material when a
  region is first needed;
- residency interests, such as spatial areas around cameras, tools, agents, or
  other activity;
- persistence inputs, if the world is being restored; and
- optional presentation and physics integrations.

Establishment validates the configuration before the world becomes available.
Invalid domains, missing material references, incompatible restored content,
unsupported required capabilities, or an unusable GPU execution environment
produce actionable failure rather than a partially working world.

Moria does not require the content provider to be deterministic or procedural.
It may read authored data, compute material, stream it from elsewhere, or mix
those approaches. The provider supplies content through the public seam; it
does not take ownership of substrate storage.

### 4.2 Reach readiness

World establishment and region activation may be asynchronous. The consumer can
observe:

- whether the world contract is accepted;
- which requested domains have enough authoritative data for the requested
  use;
- which regions are waiting for content, materializing, resident, or failed;
- whether collision and presentation products are ready for a given revision;
  and
- whether persistence restoration is complete.

There is no single global promise that the entire world is resident. Readiness
is scoped to a requested operation or spatial interest.

### 4.3 Use the world

During normal use, the consumer:

1. updates residency interests;
2. issues finite inspection requests or consumes subscribed change events;
3. submits explicit mutation commands;
4. observes admission and completion separately when work is asynchronous;
5. uses occupancy and collision truth for material contact;
6. renders current derived presentation while observing rebuild status;
7. attaches optional physics logic through public bindings; and
8. requests persistence checkpoints when its own save policy requires them.

The consumer never schedules internal storage work or edits an internal buffer.

### 4.4 Shut down or detach

Before a world or volume is released, Moria exposes outstanding mutations,
unsaved committed scars, active persistence work, and failed regions. A
consumer can request an orderly drain and checkpoint. Forced shutdown may
discard uncommitted work, but must not describe it as saved or committed.

## 5. World and matter model

### 5.1 Material volumes

Moria supports two behavioral classes under one truth model:

- **World-fixed volumes** hold material whose frame is currently fixed relative
  to its world.
- **Dynamic volumes** hold material in their own local frame and may change
  pose under consumer or physics-plug-in control.

Both classes support bounded inspection, admitted mutation, derived
presentation, collision truth, persistence, and material identification.
Changing a dynamic volume's pose does not rewrite its cells into a
gravity-aligned terrain grid. Damage to it is an admitted material mutation,
not a separate health overlay.

Volumes have stable consumer-visible identities for the duration of their
lifecycle. Results name the volume and revision they describe so overlapping
or moving volumes are not conflated.

### 5.2 Material meaning

A material definition establishes the identity needed to keep systems in
agreement. It includes:

- a stable consumer-facing material identity;
- occupancy and surface meaning;
- presentation associations for intact and newly exposed matter;
- collision-relevant classification;
- persistence compatibility identity; and
- physics-facing properties or extension fields required by the selected
  plug-in contract, including support for material strength.

Not every world must simulate every property. A material may expose data that a
consumer does not use. Moria validates that data needed by an enabled
integration is present, and does not invent physics behavior when it is absent.
Exact property schemas, precision, and extensibility mechanisms are technical
design decisions. World- or volume-level physics context separately supports
gravity parameters and the association of applied force with matter, without
making either a substrate simulation.

Material definitions may differ by consumer and domain. Moria does not ship a
mandatory geology, masonry, vegetation, or ship-interior palette.

### 5.3 Base matter, edits, and scars

Authoritative truth is the base matter accepted from a consumer content source,
plus committed mutations and other substrate-owned scars. The source remains
responsible for producing base content; once accepted for a region, that
content participates in the same truth contract as restored or edited matter.

A scar is durable state that cannot be reconstructed from the declared base
alone. It includes placed, removed, replaced, or damaged matter and may include
the lifecycle state of a matter-backed assembly when Moria owns that state.
Game-only state remains in the consumer's save.

Moria does not assume that re-running a generator recreates a base. Persistence
therefore associates scars with a consumer-provided base-content identity and
reports a mismatch on restore. The consumer owns migration or replacement
policy; Moria does not silently apply scars to an incompatible base.

### 5.4 Matter-backed assemblies

Vegetation, boulders, micro-objects, constructed chunks, and similar content can
register as matter-backed assemblies. Registration makes their occupied
material visible to inspection, mutation, collision, persistence, and derived
presentation through the same facade.

An assembly may be fixed or dynamic. Its game logic, growth, felling, rigid
response, or scripted behavior is not supplied by Moria. If a consumer changes
the assembly through those systems, the resulting pose or matter change
returns through the public volume and mutation contracts.

### 5.5 Derived dressing

Cosmetic vegetation, decals, surface scatter, and similar dressing may derive
from current material surfaces and consumer-provided presentation rules. Its
contract is:

- it is anchored to an identified material surface revision;
- it contributes no collision or authoritative occupancy;
- it is never persisted as material truth;
- removal or alteration of its supporting matter invalidates it; and
- regeneration cannot restore matter or overwrite a mutation.

If a consumer needs a blade of grass, plant, or object to block, burn, break, or
persist independently, that content must instead use authoritative matter or a
consumer entity registered with appropriate matter-backed occupancy.

## 6. Content injection

### 6.1 Region requests

Moria asks a content provider for a finite, identified region and the context
needed to interpret its returned material. A provider response is one of:

- complete base matter for the requested region;
- a compact declaration of known homogeneous matter or known empty space;
- pending, when the provider will answer later;
- unavailable, when the provider cannot currently answer; or
- failed, with a reason the consumer can act on.

This allows authored and generated content to remain lazy without treating
unknown space as empty. Provider-specific parameters and algorithms stay
outside Moria.

### 6.2 Admission of supplied content

Supplied content is checked for domain bounds, valid material identities,
internal completeness claims, and compatibility with already committed truth.
Conflicting base responses do not silently replace a region that has already
been accepted or edited.

Once admitted, supplied base matter is indistinguishable in public behavior
from any other authoritative matter. Harness-side generation receives no
special route.

### 6.3 Provider failure

A provider timeout, cancellation, invalid response, or permanent failure leaves
the affected region explicitly unresolved or failed. Existing committed
regions remain usable. Moria reports which residency interests and operations
are blocked and retains enough context for the consumer to retry, replace the
provider, reduce the request, or stop the world.

## 7. Inspection

### 7.1 Inspection families

The public inspection experience supports finite forms needed by tools, games,
collision consumers, and physics plug-ins:

- point or cell sampling;
- bounded region sampling or summary;
- rays and line-of-sight through material;
- overlap tests for bounded shapes or volumes;
- swept occupancy/contact tests for movement;
- material and surface observations;
- volume identity and pose observations; and
- lifecycle, revision, mutation, residency, memory, and work telemetry.

These are contract families, not a choice of geometry algorithm or source API.
Every request has finite spatial scope. Potentially large requests also accept
consumer-visible work limits or pagination so inspection cannot accidentally
require an unbounded world read.

### 7.2 Result semantics

An inspection result communicates:

- the requested scope and world/volume revision observed;
- confirmed material or empty-space findings;
- whether the requested scope was completely inspected;
- any unresolved subregions;
- whether more work is pending or another bounded request is required; and
- validation or execution failures.

Partial results are usable only as partial results. They never imply that
unreported space is empty. A consumer can request the missing scope, await
readiness, or apply its own conservative policy.

### 7.3 Snapshots and change observation

For multi-step reasoning, a consumer can request a bounded snapshot or revision
token and can subscribe to bounded or filtered change observations. If truth
advances during a sequence, Moria makes staleness visible rather than combining
facts from incompatible revisions without notice.

Events report committed truth changes, lifecycle changes, failed work, and
presentation invalidation. Events are observations, not a separate authority;
the consumer can query current truth when it needs a fresh answer.

## 8. Mutation

### 8.1 Mutation verbs

Moria provides material-level operations sufficient to remove, add, replace,
or otherwise change exposed matter. Dig and place are canonical consumer
intent, but their game costs, tools, shapes, permissions, and user interfaces
belong above the substrate.

A mutation identifies:

- its finite target and intended material effect;
- the world or dynamic volume it addresses;
- any expected revision or precondition needed to avoid stale writes;
- the materials and properties involved; and
- the consumer correlation needed to observe its outcome.

No consumer, harness, physics plug-in, or presentation system bypasses this
path.

### 8.2 Admission

Admission checks the request before it can alter truth. Reasons for rejection
include:

- invalid or unbounded target;
- unknown world, volume, or material;
- target outside the volume domain;
- stale required precondition;
- unresolved target matter when the operation requires known truth;
- conflict with volume lifecycle, restoration, or shutdown;
- unsupported requested material behavior; and
- exhausted capacity or an unavailable execution environment.

Rejection is explicit and changes no authoritative matter. Admission means the
request is valid and queued for execution; it is not a claim that the mutation
has already committed.

### 8.3 Execution and commit

A mutation progresses through observable submitted, admitted, executing, and
terminal states. Terminal outcomes are committed, rejected, failed, or
cancelled-before-commit. Once committed:

- inspection and collision truth observe the new revision;
- a persistence scar is dirty;
- affected derived presentation is invalidated;
- relevant change events identify the changed scope; and
- dependent physics-facing observations can identify the new truth.

A compound mutation never silently commits an undocumented subset. Its outcome
identifies the complete committed effect; if the product permits partial
progress for a large operation, the admitted units and their separate outcomes
must be visible before the consumer treats the whole intent as complete.

Competing mutations are resolved by a documented, observable ordering and
precondition policy. A stale operation is rejected or reports its actual
committed effect; it never silently overwrites newer matter.

### 8.4 Cancellation and failure

Cancellation is guaranteed only before commit. A late cancellation reports
that the mutation already committed and must be reversed, if desired, by a new
admitted mutation.

Execution failure leaves either the prior revision intact or a clearly
identified set of already committed units. It never produces an unreported
half-state. Presentation failure after commit does not roll truth back; the
world reports current matter and a degraded or rebuilding view.

## 9. Streaming and residency

### 9.1 Interests rather than a single camera

A consumer expresses finite residency interests around any relevant source:
cameras, players, tools, dynamic volumes, simulation areas, or offline
processing jobs. An interest states the spatial scope and the capability it
needs, such as authoritative inspection, collision, mutation, or presentation.

Moria combines concurrent interests without assigning a privileged camera.
This supports tools, split views, background activity, and moving matter while
remaining neutral about game design.

### 9.2 Region lifecycle

Consumers can observe these behavioral conditions, regardless of internal
representation:

- **known compactly:** the region is known to be homogeneous or empty without
  detailed materialization;
- **requested:** an interest needs more capability than is currently ready;
- **awaiting content:** the base provider has not completed the region;
- **materializing:** authoritative detail is being prepared;
- **resident for a capability:** the requested inspection, mutation, collision,
  or presentation use is ready;
- **quiescent:** no current interest needs active work and committed truth is
  safe to retain or persist compactly;
- **evictable/cold:** detailed residency may be released without losing
  committed truth; or
- **failed:** the region cannot satisfy the requested capability, with a
  reported cause.

A region may be ready for authoritative inspection before its derived surface
is ready. Readiness is therefore capability-specific, not one ambiguous
"loaded" flag.

### 9.3 Eviction

Eviction may discard derived views and reconstructible detail. It must retain
or durably checkpoint committed scars according to the active persistence
contract. Regions with executing mutations, unresolved persistence writes, or
active required interests are not reported as safely cold.

Returning to a cold region restores the same committed matter before reporting
the relevant capability ready. A consumer observes loading or failure rather
than a temporary false-empty world.

### 9.4 Capacity pressure

When requested residency exceeds capacity, Moria reports pressure, the
affected interests, and the capabilities that cannot become ready. It may
prioritize according to consumer-supplied importance, reduce only explicitly
degradable presentation work, or reject new work. It does not silently evict
required collision truth or discard unsaved edits.

## 10. Collision and occupancy truth

Collision is a truth service, not a physics simulation. It answers bounded
questions about whether and where material occupies space, including contacts
with world-fixed and dynamic volumes. Render triangles may help presentation
but are never the contract source.

Collision observations:

- name the material volume and revision involved;
- distinguish confirmed contact, confirmed clearance, unresolved space, and
  failure;
- expose material identity needed by a consumer or physics plug-in; and
- account for a dynamic volume's current accepted pose.

When a sweep crosses an unresolved streaming boundary, Moria returns an
incomplete or pending result rather than clearance. The consumer or physics
engine decides whether to pause, clamp, retry, or apply another conservative
policy.

Moria does not integrate velocity, apply gravity, resolve contacts, simulate
rigid bodies, or decide damage from impact. Those are adjacent physics
responsibilities.

## 11. Dynamic material volumes

A consumer can create or restore a movable material volume, supply its base
matter, inspect it, mutate it, and update its accepted pose. Typical future
consumers include voxel-bodied players and enemies, but Moria does not provide
those characters.

Dynamic-volume rules are:

1. local material truth survives pose changes;
2. collision and inspection use the current accepted pose and identify the
   observed revision;
3. damage changes cells through normal mutation admission;
4. presentation follows both pose and material revision;
5. persistence distinguishes pose/lifecycle state from material scars; and
6. deleting, merging, splitting, or converting volumes is not implied by
   ordinary damage or motion.

The last rule avoids silently importing rigid conversion, debris,
re-voxelization, or structural-fracture simulation. Such transformations need
explicit future product scope or consumer orchestration through available
volume lifecycle operations.

## 12. Physics plug-in experience

Moria exposes enough public truth and bindings for a hand-rolled or third-party
physics engine to attach without special storage access. A plug-in can:

- inspect bounded occupancy and contacts;
- identify material and physics-facing properties such as strength;
- observe configured gravity context without assuming a global down axis;
- associate applied forces or impulses with bounded material targets;
- submit pose updates for dynamic volumes; and
- request admitted material damage when its own simulation decides matter
  should change.

Moria records and exposes the association between a force-related request,
observed truth revision, and resulting admitted mutation or pose update. It
does not decide equations of motion, fracture thresholds, collapse propagation,
contact resolution, or damage rules.

A world without a physics plug-in remains valid. Enabling one validates the
required fields and capabilities and fails explicitly if they are unavailable.
A simple proof engine may test these bindings, but is not a Moria deliverable.

## 13. Presentation

### 13.1 Domain-coherent surfaces

Moria derives visible surfaces from current material truth and consumer
presentation definitions. The result must be capable of reading coherently for
its domain: organic cuts can look like cut earth, hard materials can preserve
constructed character, and newly exposed interiors can reveal their actual
material instead of a generic hole texture.

The design does not select raw cubes, surface nets, dual contouring, marching
cubes, raymarching, or a hybrid. Voxel scale, surface technique, level of
detail, texture strategy, and mesh pooling remain technical and art-direction
decisions validated against this behavior.

### 13.2 Truth-to-view lifecycle

For each presented region, consumers can distinguish:

- current for the authoritative revision;
- rebuilding after a committed change;
- stale relative to named truth;
- unavailable because required matter is not ready; and
- failed, with current truth still queryable when possible.

An accepted mutation changes authority before it is considered visually
complete. Old geometry may remain briefly as a marked stale view if the
presentation policy allows it, but collision and queries must not continue to
treat that geometry as truth. The rebuild eventually reveals honest cut faces,
new surfaces, and dressing appropriate to the committed material.

### 13.3 Debug and evidence views

Raw-material, occupancy, volume-boundary, residency, revision, and
truth-versus-view overlays may be supplied for tools and harnesses. They use
public inspection and telemetry. A debug view can reveal that a smooth surface
is backed by cells, but does not create a privileged edit route.

## 14. Persistence

### 14.1 Checkpoint contents

A persistence checkpoint represents:

- world and volume identities needed for restoration;
- consumer-provided base-content identities;
- committed material scars and compatible material-definition identities;
- substrate-owned lifecycle state for matter-backed and dynamic volumes;
- the authoritative revisions included; and
- whether any admitted work or dirty truth remains outside the checkpoint.

It excludes disposable meshes, dressing, debug geometry, and game-only state.
The consumer coordinates Moria's checkpoint with its own save data.

### 14.2 Checkpoint behavior

Checkpointing is asynchronous-capable and has an explicit completion result.
Only committed truth is eligible. Mutations that commit after the checkpoint's
captured revision remain dirty for a later checkpoint rather than being
ambiguously included.

An unsuccessful checkpoint leaves the prior durable checkpoint valid and
reports the unsaved revision range. Moria never clears dirty status merely
because a write was attempted.

### 14.3 Restore behavior

Restore validates world identity, base-content compatibility, material
compatibility, and checkpoint completeness before affected regions are
reported ready. Missing or incompatible base content, unknown materials,
corrupt scars, and unsupported volume state are explicit failures.

Moria does not silently regenerate a different base, drop unknown edits,
serialize a derived surface back into matter, or claim a partial restore is
complete. Recovery, migration, and content-version policy remain consumer
decisions supported by diagnostics.

## 15. Asynchrony and observability

GPU residency is a binding product property, so consumers interact with work
through commands, bounded queries, events, snapshots, and completion records
rather than synchronous internal-buffer access.

For every asynchronous operation, the consumer can determine:

- whether it was accepted;
- the scope and revision it targets;
- current progress state at a useful product level;
- its terminal outcome;
- what authoritative revision, if any, it created or observed; and
- what follow-up is needed after partial readiness or failure.

Telemetry makes at least the following reviewable by bounded world, volume, and
region scopes:

- authoritative versus derived readiness;
- residency and materialization state;
- pending and failed content requests;
- mutation admission, queueing, commit, and failure;
- stale presentation scope;
- dirty and checkpointed scars;
- unresolved collision/inspection scope;
- capacity and GPU-residency pressure; and
- timings and counts needed to compare mutation response, streaming behavior,
  and presentation convergence.

Telemetry itself is bounded and must not require a full world read.

## 16. Failure and recovery behavior

### 16.1 Configuration and capability failure

The world does not enter ready state when a required product capability cannot
be honored. Moria reports invalid configuration, unsupported GPU capability,
missing material semantics, incompatible integrations, and restore mismatch at
the narrowest useful scope. It does not silently become a CPU-authoritative
voxel engine or weaken the public contract.

Whether a particular implementation supports an explicitly declared fallback
is a technical decision; any fallback must preserve product semantics and be
visible to the consumer.

### 16.2 Runtime execution failure

Failure of GPU work, content provision, presentation derivation, or
persistence is isolated where possible:

- committed matter remains authoritative;
- uncommitted mutations are failed or retried without being reported committed;
- stale presentation is identified;
- unresolved regions stay unknown rather than empty;
- unsaved scars stay dirty; and
- unaffected volumes and regions remain usable.

After execution-environment loss, Moria reports the world and operations as
unavailable or recovering until authoritative state has been re-established.
It does not accept new work under a false ready state. Exact device-recovery
strategy belongs to technical design.

### 16.3 Consumer misuse

Invalid bounds, unknown identities, stale preconditions, oversized requests,
and operations against closing volumes return structured, actionable
rejections. Normal consumer errors do not corrupt or partially mutate truth.

## 17. Validation design

Moria is ready for review when its contracts can be evidenced independently of
a game. Validation should compose small proof consumers around the following
outcomes.

| Outcome | Required evidence |
| --- | --- |
| One material world | Sampling, collision, mutation, restore, and presentation identify the same material and revisions; no mesh-only occupancy exists. |
| Contract-only access | An external-style harness can perform every proof through the public facade, with no internal storage access. |
| Sparse scale | A domain too large for full raw-cell residency keeps known homogeneous and cold space cheap while active detail remains bounded. |
| Honest mutation | Removal and placement commit through admission, immediately change query/collision truth, persist as scars, and converge to correct derived surfaces and dressing. |
| Genuine depth | A proof volume contains reachable or inspectable material, voids, and changes across the full third axis rather than a heightmap plus decoration. |
| Dynamic matter | A moving material volume changes pose, collides through truth, takes admitted cell damage, presents that damage, and restores consistently. |
| Physics readiness | A minimal adjacent client can read configured fields, query contacts, associate force input, and submit pose/damage effects without privileged access; Moria performs no simulation for it. |
| Streaming honesty | Crossing resident, pending, cold, and failed regions yields explicit readiness or unknown states, never false empty space; returning restores committed scars. |
| Persistence | A base plus scars restores exact committed material; derived geometry is absent from authority; incompatible bases and corrupt data fail closed. |
| GPU/async contract | Operations remain correct without direct buffers or assumed synchronous completion, and residency/capacity behavior is observable. |
| Domain-coherent view | At least one selected validation domain shows intact and newly exposed material coherently; debug evidence connects the view to truth. |

Machine context, workload scope, world revision, and completeness accompany
performance evidence. Targets and scenes are chosen in validation and
technical design; the Product One seed's 1 km region, 25 cm cells, 16-cubed
bricks, frame targets, GPU models, content postcard, and milestone order are
not product requirements.

A visual walkable proof can combine hills, caves, strata, vegetation, a river,
and a ruin to communicate several claims efficiently. Equivalent non-game
proofs are valid. Full fluid behavior, fire, growth, weather, tree felling,
collapse, navigation, or a character controller cannot be required merely
because that scene would benefit from them.

## 18. Reconciliation of source concepts

### 18.1 Selected into this design

- **Voxel truth is not the look.** Smooth or domain-appropriate presentation is
  derived, while matter remains authoritative.
- **Mutable everywhere means honest cuts.** Exposed material is changed through
  admitted operations and presentation rebuilds from the result.
- **Deep Z is real content.** Caves, strata, ore, aquifer bands, and buried
  structures are useful validation content because they exercise volume rather
  than a painted underground; Moria does not generate them.
- **Homogeneous space is cheap.** Air and untouched solid volume can remain
  compact, while boundaries and scars pay detailed cost.
- **Dig and place are proof verbs.** They are substrate mutations without
  importing a building game, resource policy, or debug-key UI.
- **Objects and dressing differ.** Independently meaningful vegetation or
  clutter is matter-backed; cosmetic scatter derives from matter and cannot
  desynchronize.
- **Base plus scars is the persistence model.** The base is consumer-supplied
  and identity-checked; derived geometry is never saved as authority.
- **Commands, bounded queries, events, and telemetry form the consumer
  boundary.** Async GPU execution and public-only harnesses make explicit
  completeness and revisions essential.
- **Collision reads matter.** A controller or physics plug-in may consume that
  truth, but neither belongs to Moria.

### 18.2 Excluded by the approved boundary

This design does not select the broad seed's generation pipeline, column
scheme, biome or geology algorithms, fluid tiers, cellular automata, weather,
fire ecology, structural integrity, granular settling, navigation, building
semantics, blueprints, mechanisms, scripting, multiplayer services, or game
layers. It does not select the Product One character, camera, curated region,
forest palette, route, demo controls, performance numbers, hardware targets,
platform rules, milestones, or schedule.

It also does not import freeform ships or stations as delivery targets. Their
only current effect is to forbid terrain-only and global-gravity assumptions
in the public volume contract.

The excluded System pivot contributes no requirements. Its compatible
substrate principles are represented by the retained GPU-resident note and the
approved vision.

### 18.3 Deliberately deferred to technical design

Technical design must choose and measure, without changing the product
behavior above:

- spatial representation, brick or cell dimensions, payloads, compression, and
  allocation;
- surface derivation, GPU work scheduling, rendering, and level of detail;
- query, collision, and mutation algorithms;
- exact command, result, event, snapshot, and material-extension shapes;
- work ordering, batching, pagination, and concurrency mechanisms that realize
  the stated observable semantics;
- persistence encoding, migration mechanisms, and storage integration;
- GPU portability and recovery mechanisms;
- crate boundaries and source layout; and
- concrete benchmark workloads and target thresholds.

Physics-property prioritization, non-planetary gravity parameterization, and
the exact force/strength binding shape are also deferred engineering and
milestone decisions. They must preserve the requirements that physics is
adjacent, matter remains authority, gravity is not assumed to be one global
axis, and consumers receive explicit capability validation.

## 19. Human questions

There are no unanswered product-design questions inherited from the approved
vision.

Any proposal to add a generator, physics simulation, fluid or cellular
simulation, structural collapse, tree felling, navigation, multiplayer,
freeform-hull delivery, or game layer to Moria requires a new human scope
decision rather than an implementation-level assumption.
