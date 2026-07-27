# Runtime, lifecycle, and failure behavior

## 1. Bevy schedule integration

`MoriaPlugin` installs ordered system sets:

```text
PreUpdate:
  ReceiveConsumerRequests -> ValidateAndReserve -> AdvanceCpuWork
Update:
  PublishConsumerEvents
PostUpdate:
  ExtractGpuWork
RenderApp:
  PrepareMoria -> DispatchMoria -> ReadbackMoria
next PreUpdate:
  ConfirmGpuWork -> AdvanceLifecycle -> PublishConsumerEvents
```

Consumer calls enqueue into bounded facade queues; they do not perform GPU work
or block an ECS system. Requests submitted during a set become visible at the
next `ReceiveConsumerRequests`. Tests drive the same sets with a deterministic
manual frame runner.

Each work item has a monotonically increasing internal sequence for diagnostics
only. Public ordering comes from per-volume lanes, captured revisions, receipt
states, and observation sequences.

## 2. Region lifecycle

Lifecycle is tracked per brick-aligned region and capability set:

```text
Cold
  -> Requested(reason, priority)
  -> Materializing(source_batch)
  -> Ready(revision, capabilities)
  -> Retiring(pins_remaining)
  -> Cold

Requested | Materializing -> Failed(error, retry)
Failed -> Requested       (explicit retry or source/device recovery policy)
Retiring -> Requested     (new interest cancels retirement before release)
```

A region is `Ready` only after verified base, applicable scars, page-table
publication, and the requested truth capabilities are installed. `PRESENT`
readiness is not part of authoritative readiness; its separate state is in
`queries-and-presentation.md`.

Adjacent regions with the same state/revision/reason may be coalesced for
observations and telemetry, but queries reason at brick granularity. Lifecycle
failures retain base lineage and scars. Retry is automatic only when the source
error explicitly supplied `retry_after` and the interest remains active;
otherwise the consumer calls `retry_region`.

## 3. Scheduler

The scheduler has finite queues for source, upload, mutation, query,
presentation, checkpoint, readback, and retirement work. It selects work by:

1. safety obligations (device recovery, dirty scar capture, shutdown);
2. already-admitted receipt work;
3. priority;
4. earliest consumer deadline;
5. FIFO sequence.

Deadlines are diagnostic and scheduling inputs, never permission to fabricate
truth or cancel admitted commands. Starvation counters promote an eligible
item one priority band after a configured frame count, except above safety
work.

Materialization batches are same-volume and adjacent-brick when possible, up
to explicit source/GPU byte limits. A source may complete out of order; install
waits only for its own validated batch. Mutations for one volume remain ordered
even while their source batches run concurrently.

## 4. Observation and receipt retention

Committed facts enter a world journal once. Each subscription ring contains
references to matching journal records and has an independent sequence. Rings
are fixed capacity. When adding an event would overwrite an unread event:

1. retain the last fully delivered sequence/revision vector;
2. replace unread entries with one `Gap` marker;
3. pause matching delivery until snapshot/resume; and
4. increment gap telemetry.

The journal retains payloads while any non-gapped subscription references them,
bounded by the aggregate observation limit. If the journal itself is full,
lagging subscriptions are gapped oldest-backlog-first; active subscriptions do
not block commits.

Receipts use a separate bounded terminal-result table. Pending receipts are
never evicted. On terminal overflow, the oldest acknowledged result is removed
first, then the oldest unacknowledged result creates a range-valued
`ReceiptGap`. Consumers can reconcile command facts with a bounded snapshot and
correlation IDs, but a lost terminal payload is never reported as success.

## 5. Device loss and recovery

GPU device loss transitions every world to `Recovering`:

- stop new command/query/checkpoint admission;
- fail unconfirmed GPU work with `DeviceLost` and no logical commit;
- keep confirmed CPU metadata, base descriptors, sparse scars, and placements;
- mark presentation absent and ready regions requested;
- request a new adapter/device using the same configured limits; and
- rematerialize previously pinned interest from verified base plus scars.

Confirmed revisions do not roll back. During recovery queries return
`Unavailable(Recovering)`, never empty. If adapter recreation or source replay
fails, affected worlds become `Failed`; consumers may retry startup/recovery or
shut down. Moria does not silently switch to CPU authority.

The sparse scar capture is CPU-confirmed before logical commit specifically so
device recovery cannot lose a committed mutation. Placement and topology scars
are small CPU metadata recorded by the same confirmation step.

## 6. Failure containment

- Source failure affects only requested batches for that source. Invalid source
  data is nonretryable until consumer action; declared transient unavailability
  may be retried.
- Mutation/query kernel validation failure fails that work item before
  publication. An unexpected GPU invariant fails the world and captures
  diagnostics; it does not continue with uncertain truth.
- Presentation failure changes only presentation state.
- Persistence sink failure leaves scars dirty and pinned.
- Behavior extension failure discards its proposal batch. Commands already
  admitted independently continue.
- A panic in consumer content code is outside Rust's cross-thread safety
  contract; Moria catches unwinds at its task boundary when panic strategy
  permits, reports `ContentUnavailable`, and never installs partial output.

Every failure log includes world/volume scope, stable error code, internal work
sequence, captured revision, limits usage, and adapter identity. Logs never dump
consumer matter payloads by default.

## 7. Shutdown

`begin_shutdown(mode)` closes admission immediately and returns a
`ShutdownTicket`.

Modes are:

- `Drain { checkpoint: Option<CheckpointRequest> }`: finish admitted commands
  and queries, optionally checkpoint their confirmed revisions, then release.
- `CancelPending { checkpoint: Option<CheckpointRequest> }`: fail work that has
  not begun execution, finish atomic work already executing, optionally
  checkpoint confirmed results, then release.

There is no discard-dirty mode in the normal API. If no checkpoint is requested
and dirty scars exist, shutdown completes as
`Blocked { dirty_revisions, dirty_bytes }`. The consumer must provide a sink,
keep the process alive, or call the explicitly named
`authorize_dirty_discard(DiscardToken)`. Creating a `DiscardToken` requires the
consumer to acknowledge the exact world and dirty revision vector; the action
is observed and logged.

Region withdrawal and application shutdown never call this authorization
implicitly. A completed shutdown reports every receipt outcome, checkpoint
coverage, and whether any explicitly authorized discard occurred.
