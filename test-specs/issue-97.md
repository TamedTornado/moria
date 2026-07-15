# Issue 97 — Record and validate the production mutation trace

References: `docs/tdd/implementation-plan.md §Gate F2`; `docs/tdd/data-model.md §Feasibility evidence`.

## Properties that must hold

- For every traced request/stage, preallocated ring records request/batch/revision/frame, monotonic start/end, count and bytes; every required stage exists, including explicit count=0 branches.
- For every evidence calculation, first progress, throughput, runnable wait, primary latency, frame pacing and reconciliation derive from trace records without timed-path allocation/file I/O.

## Entity configurations to test

- All required stage IDs; nonempty and legitimate zero-work branches; capacity at exact F2 maximum; 256 then 257 entries; interleaved requests/revisions.
- Known synthetic timestamps for each percentile/throughput/wait formula.

## Edge cases and type boundaries

- Ring wrap/drop, NaN/Inf, missing/duplicate/mismatched stage, nonmonotonic time/frame, capacity overflow or expected/ack mismatch fails deterministically.

## Error paths

- Validator cannot own or alter scheduler state and cannot fill missing evidence with zeros.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

