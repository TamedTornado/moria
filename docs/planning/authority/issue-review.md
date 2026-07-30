## Issue Review Turn — 2026-07-29T21:55:19Z

**Verdict:** changes_requested

### Review basis

Reviewed the sealed sources and reference inputs in
`docs/planning/authority/source-manifest.json`, the authority ledger, approved
product vision and design decisions, all 43 `REQ-###` blocks in
`docs/design-document.md`, the complete approved TDD and its final review
ledger, `docs/tdd/traceability.md`, and all 113 entries in `docs/issues.json`.
All sealed source and authority-package SHA-256 digests match the manifest.

Mechanical checks found 70 active technical contracts
(`TECH-001`–`TECH-062`, `TECH-064`–`TECH-071`), 346 exact normative
contract/requirement pairs with traceability parity, and the approved
supersession of `REQ-039`/retirement of `TECH-063`.

### Findings

1. **No issue implements active `TECH-060`; `M-113` can therefore approve a
   completion gate whose mandatory headless suite was never decomposed.**

   - **Issue IDs:** no current issue ID for the missing work; existing affected
     gate issue `M-113`.
   - **Evidence:** `docs/tdd/validation.md` defines `TECH-060 — Headless Bevy
     contract tests` and requires the facade/admission/receipt/query/
     observation/lifecycle/shutdown state matrices, genesis numbering,
     provider and replay completion behavior, participant policy rows,
     bounded-owner/public-closure checks, configuration cross-limit fixtures,
     and source/AST lints. No issue's
     `provenance.technical_contracts` contains `TECH-060`. `M-113` implements
     `TECH-069`, whose completion gate explicitly requires all headless tests,
     but it has no dependency that produces the `TECH-060` suite.
   - **Required changes:** add dependency-ordered, independently testable
     issues for the distinct `TECH-060` proof slices; do not place the whole
     contract in one catch-all issue. At minimum separate (a) headless Bevy
     runtime/genesis/query/observation/lifecycle state machines, (b)
     provider/store/replay/configuration-boundary fixtures, (c) participant
     and external-facade ownership/failure-policy fixtures, and (d) mechanical
     public-closure and source/AST checks. Each new issue must cite only
     `TECH-060`, use the exact design-requirement set `REQ-002`, `REQ-005`,
     `REQ-012`, `REQ-015`, `REQ-016`, `REQ-023`, and use the mechanically
     derived exact authority set `C-003`, `C-013`, `C-010`, `AD-003`, `D-002`,
     `D-007`, `D-006`, `D-005`, `C-004`, `D-009`, `C-015`. Give each slice
     explicit produced test modules, concrete implementation dependencies,
     failure-state assertions, and exact acceptance criteria. The Rust
     normative-snippet/AST slice must name and pin a maintained Rust parser
     such as `syn` in both Inputs and Acceptance Criteria; it may not grow a
     shared home-grown Rust parser. Reuse the approved Naga path for WGSL
     inspection. Add every new `TECH-060` issue to `M-113.depends_on`, and make
     `M-113`'s acceptance criteria explicitly reject absent or incomplete
     `TECH-060` evidence.

2. **Issues `M-079`–`M-083` cite `TECH-059`, but their combined acceptance
   criteria omit three mandatory, independently testable proof obligations.**

   - **Issue IDs:** `M-079`, `M-080`, `M-081`, `M-082`, `M-083`; downstream
     completion gate `M-113`.
   - **Evidence:** `TECH-059` requires decoder fuzzing for truncation, trailing
     bytes, invalid tags/counts, zip bombs, corrupt hashes, and allocation
     overflow; schedule perturbation plus placement-split/configuration-
     fingerprint replay/restore mismatch proofs; and a participant RNG oracle
     covering seed decoding, every state/output step, snapshot and
     reconstruction bytes, rollback/checkpoint/replay, exhaustion, undeclared
     streams, and absence of Moria-owned entropy. `M-079`/`M-080` cover fixed
     math, `M-081` covers logical transition/sparse state, `M-082` covers
     commitments/replay, and `M-083` covers collision, but none names those
     three required slices in Description or Acceptance Criteria. Component
     decoder tests elsewhere do not satisfy `TECH-059`'s independent-oracle
     proof and do not cite that contract.
   - **Required changes:** add separate dependency-ordered `TECH-059` issues
     for (a) bounded canonical/persistence/replay decoder fuzzing, (b)
     schedule/configuration-fingerprint perturbation and mismatch proof, and
     (c) the participant RNG reference/golden suite. Do not broaden
     `M-079`–`M-083` or convert these into shared format-validation
     infrastructure. Each new issue must use exactly `REQ-007`, `REQ-021`,
     `REQ-023`, `REQ-036`, `REQ-038` and exactly `C-015`, `D-003`, `D-004`,
     `C-003`, `D-002`, `D-005`, `D-006`, `D-007`, `D-008`, `D-009`, `C-013`
     as structured and body provenance. Name the selected maintained
     property/fuzzing tool in both Inputs and Acceptance Criteria. Also change
     `M-079` from the nonbinding phrase “such as `rug`” to one selected,
     pinned maintained arbitrary-precision crate and name that crate in its
     Acceptance Criteria. Make `M-113` depend on the new proof issues.

3. **`M-084` and `M-085` do not completely and safely decompose
   `TECH-061`.**

   - **Issue IDs:** `M-084`, `M-085`.
   - **Evidence:** `TECH-061` requires the shader-validation command to
     regenerate and validate the `TECH-071` CPU/WGSL fixed-math sources and
     CORDIC table for every supported fractional split, in addition to Naga
     parse/validation/reflection and the contamination inventory. `M-084`
     omits the fixed-math/table regeneration from its Description and
     Acceptance Criteria and does not depend on generator issues `M-079` and
     `M-080`. Its Inputs name Naga, but its Acceptance Criteria do not name the
     established validator. `M-085` describes a transitive WGSL audit without
     requiring that WGSL parsing/module discovery consume matching Naga IR,
     leaving room for a home-grown WGSL parser or regex validator.
   - **Required changes:** add `M-079` and `M-080` to `M-084.depends_on` and its
     Inputs, and require exact regeneration/byte comparison of fixed-math
     sources and the CORDIC table for every supported split in `M-084`'s
     Description and Acceptance Criteria. Name matching `naga = 29.x`
     explicitly in both `M-084` Inputs and Acceptance Criteria. In `M-085`,
     require the WGSL inventory and analysis to consume the matching Naga
     parsed/validated module graph or IR, name Naga in Inputs and Acceptance
     Criteria, and limit custom code to the TDD-specific contamination rules;
     do not implement a WGSL parser or general validator. Keep both issues
     within their current technical contracts and exact mechanically derived
     provenance sets.

### Checks that passed

- All 113 issue IDs are unique. Every dependency target exists; there are no
  self-dependencies, cycles, or dependency-order inversions.
- For every current issue, the union of each cited technical contract's
  `Implements:` IDs exactly equals structured
  `provenance.design_requirements`; the union of each cited requirement's
  `Authority:` IDs exactly equals structured `provenance.authority`. There are
  no unknown IDs or duplicate provenance members.
- Every current issue's claimed implementation and Inputs citations match its
  structured provenance. `M-111`'s additional `TECH-067` mention is a
  non-implementation boundary that correctly leaves the tier judgment to
  `M-104`; it is not false provenance.
- The current issues contain no unauthorized game, character, controller,
  authored route, forest population, generator, physics/damage policy,
  networking, ship/station delivery, cross-machine qualification matrix,
  hardware-universal gate, or privileged consumer path.
- Existing custom checkpoint, replay, and canonical wire parsers are
  explicitly selected versioned TDD formats, not accidental replacements for
  standardized external formats. `M-112` correctly names `serde_json`;
  `M-084`'s Naga acceptance wording remains a required correction above.
- Apart from the missing/underspecified validation slices above, issue scope,
  produced artifacts, integration inputs, failure-mode slicing, and primary
  acceptance criteria are independently testable and remain within their
  cited technical contracts.

### Authority-boundary disposition

All findings are correctable in `docs/issues.json`. No approved requirement,
technical contract, authority ID, decision, source-manifest entry, or digest
needs to change. This turn therefore requests manifest changes rather than
failing for upstream human resolution.

## Coder Response Turn — 2026-07-29T22:04:57Z

**Answering:** `Issue Review Turn — 2026-07-29T21:55:19Z`

**Verdict:** addressed

### Responses to findings

1. **Missing `TECH-060` decomposition — addressed.**

   Added eleven dependency-ordered issues, each citing only `TECH-060` and the
   exact required `REQ-002`, `REQ-005`, `REQ-012`, `REQ-015`, `REQ-016`,
   `REQ-023` / `C-003`, `C-013`, `C-010`, `AD-003`, `D-002`, `D-007`,
   `D-006`, `D-005`, `C-004`, `D-009`, `C-015` chain:

   - `M-116` covers headless admission, receipt, cancellation, lifecycle,
     generation, missing-renderer, and shutdown state machines.
   - `M-117` covers genesis numbering, query readiness/unavailability,
     collision/frontier identity, and interest-pin behavior.
   - `M-118` covers historical observation filtering, gaps, bounded
     resnapshot, resume, and correlation expiry.
   - `M-119` covers base-content completion ownership and provider registry
     freeze.
   - `M-120` covers content/checkpoint/replay store completions, manifest
     framing and visibility, replay stream failure, retirement, and restore
     continuation.
   - `M-121` covers generated resource/configuration cross-limit fixtures and
     the exact default-budget smoke/rejection cases.
   - `M-122` covers CPU/GPU participant ownership, completion, failure-policy,
     event, and lifecycle rows.
   - `M-123` covers external-crate callable use, bounded owners, checked
     identities, admission/readiness/telemetry/failure pattern matching, and
     facade non-escape.
   - `M-124` covers normative Rust public closure with the selected pinned
     `syn = "=2.0.106"` parser.
   - `M-125` covers canonical Rust source boundaries with
     `syn = "=2.0.106"`.
   - `M-126` covers canonical WGSL source boundaries with matching
     `naga = 29.x` parsed/validated IR.

   The former completion gate `M-113` is now `M-127`. This is the only
   existing ID changed: newly required proof issues must precede their gate,
   and the manifest requires sequential IDs and dependencies only on earlier
   IDs. `M-127` depends on every new `TECH-060` issue and explicitly rejects
   absent, incomplete, skipped, or failing evidence for each named slice.

2. **Incomplete `TECH-059` proof obligations — addressed.**

   Added three separate issues, each with exactly the required `TECH-059`,
   requirement, and authority provenance:

   - `M-113` fuzzes bounded canonical, checkpoint, replay, and active-history
     decoders for truncation, trailing bytes, invalid tags/counts, zip bombs,
     corrupt hashes, and allocation overflow through public decoders/loaders.
   - `M-114` proves schedule perturbation and placement-split,
     simulation-unit, cell-extent, configuration-fingerprint, replay, and
     restore mismatch behavior.
   - `M-115` supplies the independent participant RNG reference and golden
     suite across seed/state/output, snapshot/reconstruction, rollback,
     checkpoint, replay, exhaustion, undeclared streams, and absence of
     Moria-owned entropy.

   All three name `proptest = "=1.7.0"` in Inputs and Acceptance Criteria.
   `M-113` is explicitly an artifact-local public-loader fixture and does not
   introduce shared parsing or validation infrastructure. `M-079` now selects
   and pins `rug = "=1.27.0"` in both Inputs and Acceptance Criteria. `M-127`
   depends on `M-113`–`M-115` and rejects absent or incomplete evidence for
   each.

3. **Incomplete `TECH-061` decomposition — addressed.**

   `M-084` now depends on and names outputs from `M-079` and `M-080`; its
   Description and Acceptance Criteria require regeneration and exact byte
   comparison of TECH-071 CPU/WGSL fixed-math sources and the CORDIC table for
   every supported fractional split. Matching `naga = 29.x` is named in its
   Description, Inputs, and Acceptance Criteria.

   `M-085` now requires inventory and contamination analysis to consume the
   matching Naga parsed/validated module graph or IR, names
   `naga = 29.x` in Inputs and Acceptance Criteria, and limits custom code to
   the approved contamination/dataflow rules rather than WGSL parsing or
   general validation. Both issues retain their prior technical-contract,
   requirement, and authority provenance.

### Exact `docs/issues.json` changes

- Preserved `M-001` through `M-112`.
- Added `M-113` through `M-126` as the fourteen independently reviewable
  `TECH-059` and `TECH-060` proof issues described above.
- Renumbered only the former final gate from `M-113` to `M-127`, added all
  fourteen new dependencies to its structured and body dependency lists, and
  added fail-closed acceptance criteria for complete `TECH-059`/`TECH-060`
  evidence.
- Replaced the optional arbitrary-precision wording in `M-079` with pinned
  `rug = "=1.27.0"` Inputs and Acceptance Criteria.
- Added `M-079` and `M-080` dependencies and fixed-math/CORDIC regeneration
  obligations to `M-084`.
- Added Naga-IR discovery/analysis and no-home-grown-parser boundaries to
  `M-085`.

### Authority-boundary disposition

No finding requires a source, requirement, technical contract, claim,
decision, TDD, authority ledger, source manifest, or digest change. All
findings were satisfied solely in `docs/issues.json`; there is no unsatisfied
finding or upstream-authority conflict requiring human resolution.

## Issue Review Turn — 2026-07-29T22:14:44Z

**Verdict:** changes_requested

### Review basis

Reviewed the sealed sources and reference inputs named by
`docs/planning/authority/source-manifest.json`, the authority package, all 43
`REQ-###` blocks in `docs/design-document.md`, all 70 active technical
contracts in the complete approved TDD, `docs/tdd/traceability.md`, all 127
entries in `docs/issues.json`, and the preceding review and coder-response
turns in this ledger. All sealed source and approved-artifact SHA-256 digests
match the source manifest.

Mechanical traversal found 127 sequential unique issue IDs, no unknown
dependency targets, self-dependencies, forward dependencies, or cycles, and
no uncovered active technical contract. For every issue, the union of the
cited contracts' `Implements:` IDs exactly equals structured
`provenance.design_requirements`, and the union of those requirements'
`Authority:` IDs exactly equals structured `provenance.authority`. Structured
REQ and authority sets are cited exactly in each body.

### Findings

1. **The new `TECH-060` decomposition still omits two explicitly mandatory
   headless proof slices.**

   - **Issue IDs:** `M-116`, `M-127`; there is no current issue ID for either
     missing proof slice.
   - **Evidence:** `TECH-060` expressly requires both “private correction
     success/abort” and “completion-bridge
     reservation/exhaustion/duplicate/old-generation drain”
     (`docs/tdd/validation.md`, lines 102–103). `M-116`'s Description and
     Acceptance Criteria cover admission, receipt-family rows, tick-global
     failures, device loss, shutdown, and missing `RenderApp`, but neither
     named obligation. No other `M-113`–`M-126` Description or Acceptance
     Criteria names private-correction success/abort or completion-bridge
     reservation/exhaustion/duplicate/old-generation drain. `M-122` correctly
     covers participant snapshot-export failure, but that is a different
     adjacent `TECH-060` obligation. `M-127` can therefore accept an
     implementation-completeness result without evidence rows for the two
     missing slices.
   - **Required changes:** do not broaden `M-116` with two unrelated implicit
     obligations. Add independently testable, dependency-ordered `TECH-060`
     issues for (a) private correction success and abort through the actual
     correction transaction/public integration path and (b) completion-bridge
     reservation, exhaustion, duplicate completion, and old-generation drain
     through controlled headless completions. The correction proof must
     consume the outputs of at least `M-049`, `M-077`, and the headless
     lifecycle harness; the bridge proof must consume the applicable
     publication, bounded-pool, completion/generation, recovery, facade, and
     headless-harness outputs (`M-060`, `M-065`, `M-066`, `M-067`, `M-077`,
     and `M-116`). Each issue must cite only `TECH-060`, exactly
     `REQ-002`, `REQ-005`, `REQ-012`, `REQ-015`, `REQ-016`, `REQ-023`, and
     exactly `C-003`, `C-013`, `C-010`, `AD-003`, `D-002`, `D-007`, `D-006`,
     `D-005`, `C-004`, `D-009`, `C-015`, in both structured provenance and
     body citations. Give each slice a concrete produced test module and exact
     success/failure/state/resource-release assertions. Make the current
     completion gate `M-127` (or its sequentially renumbered successor) depend
     on both new issues, enumerate both evidence rows, and reject either row
     when absent, incomplete, skipped, or failing.

2. **`M-124` constrains Rust parsing but leaves discovery of normative Rust
   snippets in standardized Markdown open to a home-grown parser.**

   - **Issue ID:** `M-124`.
   - **Evidence:** `M-124` must mechanically find and parse every normative
     Rust snippet from the approved Markdown TDD. Its Inputs and Acceptance
     Criteria select `syn = "=2.0.106"` for Rust syntax and prohibit a custom
     Rust parser, but `syn` does not parse Markdown or identify fenced code
     blocks. The issue names no maintained CommonMark parser and does not
     prohibit regex/manual fence scanning. Its proof obligation can therefore
     expand into an unapproved home-grown parser for a standardized external
     format.
   - **Required changes:** in `M-124` Inputs and Acceptance Criteria, name and
     pin one maintained CommonMark parser (for example,
     `pulldown-cmark`) to enumerate the approved Rust-tagged fenced blocks;
     retain pinned `syn` for the extracted Rust AST. Require fail-closed,
     complete snippet discovery and prohibit regex, a custom Markdown
     tokenizer/fence scanner, or shared general Markdown/Rust validation
     infrastructure. Keep the issue within `TECH-060` and its existing exact
     provenance sets.

### Checks that passed

- The coder response fully closes the previously identified `TECH-059`
  decoder-fuzz, schedule/configuration-perturbation, and participant-RNG
  slices. `M-079` selects pinned `rug = "=1.27.0"`, and `M-113`–`M-115`
  select pinned `proptest = "=1.7.0"` in both Inputs and Acceptance Criteria.
- `M-084` now depends on the fixed-math/orientation oracle work, names matching
  Naga in Inputs and Acceptance Criteria, and requires exact source/table
  regeneration. `M-085` consumes matching Naga IR and limits custom work to
  the approved contamination rules.
- The remaining `TECH-060` slices are separated into independently testable
  runtime/genesis-query/observation/provider/store/configuration/participant/
  external-facade/public-closure/source-lint issues with explicit integration
  inputs and fail-closed criteria. `M-125` names pinned `syn`, and `M-126`
  consumes matching Naga IR rather than implementing a WGSL parser.
- Extra body mentions of `TECH-071` in `M-084`, `TECH-021` in `M-116`,
  `TECH-053` in `M-117`, `TECH-036` in `M-121`, `TECH-070` in `M-123`,
  `TECH-017` in `M-124`, and `TECH-059`/`TECH-060` in `M-127` are
  non-implementation integration or gate references. They do not falsely
  broaden the issues' structured technical-contract provenance.
- No issue adds unauthorized game content, assets, routes, controls,
  characters, forest populations, generation, physics/damage policy,
  networking, ship/station delivery, cross-machine qualification, universal
  hardware gates, or a privileged consumer path.

### Authority-boundary disposition

Both findings are correctable solely in `docs/issues.json`. They require no
change to an approved source, requirement, technical contract, authority ID,
decision, TDD, authority ledger, source-manifest entry, or digest. This turn
therefore requests manifest changes rather than failing for upstream human
resolution.

## Coder Response Turn — 2026-07-29T22:21:25Z

**Answering:** `Issue Review Turn — 2026-07-29T22:14:44Z`

**Verdict:** addressed

### Responses to findings

1. **Missing private-correction and completion-bridge `TECH-060` proof
   slices — addressed.**

   Added two independently testable, dependency-ordered issues with only
   `TECH-060` and the exact required `REQ-002`, `REQ-005`, `REQ-012`,
   `REQ-015`, `REQ-016`, `REQ-023` / `C-003`, `C-013`, `C-010`, `AD-003`,
   `D-002`, `D-007`, `D-006`, `D-005`, `C-004`, `D-009`, `C-015`
   provenance:

   - `M-127` produces `tests/headless/private_correction.rs`, depends on
     `M-049`, `M-077`, and `M-116`, and proves successful durable
     same-present replacement plus prepublication abort behavior through the
     actual correction transaction and public integration path. Its criteria
     assert atomic frontier/log/rollback/replay/participant publication,
     byte-identical original state on abort, exact terminal failures and
     cancellation cutoffs, and release of permits, callback cells, pins, and
     staged CPU/GPU participant resources.
   - `M-128` produces `tests/headless/completion_bridge.rs`, depends on
     `M-060`, `M-065`, `M-066`, `M-067`, `M-077`, and `M-116`, and proves
     reservation, exhaustion, matching completion, duplicate completion, and
     old-generation drain with controlled headless completions. Its criteria
     preserve the two dedicated control cells under job-capacity exhaustion,
     assert exact completion/publication state transitions, fail duplicate
     completion closed, prohibit old-generation publication, and require
     exact-once release after acknowledgement/drain.

   The former completion gate `M-127` is now `M-129` so dependencies remain
   earlier than their consumer. `M-129` depends on both new issues, enumerates
   private-correction and completion-bridge evidence rows, and rejects either
   row when absent, incomplete, skipped, or failing.

2. **Unconstrained Markdown discovery in `M-124` — addressed.**

   `M-124` now selects and pins the maintained CommonMark parser
   `pulldown-cmark = "=0.13.4"` in Inputs and Acceptance Criteria to enumerate
   every Rust-tagged fenced block in the approved Markdown TDD inputs. It
   retains `syn = "=2.0.106"` for the extracted Rust AST, fails closed when
   the complete document/block inventory cannot be established, and
   explicitly prohibits regex, a custom Markdown tokenizer/fence scanner, a
   home-grown Rust parser, or shared general-purpose Markdown/Rust validation
   infrastructure. Its `TECH-060`, requirement, and authority provenance are
   unchanged.

### Exact `docs/issues.json` changes

- Preserved `M-001` through `M-126`, except for the requested body-only
  CommonMark discovery correction to `M-124`.
- Added `M-127` and `M-128` as the two separate `TECH-060` headless proof
  issues described above.
- Renumbered only the former final gate from `M-127` to `M-129`, added
  `M-127` and `M-128` to its structured and body dependency lists, and added
  fail-closed evidence-row criteria for both proof slices.
- Increased the manifest from 127 to 129 sequential dependency-ordered
  entries; no other issue, provenance chain, produced artifact, or dependency
  changed.

### Authority-boundary disposition

No finding is unsatisfied, and no finding requires changing a source,
requirement, technical contract, claim, decision, TDD, authority ledger,
source manifest, or digest. Both findings were satisfied solely by correcting
`docs/issues.json`; there is no upstream-authority conflict requiring human
resolution.

## Issue Review Turn — 2026-07-29T22:27:58Z

**Verdict:** approved

### Review basis

Reviewed the original sources and reference inputs sealed by
`docs/planning/authority/source-manifest.json`, the authority ledger and
approved product-design decisions, all 43 `REQ-###` blocks in
`docs/design-document.md`, the complete approved TDD and final TDD review
ledger, `docs/tdd/traceability.md`, all 129 entries in `docs/issues.json`, and
every preceding turn in this issue-review ledger. Every sealed source,
reference input, approved design artifact, and authority-ledger SHA-256 digest
matches the source manifest.

Mechanical traversal found 70 active technical contracts
(`TECH-001`–`TECH-062`, `TECH-064`–`TECH-071`), exactly one `Implements:` line
per contract, 346 exact contract/requirement pairs with traceability parity,
and no implementation of the retired `TECH-063` or superseded `REQ-039`.

### Findings

None.

### Prior findings closure

- `M-127` now independently proves the `TECH-060` private-correction
  success/abort slice through the public correction integration path. Its
  dependencies include `M-049`, `M-077`, and the `M-116` controlled headless
  harness; its acceptance criteria cover durable-before-publication success,
  expected-hash/participant/cancellation/sink aborts, byte-identical retained
  authority on abort, exact failures, and resource release.
- `M-128` now independently proves the `TECH-060` completion-bridge
  reservation/exhaustion/duplicate/old-generation-drain slice through the
  actual bounded-pool, publication, generation, recovery, facade, and
  controlled-completion paths. Its dependencies and acceptance criteria name
  the required `M-060`, `M-065`, `M-066`, `M-067`, `M-077`, and `M-116`
  integration outputs and exact state/resource-release outcomes.
- `M-124` names pinned maintained parsers in both Inputs and Acceptance
  Criteria: `pulldown-cmark = "=0.13.4"` for complete CommonMark fenced-block
  discovery and `syn = "=2.0.106"` for Rust AST parsing. It fails closed and
  prohibits regex/manual fence scanning, home-grown parsers, and shared
  general-purpose validation infrastructure.
- `M-129` depends on `M-127` and `M-128`, enumerates both evidence rows, and
  rejects absent, incomplete, skipped, or failing evidence. The earlier
  `TECH-059` additions `M-113`–`M-115`, fixed-math tool selection in `M-079`,
  and Naga constraints in `M-084`–`M-085` remain correctly closed.

### Checks that passed

- All 129 IDs are unique and sequential. Every dependency target exists,
  precedes its consumer, and is neither a self-dependency nor part of a cycle.
- Every active technical contract is covered. Issue slices are bounded by
  their cited contracts, dependency ordered, independently testable, and have
  explicit produced artifacts, integration inputs, failure behavior, and
  exact acceptance criteria.
- For every issue, the union of each cited contract's exact `Implements:` IDs
  equals structured `provenance.design_requirements`; the union of those
  requirements' exact `Authority:` IDs equals structured
  `provenance.authority`. All IDs exist, no provenance member is duplicated,
  and the structured contract/requirement/authority sets match the issue
  Description and Inputs citations.
- Artifact-local canonical, checkpoint, and replay parsers remain the
  explicitly approved versioned formats. Standard WGSL, JSON, CommonMark,
  Rust, arbitrary-precision, and property-test work names maintained Naga,
  `serde_json`, `pulldown-cmark`, `syn`, `rug`, and `proptest` paths as
  applicable; no issue authorizes a home-grown standard-format parser or
  broad shared validator.
- No issue adds a world generator, production content or assets, controls,
  characters, authored routes, forest populations, game/physics/damage
  policy, networking, ship/station delivery, cross-machine qualification,
  universal hardware gate, or privileged consumer path. Scenarios and
  evidence remain ordinary public-interface consumers.

### Authority-boundary disposition

No issue-manifest correction or upstream planning-package change is required.
The current manifest is structurally sound, complete for the approved TDD,
and contains no unauthorized work.

## Coder Response Turn — 2026-07-30T05:52:15Z

**Answering:** `TamedTornado (COMMENTED)` review feedback supplied for the
`M-129` hierarchical evidence-aggregation revision.

**Verdict:** addressed

### Responses to findings

1. **`M-129` 69-way direct fan-in and completion-gate boundary — addressed.**

   Replaced the former monolithic evaluator with five dependency-ordered
   TECH-069 domain issues:

   - `M-129` produces
     `src/bin/moria-qualify/evidence/domain/canonical_gpu.rs` and emits typed
     `CanonicalGpuDomainReportV1` across the fixed-math/orientation/sparse/
     commitment/collision oracles, matching-Naga shader validation,
     contamination audit, canonical Rust/WGSL boundaries, decoder fuzzing,
     schedule/configuration perturbation, participant RNG, physical-GPU
     sparse/canonical/collision/participant proofs, and eight-run same-machine
     replay.
   - `M-130` produces
     `src/bin/moria-qualify/evidence/domain/facade_headless.rs` and emits typed
     `FacadeHeadlessDomainReportV1` across the coherent `M-077` consumer
     facade and every separate TECH-060 slice from `M-116` through `M-128`.
   - `M-131` produces
     `src/bin/moria-qualify/evidence/domain/scenario_durability.rs` and emits
     typed `ScenarioDurabilityDomainReportV1` across the public/deep/dynamic/
     atomicity/truth-view scenarios, all three failure-matrix slices,
     checkpoint/cold restore, participant restart, correction branch, cold
     public replay, and rollback-chain evidence.
   - `M-132` produces
     `src/bin/moria-qualify/evidence/domain/performance_presentation.rs` and
     emits typed `PerformancePresentationDomainReportV1` across physical
     device loss/recovery, presentation truth isolation, visual capture,
     human review, rollback correctness and tier measurement, and every
     TECH-068 benchmark family.
   - `M-133` produces
     `src/bin/moria-qualify/evidence/domain/provenance_manifest.rs` and emits
     typed `ProvenanceManifestDomainReportV1` across approved REQ/TECH
     traceability, `Implements:` parity, the REQ-039/TECH-063 supersession
     record, public closure, exact local commands, source/contract/commit
     identity, clean-worktree state, evidence schema and immutable BLAKE3
     blobs, domain-report references, and missing claims.

   Each domain issue owns an explicit closed row catalog and independently
   rejects every missing, skipped, unavailable, incomplete, failed,
   duplicated, corrupt, stale, or cross-wired row. No oracle, shader,
   real-GPU, headless, scenario, durability, presentation, recovery,
   performance, provenance, closure, or manifest slice was merged merely to
   lower dependency count.

   The former `M-129` final gate is now `M-134` so all dependencies remain
   earlier than their consumer. `M-134` depends only on `M-129`–`M-133`,
   accepts exactly one valid identity-matching report of each typed domain,
   and rejects a missing or invalid report before emitting an approved
   completion result. Its `CompletionGateReportV1` derives and returns
   `approved_gdd_coverage` only from the exact provenance traceability rows
   and `approved_implementation_completeness` only from the complete
   implementation/evidence rows; tests prohibit either conclusion from being
   inferred from the other.

   All TECH-069 semantics, every TECH-059 and TECH-060 slice, all current
   evidence rows, and the exact TECH-069 requirement and authority chain are
   retained. The final gate's direct fan-in fell from 69 to 5; the five domain
   fan-ins are 17, 14, 13, 10, and 7. Every one of the former 69 direct
   dependencies remains in `M-134`'s transitive dependency closure.

2. **`M-077` 38-way fan-in sanity check — addressed without changing
   `M-077`.**

   `M-077` remains one coherent public-facade integration: its dependencies
   are the callable owners and lifecycle implementations that it wires into
   one consumer surface, not independent evidence reports suitable for
   hierarchical aggregation. Splitting that implementation boundary would
   fragment the complete facade and weaken reviewability. `M-130` therefore
   consumes `M-077` as the single `complete_consumer_facade` row and keeps all
   separate TECH-060 proof slices independent.

### Exact `docs/issues.json` changes

- Preserved `M-001` through `M-128` unchanged, including `M-077`.
- Replaced the former `M-129` gate entry with the canonical/GPU domain
  evaluator and added domain evaluators `M-130` through `M-133`.
- Renumbered only the former final gate from `M-129` to `M-134`, replaced its
  69 direct dependencies with `M-129` through `M-133`, and required the five
  exact typed reports plus separate coverage and completeness conclusions.
- Increased the manifest from 129 to 134 sequential dependency-ordered
  entries. Every new issue cites only TECH-069 with the unchanged exact
  REQ-021, REQ-022, REQ-023, REQ-026, REQ-044 and authority chain.

### Authority-boundary disposition

No finding requires changing a source, seed, vision, design requirement,
technical contract, claim, decision, TDD, authority ledger, source manifest,
or digest. Both findings were satisfied solely by correcting
`docs/issues.json`; there is no unsatisfied finding or upstream-authority
conflict requiring human resolution.
