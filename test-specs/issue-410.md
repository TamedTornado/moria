# Issue 410 — Implement the closed public failure taxonomy

References: `interfaces.md` TECH-027; issue M-005.

## Input validation and closed-shape correctness

- Construct and pattern-match every `ErrorCode`, `FailureScope`, `Retryability`, `CommittedEffect`, `AdmissionCode/Context`, `QueryUnavailable`, `TelemetryError`, `CorrectionError`, `TickNoAdvanceCause`, and `CanonicalFailure` variant.
- For every `AdmissionCode`, accept only its mandated context: tick eligibility, invalid batch, interest/query capacity, correction hash count, or retired-stream budget; all other codes require `None`.
- `ResourceBudgetField` accepts only declared group ordinals; unknown ordinals and unknown enum tags fail decoding.

## Transformation invariants

- Every `FailedNoAdvance` satisfies `source_frontier.next_tick() == attempted_tick`, tick-scoped `OperationError`, `CommittedEffect::None`, and cause/error-code agreement.
- Every `CorrectionError` retains the byte-identical live `original_frontier` and `CommittedEffect::None`; replay failure metadata exists only for an invoked correction-branch append failure.
- Ordinary post-confirmation replay append failure carries `CommittedEffect::Frontier(confirmed)`, never the correction semantics.
- `TelemetryBusy` is retryable without progress; unknown/closed are distinct concrete variants.

## Edge and error paths

- Reject mismatched code/context pairs, participant/provider causes without the matching ID, device causes without generation, incorrect scope/tick/world, and correction errors claiming a committed correction.
- Unknown wire tags, corruption, and trailing data produce typed decode failure without panicking or collapsing distinct failure classes.

No rendering state is defined.
