# Issue 536 — Implement canonical, query, and content failure-matrix slices

References: `validation.md` TECH-065; issue M-097.

## Properties

- For every required matrix condition, exactly one row with the specified layer, code, retryability, authority effect, observation, and dirty-state result must exist and pass.
- Each matrix row records exact injected condition, expected layer/code, retryability, receipt terminal state, committed frontier/revision effect, observation, and dirty-state result.
- Missing/skipped/unavailable row cannot be interpreted as pass.

## Required rows/configurations

- Invalid bounds/cell/orientation/encoding, stale revision/hash, arithmetic/revision/tick exhaustion, duplicate/closed tick, canonical budget, injected candidate failure.
- Query cold/materializing/failed truth, revision wait/stale, capacity before/after admission, observation count/byte overwrite.
- Base source unavailable/invalid/drop/panic/digest mismatch, activation content mismatch/dependency readiness, schedule permutation.

## Failure invariants

- Unknown/missing/corrupt content never becomes empty/no-hit.
- Command failure changes only its outcome; tick-global failure publishes no root/revision/outcome/participant/event.
- Schedule/callback timing cannot change a canonical row result.
- Scars/last trustworthy frontier/dirty evidence remain reachable exactly as the owning contract specifies.
- Provider/capacity identities and owned request returns are asserted, not inferred from diagnostics.
