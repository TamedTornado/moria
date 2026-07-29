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
