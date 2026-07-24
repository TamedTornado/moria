## Auditor Turn — 2026-07-24T07:00:08Z

Mode: continue

Responding to: none

### Prior Findings Status

None — this is the initial auditor turn.

### New Findings

1. **unresolved — The central product-target conflict was normalized through document roles instead of recorded as an explicit decision.** `docs/seeds/product-one-seed.md:1-3` names the walkable world “Product One” and a “product-shaped demo”; `docs/seeds/product-one-seed.md:77-79` calls its performance section “the actual product spec” and describes the product's customers. In contrast, `docs/seeds/project-boundary.md:3-14` says Moria's product is the reusable crate substrate and the executable is only a public-interface validation harness. The seed index supplies the authority provenance by calling the latter an operator's binding clarification at `docs/seeds/README.md:11-13`. The ledger adopts the clarification in `product_target` and labels Product One a `downstream_consumer`, but `product_target.supporting_decision_ids` is empty and the only decision concerns voxel size. Add a resolved product-boundary decision with both readings as options, the conflicting evidence, the selected substrate-plus-harness reading, and the index/boundary evidence as its resolution source; then cite that decision from `product_target`.

2. **unresolved — The public-versus-engine-internal API ambiguity is silently resolved.** The binding boundary requires the harness to use “the same public interfaces available to an external game” and forbids privileged paths at `docs/seeds/project-boundary.md:6-10`, while Product One calls the dig/place verbs and mirror queries an “engine-internal API” at `docs/seeds/product-one-seed.md:64`. The target asserts public Rust crate interfaces, yet the ledger records both claims as binding without a decision or explanation of whether “engine-internal” means a public Rust engine API rather than a private harness-only API. Record an explicit resolved decision or fold this question unambiguously into the product-boundary decision; the selected result must preserve the non-optional public consumer boundary.

3. **unresolved — Material Product One benchmark decisions are missing.** Product One says its decision bed answers voxel size, LOD strategy, and object-layer scaling with measurements at `docs/seeds/product-one-seed.md:116`. The architecture lists the corresponding open questions at `docs/seeds/voxel-world-substrate.md:227-229`. The ledger captures only voxel size; it has no claims or unresolved decisions for distant-terrain LOD or object-layer capacity/scaling. Add evidence-backed unresolved decisions for both. Also classify the remaining architecture questions at `docs/seeds/voxel-world-substrate.md:230-231` explicitly as current, deferred, or excluded. Product One excludes fluids tier 2/3 at `docs/seeds/product-one-seed.md:61`, so silently treating every architecture open question as a current deliverable would be equally inaccurate.

4. **unresolved — “GPU-resident” is not supported by the product target's declared evidence set, and its architectural provenance is incomplete.** None of `claim-boundary-product`, `claim-boundary-harness`, `claim-product-one-scope`, or `claim-product-one-output` states GPU residency, although the composite target does. `claim-substrate-gpu` does state it, but is not cited by the target and its source expressly delegates that premise to the missing `system-substrate-pivot.md` at `docs/seeds/voxel-world-substrate.md:13`. The manifest honestly records that missing reference, but the ledger never states its evidentiary impact. Add direct extant Product One evidence for the current GPU requirement (for example, the GPU meshing and residency constraints at `docs/seeds/product-one-seed.md:56-58` and `:81-96`) and cite it from the target, or narrow the target. Explicitly disclose that pivot-derived details cannot be independently verified until the missing source is supplied.

5. **unresolved — The seed index is labeled stale without evidence, obscuring the sole provenance for human feedback.** `src-seed-index` is classified as `stale_index` merely because it is dated and derivative. `docs/seeds/README.md:3-13` says the bundle was preserved on a date and identifies the operator's binding clarification; it does not say the index is superseded or stale. The boundary file states the substantive rule, but only the index attributes that rule to the operator. Reclassify it as an index/provenance source (or provide evidence of staleness) and make clear that the operator attribution is supported by the index rather than invented from `project-boundary.md` alone.

6. **unresolved — A substrate build-order claim is misclassified as a consumer.** `claim-substrate-later-simulation` cites `docs/seeds/voxel-world-substrate.md:218-220`, which is explicitly the substrate's own future build order, but its disposition is `future_consumer`. Product One excludes most of these mechanisms from the current slice at `docs/seeds/product-one-seed.md:60-62`; that makes them deferred or excluded substrate capabilities, not consumers. Correct the disposition so current product mechanisms, deferred substrate work, and actual downstream game consumers remain distinct.

### Questions For Coder

1. Will you model the product/harness boundary and public-interface requirement as one resolved authority decision or as two linked decisions? Either is acceptable if both source tensions are explicit and the product target cites the resolution.
2. Is `system-substrate-pivot.md` obtainable? If not, please state that limitation in the ledger and avoid using it as unqualified authority for details not independently established by an extant binding source.

### Answers To Coder

None — no coder question preceded this turn.

### Notes To Coder

Independent SHA-256 checks matched all five recorded source digests, and the independently computed manifest digest matched `authority-ledger.json.source_manifest_sha256`. All 27 existing claim line locators contain the quoted evidence, JSON parsing and ID references are valid, the manifest base revision equals `HEAD` (`058d50cb687da5d0b42061d9b2542cc912bd220f`), and the absent pivot is correctly listed under `missing_references`.

The proposed substrate-plus-public-harness target is supported by the operator-clarification provenance; the problem is that the ledger currently hides the authority resolution rather than recording it. Structural approval, once earned, will not resolve any still-unresolved material decisions and will not authorize downstream planning.

Authority package rejected.

### Approval

Not approved.

## Coder Turn — 2026-07-24T07:03:46Z

Mode: continue

Responding to: 2026-07-24T07:00:08Z

### Responses To Findings

1. Addressed: added `decision-product-boundary`, resolved to the substrate-plus-validation-harness reading, cited it from `product_target`, and preserved the conflicting Product One reading as evidence.
2. Addressed with a linked decision: `decision-public-consumer-api` resolves the ambiguous “engine-internal API” wording to public substrate interfaces for the harness, based on the operator clarification and direct boundary rule.
3. Addressed: added unresolved `decision-distant-terrain-lod-strategy` and `decision-object-layer-scaling`. Added explicit scope classifications for the other two architecture questions: tier-2 fluids are excluded from Product One and multiplayer implementation is deferred (compatibility implication only).
4. Addressed: added direct Product One GPU meshing, GPU-residency, and wgpu/WGSL evidence to `product_target`. Added `decision-pivot-derived-authority`, which prohibits treating details delegated solely to the missing pivot as independently verified.
5. Addressed: reclassified the seed index from `stale_index` to `context` and documented its role as provenance for the operator attribution.
6. Addressed: reclassified the substrate build-order claim from `future_consumer` to `supporting`; it is architecture/build-order context, not a downstream consumer.

### Answers To Auditor

1. I modeled the boundary and API question as two linked resolved decisions, so the product-target conflict and the harness-interface consequence remain independently reviewable.
2. `system-substrate-pivot.md` is not present in the repository. The manifest continues to record it as `not_found`; the new pivot-authority decision records the resulting evidentiary limitation.

### TDD Changes Made

- Validated that both JSON files parse.
- Recomputed and matched `source_manifest_sha256` against the completed manifest.
- Validated every decision evidence-claim reference and every target/unresolved decision reference.
- Ran `git diff --check` successfully.

### Questions For Auditor

None.

### Notes To Auditor

The unresolved set now intentionally contains the voxel-size, distant-terrain LOD, and object-layer scaling decisions. No source or operator feedback selects those options.

## Auditor Turn — 2026-07-24T07:05:50Z

Mode: continue

Responding to: 2026-07-24T07:03:46Z

### Prior Findings Status

1. **resolved — “The central product-target conflict was normalized through document roles instead of recorded as an explicit decision.”** `decision-product-boundary` now preserves both readings, selects the substrate-with-validation-harness interpretation from the operator clarification, and is cited by `product_target`.
2. **resolved — “The public-versus-engine-internal API ambiguity is silently resolved.”** `decision-public-consumer-api` explicitly records the tension and preserves the non-optional public-consumer boundary.
3. **partially_resolved — “Material Product One benchmark decisions are missing.”** The LOD and object-layer decisions now exist and remain visibly unresolved, and the fluid/multiplayer questions have explicit current-scope decisions. However, both new benchmark decisions cite the wrong Product One claim, as detailed under New Findings.
4. **resolved — “‘GPU-resident’ is not supported by the product target's declared evidence set, and its architectural provenance is incomplete.”** `claim-product-one-gpu-residency` directly supports the current GPU requirement and is cited by the target; `decision-pivot-derived-authority` records the missing pivot's evidentiary limit.
5. **resolved — “The seed index is labeled stale without evidence, obscuring the sole provenance for human feedback.”** `src-seed-index` is now `context`, and its rationale accurately identifies it as the source of the operator attribution.
6. **partially_resolved — “A substrate build-order claim is misclassified as a consumer.”** `claim-substrate-later-simulation` is no longer called a future consumer, but its replacement `supporting` disposition still combines current and deferred/excluded work instead of distinguishing them, as detailed under New Findings.

### New Findings

1. **unresolved — The new LOD and object-layer decisions rely on Product One evidence that their cited claim does not contain.** `decision-distant-terrain-lod-strategy` and `decision-object-layer-scaling` both cite `claim-product-one-output`, whose statement and locator cover only `docs/seeds/product-one-seed.md:114` (“The substrate crates exist, benchmarked…”). Their rationales instead assert that Product One identifies LOD and object-layer scaling as a measurement-backed decision bed; that evidence is at `docs/seeds/product-one-seed.md:116`, outside the cited locator and absent from the claim statement. Add a claim for line 116 and cite it from both decisions (or expand the existing claim accurately).

2. **unresolved — The revised build-order disposition still obscures the current/deferred split.** `claim-substrate-later-simulation` combines CA/fire, grass dressing, tree felling, fluid tiers 1+2, integrity, and granular work from `docs/seeds/voxel-world-substrate.md:218-220` under a single `supporting` disposition. Product One makes grass dressing, tree placement, and tier-1 static water current, while excluding CA/fire, tier-2 fluids, integrity, granular settle, and ordinary tree felling at `docs/seeds/product-one-seed.md:56-62` (with felling only a stretch goal at line 108). Split this claim or mark the excluded/deferred capabilities explicitly so `supporting` cannot be mistaken for the selected current slice.

### Questions For Coder

None. The two required corrections are concrete.

### Answers To Coder

None — the coder asked no question.

### Notes To Coder

I independently reread all five available source documents and followed every repository-relative document reference. All five source SHA-256 digests match the manifest; the independently computed manifest digest matches `authority-ledger.json.source_manifest_sha256`; `base_revision` matches `HEAD`; the absent `docs/seeds/system-substrate-pivot.md` remains correctly recorded; both JSON files parse; all IDs and references resolve; resolved selections are valid; and the unresolved-decision index is exact.

The product-boundary and public-interface decisions are now honest authority resolutions rather than precedence inferred from filenames, repetition, detail, or README placement. The remaining corrections are evidence/disposition integrity issues. If structural approval is later granted while material decisions remain unresolved, that approval will not resolve those decisions and will not authorize downstream planning.

Authority package rejected.

### Approval

Not approved.

## Coder Turn — 2026-07-24T07:08:39Z

Mode: continue

Responding to: 2026-07-24T07:05:50Z

### Responses To Findings

1. Addressed: added `claim-product-one-open-question-decision-bed` with the exact Product One line-116 statement, and both `decision-distant-terrain-lod-strategy` and `decision-object-layer-scaling` now cite it instead of `claim-product-one-output`.
2. Addressed: removed the aggregate `claim-substrate-later-simulation`, whose single supporting disposition blurred mutually different scope outcomes. Added `claim-product-one-current-dressing-and-water` for the selected grass-dressing, object-placement, and tier-1-water slice; narrowed `claim-product-one-matter-exclusions` to the exact excluded-mechanics line. The architecture build order remains preserved as supporting-reference context, but is no longer represented as one falsely uniform current-slice claim.

### Answers To Auditor

None.

### TDD Changes Made

- Parsed both JSON files.
- Recomputed and matched `source_manifest_sha256` against the completed manifest.
- Validated decision evidence-claim references, product-target references, resolved selections, and the exact unresolved-decision index.
- Ran `git diff --check` successfully.

### Questions For Auditor

None.

### Notes To Auditor

The unresolved set remains intentionally limited to voxel size, distant-terrain LOD strategy, and object-layer scaling. No supplied source or operator feedback selects any of those options.

## Auditor Turn — 2026-07-24T07:11:49Z

Mode: approved

Responding to: 2026-07-24T07:08:39Z

### Prior Findings Status

1. **resolved — “The new LOD and object-layer decisions rely on Product One evidence that their cited claim does not contain.”** `claim-product-one-open-question-decision-bed` now quotes and locates `docs/seeds/product-one-seed.md:116`, and both `decision-distant-terrain-lod-strategy` and `decision-object-layer-scaling` cite that claim.
2. **resolved — “The revised build-order disposition still obscures the current/deferred split.”** The falsely uniform architecture build-order claim has been removed. `claim-product-one-current-dressing-and-water` now records the selected grass/clutter, object-placement/rendering, and tier-1-water slice while preserving the no-felling rule; `claim-product-one-matter-exclusions` separately records the excluded CA, fire, tier-2/3 fluids, integrity, and granular-settle mechanics.

### New Findings

None.

### Questions For Coder

None.

### Answers To Coder

None — the coder asked no question.

### Notes To Coder

I independently reread all five configured and available source documents and followed their repository-relative document references. All five source SHA-256 digests match `source-manifest.json`; the independently computed manifest digest matches `authority-ledger.json.source_manifest_sha256`; `base_revision` matches `HEAD` (`058d50cb687da5d0b42061d9b2542cc912bd220f`); and the unavailable `docs/seeds/system-substrate-pivot.md` is represented accurately as a missing reference with an explicit evidentiary limitation.

Both JSON files parse. Every configured source has a document role, all 34 claim source IDs and line ranges are valid, all decision and product-target references resolve, every resolved selection names a real option, and the unresolved-decision index exactly matches the three unresolved decisions. The document roles and explicit product-boundary/API decisions distinguish the reusable substrate, its current validation harness, future game consumers, supporting architecture, and bundle provenance without deriving the target from filename, detail, repetition, or README placement. The operator attribution is confined to what `docs/seeds/README.md:11-13` actually records.

The remaining voxel-size, distant-terrain LOD, and object-layer-scaling decisions are materially unresolved and remain visible as such. This structural approval does not resolve them and does not authorize downstream planning.

Authority package structurally ready for human decision.

### Approval

Approved.
