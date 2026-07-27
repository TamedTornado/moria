# Asynchronous Lifecycles and Failure Semantics

## World lifecycle

```text
Configured
   -> Starting
      -> Ready
      -> Failed
Ready
   -> Recovering -> Ready | Failed
   -> ShuttingDown -> Stopped | Failed
```

`Configured` is host-validated and has no GPU state. It is represented by the
handles and startup receipt returned from `ValidatedMoria::into_bevy`.
Installing the returned plugin enters `Starting`, negotiates the device,
records effective adapter-clamped limits, and installs pipelines/directory
state. Fresh startup resolves `StartupApplied::Fresh`; restore resolves
`StartupApplied::Restored(RestoreApplied)` with the complete restored revision
context. Only `Ready` accepts ordinary permits. `Recovering` rejects new
admissions with retryable state, keeps unsubmitted owned payloads, and does not
answer material queries. `Failed` and `Stopped` are terminal for that world
handle.

Startup failure includes a stage and all actionable causes. There is no
partially usable hidden world.

## Region lifecycle

The consumer-visible region states are exactly:

- `Cold`
- `Requested`
- `Materializing`
- `Ready { revision }`
- `Retiring`
- `Failed { failure: RegionFailure }`

Regions are reported at brick granularity and can be aggregated only when
state/revision/cause are equal. Legal transitions:

```text
Cold -> Requested
Requested -> Materializing | Cold | Failed
Materializing -> Ready | Failed
Ready -> Requested | Retiring | Failed
Retiring -> Cold | Requested | Failed
Failed -> Requested | Cold
```

`Ready` means authoritative inspection and collision are available for its
reported revision. It says nothing about presentation. A mutation may advance a
ready region directly to `Ready(new_revision)` at commit. An observation is
emitted after each externally visible transition; adjacent equal-brick
transitions may be coalesced into bounded AABBs with the same sequence fact.

A failed content request records affected bricks. `RegionFailure` retains its
machine-actionable content/budget/device/retirement cause, retryability,
generation where relevant, and bounded diagnostic. Retry occurs only through
new interest/query work and that classification; no busy loop is built in.

## Interest lifecycle

1. Validate scope, capabilities, max bricks, and queue capacity.
2. Snapshot the volume filter (`All` means every currently live ID), placement
   revisions, and deterministic sorted local brick set. If volume or brick
   count would exceed its request bound, reject with required count; never
   clip.
3. Add reference counts by capability and priority.
4. Move cold bricks to `Requested` and enqueue bounded content work.
5. When every brick needed for a capability is ready, report capability
   readiness on the lease.
6. Update atomically removes old/adds new references after the replacement has
   been fully validated.
7. Withdrawal decrements references. Bricks enter `Retiring` only after pins,
   admitted work, and dirty-state obligations clear.

Multiple interests coalesce storage work but retain independent leases and
readiness. Presentation interest does not imply collision interest; all three
still depend on authoritative material readiness.

Create, retire, and move never expand or spatially recompute an accepted lease.
Retirement makes that pinned member failed/retired. `InterestLease::update` is
the only operation that takes a new membership, placement, and brick snapshot.

## Command lifecycle

```text
owned by consumer
  -> capacity reserved
  -> admitted/Queued
  -> WaitingForMatter
  -> Preparing
  -> Submitted
  -> Applied
or terminal Rejected/Failed/CancelledBeforePreparation
```

Structural rejection occurs synchronously and consumes no command identity.
Admission consumes the permit/payload, assigns `CommandId`, and creates the
receipt. After admission:

- stale precondition discovered before publication is terminal `Conflict`;
- cancellation succeeds only while queued/waiting; the atomic transition to
  `Preparing` is the point of no return;
- a preparation failure commits nothing;
- `Preparing` and later cannot be cancelled, even if every receipt is dropped;
- `Applied` means the revision gate was executed successfully; it does not mean
  the revision is durable until a checkpoint receipt covers it;
- an observation is appended before the receipt wakeup is delivered, so a
  consumer awakened on `Applied` can poll that fact or later encounter a gap.

Per-volume FIFO determines preparation order, not necessarily receipt wakeup
order between volumes. Independent volume commands may execute concurrently.

Every cancellable operation family uses the same race: cancellation CASes
`Queued | WaitingForMatter` directly to terminal
`CancelledBeforePreparation`, while the worker CASes those states to
`Preparing`. The winner determines `Accepted` versus `TooLate { stage }`.
Startup and shutdown lifecycle receipts are explicitly noncancellable.

Create/retire failure reports any committed directory revision. Matter-command
failure always commits no revision. Move uses the same prepare/publish rule as
matter but changes only directory state.

Create consumes one concurrent `live_volumes` slot and one permanent
`volume_records` slot. Retire releases only the live slot and converts the
permanent record into a tombstone. Live capacity is retryable after retirement;
lifetime record exhaustion is nonretryable for new keys in that world and is
reported before admission.

## Query lifecycle

```text
admitted
  -> acquire snapshot
  -> Ready to dispatch
  -> Submitted
  -> AwaitingReadback
  -> Complete | PartialRequested | Pending | Unavailable | Failed
```

A query with `ReadinessPolicy::Pending` may terminate as `Pending` immediately
after the control plane identifies cold/failed scope. This is a successful
availability response, not an empty query result. With `Materialize`, internal
interest is held through query completion and follows region lifecycle.

Minimum revision policy is:

- `AnyCommitted`: capture current committed revisions;
- `AtLeast(volume, R)`: wait until `>= R`, fail if retired/terminal;
- `Exact(volume, R)`: use R only while its version is still retained,
  otherwise return `RevisionUnavailable`.

Waiting is bounded by retained receipt and residency limits, not by an implicit
wall-clock timeout. Consumers implement deadlines by explicit cancellation.

Readback map completion and successful decode are required before a CPU result
is ready. Queue submission alone is never result visibility.

Collision queries authorize conservative traversal independently from result
bytes/hits. Shape/region occupancy, trace, overlap, and sweep are rejected
before admission unless their transformed aggregate candidates fit the stated
and fixed brick/cell limits. Partial coverage can omit unavailable bricks only;
result-cap overflow always fails with no truncated result.

## Observation lifecycle

One per-world ring stores immutable public facts together with fixed 128-byte
append-time filter envelopes. Append computes local and world extents from the
fact's committed placement/version before reclamation, charges the complete
fact-plus-envelope record to both ring limits, and assigns a checked sequence.
Subscribers retain cursors, not private unbounded queues.

Filtering occurs while polling against that retained envelope, never by
consulting the current directory. A cursor advances across nonmatching facts so
one narrow subscriber cannot pin ring history. Ring overwrite evicts a public
fact and its envelope together; it turns lag into the explicit gap protocol in
[public-api.md](public-api.md). Creating a subscription begins at
`CurrentHead` by default or at a retained sequence explicitly requested.

Subscription membership is also a snapshot. `All` pins all volumes live at
subscription acceptance; later creates are excluded and retirement terminates
only that member. Spatial bounds remain an event predicate for pinned IDs, with
move facts matching either old or new placement. Resubscription is the only
way to include a later-created volume.

Gap recovery:

1. subscriber receives `Gap` and becomes `NeedsSnapshot`;
2. subscriber requests a bounded snapshot covering the accepted resolved
   subscription membership;
3. snapshot result contains the observation head and exactly one live or
   retired state record for every pinned member; a retired record carries its
   stable key and terminal revision even if the retirement fact was lost;
4. `resume_after` validates the subscriber/scope and advances its cursor to
   that head;
5. facts after the captured head become deliverable.

This closes the race between snapshot and resume without retaining every
intermediate event.

GPU observation-delta capture is a separate read cursor supplied per request.
It pins the subscriber's accepted membership/filter but neither observes nor
changes the CPU cursor state. Each capture freezes oldest/head, scans retained
fact-plus-envelope records through that head, and returns `Complete`,
`MoreAvailable`, `NeedsSnapshot`, or `UnsupportedFact` in both the ABI header
and public outcome. A gap or matching nonrepresentable fact emits zero records,
forbids candidate effects, and never skips the boundary. Recovery takes a
bounded non-resuming `SubscriptionState` snapshot and restarts after that
snapshot head; only an independently gapped CPU cursor uses `resume_after`.

## Presentation lifecycle

For each interested brick artifact:

```text
Absent
  -> Building(target R)
  -> Current(R) | Failed(R)
Current(R)
  -> Stale(visible R, target N)
  -> Absent
Stale
  -> Current(N) | Failed(N) | Absent
Failed
  -> Building(same/new target) | Absent
```

One active build and one visible artifact maximum exist per brick/style.
Commits may supersede queued builds before submission. Submitted obsolete
builds finish but are discarded by source-revision comparison. Failure never
changes truth or collision.

One command can enqueue at most 13,824 unique halo-invalidated artifact keys,
not 27. Exact invalidations drain through at most `presentation_jobs`
concurrent slots. If dirty-record pressure coalesces them to a volume marker,
bounded scans of active `presentation_artifacts` recreate the exact current
target set. Within one priority, stable volume/brick ordering plus a rotating
cursor guarantees that a continuously interested, nonsuperseded artifact is
eventually scheduled; newer commits may replace its target revision but cannot
starve the artifact indefinitely.

Placement-only revisions reuse mesh buffers and update render transforms after
the placement commit. Their presentation status becomes current for the new
revision only when the tagged entity transform is installed.

## Checkpoint lifecycle

```text
admitted frontier F
  -> PinningScarVersions
  -> GPUReadback
  -> Encoding
  -> WritingChunks
  -> PublishingManifest
  -> Durable(F) | Failed
```

The captured frontier never expands. Later commits remain dirty. Chunk writes
are not a successful checkpoint; atomic manifest publication is. On failure,
pins are released only if the scar remains retained by another safe form.
Incomplete store transactions are aborted best-effort and ignored by restore.
V1 captures the whole live-world directory; no scope variant can omit a live
volume.

Restore:

```text
Configured
  -> ReadingManifest
  -> ValidatingContractAndRegistrations
  -> ReadingAndValidatingChunks
  -> ReconstructingDirectory
  -> ReadyCold
```

No restored volume is public before all manifest-level checks pass. Individual
bricks remain cold and materialize as base plus scar under interest. A corrupt
chunk fails restore rather than failing later as an apparently empty region.

## GPU extension lifecycle

```text
registered/validated
  -> extension + worst-case child batch capacity reserved
  -> admitted closed ABI inspection + prior/inline state
  -> packet captured at revisions/ring head with explicit inspection status
  -> external shader submitted
  -> whole candidate output validated
  -> every child admitted or zero children admitted
  -> extension dispatch outcome with diagnostics, next-state lease,
     and every child receipt
```

Registration itself consumes bounded world-lifetime descriptor and WGSL bytes;
registry exhaustion fails synchronously before pipeline creation. Prior GPU
state is an immutable lease and each dispatch writes a new bounded state
generation. State pressure follows extension admission policy, and device loss
makes every old state lease stale without changing material truth.

Before dispatch, `EffectBatchPermit` reserves the descriptor's worst-case
ordinary command record count, aggregate encoded payload bytes, and child
completion slots. Candidate output is a batch with all-or-none validation and
all-or-none child admission. Invalid output assigns no command IDs and commits
nothing. A smaller valid output releases unused capacity immediately; the
outer receipt returns all child receipts in output order. Those effects then
remain independent public commands unless the external system encoded one
bounded patch command. A later conflict/failure of one child does not undo
another child and does not create cross-volume atomicity.

If the inspection snapshot becomes stale before an effect prepares, its
mandatory revision precondition causes conflict. Moria never silently reruns
external behavior against newer matter.

## Shutdown lifecycle

`shutdown(policy)` is itself accepted once. It:

1. closes all permit waiters and rejects new submissions;
2. with `CancelNotPrepared`, atomically cancels work still
   `Queued | WaitingForMatter`; otherwise drains it; `Preparing` and later
   always drain;
3. stops accepting interest updates and withdraws ordinary interest after
   dependent work;
4. waits for submitted GPU work through renderer completion or terminal loss;
5. runs the required checkpoint against final committed revisions if requested;
6. appends final observations and freezes telemetry;
7. resolves outstanding receipts and the shutdown receipt;
8. releases render resources after their last submission.

Content/store callbacks receive cancellation when their results are no longer
needed. The shutdown report distinguishes clean durability, cancelled work,
failed work, and unrecoverable dirty state.

## Retry semantics

Every error includes one of:

- `Never`: malformed request, missing identity, incompatible contract/base,
  stale exact revision, counter exhaustion;
- `AfterInputChange`: invalid content, missing material registration,
  insufficient configured maximum;
- `AfterPressureRelief`: bounded queues/pools/staging exhausted;
- `AfterRecovery`: device lost or renderer temporarily unavailable;
- `Immediate`: transient source/store failure explicitly classified retryable.

Moria does not automatically retry consumer commands after conflict, because
their meaning may have changed. It may retry internal presentation derivation
for the same revision only on explicit consumer policy and with a bounded retry
count (default 1).

## Failure table

| Failure | Scope/state | Receipt/result | Truth outcome |
| --- | --- | --- | --- |
| Invalid configuration | World startup | `ConfigurationErrors` | No world |
| Source unavailable | Region | Pending or Failed | Unknown, never empty |
| Invalid source batch | Region | `Content::InvalidBatch` | Nothing installed |
| Cold query | Query | Explicit Pending | No facts fabricated |
| Query/output too large | Request | Rejected/Overflow | No clipping |
| Missing/stale ID | Request | Rejected | No GPU work |
| Stale revision | Command | Conflict | No commit |
| Queue/pool pressure | Admission/operation | Full, defer, or BudgetExhausted | Existing truth unchanged |
| Shader validation/dispatch error | Operation | GpuValidation | No publication |
| Mutation validation sentinel | Command | InternalInvariant | No publication |
| Presentation overflow | Artifact | Failed | Matter/collision usable |
| Observation overwrite | Subscriber | Gap | Truth intact |
| Checkpoint write/publish failure | Checkpoint | Persistence error | Dirty remains nondurable |
| Lineage/fingerprint mismatch | Restore | IncompatibleBase | Nothing restored |
| Device loss, durable scars | World | Submitted work fails; Recovering | Unavailable until rebuilt |
| Device loss, volatile dirty scars | World | UnrecoverableDirtyState | Terminal, no false rollback |
| External shader failure | Extension request | Extension error | Only earlier ordinary commits remain |

## Time and determinism

Correctness uses revisions and queue order, never wall-clock time. Worker and
GPU completion order between independent volumes is allowed to vary. Tests
advance Bevy apps explicitly and inject completions; they do not sleep.

Given the same registered base, stable keys, ordered commands, and starting
revision, material results and checkpoint bytes are deterministic. Presentation
vertex order and dressing placement are deterministic for a
`(volume key, brick, source revision, style key)` tuple. Floating-point rendered
pixels are not promised identical across GPU backends.
