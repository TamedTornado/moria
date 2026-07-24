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

## Auditor Turn — 2026-07-24T07:23:47Z

Mode: continue

Responding to: 2026-07-24T07:11:49Z

### Prior Findings Status

1. **resolved — “The new LOD and object-layer decisions rely on Product One evidence that their cited claim does not contain.”** `claim-product-one-open-question-decision-bed` still quotes and locates `docs/seeds/product-one-seed.md:116`, and both decisions still cite it.
2. **resolved — “The revised build-order disposition still obscures the current/deferred split.”** The former aggregate build-order claim remains absent, and the Product One dressing/water and matter-exclusion claims remain separate.
3. **superseded — The prior structural approval no longer applies to the present package.** Since the approval at `2026-07-24T07:11:49Z`, both authority JSON files have been materially revised to incorporate later human feedback, including a new harness-scope decision and wholesale changes to Product One claim dispositions. Those changes require fresh authority review.

### New Findings

1. **unresolved — The quoted human feedback was broadened into a wholesale loss of Product One's technical authority without recording that material interpretation.** The recorded feedback says Product One is a downstream consumer/validation example rather than the repository product and specifically excludes a third-person player or character, skeletal mesh, animation, curated traversal, a prescribed forest/canopy workload, and machine-specific performance thresholds. The ledger additionally changes Product One's dig/place proof, generation slice, matter exclusions, dressing/static-water slice, GPU/meshing constraints, API sliver, benchmarks, portability rule, and substrate output from binding to `future_consumer`. That broader demotion conflicts with the seed index's statement that Product One “defines the binding substrate implementation” at `docs/seeds/README.md:6-9`, yet no decision asks whether the feedback supersedes Product One only as the product/demo deliverable or also as the selector of substrate requirements. The inconsistency is visible inside `documents`: `src-product-one` says Product One does not define current deliverables, while `src-voxel-substrate` still says “Product One and the project boundary narrow its current scope.” Add an explicit authority decision preserving both interpretations and the exact feedback limits, or obtain feedback that expressly selects the wholesale demotion. Until then, the current substrate slice is not honestly established.

2. **unresolved — Tier-2 fluid scope is now silently excluded with no applicable authority.** `claim-substrate-open-fluid-tier-2` is marked `excluded`, and the former `decision-fluid-tier-2-current-scope` has been removed. But the architecture places tier-2 coarse flow in the substrate at `docs/seeds/voxel-world-substrate.md:123-129`, includes fluids tier 1+2 in the substrate build order at lines 218-220, and leaves only the pressure-versus-momentum implementation choice open at line 230. Product One excludes tier 2 only from its own slice at `docs/seeds/product-one-seed.md:61`; under the revised `downstream_consumer` role, that cannot silently exclude tier 2 from the reusable substrate. The quoted feedback does not mention fluids. Restore an explicit current/deferred/excluded scope decision and, if current, keep the solver choice visibly unresolved.

3. **unresolved — The package no longer decides whether GPU residency is a current product constraint.** The current repository summary calls Moria “GPU-resident” at `README.md:3-6`, the architecture calls GPU residency a design goal at `docs/seeds/voxel-world-substrate.md:13`, and the target cites `claim-readme-summary` while omitting that material property from its statement. `decision-pivot-derived-authority` correctly limits details attributable only to the missing pivot, but it does not select whether the independently extant high-level GPU-residency requirement is current, deferred, or merely contextual. The quoted feedback identifies the reusable substrate as the product and excludes listed harness additions; it does not exclude GPU residency. Record that authority decision explicitly rather than dropping the property through omission.

4. **unresolved — The multiplayer claim disposition contradicts the selected decision.** `claim-substrate-open-multiplayer` is marked `excluded`, although `decision-multiplayer-current-scope` selects an option that excludes implementation while retaining the verb/command architecture's compatibility implication. Split implementation from compatibility or use a disposition that preserves the retained part; the current single `excluded` label hides part of the selected outcome.

### Questions For Coder

1. Was the human feedback intended to remove Product One's authority only as the repository product and for the specifically listed demo/harness additions, or to supersede every Product One technical substrate requirement? If no additional feedback answers that, please leave the interpretation explicitly unresolved.
2. Should GPU residency remain a current Moria constraint based on the root summary and architecture, or is it intentionally deferred/contextual? The supplied feedback quote does not answer this.

### Answers To Coder

None — no coder question followed the prior auditor turn.

### Notes To Coder

I independently reread all five configured and available source documents and followed every repository-relative document reference. All five source SHA-256 digests match `source-manifest.json`; the independently computed manifest SHA-256 is `00da87328bf38a3bd7eb6f35272e7fbea04cdeec05b10ac55f2fb594ab27a4e3` and matches the ledger; `base_revision` matches current `HEAD` (`6c68baaaa83b01d1080b3b591591569816ea3305`); and `docs/seeds/system-substrate-pivot.md` is correctly represented as missing. Both JSON files parse, all configured sources have roles, all claim source IDs and line ranges are valid, all decision/target references resolve, resolved selections name real options, the unresolved index matches the three currently labeled unresolved decisions, and `git diff --check` passes.

The source inventory and mechanical integrity are sound. The blockers are authority interpretation and visible scope decisions introduced by the post-approval feedback rewrite. A later structural approval may leave material decisions unresolved, but it will not resolve them and will not authorize downstream planning.

Authority package rejected.

### Approval

Not approved.

## Coder Turn — 2026-07-24T07:27:50Z

Mode: continue

Responding to: 2026-07-24T07:23:47Z

### Responses To Findings

1. Addressed: added unresolved `decision-product-one-technical-scope`. It preserves the index's statement that Product One defines the binding substrate implementation alongside the later feedback's downstream-consumer statement. The explicitly named demo/harness exclusions remain excluded; the remaining technical claims are now `unresolved`, not silently demoted to `future_consumer`.
2. Addressed: restored tier-2 fluid scope as unresolved through `decision-fluid-tier-2-current-scope`, backed by the substrate build order, the open solver question, and Product One's consumer-slice exclusion. The pressure-versus-momentum choice remains visible and cannot be selected until tier-2 scope is selected.
3. Addressed: added unresolved `decision-gpu-residency-current-scope`, with root-summary, architecture, Product One, and missing-pivot provenance evidence. The target no longer implies a selection that the feedback did not make.
4. Addressed: split multiplayer into the retained compatibility claim (`supporting`) and the excluded implementation claim (`excluded`), both cited by the resolved current-scope decision.

### Answers To Auditor

1. No additional feedback establishes whether the comment removes only Product One's repository-product/demo authority or every Product One technical substrate requirement. I have therefore left that interpretation explicitly unresolved rather than choosing either reading.
2. The supplied feedback does not decide GPU residency. I have left it explicitly unresolved as a current-constraint versus deferred/contextual decision; the missing pivot continues to limit verification of its detailed implementation claims.

### TDD Changes Made

- Parsed both authority JSON files.
- Recomputed and matched `source_manifest_sha256` against the completed manifest.
- Validated all claim and decision references, resolved selections, and the ordered unresolved-decision index.
- Ran `git diff --check` successfully.

### Questions For Auditor

None.

### Notes To Auditor

The unresolved set now also includes the Product One technical-scope interpretation, tier-2 fluid scope, and GPU-residency current-scope decision. The previously resolved repository boundary, public-interface boundary, minimal-harness exclusions, deferred multiplayer implementation, and missing-pivot limitation remain intact.

## Auditor Turn — 2026-07-24T07:31:35Z

Mode: approved

Responding to: 2026-07-24T07:27:50Z

### Prior Findings Status

1. **resolved — “The quoted human feedback was broadened into a wholesale loss of Product One's technical authority without recording that material interpretation.”** `decision-product-one-technical-scope` now preserves the seed index's binding-implementation statement and the later downstream-consumer feedback as competing readings. It remains unresolved, and Product One's non-explicitly-excluded technical claims are consistently marked `unresolved`.
2. **resolved — “Tier-2 fluid scope is now silently excluded with no applicable authority.”** `decision-fluid-tier-2-current-scope` now presents current, deferred, and excluded scope options using the substrate build order, its open solver question, and Product One's slice exclusion. No option is selected.
3. **resolved — “The package no longer decides whether GPU residency is a current product constraint.”** `decision-gpu-residency-current-scope` records the current-constraint and deferred/contextual readings, cites all extant GPU evidence and the missing-pivot limitation, and remains visibly unresolved.
4. **resolved — “The multiplayer claim disposition contradicts the selected decision.”** The ledger now separates the retained server-authoritative compatibility implication (`claim-substrate-open-multiplayer`, `supporting`) from excluded multiplayer implementation (`claim-substrate-multiplayer-implementation`, `excluded`), consistent with `decision-multiplayer-current-scope`.

### New Findings

None.

### Questions For Coder

None.

### Answers To Coder

None — the coder asked no question.

### Notes To Coder

I independently reread all five configured and available source documents and followed every repository-relative document reference. Their SHA-256 digests match `source-manifest.json`; the independently computed manifest digest is `00da87328bf38a3bd7eb6f35272e7fbea04cdeec05b10ac55f2fb594ab27a4e3` and matches the ledger; `base_revision` matches `HEAD` (`6c68baaaa83b01d1080b3b591591569816ea3305`); and `docs/seeds/system-substrate-pivot.md` remains accurately recorded as missing with an explicit evidentiary limitation.

Both JSON files parse. Every configured source has exactly one document role; all 39 claims have valid sources and line ranges; all claim, decision, and product-target references resolve; every resolved selection names an actual option; the unresolved-decision index exactly matches the six unresolved decisions; and `git diff --check` passes. The roles and decisions distinguish the repository product, downstream consumers, validation harness, architecture reference, and provenance index without deriving the target from filename, detail, repetition, or README placement. The recorded human feedback is applied to the product boundary and its expressly named harness exclusions without silently extending it to Product One's remaining technical requirements.

The Product One technical-authority interpretation, voxel size, distant-terrain LOD, object-layer scaling, tier-2 fluid scope, and GPU-residency scope remain materially unresolved. This structural approval does not resolve any of them and does not authorize downstream planning.

Authority package structurally ready for human decision.

### Approval

Approved.
