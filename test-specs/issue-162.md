# Issue 162 — Prove exact round trip in a second public consumer

References: `docs/tdd/benchmarks.md §Save size and exact restoration`; `docs/tdd/api.md §Save/load protocol`.

## Boundary contracts

- After headed app returns, a second presentation-disabled App uses public MoriaWorldPlugin/LoadWorldRequest; it never invokes codec/store directly.
- RoundTripEvidence compares every saved delta's four bytes, deterministic unedited base samples, identity, and absence of derived bytes before report writing.

## Multi-system scenarios

- Valid heavy save; empty/edited/reverted bytes; deterministic random unchanged points active/inactive; wrong identity/corrupt file; headless readiness/load rebuild.
- Compare saved snapshot to source and second consumer, not regenerated derived geometry.

## Failure propagation

- Any byte/base/identity mismatch, derived bytes, load failure or missing evidence sets round_trip.passed=false and overall mutation report false.
- Second app cannot borrow source App/store/cache; failure does not fabricate counts or continue to passing report.

## Ordering guarantees

- Headed reconciliation/save complete -> headed app returns -> second app ready -> public load completed -> comparison -> final report.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

