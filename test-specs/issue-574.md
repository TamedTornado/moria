# Issue 574 — Verify provenance and evidence-manifest integrity

References: `validation.md` TECH-069; `traceability.md`; issue M-134.

## Properties

- Closed catalog is exactly: `approved_req_tech_traceability`, `active_tech_implements_parity`, `superseded_req039_retired_tech063`, `public_closure_inventory`, `local_command_receipts`, `source_contract_commit_digests`, `clean_worktree`, `evidence_schema`, `immutable_blob_digests`, `domain_report_references`, `missing_claims`.
- Traceability authority is evaluated independently of implementation results. Report records row identity, immutable source/artifact digest, expected/observed authority/schema identity, report-reference digest, and validity.
- Every active TECH has exactly one matching `Implements:` line and exact pair parity; REQ-039 is superseded and TECH-063 remains retired.

## Configurations

- Validate one complete set from approved documents and M-078/M-112/M-124/M-129–M-131/M-133.
- For each row/reference: omit, duplicate, reorder, corrupt digest, substitute stale/cross-wired report, change source/contract/commit identity, mark skipped/unavailable/incomplete/failed.
- Exercise dirty worktree, unreadable closure, missing exact local command, malformed/trailing evidence JSON, bad blob/report digest, unknown required field, and hidden missing claim.

## Error paths

- Any defect invalidates the domain; structurally valid but identity-mismatched/incomplete reports cannot pass.
- Coverage rows expose only approved GDD coverage evidence and cannot derive implementation completeness.
- Use maintained serde_json/blake3 paths and keep binary blobs outside JSON numbers.
