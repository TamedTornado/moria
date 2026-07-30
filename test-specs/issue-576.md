# Issue 576 — Enforce separate coverage and implementation-completeness gates

References: `validation.md` TECH-069 completion gate; issue M-135.

## Properties

- `CompletionGateReportV1` contains distinct `approved_gdd_coverage` and `approved_implementation_completeness` conclusions with separate derivations.
- Coverage derives only from exact traceability rows in one valid `ProvenanceManifestDomainReportV1`.
- Completeness derives only from complete implementation/evidence rows across exactly one valid CanonicalGpu, FacadeHeadless, ScenarioDurability, PerformancePresentation, and ProvenanceManifest report.
- The gate has exactly five direct dependencies and no producer-specific row catalog/direct dependency on underlying producers.

## Entity configurations

- Baseline: five identity-matching, digest-valid, internally valid typed reports yield both conclusions according to their independent source rows.
- Four independence cases: coverage pass/completeness fail; coverage fail/completeness pass; both pass; both fail. Changing one valid source conclusion changes only its corresponding output.
- For each report: omit, duplicate, wrong version, stale/cross identity, bad digest, internal invalidity, skipped/unavailable/incomplete/failed status.

## Error paths

- Reject before approved completion on dirty/digest-mismatched worktree, unexpected GPU errors, unavailable/divergent replay, incomplete contamination, undefined callable/owner/bound/lifecycle, missing evidence row, invalid/absent human review for claimed presentation, or incomplete benchmark path through the owning domain.
- One valid domain/report/conclusion cannot substitute for another; no re-expansion or inference over individual producers is allowed.
