# Deterministic Simulation and Rollback Amendment

## Status and provenance

**Status:** Proposed authoritative input for product-design regeneration. This
document has not yet been synthesized into or approved as part of
`docs/design-document.md`, and it is not yet part of the sealed planning
package.

**Source:** Requirements supplied by the human operator from an external Fable
design review, followed by an engineering reconciliation against Moria's
approved product design and technical design.

**Recorded:** 2026-07-29.

This document preserves the new requirements and the decisions already made
about them. The product-design regeneration must state the resulting product
contract in full. It must not merely cite this amendment or append an isolated
"determinism" section whose consequences are absent from the rest of the
design.

## Purpose

Moria must be suitable as the authoritative voxel-world substrate inside a
deterministic simulation with rollback, replay, and cross-machine desync
detection. These are architectural product properties, not a request for
Moria to implement a game, networking protocol, physics engine, damage model,
or world generator.

The required result is:

> Given the same canonical genesis state and the same ordered stream of
> tick-stamped inputs, every qualified backend produces the same canonical
> simulation state and bit-identical canonical hashes at every confirmed tick.
> A consumer can restore a recent confirmed tick without copying or traversing
> the whole voxel world, replay the subsequent inputs, and reproduce the
> original canonical hash sequence.

The guarantee applies across runs on one machine and across qualified GPU
vendors, drivers, and backend families. Cross-GPU determinism is a distinct
named invariant with distinct evidence; local replay success does not prove
it.

## Resolved product decisions

The following decisions are settled inputs to product-design regeneration.
They are not open implementation choices.

### Mandatory deterministic tick authority

Deterministic tick authority is mandatory for Moria worlds. It is not an
optional mode.

There is no nondeterministic convenience mutation path. A mode flag would fork
the public contract, persistence model, validation matrix, and documentation
while leaving the difficult deterministic representation necessary anyway.

After genesis, every operation capable of changing canonical state belongs to
exactly one numbered simulation tick and enters through the canonical input
stream.

### GPU residency remains valid

Canonical state is a logical and byte-level contract, not a requirement for a
full CPU voxel mirror. Moria may keep authoritative matter GPU-resident.
Persistent snapshots, hashes, and deterministic transitions may be implemented
with GPU-resident storage so long as their canonical meaning and output bytes
meet this amendment.

GPU residency remains a performance direction subordinate to correctness. A
GPU optimization may not weaken deterministic state semantics.

### Physics and damage remain external

Moria does not gain built-in physics, damage, fracture policy, health, bonds,
gameplay events, or another behavior vocabulary.

Moria must provide the deterministic scheduling, state, collision-derivation,
snapshot coordination, hashing, and effect-publication seams needed by
external CPU or GPU behavior engines. The external participant continues to
own the meaning and algorithms of its state.

### Rollback strategy is explicit per participant

Every behavior participant whose state can affect canonical simulation state
must declare exactly one rollback strategy:

1. `PerTickSnapshot`: retain participant state at the coordinated confirmed
   tick frontier; or
2. `ReconstructibleFromCanonicalStateAndLog`: reconstruct participant state
   from the canonical substrate state and canonical input log.

There is no default strategy. The selected strategy is part of the
participant's registered contract and is tested for that participant.

### Derived caches are outside the determinism boundary

Rendering, meshing, LOD, lighting, dressing, GPU uploads used only for
presentation, and other derived caches are explicitly exempt from canonical
determinism.

The exact boundary is:

> Every byte included in the canonical hash, and every value capable of
> changing such a byte in a later tick, is canonical simulation state.
> Everything else is a derived cache or non-authoritative observation.

A derived cache may be rebuilt, delayed, discarded, or differ between
backends. It may read canonical state but may never write canonical state or
change simulation behavior based on readiness, completion order, or contents.

## Binding product requirements

### 1. Canonical tick transition

The authoritative simulation transition is conceptually:

```text
State[t + 1] = Transition(State[t], TickBatch[t])
```

`TickBatch[t]` is a versioned, canonically encoded, bounded collection of all
inputs admitted for tick `t`. It has a deterministic total order independent
of submission thread, worker completion, GPU dispatch completion, hash-table
iteration, or callback arrival.

Tick-stamped canonical inputs include every operation that can change:

- voxel matter;
- volume creation, retirement, identity, or placement;
- simulation-domain activation or deactivation;
- substrate-owned allocation, revision, RNG, or lifecycle state;
- participant inputs whose execution may propose canonical effects; and
- administrative, editor, or test operations performed after genesis.

Public convenience APIs may construct and submit canonical events. They may
not publish an authoritative mutation outside the tick transition.

World construction and verified base-content installation may occur before
tick zero. Completing that process creates the canonical genesis state.
Nothing after genesis is an implicit bootstrap exception.

No transition-path code may read wall-clock time, thread identity, worker
identity, nondeterministic OS entropy, IO completion time, render readiness, or
another environmental value not present in canonical state or the current
`TickBatch`.

Any RNG capable of affecting canonical state is explicitly seeded, uses a
specified deterministic algorithm, and stores its complete state in the
rollback snapshot and canonical hash. There is no implicit reseeding after
genesis.

### 2. Canonical integer simulation representation

Integer or precisely specified fixed-point arithmetic is required throughout
the authoritative transition path.

This includes sim-facing:

- cell, brick, and volume coordinates;
- placement translation and orientation;
- collision facts that influence behavior;
- mutation and destruction geometry;
- ordering, allocation, counters, and revisions; and
- participant values that directly produce hashed authoritative effects.

Presentation may derive floating-point transforms from canonical simulation
transforms. Those presentation values are not canonical and may not feed back
into simulation.

The technical design must specify:

- storage widths and signedness;
- scale factors and representable ranges;
- overflow behavior;
- division and remainder semantics;
- right-shift behavior;
- rounding and saturation rules;
- canonical encoding and endianness;
- orientation normalization and composition; and
- CPU, WGSL, and persistence parity.

Undefined overflow, backend-selected precision, relaxed floating-point
behavior, and implementation-defined conversions are forbidden in the
authoritative path.

### 3. Local replay determinism

For a fixed qualified implementation and backend, identical genesis bytes and
identical `TickBatch` bytes must produce:

- identical canonical state hashes at every confirmed tick;
- identical canonical mutation, identity, placement, activation, and
  participant-outcome records;
- identical failure and rejection classifications; and
- identical rollback-and-replay results.

Repeated local runs, different worker counts, different thread schedules,
different valid GPU completion schedules, and deliberate timing perturbations
must not change those results.

### 4. Cross-GPU canonical determinism

Cross-GPU determinism is a first-class named invariant distinct from local
replay determinism.

Identical canonical genesis and `TickBatch` streams must produce bit-identical
canonical hashes across every GPU vendor, driver, and graphics backend that
Moria claims as qualified.

Backend qualification is fail-closed. A backend that has not passed the
cross-GPU conformance fixture is unqualified for deterministic authority even
if it renders correctly or passes local replay tests.

The canonical guarantee does not require identical:

- rendered pixels;
- meshes or vertex order unless those bytes are explicitly made canonical;
- LOD selection;
- lighting;
- presentation timing;
- debug output; or
- non-authoritative telemetry.

### 5. Race-free canonical bytes

The binding kernel rule is:

> No byte included in the canonical hash may be selected, ordered, allocated,
> or produced by a race.

Consequently:

- authoritative kernels use integer/fixed-point arithmetic only;
- floating-point atomics are forbidden in the transition path;
- "first writer wins," "last writer wins," or arrival-order authority is
  forbidden;
- append and compaction outputs use canonical keys such as stable volume and
  cell indices, never invocation or arrival order;
- canonical output is sorted or scattered into a uniquely determined slot
  before publication and hashing;
- stable identities and canonical slots are allocated by simulation-owned
  counters or deterministic prefix/index assignment, not atomic-race winners;
- unordered maps and sets may be used only when their iteration order cannot
  affect canonical output and output is canonicalized before use;
- parallel reductions are permitted only when the operation and exact integer
  semantics make the final value independent of reduction order; and
- atomics are permitted only where every legal interleaving produces the same
  canonical result.

Subgroup width, warp scheduling, workgroup dispatch order, and cross-workgroup
progress may not be assumed in portable authoritative kernels.

### 6. Cheap bounded snapshot and restore

Moria must retain a configurable bounded rollback window with a required
minimum capacity of 20 confirmed ticks.

A snapshot is a persistent reference to canonical state, not a full-world
copy. Mutation copies or replaces only state changed by that tick and the
bounded hierarchy needed to publish it. Unchanged voxel storage is shared
across retained snapshots.

The snapshot includes or binds all substrate-owned state capable of affecting
later canonical results, including:

- world and volume directory roots;
- voxel matter revisions;
- canonical placements;
- identity and slot allocator state;
- revision and tick counters;
- RNG state;
- active simulation-domain membership;
- content lineage and canonical content identity;
- retained participant strategy and state commitments; and
- every other future-transition input not reproduced directly from the log.

Restore must not copy or traverse the whole voxel world. It installs a retained
canonical root/frontier and restores every coordinated participant according
to its declared strategy.

Reclamation may not reuse or discard storage, identities, versions, or
allocator state reachable by the retained rollback window or an active replay.

The final product design must state a measurable restore budget and reference
workload. The unresolved measurement is recorded below and must be resolved
before product-design approval.

### 7. Incremental hierarchical hashing

Every confirmed tick produces a canonical state hash without traversing the
entire world.

The hash hierarchy must support dirty-leaf invalidation and canonical
recombination. Conceptually it includes:

```text
canonical brick state
  -> canonical volume state
  -> canonical substrate/world state
  -> coordinated participant commitments
  -> canonical simulation hash for tick t
```

Only dirty leaves and affected ancestors are recomputed. World and volume
combination order uses stable canonical identities, not storage placement or
map iteration.

The canonical hash covers every state value capable of changing future
authoritative results. It excludes derived caches and nondeterministic
observations.

The technical design must version the hash domain and encoding so an algorithm
or canonical-layout change cannot masquerade as continuity with an older
replay.

### 8. Fanatical simulation and presentation separation

Meshing, LOD, lighting, dressing, render-resource preparation, and presentation
are derived, disposable caches.

They:

- read a named canonical revision or tick;
- never mutate canonical state;
- never determine collision, occupancy, simulation-domain membership, or
  behavior;
- never make simulation depend on whether an artifact is ready;
- can be discarded and rebuilt without changing the canonical hash; and
- are regenerated only from the final corrected state after rollback replay,
  using the resulting dirty set.

Rollback does not require remeshing every intermediate replayed tick. Derived
work is scheduled for the final corrected canonical state.

### 9. Deterministic collision derivation

Collision authority used by deterministic behavior is a pure, canonical,
tick-synchronous derivation of confirmed voxel state.

The same canonical voxel contents and placement must produce the same
authoritative collision facts or the same canonical collider artifact,
regardless of GPU vendor, worker schedule, or asynchronous preparation timing.

Moria must support both valid consumer shapes:

- a conforming GPU behavior participant directly consumes canonical occupancy
  or collision inputs without mandatory CPU readback; and
- a CPU or external physics participant consumes a canonical collider artifact
  derived from voxel state.

When an external collider artifact is produced, it is keyed by the canonical
source hash and generated in canonical order. It may be cached.

Preparation may use asynchronous work, but completion timing cannot choose
simulation truth. At the designated tick the required collision representation
is either:

- present and bound to the expected canonical hash;
- deterministically unavailable, causing the declared fail-closed or
  no-advance result; or
- handled by another explicitly canonical representation.

"Collider not ready, therefore collision is disabled" is forbidden.

Moria does not prescribe Rapier or another physics engine. A consumer
integration may qualify a deterministic external engine, but the Moria product
owns only the deterministic voxel/collision and coordination seam.

### 10. Coordinated participant rollback and hashing

A participant whose state can change canonical simulation results registers:

- stable participant identity and contract version;
- rollback strategy;
- canonical input schema identity;
- canonical state-hash contribution or reconstruction proof;
- maximum snapshot or reconstruction resources;
- restore/reconstruction failure behavior; and
- qualification fixtures for the declared strategy.

`PerTickSnapshot` participants retain state at the same confirmed frontier as
the Moria substrate snapshot. Snapshot cost, retained bytes, restore work, and
reclamation are bounded and included in rollback-budget tests.

`ReconstructibleFromCanonicalStateAndLog` participants prove that restoration
from the selected substrate snapshot followed by canonical replay reproduces
their original per-tick hash contributions and effects.

Moria coordinates the frontier and combined commitment but does not interpret
participant vocabulary or silently repair participant divergence.

A participant may not "accept divergence." Failure to restore or reproduce its
canonical commitment is an explicit rollback or conformance failure.

### 11. Tick-stamped simulation-domain lifecycle

The simulation domain is canonical state and is distinct from render,
inspection, or materialization interest.

Volume or chunk activation and deactivation occur only through canonical
tick-stamped events. The active set and any required content identity are
included in snapshots and hashes.

Per-client render distance, camera position, local IO timing, or render
readiness may not independently determine what is simulated.

Asynchronous IO may preload content that is not yet active. A canonical
activation event identifies the exact content lineage or digest being
activated. If required content is unavailable or does not match, the tick
cannot substitute empty or alternate content and advance as if activation
succeeded.

Player- or consumer-defined activity regions may drive canonical activation
events. Their overlap resolves to a deterministic union so a volume, body, or
voxel is not processed twice merely because regions overlap.

The initial implementation may require the complete session-scale simulation
domain to be resident if that is the simplest honest contract. Streaming
simulation domains remain permitted only when they meet the same tick-stamped,
content-bound, fail-closed semantics.

Objects outside the full-physics region may continue through a
consumer-defined coarse simulation. Moria transports and coordinates that
state without defining its gameplay meaning.

### 12. Input-log replay as a first-class feature

Moria provides a versioned replay format or public replay contract containing:

- canonical genesis identity and configuration fingerprints;
- ordered canonical `TickBatch` records;
- participant identities and deterministic contract versions;
- expected canonical per-tick hashes when recorded for validation; and
- sufficient frontier information to reproduce or diagnose a divergence.

Initial canonical state plus the complete canonical input log must reproduce
the canonical state and hash sequence.

Replay is part of the public debugging and validation story, not merely a test
helper. It supports:

- desync reproduction;
- rollback verification;
- deterministic regression fixtures;
- agent-authored tests;
- captured demonstrations of canonical simulation; and
- a portable failure artifact containing the earliest divergent tick.

The replay contract does not require presentation frames, derived meshes, or
render timing to reproduce identically.

## Required validation outcomes

The regenerated product design must require evidence for at least the
following outcomes. The fresh TDD chooses precise fixtures and mechanisms.

### Mutation authority

- Every public mutation route produces a tick-stamped canonical event.
- Attempts to mutate canonical state outside the tick transition fail.
- Editor, administrative, restoration, and behavior-adapter paths cannot
  bypass the same authority boundary.
- Different submission thread and callback schedules produce the same ordered
  `TickBatch` and state.

### Deterministic kernel behavior

- Deliberately permuted dispatch, workgroup, worker, insertion, and completion
  order does not change canonical bytes.
- Collision-heavy allocation and compaction produce stable identities and
  canonical ordering.
- No float-tainted or order-tainted operation reaches canonical publication.
- Overflow and exhaustion produce identical typed outcomes across backends.

### Snapshot and rollback

- Retain at least 20 confirmed ticks while mutating several disjoint and
  overlapping voxel regions.
- Restore multiple selected ticks and replay to the original frontier.
- Match every original per-tick canonical hash.
- Prove restore does not traverse or copy untouched world material.
- Prove storage remains live while reachable and becomes reclaimable only
  after leaving the rollback window.
- Exercise each participant rollback strategy and its bounded failure modes.

### Incremental hashing

- A one-brick mutation rehashes only that leaf and its ancestors.
- Unchanged volumes retain their previous hash nodes.
- Canonical root combination is independent of physical slot and insertion
  order.
- Any mutation to canonical voxel, placement, allocator, RNG,
  simulation-domain, or participant state changes the appropriate commitment.
- Mutating derived render caches does not change the canonical hash.

### Replay

- Fresh execution and replay produce the same hash sequence.
- Rollback followed by replay rejoins the original sequence.
- A deliberately poisoned event or state byte causes a mismatch at the first
  affected tick.
- The failure artifact identifies genesis, contracts, input prefix, expected
  hash, actual hash, backend, and earliest divergent tick.

### Cross-GPU conformance

- One retained fixture runs identical canonical inputs on every claimed
  qualified backend.
- Canonical state hashes are compared byte-for-byte at every tick.
- The observer compares retained output bytes rather than trusting a
  self-reported boolean.
- A backend or driver combination with divergent hashes is reported
  unqualified; the result is not averaged or softened into a performance
  warning.

### Simulation and presentation isolation

- Missing, delayed, stale, or deliberately corrupted derived presentation
  cannot change canonical collision or state.
- Rollback replay performs no mandatory intermediate remeshing.
- Final corrected dirty regions rebuild presentation from the corrected
  canonical state.

### Simulation-domain lifecycle

- Different IO and render completion schedules produce the same activation
  state and hash sequence.
- Missing or mismatched activation content fails closed.
- Overlapping activity regions produce one deterministic union.
- Client-local rendering interest cannot activate or deactivate canonical
  simulation state.

## Sequencing gates before TDD regeneration

The premise must be tested before a fresh full TDD is accepted. Do not
reconcile the existing TDD around an untested assumption.

### Rung 0a: transition-path kernel audit

Run this audit concurrently with product-amendment drafting.

Classify every existing or proposed operation that can affect canonical state
as:

- `clean`: its result is already canonical and order-independent under
  specified semantics;
- `float-tainted`: authoritative output depends on floating-point arithmetic
  or conversion;
- `order-tainted`: authoritative output may depend on iteration, race,
  allocation, append, reduction, or completion order; or
- `unknown`: evidence is insufficient to classify it.

The audit includes:

- public mutation and publication paths;
- placement and coordinate transforms;
- GPU page/hash-table mutation;
- allocation and identity assignment;
- compaction, append, scan, sort, and reduction kernels;
- collision facts used by behavior;
- behavior proposal validation and composition;
- directory and lifecycle publication;
- hashing and persistence encoders;
- snapshot, restore, and reclamation;
- base-content activation; and
- any participant proof adapter used to validate the seam.

For every non-clean entry, retain:

- source document and, when code exists, source location;
- current arithmetic and ordering assumptions;
- canonical state affected;
- proposed remediation;
- portability risks;
- estimated implementation and validation cost; and
- whether the old mechanism should be replaced rather than repaired.

The audit report becomes an appendix to this amendment before product-design
approval. It is a costed blast radius, not a claim that the historical TDD or
discarded implementation remains authoritative.

### Rung 0b: cross-vendor feasibility spike

Before the planning package is resealed, execute a small representative
implementation that proves the proposed deterministic kernel discipline across
available GPU families.

The spike must exercise:

- fixed-point translation and orientation;
- canonical voxel mutation;
- conflict resolution;
- dirty-brick discovery;
- deterministic allocation;
- canonical compaction;
- leaf and hierarchical hashing;
- persistent snapshot capture;
- rollback and replay; and
- observer-side byte comparison of per-tick hashes and canonical records.

Identical fixture bytes must run on actual Metal and Vulkan hardware. DX12 must
pass before Windows/DX12 is claimed as a qualified deterministic backend.
Additional vendors and drivers join the same fixture as they are claimed.

Rung 0b proves feasibility of the architectural primitives; it cannot prove a
whole engine that has not yet been implemented. The regenerated TDD must
require the final production implementation to pass the complete cross-vendor
conformance suite before deterministic backend qualification.

A divergent canonical byte or hash fails the rung. Performance is recorded
separately and cannot convert a correctness divergence into a pass.

## Instructions for product-design synthesis

The product-design drafter must:

1. Read this amendment alongside the approved vision, product vision,
   product-design decisions, and current product design.
2. Regenerate or comprehensively revise the product design so these
   requirements are expressed in the relevant authority, consumer journey,
   lifecycle, behavior-extension, persistence, validation, performance,
   failure, and non-goal sections.
3. Assign durable product requirement and authority provenance identifiers.
4. Reconcile asynchronous GPU work with deterministic tick publication:
   computation may finish asynchronously, but completion time may not select
   canonical state.
5. Preserve GPU-resident authority without introducing a full canonical CPU
   voxel mirror.
6. Preserve external ownership of physics, damage, and gameplay policy while
   requiring coordinated deterministic participants.
7. Clearly distinguish local replay determinism, cross-GPU determinism, and
   derived presentation freedom.
8. State every remaining human decision explicitly rather than inventing a
   value.
9. Avoid prescribing technical mechanisms that belong in the TDD unless this
   amendment marks them as binding constraints.
10. Produce a coherent design that stands alone without requiring a future TDD
    author to infer requirements from this amendment.

The old TDD is historical evidence only. It must not be supplied as a revision
target for the fresh TDD. Once the revised product design and feasibility gates
are approved, the technical design is generated from zero.

## Explicit non-goals

This amendment does not add:

- a networking or rollback-netcode implementation;
- a built-in physics engine;
- a built-in damage, fracture, bond, or gameplay event system;
- a player, camera, controls, vehicle, weapon, or game loop;
- deterministic rendered pixels;
- deterministic meshes, LOD, lighting, or dressing;
- a mandatory CPU mirror of GPU voxel state;
- a prescribed external physics library;
- procedural world generation;
- an unbounded rollback history; or
- a guarantee for a backend that has not passed conformance.

## Open human decisions

These questions must be resolved during product-design review. They are not
permission for the drafter to weaken the requirements.

### OD-001. Rollback restore budget

The rollback window has a fixed minimum of 20 confirmed ticks, and restore may
not traverse or copy the whole world. The exact wall-time budget and reference
workload remain to be selected.

The design must propose a measurable workload including at least:

- total resident voxel/brick scale;
- number and distribution of dirty bricks per tick;
- number and strategy of rollback participants;
- rollback depth;
- target hardware class; and
- whether replay work after root restoration is measured separately.

The human must approve the final threshold. Platform-specific performance
results may be reported independently; canonical correctness is not
platform-relative.

### OD-002. Initial conformance hardware matrix

Metal and Vulkan are required for rung 0b because those hosts are presently
available. Windows/DX12 cannot be called qualified until it passes the same
fixture. Product-design review must identify the initial vendor/device/driver
matrix used for the first public deterministic-backend qualification.

### OD-003. Canonical orientation envelope

The product requires canonical fixed-point/integer placement orientation, but
the supported orientation range and precision must be sufficient for
volume-general dynamic bodies. The product design must state the required
consumer outcome and error behavior; the fresh TDD selects and proves the
encoding.

## Rung 0a audit appendix

**Status:** Not yet performed.

The audit report required above must be appended or linked here before this
amendment is approved as product-design authority. Absence of the audit blocks
approval; it must not be silently treated as an empty finding set.
