# Issue 470 — Publish completed canonical candidates in schedule order

References: `architecture.md` TECH-013; `gpu-runtime.md` TECH-032; issue M-060.

## Boundary contract and ordering

- Enforce main `First`: collect completions -> exclusive canonical publication -> other receipt finalization; PostUpdate coordinates requests; extraction precedes ordered render prepare/encode/submit/drive/derived/presentation sets.
- Candidate follows TECH-013 phases 1–12. Before phase 12 all slots/tokens are private; publication is one `Arc<FrontierBundle>` swap plus receipt, rollback, active log/replay position, participant tokens/commitments, revisions, and observations.

## Multi-system scenarios

- Hold a decoded valid envelope in render world across frames; main world remains entirely old until bridge drain, then every publication field changes together.
- Publish ordinary tick and durable correction candidate; correction envelope must contain prior durable branch completion and splice log/rollback atomically.
- Queries/presentation reading old root remain valid during candidate work.

## Failure propagation

- Revalidate world, job, nonce, source frontier, generation, diagnostics, participant products, output counts, and root hash. Any mismatch/overflow/failure discards candidate and preserves source bundle.
- Duplicate envelope terminally fails generation; unknown/aborted/old-generation envelope performs cleanup only.
- Race completion/cancellation/device loss/shutdown; callback timing cannot select a tick, no intermediate observation/presentation is emitted, and permits release after last use.
