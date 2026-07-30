# Issue 519 — Prove private correction success and abort headlessly

References: `validation.md` TECH-060 private-correction proof; TECH-048; issue M-127.

## Valid lifecycle

- Through public `request_correction`, execute target pin -> private participant/root restore -> contiguous private replay -> expected comparison -> one durable branch -> one coordinated publication.
- On success atomically replace frontier bundle, rollback suffix, active log/history, replay position, participants, receipt, and one observation; schedule only final dirty union.

## Abort and invalid paths

- Exercise expected-hash divergence, participant failure, capacity/validation failure, cancel before branch invocation, and branch sink failure.
- Every prepublication failure leaves original frontier/rollback/log/replay/participants/readable pins byte-identical with no intermediate observation/presentation.
- Cancellation before invocation is accepted; after invocation is `NotCancellable`.
- Branch failure reports exact provider-scoped `CorrectionError`, committed none, correction/lifecycle observations, original frontier, and terminal-world policy.

## Concurrency/resource guards

- Hold old query/checkpoint/readers during success/failure and prove pin validity.
- Control GPU completion so staged GPU tokens retire only after last use; CPU tokens drop after callback closure.
- Every result returns correction bytes/count/callback/private-root permits exactly once. No test-only authority/storage mutation path is allowed.
