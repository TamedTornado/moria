# Runtime, Lifecycles, Failure, and Shutdown

## 1. Scheduling model

Moria uses five bounded stages:

1. main-world intake validates requests and advances CPU state machines;
2. I/O tasks fetch base content or persistence records;
3. render extraction copies immutable packets into the RenderApp;
4. Moria render-graph nodes dispatch ordered GPU work and enqueue readbacks;
5. completion pump returns compact results to the main world and publishes
   terminal state.

Channels are bounded MPSC queues with explicit `QueueFull` rejection. There is
one logical ordered control stream per world and parallel query/presentation
streams. Commands for the same volume are dispatched FIFO after admission.
Independent volumes may run concurrently. Queries capture a revision set when
dispatched and order relative to commits through GPU queue ordering.

The scheduler owns an immutable `Arc<CatalogSnapshot>` containing active
volume descriptors and the world-space AABB index. World-scope query,
interest, observation-snapshot, and checkpoint planning capture one snapshot
under a short read barrier. Create/final-retire publication and placement
publication take the matching write barrier and replace the snapshot; no
consumer callback or I/O runs while either barrier is held. Per-volume commit
barriers described in `matter-and-storage.md` prevent a query from pairing an
old CPU revision with post-swap GPU cells.

Priorities select ready work but do not reorder admitted commands on one
volume. Starvation protection increments effective priority once per 60 frames
up to 255. Deadlines never weaken truth.

## 2. World state

```text
Configuring -> Ready -> RecoveringDevice -> Ready
      |          |  \
      |          |   +-> Failed
      |          +-----> Quiescing -> Stopped
      v
    Failed
```

- `Configuring`: collaborators and adapter/pipelines are validated; no world
  operation is admitted.
- `Ready`: normal admission.
- `Quiescing`: new interests, queries, commands, and checkpoints are rejected;
  prior work follows shutdown policy.
- `Stopped`: all receipts are terminal and resources released.
- `Failed`: startup failed with no usable world, or a ready world stopped after
  an internal/device-recovery failure while retaining its last known published
  revision context and dirty scars.
- `RecoveringDevice`: all GPU results and unpublished transactions from the
  lost device are rejected, new admissions pause, and authority is rebuilt
  from exact base plus CPU scars. Applied revisions remain known.

A post-publication `Failed` world admits only ticket polling/acknowledgment,
telemetry, observation draining, discard handshake, and
durability-preserving shutdown. It rejects new truth, presentation, interest,
extension, and checkpoint operations; shutdown owns any required final
checkpoint so failure cannot be bypassed with ordinary work.

Device recovery uses the configured source retry policy and the same proof
validation as ordinary materialization. A second device loss during recovery,
adapter recreation failure, permanent source/proof failure, or exhausted retry
budget transitions the world to `Failed`; it never exposes a partially rebuilt
ready set. Consumers may still inspect failure/telemetry and request a
durability-preserving shutdown.

## 3. Region lifecycle

```text
Cold -> Requested -> Materializing -> Ready -> Retiring -> Cold
           |              |           |          |
           +------------> Failed <----+----------+
```

Every state record contains volume, aligned region bounds, target/current
revision, required capabilities, pin counts, retry metadata, and last error.

- `Cold`: source identity and durable scars are known; no detailed authority is
  queryable.
- `Requested`: at least one interest/operation pin requires capabilities.
- `Materializing`: source/staging/GPU work is in flight.
- `Ready`: required authoritative matter/occupancy is published for the stated
  revision. Presentation is tracked separately.
- `Retiring`: no new interest pins; reads already dispatched and dirty/durable
  obligations drain.
- `Failed`: the error and retryability are visible. A new explicit retry or a
  policy retry moves to `Requested`.

An edit or placement commit changes a volume revision. Unaffected ready regions
are relabeled to that volume revision because their cell content is unchanged
and the volume state revision advanced; affected regions rebuild aggregates
inside the commit. This avoids falsely treating unaffected data as stale while
preserving one volume revision.

Pin kinds are interest, admitted command, query, checkpoint, presentation, and
extension exchange. A region can enter `Retiring` only at zero pins. It can
become `Cold` only when GPU readers are fenced, presentation allocations are
released/retained under their budget, and dirty scars exist in the CPU scar
store or are durable.

## 4. Operation state machines

### Command

```text
Admitted -> WaitingForMatter -> Staging -> Validating -> Committing -> Applied
    |              |             |            |
    +------------> Failed <------+------------+
    +------------> Cancelled (only before Committing)
```

`Committing` is noncancellable. `Applied` includes revision and affected bounds.
Matter-command `Failed` before commit guarantees `CommitEffect::None`.

### Query

```text
Admitted -> WaitingForMatter -> Dispatched -> ReadingBack -> Complete
    |              |              |              |
    +------------> Unavailable/Failed/Cancelled <-+
```

Query cancellation is accepted until dispatch. After dispatch, Moria may drop
the readback result and report cancelled but keeps resources pinned until its
fence. `Unavailable` is a terminal typed outcome distinct from a complete
empty/no-hit result.

If cancellation is requested after dispatch, `request_cancel` returns
`TooLate` unless the implementation has already installed a discard-result
flag before GPU submission. Thus one call never reports `Requested` and later
publishes `Complete`; `Requested` deterministically leads to `Cancelled`.

### Checkpoint

```text
Admitted -> CapturingCut -> Encoding -> Writing -> Verifying -> Durable
    |             |            |         |            |
    +-----------> Failed/Cancelled (cancellation only before Writing)
```

Once the store begins its commit protocol, cancellation is refused so Moria
can determine whether a checkpoint became durable. Later mutations are not
blocked and remain dirty beyond the captured cut.

### Presentation

Presentation status is per region:

```text
Absent -> Building -> Current
             |          |
             v          v
           Failed     Stale -> Building
```

An older allocation may remain visible in `Stale`. It retains its actual
revision label and is never queried for collision.

## 5. Retry and failure policy

Errors declare:

- scope (world, volume, region, operation, presentation, checkpoint);
- stable category;
- retry advice (`Never`, `After(Duration)`, `AfterResourceChange`,
  `AfterSourceRepair`, `RestoreRequired`);
- committed effect; and
- bounded diagnostic context.

Default automatic retries apply only to base-source transient unavailability
and persistence transport errors: exponential backoff from 50 ms to 2 s,
maximum five attempts, with deterministic per-operation jitter derived from
its ID. Invalid input, proof mismatch, corruption, stale revision, adapter
capability, and invariant errors are never auto-retried.

A failed region never answers matter/collision. A presentation failure leaves
matter ready. A failed external extension terminates its exchange and any
not-yet-admitted effects; previously admitted commands continue normally.

Internal invariant violations are not converted to consumer mistakes. Moria
records diagnostics, stops admissions for the affected world, fences work when
possible, and reports `Failed` with the last known published revision set.
Publication ordering ensures an operation is either already published with a
known revision or has no committed effect.

## 6. Revision and publication ordering

Within a volume:

1. admitted commands have a monotonically increasing admission sequence;
2. a query captures the latest published revision at dispatch;
3. GPU commit completion precedes CPU revision publication;
4. receipt completion and committed observation are enqueued from one main
   world transaction; and
5. observation sequence establishes publication order, not cross-volume
   revision order.

A consumer may observe either receipt or event first depending on which bounded
queue it drains, but both contain the same operation ID/revision. Polling after
either publication returns the same terminal fact.

No global atomic snapshot across independent volumes is promised.
`RevisionSet` tells exactly what a multi-volume query used.

World-scope dispatch linearizes at catalog-snapshot capture. Volume revisions
in that result may differ because prior independent commands publish
independently, but every entry and placement comes from the captured catalog
snapshot and every GPU reader is ordered at that entry's recorded revision.

## 7. Observation retention and backpressure

Each subscription owns only its bounded ring. Moria does not retain a global
unbounded event log. At enqueue overflow it:

1. records the last sequence known delivered and current revision summary;
2. drops all undelivered events for that subscription;
3. enqueues one `Gap`;
4. increments gap telemetry; and
5. blocks normal delivery until bounded snapshot/resume completes.

If even the gap cannot be retained because the consumer never polls, the
subscription state itself remains `Gapped`; the next poll synthesizes the same
marker. Subscription expiration after configured idle time is explicit and
does not affect world truth.

`ObservationSequence` is allocated once per world publication before filters
are evaluated. Filtered-out sequences need not appear in a subscription. A gap
names the first and last lost event that matched that subscription, not merely
the next global number. Snapshot/resume uses the global sequence barrier in
`public-api.md`, so commits concurrent with snapshot capture appear exactly
once either in the snapshot state or after the resume token.

## 8. Resource pressure behavior

On pressure, runtime acts in this order:

1. reclaim fenced transaction/readback allocations;
2. retire zero-pin presentation/dressing allocations;
3. retire zero-pin authoritative regions with no unsafe persistence
   obligation, lowest effective priority first;
4. delay queued low-priority materialization/presentation;
5. reject the new request that cannot reserve its stated worst case.

It never revokes an admitted command's reservation, evicts pinned authority,
or discards dirty scars. Every delay/retire/reject emits aggregate telemetry;
consumer-relevant decisions also emit bounded resource-pressure observations.

## 9. Explicit discard

The only dirty-data discard path is the explicit two-step pair:

```rust
let proposal = world.prepare_discard_undurable()?;
world.confirm_discard_undurable(proposal.token, proposal.exact_revisions)?;
```

`DiscardProposal` contains the world ID, exact dirty `RevisionSet`, dirty scar
bytes, token, and proposal generation. It contains no boolean shorthand that
could authorize a later revision accidentally.

The first call returns the exact dirty revisions and a random nonce; the
second call must echo both. Confirmation is rejected during an in-flight
checkpoint or with mismatched revisions. Success emits an observation and
telemetry audit record.

For discard-and-stop, `shutdown(DiscardWithToken { token, exact_revisions })`
is the second step instead of `confirm_discard_undurable`: it atomically
validates the proposal, enters `Quiescing`, and only then discards. A token is
single-use in either path. Calling `confirm_discard_undurable` discards while
the world remains ready and does not itself authorize shutdown.

This path is not used by eviction or ordinary shutdown. It exists because the
approved design allows explicit consumer authorization while forbidding silent
loss.

## 10. Shutdown

`shutdown(policy)` atomically enters `Quiescing`. Policies:

```rust
pub enum ShutdownPolicy {
    RequireDurable { timeout: Duration },
    DiscardWithToken {
        token: DiscardToken,
        exact_revisions: RevisionSet,
    },
}

pub struct ShutdownResult {
    pub final_revisions: RevisionSet,
    pub durable: Option<RevisionSet>,
    pub discarded: Option<RevisionSet>,
}
```

- `RequireDurable { timeout }`: require the world's configured
  `Checkpointed` store, stop admissions, finish already
  committing work, cancel other cancellable work, checkpoint the resulting
  dirty revisions, then release resources. Timeout fails that shutdown ticket
  with dirty revisions intact and leaves the world in `Quiescing`; the
  consumer may submit another `RequireDurable` shutdown attempt or use the
  explicit discard handshake.
- `DiscardWithToken`: requires the two-step exact discard authority above.

While `Quiescing`, ordinary operations remain rejected, but repeated shutdown,
ticket polling/acknowledgment, telemetry, discard preparation/confirmation,
and observation draining remain legal. `DiscardWithToken` is rejected unless
its token and exact revision set were prepared after the most recent commit
and no checkpoint is in its writing/verification phase.

`RequireDurable` is rejected before entering `Quiescing` for a `MemoryOnly`
world because no durability collaborator exists. That world can remain alive
or use the explicit discard handshake; Moria does not pretend process memory
is durable.

Dropping the Bevy app without completed shutdown cannot preserve process memory.
Moria logs a structured critical diagnostic naming undurable revisions; it
does not claim a successful shutdown. Conformance verifies normal explicit
shutdown. OS/process termination is outside any in-process durability
guarantee.

On successful stop, all operations have terminal statuses, store writes are
known durable or explicitly discarded, GPU queues are fenced, and an immutable
shutdown report identifies durable and discarded revision sets.
