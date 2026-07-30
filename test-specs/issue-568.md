# Issue 568 — Write immutable evidence manifests

References: `validation.md` TECH-069; issue M-112.

## Input validation

- `moria-evidence-v1` includes exact command, fixture/contract/source/commit/worktree/environment/adapter/limits/result/measurement/expected-actual-error/artifact/missing-claim fields with closed bounded types.
- JSON is parsed/serialized with maintained `serde_json`; referenced immutable blobs and cross-artifact bindings use maintained `blake3`.
- Canonical binary values remain native blobs/digests, not lossy JSON numbers.

## Transformation correctness

- Round-trip each valid manifest to an equivalent closed Rust value and stable intended schema; verify every referenced blob digest against exact bytes.
- Validate exact `presentation/capture-manifest-v1.json` capture catalog/claim identities/digests and `presentation/human-review-attestation-v1.json` dispositions/notes/capture-manifest digest.
- Empty optional collections/statuses remain explicit; missing claims are retained, never treated pass.

## Edge and error paths

- Reject malformed/trailing JSON, duplicate/unknown required fields, excessive strings/counts/bytes, invalid enums/statuses, wrong digest encoding/reference, stale command/source/commit/fixture identities, absent artifact, and mismatched capture/attestation binding.
- Do not introduce custom JSON parsing, a second validator, or general validation framework.
- Serialization/validation failure emits no partially valid manifest/report.
