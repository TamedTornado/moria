# Issue 491 — Replay owned records in a private builder

References: `content-persistence.md` TECH-047 public replay; issue M-048.

## Valid transitions

- `LoadingOwnedRecords -> VerifyingHeader -> ReplayingPrivate -> ComparingExpected -> ExportingReplayHeader/Prefix -> Publishing -> Ready`.
- Genesis anchor builds private genesis; checkpoint anchor first executes bounded private restore. Tick/branch records fold and advance only private frontier after all expected root/outcome/participant/event comparisons.
- After semantic success, exact source physical header/records copy to a fresh stream sequences 0..N; final publication waits for all durability.

## Invalid transitions and guards

- Reject wrong world/identity/status/config/anchor limits, count/byte/artifact/private-root bounds, gaps/reordering, invalid branch/expected values, duplicate stream, and tick/sequence overflow before publication.
- Earliest mismatch stops at that tick and returns bounded exact-prefix `DivergenceArtifact`; non-divergence failures carry no artifact.

## Lifecycle and concurrency

- Destroy all live state and replay ordinary and correction streams from owned bytes only.
- Cancel before/after private submission or during destination prefix copy; drain private resources/sink calls and publish nothing.
- Sink failure/wrong completion retires invoked destination stream and publishes no world.
- Success publishes final corrected projection only; first new tick appends at N+1. Intermediate/superseded roots never emit live observations/presentation.
