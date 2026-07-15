# Issue 160 — Save the heavy mutation-workload world

References: `docs/tdd/benchmarks.md §Mutation workloads and heavy save`; `docs/tdd/api.md §Save/load protocol`.

## Boundary contracts

- After every mutation request terminally reconciles and >=256-brick defacement passes, runner sends exactly one public SaveWorldRequest and matches exactly one terminal result.
- Save result bytes/counts must equal fixed snapshot and final filesystem metadata; file must be <50,000,000 bytes.

## Multi-system scenarios

- Passing heavy world; primary-ready but unreconciled; one request failed/stuck; busy disk; temp/partial write; exact 49,999,999 and 50,000,000 byte files.
- Additional edit attempt around save boundary verifies fixed snapshot identity/count.

## Failure propagation

- Any prerequisite not terminal/valid prevents request; Busy/failure/temporary state cannot set completed or size/count fields as success.
- Mismatch, >=50 MB or second terminal/request fails report and preserves prior slot semantics.

## Ordering guarantees

- All mutation terminal events -> workload validation -> one save acceptance -> snapshot/write/rename -> matching terminal -> evidence.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

