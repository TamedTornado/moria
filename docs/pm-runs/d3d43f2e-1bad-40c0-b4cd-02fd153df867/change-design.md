# Change design: substrate-scope correction

Status: **proposed for review**

## 1. Design basis and authority

This design defines only the change from the current repository behavior to the
approved substrate scope. It does not redesign Moria's existing world, query,
mutation, streaming, presentation, or evidence contracts where those contracts
are already substrate-generic.

Requirements and observations use these trace labels:

- **[Authority: Product boundary]**, **[Authority: Capabilities]**,
  **[Authority: Data]**, **[Authority: Acceptance]**,
  **[Authority: Recovery]**, **[Authority: Ordered execution]**, and
  **[Authority: Completion]** refer to the corresponding sections of
  `docs/product-scope-authority.md`.
- **[Analysis: Current shape]**, **[Analysis: Data flow]**,
  **[Analysis: Extension points]**, **[Analysis: Affected areas]**, and
  **[Analysis: Recovery]** refer to the corresponding findings in this run's
  `change-analysis.md`.
- **[Interview: qN]** refers to supplemental clarification in
  `docs/interview-record.md`. The interview is not authoritative where it
  conflicts with the approved scope.

The approved authority wins every conflict. In particular, the interview's
older forest, player, traversal, character, animation, and named-hardware
requirements do not constrain this design.

## 2. Change summary

Moria remains a reusable voxel-world substrate, but its active product contract
will no longer be expressed through one globally enumerated curated forest, a
third-person walkable demo, or a proof whose correctness depends on a named
machine.

The corrected product will:

- open deterministic worlds from compact, versioned generation inputs;
- generate material truth and registered objects only for bounded requests,
  with stable identities independent of activation order;
- retain sparse truth, public bounded reads and edits, exact object indexing,
  mutation attribution, persistence, reconciliation, accounting, and evidence
  protections;
- validate those capabilities with a headless, cross-platform
  `prove-substrate` proof and parameterized generated stress workloads; and
- provide a thin headed diagnostic viewer for human inspection of the
  substrate, not a game or character experience.

This change also corrects active documentation, test and benchmark contracts,
assets, issue planning, contributor guidance, and recovery dispositions so that
rejected scenery assumptions cannot remain prerequisites for generic substrate
work. [Authority: Product boundary; Authority: Capabilities; Authority: Data;
Authority: Acceptance; Authority: Recovery]

## 3. Existing behavior relevant to the change

### 3.1 Product and executable state

- The top-level README already calls Moria a reusable voxel-world substrate and
  excludes player controllers, characters, skeletal animation, and
  game-specific presentation. [Analysis: Current shape]
- The reusable library exposes deterministic terrain evaluation, sparse brick
  and delta primitives, bounded world queries, registered-object shapes and
  indexes, mutation admission, streaming lifecycle primitives, telemetry, and
  evidence schemas. [Analysis: Current shape]
- These primitives are not yet integrated into a complete opened-world runtime:
  the world plugin does not yet execute accepted edits, run end-to-end
  streaming, mesh terrain and objects, persist deltas, or reconcile renderer
  work. [Analysis: Current shape]
- The headed demo currently opens only a Bevy shell with the world plugin. It
  has no player, controller, camera, HUD, menu, or traversal experience to
  preserve or remove. [Analysis: Current shape]
- The benchmark executable currently writes a truthful
  `failed_before_start` report rather than claiming uncaptured evidence. Its
  query-cost probe and report machinery are real, but the advertised scenarios
  are not end-to-end. [Analysis: Current shape]

### 3.2 Current world data and generic contracts

- `WorldIdentity` already combines a seed, parameters digest, and world bounds.
  Deterministic terrain evaluation is integer/fixed-point and tested for
  call-order independence. [Analysis: Data flow]
- Current bounds still encode the old fixed Product One region in several
  places. World opening and base truth also expect globally supplied curated
  features and objects. [Analysis: Data flow]
- Current truth is deterministic base data plus sparse, sorted brick deltas and
  revisions. Public consumers inspect through `WorldRead`, submit mutations
  through `WorldEditWrite`, and provide streaming focus through stable
  focus-source messages. [Analysis: Data flow]
- Registered-object indexes already provide stable sorted query output, bounded
  candidate and affected counts, retained-byte reporting, overlap validation,
  and independent exact-oracle tests. Some public names and assumptions are
  tree, canopy, or forest specific. [Analysis: Extension points]
- Mutation admission exists, but accepted work is not yet committed by an
  integrated runtime. Save/reload is a documented target backed by sparse delta
  primitives, not a completed public workflow. [Analysis: Data flow]

### 3.3 Current conflicting product artifacts

- `moria-curate generate`, `check`, and `prove-forest` operate on a checked-in
  global `CuratedManifest`; the current manifest is about 6.1 MB and enumerates
  the forest population. [Analysis: Current shape]
- Active commands, report schemas, benchmark arguments, TDD, issues, tests,
  configurations, and assets still encode forest counts, species, canopy,
  route, player, explorer, animation, named M4/Metal, or forest-proof
  assumptions. [Analysis: Affected areas]
- Recovery PR #363 contains generic index/report candidates but also a much
  larger generated forest manifest. Recovery PR #365 contains strong truthful
  evidence validation but binds it to forest, route, M4/Metal, and Product One
  assumptions. Neither recovery head is the current baseline. [Analysis:
  Recovery]

## 4. Requested behavior and user-visible outcomes

### 4.1 Product identity

**R1.** All active product-facing material must present Moria as the reusable
voxel-world substrate described by the approved authority. Ecology, a canonical
forest, a player, character assets, animation, curated traversal, and game
presentation must not be requirements or implied completion criteria.
[Authority: Product boundary; Authority: Completion]

**Outcome:** A downstream developer can evaluate Moria as a library substrate
without adopting a shipped world, ecology model, or player stack.

**R2.** The existing public-consumer boundary remains a product requirement:
external consumers, the diagnostic viewer, benchmarks, and proof workflows use
the same supported read, edit, streaming-focus, telemetry, and lifecycle
contracts. No viewer- or benchmark-only privileged world path becomes part of
the product. [Analysis: Data flow; Interview: q9; Authority: Capabilities]

**Outcome:** The included executables demonstrate what a separate game can do
through the reusable substrate rather than hiding required behavior in a
special demo.

### 4.2 Compact world identity and bounded generation

**R3.** Active product data must identify a generated world with:

- a world seed;
- compact procedural-generation parameters;
- explicit generator and schema versions and their digests; and
- only explicitly authored anchors needed by bounded fixtures.

Every input capable of changing authoritative generated truth or stable object
identity must be covered by the world's reproducible identity. [Authority:
Data; Authority: Acceptance]

**Outcome:** A world can be reviewed, transmitted, reopened, and compared
without a checked-in enumeration of its complete object population.

**R4.** Authoritative generation must be requested and completed by a bounded
region or cell. It must not enumerate the complete world before answering a
request, activating content, or constructing a query index. Identical identity
and bounded request inputs must produce identical material truth, registered
objects, stable object identities, and externally observed ordering regardless
of request or activation order. [Authority: Data; Authority: Acceptance;
Authority: Capabilities]

**Outcome:** Activating a small area has work and retained state proportional to
that bounded area and its declared boundary needs, rather than to the total
world extent.

**R5.** Authored fixtures must be small, bounded, and reviewable. Large
populations may be generated reproducibly as parameterized stress output, but
must not be checked in as shipped-world truth. Tree-shaped or forest-looking
fixtures are permitted only as optional bounded data and create no species,
density, ratio, spacing, canopy, or understory requirement. [Authority: Data;
Authority: Product boundary]

**Outcome:** Reviewers can distinguish product inputs from disposable workload
output, and scenery choices cannot silently become substrate invariants.

### 4.3 Preserved generic substrate behavior

**R6.** Scope correction must preserve and generalize these existing
capabilities:

- stable registered-object identities;
- deterministic bounded generation;
- compact spatial indexing and bounded broad-phase queries;
- exact agreement with an independent broad-phase oracle;
- exact mutation dependency attribution;
- object/object and object/stamp overlap safety where presentation needs it;
- invalidation and full reconciliation after mutation;
- authored-versus-derived presentation ownership;
- bounded memory/resource accounting;
- immutable, truthful pass/failure evidence; and
- machine-identified timing and resource measurements.

Species, canopy, ecology, player, character, animation, or route concepts must
not remain in the generic contract for these capabilities. [Authority:
Capabilities]

**Outcome:** Existing generic investment survives the correction, while its
public meaning no longer depends on the forest fixture that originally
exercised it.

**R7.** Mutation outcomes must remain observable through supported product
interfaces: affected regions and affected registered objects are identified
exactly, unaffected objects remain unchanged, and all invalidated derived
presentation reaches a reconciled state. Derived meshes, water meshes,
dressing, and debug geometry remain non-authoritative. [Authority: Product
boundary; Authority: Acceptance; Analysis: Data flow]

**Outcome:** A consumer can edit authoritative matter and verify both the
necessary rebuilds and the absence of unrelated changes.

**R8.** Save/reload must persist sparse authoritative deltas and reproduce the
same material truth for the same versioned world identity. Generated stress
output and derived presentation are not save truth. [Authority: Product
boundary; Authority: Acceptance; Analysis: Data flow]

**Outcome:** Reopening a world after edits restores authoritative results
without serializing the procedurally generated world or render products.

### 4.4 Substrate proof and evidence

**R9.** The active forest-oriented proof is replaced by a substrate-oriented
proof named `prove-substrate`. It must demonstrate:

- repeatable generation and query results from identical inputs;
- sparse activation without complete-world expansion;
- stable object identities and bounded queries;
- exact broad-phase agreement with an independent oracle;
- exact mutation attribution;
- unchanged unaffected objects;
- complete mesh/render invalidation and reconciliation;
- save/reload reproduction of material truth;
- honest retained-memory measurement; and
- timing evidence with complete machine identity.

[Authority: Acceptance]

**Outcome:** A successful proof establishes reusable substrate behavior without
claiming that any particular forest or route is the product.

**R10.** F1 is a cross-platform, headless correctness, resource-accounting, and
evidence-integrity stage. Platform-specific timings and resource measurements
are labeled evidence; a result must not fail functional correctness solely
because it was not produced on one named machine, GPU API, or resolution.
[Authority: Acceptance]

**R11.** Proof and benchmark artifacts must bind their results to the exact
world identity, generator/schema versions and digests, fixture or workload
parameters, proof/report version, stage accounting, and complete machine
identity. They must distinguish not-started, partial, failed, and passed work;
incomplete or inconsistent evidence must never serialize as a pass. [Authority:
Capabilities; Authority: Recovery; Analysis: Recovery]

**Outcome:** Maintainers can compare performance in its hardware context while
relying on pass/failure state and workload identity.

**R12.** Large-population scalability is exercised through reproducible,
parameterized generated workloads. Population counts are recorded workload
inputs, not shipped-world acceptance requirements. Generated workload artifacts
belong outside the checked-in product data unless a small bounded fixture is
explicitly reviewed for inclusion. [Authority: Acceptance; Authority: Data]

### 4.5 Headed diagnostic inspection

**R13.** F2 is a headed human inspection stage showing:

- three-dimensional terrain and underground geometry;
- bounded activation, streaming, and eviction behavior;
- edits to authoritative material truth;
- diagnostic voxel/material views; and
- registered-object invalidation and rebuilding after relevant edits.

F2 must not require a canonical route, third-person controller, player
locomotion, explorer mesh, skeleton, animation, or game-camera behavior.
[Authority: Product boundary; Authority: Acceptance]

**Outcome:** A reviewer can directly inspect continuous, mutable
three-dimensional voxel truth and its derived rebuilding without mistaking the
viewer for a game deliverable.

**R14.** The headed executable is a thin diagnostic viewer. A free diagnostic
camera and diagnostic controls for inspection, streaming focus, edits, and
views are permitted. Any physical input labels, screen chrome, or scene dressing
serve diagnostics only and do not establish gameplay or ecology requirements.
[Authority: Product boundary; Analysis: Extension points]

### 4.6 Repository and recovery correction

**R15.** Active design, TDD, issue, test, benchmark, asset, configuration,
command, and contributor-guidance artifacts must be inventoried as keep,
generalize, remove, or historical/superseded. Conflicting historical artifacts
remain available as project history but must be clearly marked so they cannot
be read as active authority. [Authority: Ordered execution; Authority:
Completion]

**R16.** The checked-in global forest manifest must be removed from active
product data and must not be recreated from PR #363. Removal must be coordinated
with replacement compact inputs, commands, tests, and documentation so the
ordinary repository gate is not left referring to a missing contract.
[Authority: Data; Authority: Recovery; Analysis: Affected areas]

**R17.** PR #363 is inventoried per change as substrate-generic and salvageable,
forest/ecology-specific and rejected, or generated and discarded. PR #365 is
audited to preserve generic evidence protections while removing forest,
third-person, named-machine, and universal-platform assumptions. Approved work
is adapted onto clean current `master`; neither recovery PR is merged or
rebased wholesale. [Authority: Recovery]

**R18.** Recovery epic #325 closes only when every recovery item has an explicit
merged, adapted, or superseded disposition. [Authority: Recovery]

## 5. Scope

### In scope

- Correcting the active product boundary and vocabulary.
- Replacing global enumerated product data with compact, versioned world inputs
  and small bounded fixtures.
- Making bounded deterministic generation and stable identity the product
  contract for activated content.
- Generalizing existing object, query, mutation, reconciliation, persistence,
  accounting, benchmark, and evidence requirements.
- Replacing forest proof identity and semantics with `prove-substrate`.
- Defining corrected F1 headless and F2 headed user-visible outcomes.
- Reclassifying active assets, configurations, tests, documents, commands,
  issues, and recovery work against the corrected boundary.
- Preserving the reusable substrate/public-consumer boundary.

### Non-goals

- Ecology simulation or canonical forest area, population, diversity, ratios,
  density, spacing, canopy, or understory.
- A globally enumerated curated object population.
- A player, third-person controller, locomotion, swimming, game camera, human
  explorer asset, skeleton, skinning, animation, or curated traversal
  experience.
- Combat, AI, game rules, the System/LLM layer, spells, gas, building, weather,
  seasons, growth, object dynamics, granular settling, or fluid flow/pressure
  simulation.
- Making one named machine's timing, graphics API, or resolution a
  platform-independent correctness condition.
- Choosing a new engine, framework, storage technology, serialization library,
  render pipeline, crate topology, or implementation algorithm in this product
  design.
- Treating optional tree-shaped fixtures or attractive diagnostic scenery as a
  renewed ecology commitment.
- Implementing the corrected substrate before reviewed design, TDD, issue
  decomposition, and test specifications authorize it.

[Authority: Product boundary; Authority: Ordered execution; Interview: q9]

## 6. Affected users and workflows

| User or workflow | Existing experience | Changed outcome |
|---|---|---|
| Downstream Rust/game developer | Reusable public contracts exist, but active product data and acceptance still imply one Product One forest and player demo. | Consumes a scenery-neutral substrate identified by compact versioned inputs and exercises the same public interfaces as included tools. |
| Substrate maintainer | Runs forest manifest generation/checking and reads forest/M4-oriented gates and planning artifacts. | Works from corrected substrate contracts, bounded fixtures, `prove-substrate`, and an explicit keep/generalize/remove/supersede inventory. |
| Evidence or performance reviewer | Receives truthful report foundations, but proof identity and validation are forest-, route-, and machine-bound. | Receives digest-bound workload reports with immutable status, complete machine identity, and a separation between functional correctness and machine-specific performance. |
| Headless CI/platform runner | Runs ordinary Rust gates and manifest checking; corrected F1 is not integrated. | Runs cross-platform substrate correctness and evidence checks without requiring a window or one named GPU/machine. |
| Headed human reviewer | The demo is currently an empty shell; old documents promise a third-person curated run. | Uses a free-camera diagnostic viewer to inspect terrain, depth, streaming, edits, voxel views, and object rebuilding. |
| Recovery reviewer | Two recovery PRs mix valuable generic protections with rejected forest/demo assumptions. | Records per-item disposition and ports only reviewed generic work to current `master`. |

## 7. Affected product surfaces

### Commands and generated artifacts

- The active `prove-forest` command and forest-proof inputs/outputs are
  superseded by the `prove-substrate` proof identity and substrate workloads.
- Forest-specific benchmark scenario names, route coverage, curated-seed
  restrictions, universal M4/Metal labels, and `--forest-proof` linkage are no
  longer active product requirements.
- Compact-input generation/check commands may remain where they validate
  versioned parameters or small bounded fixtures; they must not regenerate or
  require a complete-world population.
- Ordinary format, type, lint, test, and build workflows remain relevant.
  Exact corrected proof/benchmark CLI flags and file names belong in the TDD
  and issue plan, not this product design.

### Public APIs and integrations

- Existing `WorldRead`, `WorldEditWrite`, world identity, lifecycle,
  streaming-focus, query, telemetry, and registered-object capabilities remain
  the consumer-facing seams.
- Public ecology-specific types or report names are affected where they make
  species, forest, canopy, route, player, or named hardware part of a generic
  contract.
- The diagnostic viewer, proof tool, benchmark tool, and future external game
  remain consumers of the supported substrate facade.
- No network service or third-party integration is introduced by this change.

### Screens and viewer interaction

- There is no current implemented product screen or controller flow to migrate.
- The only new user-facing surface in this delta is the headed diagnostic
  inspection experience. It needs controls and visible diagnostics sufficient
  for R13, but no menu hierarchy, HUD, gamepad requirement, player avatar, or
  curated traversal is specified here.

### Assets and configuration

- The global curated manifest, explorer assets, animation contracts, player
  tuning, gameplay input bindings, route data, and ecology-specific product
  parameters lose active status.
- A terrain/object asset may remain only if it serves substrate presentation or
  a small bounded fixture and is not coupled to ecological acceptance.
- Generator/schema identity, compact parameters, explicit bounded anchors, and
  small reviewable fixtures become the active product-data categories.

## 8. Acceptance criteria

The change is accepted when all of the following are true.

### Scope and data

**AC1.** Every active product, design, TDD, contributor, issue, test, benchmark,
asset, and configuration contract is either consistent with R1 or explicitly
marked historical/superseded. A repository search may still find rejected terms
in retained history, but no active gate or dependency treats them as required.

**AC2.** No complete enumerated forest population is a checked-in product
artifact. Active world identity is reconstructible from a seed, compact
parameters, generator/schema versions and digests, and any reviewed bounded
anchors.

**AC3.** Given the same versioned world identity and bounded request set,
repeated runs produce identical authoritative material truth, registered-object
identities, query results, and externally observed ordering. Reordering,
repeating, activating, evicting, and reactivating requests does not change those
results.

**AC4.** A proof workload activating a bounded subset reports bounded work and
retained state and demonstrates that it did not enumerate or retain the
complete world population.

### Queries, mutation, and persistence

**AC5.** Registered-object IDs remain stable; every bounded broad-phase result
is sorted and agrees exactly with an independent exact oracle for the same
workload.

**AC6.** For every accepted proof edit, the reported affected region and
affected-object set are exact. Objects outside that set retain their prior
authoritative and derived identity, while every affected derived presentation
item eventually reports reconciled.

**AC7.** Saving and reloading the same supported versioned world plus deltas
reproduces authoritative material truth and query-visible results. The save
does not depend on serialized derived meshes, dressing, debug geometry, or
generated stress output.

### Proof and evidence

**AC8.** `prove-substrate` covers every capability listed in R9. A missing,
partial, contradictory, digest-mismatched, or unreconciled stage produces a
non-pass result and an explicit reason.

**AC9.** The F1 proof runs headlessly on the approved cross-platform matrix and
produces the same correctness results for the same inputs. Reports record the
complete machine identity and machine-specific timings/resources without using
one named machine as the sole correctness authority.

**AC10.** Each proof or stress artifact identifies the world, generator/schema,
fixture/workload parameters, report format, stages, outcome, and machine
profile. Reported population counts reconcile with the generated workload and
are not compared to a canonical shipped-world count.

**AC11.** Stress workloads are reproducible from recorded parameters and write
large output outside checked-in product data. Changing a population-count
parameter changes the workload, not Moria's product scope.

### Headed inspection and integration

**AC12.** An F2 reviewer can use the headed diagnostic build to inspect
three-dimensional surface and underground truth, observe streaming changes,
edit matter, switch to a diagnostic voxel/material view, and observe a
registered object's invalidation and complete rebuilding.

**AC13.** F2 can pass without a player entity, character asset, animation,
third-person controller, curated route, species ratio, forest density, or
named-machine timing threshold.

**AC14.** The viewer, proof, and benchmark obtain authoritative reads and edits
through the supported consumer interfaces. No one-off privileged product path
is required to pass F1 or F2.

### Migration and recovery

**AC15.** PR #363 and PR #365 have reviewed per-change dispositions. The global
forest population is discarded, approved generic protections are adapted to
current `master`, and no wholesale merge/rebase is used as the recovery method.

**AC16.** Epic #325 has a complete merged/adapted/superseded ledger for every
recovery item.

**AC17.** The corrected repository passes its ordinary Linux Rust gates from a
clean checkout of remote `master`; corrected compact-data checks and F1 are
included once their reviewed contracts exist. F2 has a recorded headed macOS
human-review result. [Authority: Ordered execution; Authority: Completion]

## 9. Open questions and proposed defaults

These defaults are product-level assumptions for the next TDD. They do not
select an implementation technique.

1. **World extent.** Must an opened world be finite, or effectively unbounded?
   **Proposed default:** retain a configurable finite `WorldBounds` in the
   corrected first proof because it is current behavior; remove the fixed
   Product One dimensions. The bounded-generation contract must not prevent a
   later cell-addressed extent.

2. **Bounded generation unit and seams.** What request unit and boundary context
   make adjacent results agree? **Proposed default:** the TDD selects and names
   one authoritative bounded unit and its maximum required boundary context.
   Product acceptance observes only determinism, seam agreement, bounded work,
   and order independence.

3. **Stable object identity.** What versioned inputs define an object's ID?
   **Proposed default:** identity is a deterministic function of all
   generation-affecting world identity and bounded candidate identity, and is
   independent of activation order. The precise derivation is a TDD decision.

4. **Digest coverage.** Which inputs are identity-bearing? **Proposed default:**
   include generator/schema versions, every authoritative generation
   parameter, world bounds, and authored anchors; exclude machine identity,
   diagnostic presentation, and stress-only population parameters unless they
   actually define the generated fixture under proof.

5. **Required fixtures.** Which current ruin, tree, boulder, or vegetation
   assets survive? **Proposed default:** require the smallest neutral bounded
   fixtures needed to prove registered-object generation, overlap, mutation,
   and rebuilding. Retaining any named scenery asset requires an explicit
   fixture purpose; no tree species is required.

6. **Index ownership lifetime.** Is a registered-object index scoped to a cell,
   active neighborhood, or another bounded set? **Proposed default:** leave this
   to TDD, with product acceptance enforcing bounded retained memory, exact
   results, stable IDs, and correct activation/eviction behavior.

7. **Save compatibility.** Must unaccepted historical manifests or delta files
   reload? **Proposed default:** no compatibility promise for generated forest
   artifacts or an unimplemented public save format. The corrected format must
   be explicitly versioned and must fail clearly on unsupported identity/schema
   versions.

8. **F1 cross-platform matrix.** Which hosts are required? **Proposed default:**
   require at least two materially distinct supported host stacks, including
   the ordinary Linux headless gate and the macOS environment used for F2.
   The TDD must name exact versions and distinguish required correctness from
   informative performance.

9. **Resource limits and performance policy.** Which numeric thresholds remain
   blocking? **Proposed default:** deterministic capacity and bounded-work
   violations are correctness failures. Timing and hardware-resident resource
   figures remain machine-labeled evidence until separate per-profile targets
   are reviewed; old universal M4/3060 thresholds do not carry forward
   implicitly.

10. **Stress workload matrix.** What population sizes and spatial
    distributions are required? **Proposed default:** define at least a small
    correctness workload and one larger scalability workload, both entirely
    parameterized and digest-bound. Do not encode either count as world lore.

11. **F2 record.** What captures a headed human pass? **Proposed default:** a
    checklist against AC12 plus build/world/report identity and reviewer
    disposition. A fixed route or avatar is unnecessary; captures may be added
    as supporting evidence.

12. **Public API compatibility.** Must pre-release ecology-specific names remain
    as deprecated aliases? **Proposed default:** preserve generic behavior, not
    contaminated terminology. The TDD should inventory externally visible
    changes and choose removal or a short migration period based on actual
    downstream use.

13. **Historical document marking.** What is the standard marker?
    **Proposed default:** keep the original content intact with a prominent
    superseded status that names `docs/product-scope-authority.md` and the
    corrected active replacement.

## 10. Explicit divergences from source documents

There is no intentional divergence from the committed
`docs/product-scope-authority.md`. Proposed defaults above fill questions the
authority deliberately leaves open and remain subject to review.

This design intentionally diverges from the supplemental interview record where
that record preserves the superseded Product One concept:

| Supplemental statement | Design disposition | Reason |
|---|---|---|
| A generated 1 km forest/river/cliff/cave/ruin region is the required seed world. [Interview: q2, q3] | Rejected as a product requirement. Such scenery may appear only in small bounded fixtures or headed diagnostics. | The authority rejects canonical forest/ecology and globally curated scenery requirements. |
| A third-person character must run, sprint, jump, swim, collide, carry a light, and traverse a curated vertical route. [Interview: q1, q2, q5] | Rejected. F2 uses a free diagnostic camera and substrate inspection. | The authority explicitly excludes player locomotion, game-camera behavior, human explorer assets, animation, and curated traversal. |
| Species counts, dense canopy, forest dressing, and route content are acceptance stress. [Interview: q3, q4] | Generalized to parameterized bounded object workloads and optional fixtures. | Population counts are workload parameters, not shipped-world requirements. |
| M4/Metal and 3060-class timing thresholds define product correctness. [Interview: q6] | Superseded as universal correctness. Machine-identified timing remains evidence and may support separately reviewed hardware profiles. | F1 correctness is cross-platform; one named machine is not universal authority. |
| Milestones culminate in a playable run and social-media trailer, with a 2–3 week estimate. [Interview: q7, q8] | Not part of the corrected acceptance or schedule. | Moria is a substrate and diagnostic viewer, not a playable game or curated audience artifact. |
| Dig/place proves mutability through a player action. [Interview: q2, q5] | Preserved as diagnostic editing, without gameplay semantics. | The substrate still requires mutation and F2 inspection, while the player is out of scope. |
| The reusable crate and separate consumer boundary are mandatory. [Interview: q9, q10] | Preserved. | This is consistent with the authority and current repository shape. |

The current repository also diverges from the desired end state in ways this
design makes explicit rather than concealing: the demo is not yet a diagnostic
viewer, mutation and persistence are not integrated, benchmark scenarios are
not complete, and the global manifest/proof remain forest-oriented. Those gaps
are implementation and migration work to be specified after this design is
reviewed. [Analysis: Current shape]
