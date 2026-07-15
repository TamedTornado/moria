# Issue 121 — Decode and validate a staged load

References: `docs/tdd/api.md §Save/load protocol`; `docs/tdd/data-model.md §Edit delta set and save file`.

## Properties that must hold

- For every successful stage, decode occurs off-thread into one canonical bounded identity-matched map and never touches live store/revision/presentation.
- For every documented hostile byte stream, staging returns a typed error and no partial map.

## Entity configurations to test

- Absent startup slot (empty success), explicit absent in-session slot (NotFound), empty/large valid file, each codec corruption from issue 116, wrong identity.
- Task completion before/after additional queries and consumer movement.

## Edge cases and type boundaries

- Decode/I/O/cap/checksum/identity failure leaves current truth and revision byte-identical and produces one failure.

## Error paths

- Allocation/count limits are checked before growth; derived data is neither accepted nor staged.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

