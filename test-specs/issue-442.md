# Issue 442 — Export replay records through the bounded sink lifecycle

References: `content-persistence.md` TECH-047; issue M-047.

## Boundary contract

- Per stream, invoke at most one append at a time. Reserve immutable bytes, callback cell, record count, and byte permits before `ReplaySink::append`.
- Completion must echo exact stream, sequence, and digest. Matching success advances `ReplayStreamPosition`; submit/invocation alone does not.

## Multi-system scenarios

- Export Genesis/checkpoint header at sequence zero, single tick records afterward, and one correction branch at the current sequence.
- Hold an append pending until log pressure; later tick reservation returns `PersistenceBackpressure` rather than dropping/reordering the record.
- Recompute durable prefix digest from ordered `(sequence,digest)` tuples after each success.

## Failure propagation and concurrency

- Wrong echo, first invalid completion, failure, producer drop, count/byte exhaustion, or sequence overflow produces exact `ReplayExportFailure`.
- Ordinary tick failure preserves its ready receipt/frontier but moves world to `Failed` with committed frontier. Correction-branch failure leaves original frontier/log installed and reports committed `None`.
- Duplicate after accepted success is only `AlreadyCompleted`; late/old-generation completion cannot recover a failed/closed world.
- Replay append is not consumer-cancellable. Shutdown drains/closes the one invoked cell, reports the exact failure, then releases pinned raw bytes.
