# Historical engineering and dogfood evidence — 2026-07-23

> **Historical record:** The first sections of this document describe the
> now-discarded first Moria implementation and the cargo-reapi/Bro dogfooding
> performed against it. Its implementation counts, commands, capabilities, and
> acceptance status are not claims about the clean rebuild branch. The later
> planning-pipeline section records how that discarded result was replaced and
> how human review changed the clean rebuild's still-open TDD. The build-system
> observations and linked cargo-reapi records remain valid historical evidence.

This document states what the Moria repository demonstrates today and what it
does not. It is intended to keep implementation evidence, build-system
evidence, performance acceptance, and visual acceptance from being collapsed
into one optimistic status.

## Repository-scale evidence

The current public baseline has four workspace crates, 107 Rust source files,
approximately 22,800 lines of Rust, and 224 `#[test]`/`#[tokio::test]`
functions. These counts describe the review surface; they are not quality
claims by themselves.

The test surface includes:

- deterministic terrain, biome, object, and curated-manifest behavior;
- sparse storage coordinates, material truth, and collision ownership;
- bounded ray, overlap, sweep, sampling, and diagnostic queries;
- mutation admission, rejection, execution, and observation;
- focus-source and streaming lifecycle transitions;
- public-facade and headless Bevy integration;
- configuration, checked-in asset, and canonical-generation contracts;
- benchmark report schemas, cross-field invariants, and failure output.

The ordinary local gate is:

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo run -p moria-curate -- check
```

## Deterministic curation evidence

`moria-curate` regenerates the curated world manifest from the checked-in region
configuration and sparse ruin stamp. `moria-curate check` fails if the
checked-in manifest is not the canonical output.

The `prove-forest` command additionally validates the generated manifest and
writes a machine-readable report:

```sh
cargo run -p moria-curate -- \
  prove-forest --output target/feasibility/forest.json
```

This proves only the fields emitted and validated by the current command. It
does not substitute for every F1 acceptance field in
[`tdd/benchmarks.md`](tdd/benchmarks.md), and it does not produce visual
evidence.

## Rust/Bevy build evidence

Moria is the real dependency graph used to qualify
[`cargo-reapi`](https://github.com/TamedTornado/cargo-reapi). That matters
because the graph includes the build scripts, proc macros, native-tool
discovery, links, and relocatable test binaries that small synthetic fixtures
miss.

The public cargo-reapi qualification reports currently record:

| Host | One clean warm worktree | Five simultaneous | Ten simultaneous |
| --- | ---: | ---: | ---: |
| macOS/arm64 APFS | 8.302s | 14.264s | 25.016s |
| Linux/x86_64 XFS | 6.455s | 10.818s | 18.852s |

Every warm population reported zero physical actions and zero compiler/linker
executions under external OS observation. Both platform batches also covered
adversarial invalidation, poison propagation, flag/configuration changes,
concurrent miss coalescing, sandbox denial, and linked Bevy artifact parity.

Those are build-system results, not Moria runtime-performance results. Complete
methodology and pass matrices live in cargo-reapi:

- [macOS APFS record](https://github.com/TamedTornado/cargo-reapi/blob/main/benchmarks/results/2026-07-21-macos-apfs.md)
- [Linux XFS record](https://github.com/TamedTornado/cargo-reapi/blob/main/benchmarks/results/2026-07-21-linux-xfs-schema-v3.md)
- [Moria agent-fleet dogfood](https://github.com/TamedTornado/cargo-reapi/blob/main/docs/case-studies/moria-agent-fleet.md)

## Private-harness dogfood

We currently run Moria through a private agentic coding harness with five
logical agent slots and independent mechanical quality gates. Bro, the
orchestrator, is not public. The relevant cache statistics, revisions, and
methodology are published in cargo-reapi rather than asking readers to trust a
private link.

The dogfood run is useful because it exposed defects that a controlled benchmark
did not:

- orchestration-only environment variables accidentally became Rust build
  inputs;
- agent target mounts and cargo-reapi's declared target root diverged;
- Debian's `/etc/alternatives` indirection was hidden from a native Bevy build
  script by the strict sandbox;
- mutable target populations and cache garbage collection required independent
  storage policy and telemetry.

After the environment and target-root repair, one fresh agent-run action log
contained 31 cache hits, 10 coalesced hits, 21 producer misses, and six
non-cacheable capability probes across 74 wrapper records. A separate cold
sample showed five identical `bevy_pbr` callers become one producer and four
waiters.

These are promising field results. They do not make the private orchestrator
reproducible and do not qualify a live remote REAPI service.

## Clean-rebuild planning-pipeline dogfood — 2026-07-23 through 2026-07-28

This section records process evidence rather than an accepted product or
implementation claim. It was reconstructed from the retained repository
history, public pull requests, their human-review conversations, and the active
technical-design revision. The final TDD described below remains unmerged while
this record is being written.

### Why the first implementation and TDD were discarded

The original seed set mixed several kinds of document:

- the binding request for a reusable GPU-resident voxel-world substrate;
- downstream game visions that the substrate should eventually support;
- examples and exploratory prose whose authority was not stated uniformly.

The original interview/planning path did not first ask the decisive question:
whether this repository was building the substrate or a downstream game. It
treated the supplied documents as material to accumulate rather than evidence
whose authority had to be disambiguated and synthesized.

The resulting design and decomposition promoted downstream examples into
current product requirements. The implementation DAG then spent substantial
agent and mechanical-review effort building and repairing unauthorized work,
including prescribed forest population/canopy behavior, a third-person human
explorer, skeletal assets/animation, and other game-facing systems. These were
not isolated implementation flourishes: by the time issues were decomposed,
the contaminated scope was already present in the planning artifacts.

Late recovery work made the failure more expensive. Agents attempted to
converge individual recovery issues against the contaminated TDD, and one asset
recovery path began growing a custom validator rather than selecting the
standard ecosystem tool. The orchestration and review defects exposed by that
run remain useful Bro dogfood, but repairing the implementation one issue at a
time could not restore a trustworthy product boundary.

The operator therefore made the implementation, old design documents, and old
TDD explicitly disposable. Moria `master` was reset to a clean substrate
planning baseline while retaining the public seed corpus and this historical
evidence. This was a deliberate refusal to let sunk agent work become product
authority.

### The replacement document pipeline

The replacement flow separated four questions that the earlier pipeline had
collapsed:

1. **Scope boundary:** given ambiguous seeds, what product is this repository
   actually building, and which seed material is binding, supporting, or
   downstream?
2. **Synthesized product vision:** under that approved boundary, what coherent
   product do all relevant seed documents describe?
3. **Product design:** what behavior and consumer-facing contract realizes the
   approved vision without importing downstream game scope?
4. **Technical design:** what implementable architecture satisfies that product
   design?

Each stage uses the same public PR revision loop: an agent drafts the artifact,
an independent agent reviews it, the artifact is presented to the human, and
human comments return to the drafting loop. A merge is the approval boundary
for the next stage.

The retained public artifacts are:

- [PR #389](https://github.com/TamedTornado/moria/pull/389), the approved scope
  boundary;
- [PR #390](https://github.com/TamedTornado/moria/pull/390), the synthesized
  product vision;
- [PR #393](https://github.com/TamedTornado/moria/pull/393), the explicit
  decision that physics and damage remain external behavior systems rather than
  Moria substrate policy;
- [PR #394](https://github.com/TamedTornado/moria/pull/394), the approved
  product design; and
- [PR #395](https://github.com/TamedTornado/moria/pull/395), the still-open
  technical design.

This process also exposed pipeline-design lessons that are not visible from the
final prose alone:

- A scope boundary is intentionally lean; it is not a substitute for the later
  synthesized vision.
- A synthesized vision must actually incorporate the authorized content of its
  sources, not merely link to them.
- Human clarification belongs in a durable document and PR history, not hidden
  orchestration metadata.
- Prompts and DAG/node definitions are hot-reloaded data. Changing an artifact
  flow should normally require new or revised data-backed nodes and prompts,
  not service-image edits or hard-coded TypeScript registries.
- A review node must inspect the current branch tip, not a pinned revision that
  can silently discard later human-directed changes.
- The real historical failure is a valuable evaluation fixture, but an eval
  designed around it also needs a mixed-authority holdout and a clean negative
  control so the system is not merely trained to recognize one obvious
  contamination pattern.

### First clean TDD and adversarial convergence

The clean technical-design run used the approved product documents plus curated
Rust, Bevy, wgpu, sparse-GPU-data, and compute-shader experience context. Its
first result stayed within the substrate boundary and was materially stronger
than the discarded TDD.

Human review then identified that "physics and damage are external" was not
enough. The substrate needed a real integration seam at the correct tick
boundary. The TDD was revised to add ordered CPU and GPU behavior adapters,
bounded resource admission, stable per-participant substrate views, generic
effect proposals, and adapter-owned state.

The first behavior revision still lacked a direct current-tick input route for
the first adapter. Physics could not receive timestep, forces, control input,
or other consumer-owned stimuli without a fake predecessor or hidden shared
state. Human review required bounded opaque per-participant ingress for both CPU
and GPU adapters, retained the restricted purpose-built GPU adapter boundary,
and rejected an overclaim that scheduled behavior could perform arbitrary
volume creation.

The independent reviewer then found defects that the drafting pass missed:

- GPU input upload failure occurred only after mutable planner code had run,
  contradicting fail-before-execution behavior.
- A tick-global input-preflight failure could not truthfully represent
  unaffected participants.
- The mixed CPU/GPU feasibility workload did not define its effects precisely
  enough to prove revision changes.
- Shutdown still used the obsolete pre-planning cancellation boundary after
  GPU upload became the point of no return.
- Two supposedly distinct material samples in the feasibility oracle did not
  specify distinct material IDs.

Two coder/reviewer turns closed those findings. The third adversarial pass
approved the revised TDD, and the PR returned to human review with a clean
lineage. This is evidence that the conversational review ledger can converge on
subtle lifecycle correctness when it is allowed multiple attempts. It is not
evidence that the resulting public API is pleasant or complete.

### Human API review after automated approval

The human did not merge the automatically approved TDD. A subsequent
plain-language API review exposed architectural questions that were hard to see
inside the internally consistent specification.

`BehaviorEngineDescriptor` was revealed as a large, flattened registration
manifest combining identity/order, input policy, access bounds, effect limits,
failure/composition policy, handoffs, and GPU resource budgets. The underlying
admission contract is necessary, but the direct public struct is likely
agent-shaped API overdesign. CPU/GPU-specific builders and a clearer
planner/adapter ownership story remain implementation ergonomics concerns.

The review then compared the proposed integration with two familiar physics
models:

- a classic PhysX-style persistent world of rigid actors, shapes, constraints,
  and solver state; and
- a Flex-style GPU solver built from persistent flat particle/constraint
  buffers.

That comparison clarified the intended authority boundary. A physics adapter
owns bodies, forces, velocities, constraints, solver state, damage/bond policy,
and any persistent collision proxy. Moria owns authoritative voxel matter,
volume identity/placement publication, revisions, and bounded transactional
effects. CPU-to-GPU traffic should normally be a compact command stream
(control, guidance, beam, explosion, and similar injections). Bulk simulation
state and rendering transforms remain GPU-resident. GPU-to-CPU traffic should
normally be a compact, filtered event stream rather than a physics-state
readback.

### Three gaps found by conversational discovery

The post-approval review identified three requirements not adequately covered by
the approved automated review result.

#### Atomic fracture into independently moving child volumes

Scheduled behavior could patch, move, and retire existing volumes but could not
atomically transform one volume's existing matter into newly created child
volumes. The reason was not that voxel matter is inherently CPU-owned. General
creation was tied to a host-side `BaseContentSource`, while the scheduled GPU
ABI accepted only bounded data records targeting existing snapshot identities.

The required operation is a bounded data-only extraction transaction:

- GPU-discovered connected components are selected by adapter-owned policy;
- all child identities, directory records, pages/cells, proposal records, and
  bytes are reserved before execution;
- source removal and child publication are atomic, so matter is never
  duplicated, lost, or ownerless;
- child cell size, placement, lineage, persistence, and rematerialization are
  defined without transporting a Rust source object through the GPU ABI; and
- proposal-local piece handles are mapped back to final `VolumeId`s in
  GPU-visible feedback without authority-path CPU readback.

Small debris may remain transient adapter/VFX state; Moria must not define what
counts as a significant persistent fragment.

#### CPU-defined activity regions with coarse simulation outside them

The original review concern was phrased as GPU-driven active-region selection.
Human clarification removed that requirement: the CPU/game layer defines one
or more important simulation regions, and those definitions arrive as compact
tick input. The GPU classifies its persistent bodies against those regions
without reading the body list back to the CPU.

The clarification exposed a different requirement. Outside every full-physics
bubble, significant objects must not freeze. One persistent world simulation
needs:

- coarse motion/world simulation outside CPU-defined regions;
- full collision/constraint/damage physics inside;
- a transition halo for continuous promotion/demotion;
- deterministic union/deduplication of disconnected or overlapping player
  regions; and
- continued coarse movement and coarse remote destruction/debris outcomes.

Geographic regions are not separate physics adapters or separate worlds. A body
crossing between regions remains one continuously owned body. The TDD must
select a scalable placement-authority/update mechanism instead of assuming
either that Moria placement may become stale or that one ordinary movement
proposal per coarse object per full-physics tick is automatically affordable.

#### Bounded opaque GPU-to-CPU adapter egress

Physics collision, weapon, destruction, scoring, and audio events are not Moria
semantics. The physics/damage integration owns their record layout, filtering,
and interpretation. Moria nevertheless needs a generic transport because the
restricted GPU adapter deliberately has no raw device, queue, mapped authority
buffer, or self-managed readback.

The required seam is an optional bounded opaque egress lane. Moria understands
only capacity, initialized length, tick/correlation identity, and transport
outcome. Delivery is asynchronous and exact, with explicit overflow,
cancellation, mapping failure, device-loss, shutdown, and reuse behavior.
GPU-to-GPU consumers continue to use handoffs; publication authority does not
depend on CPU interpretation of event bytes.

The complete human comment is retained on
[PR #395](https://github.com/TamedTornado/moria/pull/395#issuecomment-5102327212).

### Revision in progress and evidence status

The technical-design drafting loop consumed that comment after its normal
review debounce. At the time of this record, the agent had selected Scheduled
ABI v2 with:

- a pre-reserved component-extraction transaction;
- a compact GPU placement stream for persistent coarse/full simulation;
- an optional opaque readback lane; and
- a world-directory epoch proposed as the atomic publication boundary.

The agent added a focused `adapter-substrate-contracts.md` and began propagating
the design through public API, scheduling, lifecycle, storage, persistence,
resource limits, decisions, and validation.

That breadth is partly unavoidable: fracture reverses an explicit no-create
rule and affects identity/persistence, bulk placement affects authority, and
egress adds a GPU/CPU lifecycle. It is also a fresh overdesign risk. The
world-directory epoch and generalized placement stream are agent-selected
mechanisms, not human-prescribed solutions. The next adversarial review must
test whether they are the smallest maintainable mechanisms that satisfy the
requirements rather than accepting internal consistency as sufficient.

No passing or merged TDD claim should be inferred from this in-progress state.
The durable evidence is the sequence itself: the first generated product was
discarded rather than rationalized; the replacement document pipeline
successfully controlled scope; automated adversarial review found real
lifecycle defects; and human conversational review still uncovered product
architecture gaps after automated approval.

### The architecture amendment expanded into a rewrite

The first drafting response to the three human requirements was not a small
amendment. Commit `dab1a1a` changed 2,089 lines across eleven TDD files:
1,923 additions and 166 deletions. It introduced an 858-line
`adapter-substrate-contracts.md` plus a Scheduled ABI v2, component-extraction
transaction records, placement streams, opaque egress records, and a
world-directory epoch.

This is useful negative dogfood evidence. A prompt asking for three generic
integration capabilities was interpreted as authority to fully design a new
wire protocol, durable data model, transaction model, and validation system.
The mechanisms were plausible and internally motivated, but their size and
specificity were not evidence that they were the smallest design that met the
human requirement.

The independent auditor rejected the revision. It found one regression in the
previously approved feedback ABI and seven new defects:

- the component-piece handle space could not be preallocated as specified;
- extraction conserved collision occupancy rather than the complete
  authoritative material/scalar field;
- atomic multi-volume publication had no atomic observation representation;
- derived children lacked complete durable naming and provenance;
- the retained derived-base lookup had no bounded owner or pressure policy;
- counter overflow had no distinct Scheduled ABI v2 wire value; and
- the two new proposal kinds left common record fields undefined.

The auditor also recorded useful positive discrimination: CPU-authored
multi-fidelity placement and the bounded opaque egress lifecycle were
architecturally sound in direction. The rejection was therefore not a generic
preference for less detail; it identified concrete places where added
machinery was not implementable or did not close its own contracts.

The automatic correction turn then spread through the focused contract,
public API, scheduled behavior, lifecycle, overview, architecture,
persistence, state/storage, and validation documents. This exposed a failure
mode in a correctness-only revision loop: once a large mechanism exists,
review findings naturally demand that it be propagated and completed
everywhere. The loop can converge on a coherent but unnecessarily large design
without ever asking whether the mechanism should exist.

This also exposed a missing PM responsibility. The technical-design auditor
was adversarial about consistency and implementability, but accepted the
drafter's chosen architecture as the premise of review. Its findings therefore
made the new protocol more complete and more entrenched. No active role was
asking whether the human decision authorized that amount of mechanism, whether
an existing primitive could satisfy it, or whether the smallest sufficient
answer should delete most of the revision. An adversarial technical reviewer
is not automatically an adversarial PM.

The resulting failure is a complexity ratchet: the drafter invents a large
mechanism; the reviewer finds gaps inside it; the drafter fills those gaps; and
each valid correction increases the cost of reconsidering the original choice.
The missing control is an authority-and-simplicity review before technical
completion. It must compare the proposed delta directly to the human decision,
challenge the necessity of every new concept, and be empowered to request
deletion or human arbitration rather than only internal completion.

The first proposed operator cutoff was the next independent auditor pass. Once
the missing PM role and complexity ratchet were made explicit, the operator
chose not to spend another pass completing the suspect premise. Run
`03738f1e-b4e3-473f-bbae-9805ecf9b059` was parked and its active correction
session was cancelled before delivery. Bro therefore records the interrupted
node as operator-cancelled/failed; the incomplete correction was not published
to PR #395.

The agreed remedy, to be designed before any pipeline change, is:

1. park the run and retain this sequence as dogfood evidence;
2. return to the last approved pre-amendment TDD at `2ab6796`;
3. request the smallest sufficient amendment, reusing existing contracts
   before introducing a new global mechanism;
4. require every new concept to trace to one of the three approved human
   requirements or to an unavoidable correctness consequence;
5. keep externally observable ownership, failure, persistence, and boundedness
   contracts in the TDD while leaving private encodings and implementation
   algorithms for decomposition unless interoperability requires them; and
6. run an independent simplification review that can require deletion and
   asks which mechanisms are necessary, rather than only whether every
   mechanism is internally complete.

This is deliberately not a word-count or line-count gate. The failure was
authority and design pressure, not prose volume by itself. A legitimately
complex public contract may be long; an agent-selected global mechanism must
still prove that its complexity is necessary.

### Simplicity-intervention eval

On 2026-07-28, the proposed simplicity control was evaluated as a behavioral
experiment rather than accepted from a single plausible-looking response. The
positive case was the complete overengineered amendment from baseline
`2ab6796` to candidate `dab1a1a`: 1,923 insertions and 166 deletions across
eleven TDD files. Its authority was limited to three capabilities:

- atomic GPU-resident fracture into independently moving child volumes;
- CPU-authored multi-fidelity activity regions without Moria-owned physics or
  damage policy; and
- bounded opaque asynchronous GPU-to-CPU egress.

The negative control was the full TDD at baseline `f7e5917` and candidate
`2792aee`. Its delta contained only two necessary review corrections: exact
P10 oracle material IDs and shutdown-cancellation wording aligned with the
existing `Preparing` boundary. It was deliberately already sufficient and
minimal. A valid simplicity intervention therefore had to leave it
substantively unchanged.

Three interventions were compared:

1. a fixed mechanical question to the drafter: “Are you sure this is the
   simplest design that satisfies the requirements?”;
2. an independent PM verdict followed by a separate drafting revision; and
3. an independent PM challenge naming specific suspect mechanisms and asking
   whether each was the simplest sufficient design, followed by a separate
   drafting revision.

Every lane ended with the same independent blind evaluation. A positive-case
run passed only when the result was materially simpler, retained all three
authorized capabilities, and remained internally coherent and implementable.
A control run passed only when the two necessary corrections survived without
material rewriting. A completed DAG was not itself counted as a successful
behavioral result.

The measured batch contained five positive repetitions per intervention and
two control repetitions per intervention, for 21 runs total. The drafting
role used Grok 4.5 at high reasoning; independent review and blind evaluation
used Codex at high reasoning.

| Intervention | Positive result | Control result | Interpretation |
| --- | ---: | ---: | --- |
| Mechanical question | 5/5 passed | 2/2 preserved | Consistently removed excess architecture without damaging the minimal control. |
| Independent PM verdict | 1/5 passed | 2/2 preserved | PM advice usually identified real excess, but four drafting handoffs produced incomplete or incoherent architectural rewrites. |
| Targeted PM challenge | 1/5 passed | 2/2 preserved | Specific challenges improved diagnosis but did not make the separate revision handoff reliable. |

All 21 measured DAGs completed without an infrastructure or scheduler failure.
The two reviewer-based interventions did not fail because the PM preferred the
large design. Their reviews consistently challenged the global world-directory
epoch, cross-volume placement atomicity, normative proof-adapter algorithms,
parallel egress transport, and derived-key namespace. The dominant failure was
the next turn: the drafter removed those concepts only partially, leaving
normative references that contradicted the replacement design or weakening
atomic publication, boundedness, lifecycle, or pre-dispatch identity
contracts.

The mechanical intervention independently converged on the same smaller
boundary in all five positive runs: retain extraction, opaque activity-region
input plus placement output, and bounded egress; reuse existing
directory-generation, per-volume authority, persistence, staging, and receipt
primitives; and remove Moria-owned behavior algorithms and parallel global
infrastructure. This is evidence that the short question is a useful
complexity-ratchet interrupt for this case. It is not a claim that mathematical
minimality can be proven. The operative engineering rule is that requirements
and correctness are fixed, and among designs that satisfy them, less machinery
and a smaller ownership surface are preferred.

Measured run IDs:

| Intervention | Positive runs | Control runs |
| --- | --- | --- |
| Mechanical question | `dfdbaf2a-7000-40bb-b13a-10e66697419f`, `8743fd3d-2a8c-469d-a6ca-97037d4d5b3f`, `8a9124d3-b731-43a8-b4e3-1fd1dee0782a`, `7b0a1a88-4916-4de1-a9cf-94a0c04fc9de`, `a8cd1953-9e8b-451c-bb07-6a388986614a` | `6f906cfc-16d9-4979-8dcf-81296b8710de`, `48a79a5f-354f-4a68-886c-caecb90fe3f9` |
| Independent PM verdict | `98c01d8c-6294-458a-b3af-b5d2ee5e30b4`, `afcf42a4-9b8e-4bf7-80ea-b12b0fd86fa2`, `bfbcba8e-6bb0-43df-9091-e12b18cda9be`, `9f22258c-a0d0-4dba-b14b-beca6e2b2204`, `71c480cc-63bc-45c7-89b0-d20dcb9a26cb` | `f4f47699-4852-43fd-a573-9bbdf51b281e`, `f78a9ba1-2a55-444f-9e50-0f6e7875c394` |
| Targeted PM challenge | `aeff2e0d-ed66-44a1-91a8-c2d17176d52d`, `d9445243-47b9-4556-8e2e-d0adc9907f54`, `31a6c53c-0c68-4f09-a51d-c7434f54b1f5`, `0cd44b23-7446-4b1e-94a1-13e2334a777c`, `949b27f0-f948-4640-b7b9-2f89a556a8c7` | `df9b5222-c3d2-429b-b13d-0fc79f8244e8`, `f1bbdedd-7978-4913-b28d-756e3c051396` |

The experiment also exposed two Bro operational findings. First, the initial
reviewer lanes treated a valid `changes_requested` outcome as attempt
exhaustion because their reviewer node had `max_attempts: 1`; those runs were
parked, the node contract was corrected to give review outcomes an unbounded
outcome budget, and all reviewer measurements above were started fresh.
Second, the Linux host reported 20 CPU cores and 128 GB RAM, but Bro's global
agent limit remained hard-coded at ten. The measured batch saturated that
limit while healthy work remained queued. Capacity must become an explicit
host/operator setting rather than an embedded constant; that repair was not
mixed into the behavioral experiment.

## Acceptance still open

The following claims remain open and must not be inferred from the evidence
above:

- complete F1 forest/index feasibility under every written report contract;
- complete F2 interactive, colony, and catastrophic mutation workloads;
- the final resident graphics-memory product target;
- required milestone captures and human visual review;
- a released stable consumer API;
- a complete downstream game.

In particular, the current benchmark executable deliberately writes a failed
report when complete scenario evidence has not been captured. That is
fail-closed behavior, not a passing benchmark hidden behind an incomplete
runner.

## What this demonstrates

Moria and cargo-reapi together provide inspectable evidence for work involving:

- Rust and Bevy architecture with a deliberately bounded public API;
- deterministic world/content generation and canonical checked-in artifacts;
- sparse spatial data, bounded query contracts, streaming, and mutation;
- adversarial schema and acceptance-harness design;
- compiler-cache correctness across independent worktrees;
- Linux/macOS sandbox and filesystem behavior;
- resource diagnosis under parallel agent and CI-style workloads.

The strongest claim is not that every Moria milestone is finished. It is that
accepted, pending, and disproven claims are distinguished, measured, and made
reviewable—and that production dogfood failures become durable repairs and
tests.
