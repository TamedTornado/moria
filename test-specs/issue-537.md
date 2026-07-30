# Issue 537 — Implement participant and world-lifecycle failure-matrix slices

References: `validation.md` TECH-065; issue M-098.

## Properties

- Every `ParticipantFailurePolicy × failure site` row has an explicit exact outcome; no missing row, participant omission, stale-token reuse, fallback, or timing-selected result is accepted.
- Participant output reaches consumer only after containing tick confirms.

## Required rows/configurations

- Genesis prepare, ordinary tick prepare/validation, correction, restore/reconstruct, device loss/recovery success/mismatch, checkpoint export, and shutdown for both policies.
- Effect overlap and stale precondition, event count/bytes/per-record overflow, duplicate sequence, wrong schema/source, missing status, commitment/RNG divergence.
- World Ready/Replaying/Recovering/Failed/ShuttingDown/Closed transitions, duplicate shutdown, abandoned work, and dirty reporting.

## Failure invariants

- Assert exact participant/generation identity, retryability, `FailedNoAdvance` or specialized error, world state, unchanged/terminal frontier, and cleanup.
- Two participant completions in opposite orders produce same canonical result; same-tick dependency/handoff is rejected.
- Events/effects/state from an unconfirmed tick are never delivered/installed/replayed.
