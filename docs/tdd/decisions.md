# Technical design decisions

This record preserves human feedback received during technical-design review
and the technical interpretation applied to the TDD. Verbatim feedback is kept
separate from interpretation. Product authority remains in
[`../design-document.md`](../design-document.md).

## Human review entry

### Verbatim feedback

```text
Is this as simple as it can be while still satisfying the requirements? If yes, leave the TDD unchanged. If no, revise the TDD to make it the simplest sufficient design.
```

### Technical decision or clarification

The TDD was not yet as simple as it could be. The two-package Cargo workspace
did not provide a necessary isolation boundary: one Cargo package can build a
public `moria` library crate and a separate `moria-qualify` binary crate. Rust's
target privacy still prevents that binary from accessing the library's private
or `pub(crate)` implementation, so it remains an external-style consumer of
the public facade.

The repository contract is therefore simplified to one root package with one
library target and one qualification binary target. No canonical, GPU,
rollback, persistence, collision, participant, or validation mechanism is
removed: those mechanisms are the minimum selected implementation of explicit
approved requirements rather than optional product scope.

### Unresolved question

None.

---

## Human review entry — consumer-contract completeness

### Verbatim feedback

```text
TamedTornado (COMMENTED):
The regenerated TDD is substantially cleaner, and its determinism, rollback,
hashing, collision, persistence, and scope boundaries should be preserved.
However, comparison with the approved pre-amendment TDD found that the size
reduction also removed implementation-critical contracts. Please revise the
current TDD to close the following holes without restoring the old TDD
wholesale or reviving its over-engineered behavior scheduler.

1. **Complete the public facade.** `interfaces.md` says its Rust signatures are
   normative, but `MoriaClient` currently exposes only construction and tick
   submission. Add normative callable shapes, ownership, results, and receipt
   behavior for queries, interest upsert/withdrawal, observation subscription
   and polling, checkpoint requests, correction/restore, telemetry, and
   shutdown. The existing prose for these capabilities is not a substitute for
   a callable contract.

2. **Define `ResourceBudgets` completely.** It is named in `MoriaConfig` but has
   no normative field schema. Specify every bounded queue/pool/output retained
   by the current design, its portable maximum or configuration rule, and the
   cross-limit validation performed before genesis. In particular, make
   callback outputs, observation records and payload bytes, identity/lifetime
   records, query/readback resources, presentation work, checkpoint staging,
   rollback retention, and participant state/effects/snapshots unambiguous.

3. **Close base-content callback ownership.** Define
   `BaseBrickCompletion` (or its replacement) so capacity and worst-case bytes
   are reserved before consumer code runs, successful output lands in a
   Moria-owned bounded sink, and no unbounded consumer-owned collection or
   diagnostic allocation crosses into Moria. Define descriptor ownership,
   completion cardinality, cancellation, duplicate/late completion, exact byte
   validation, and resource release.

4. **Complete observation subscription semantics.** Define the subscription
   request and creation API, accepted finite volume membership, kind/spatial
   filtering, cursor start, close/drop behavior, gap recovery, bounded
   resnapshot, and resume behavior. Filtering must remain historically honest
   after movement, retirement, and directory-version reclamation. Either retain
   append-time filtering evidence or specify an equally exact replacement; do
   not evaluate historical facts against current placement.

5. **Connect lifecycle prose to exact APIs.** For every retained asynchronous
   operation family, define admission, cancellation point, pollable receipt
   states, terminal retention, retry rules, ownership returned on rejection,
   shutdown behavior, and device-generation behavior. Queries, observations,
   checkpoints, and correction/restore currently need particular attention.

6. **Make participant simplification explicit.** Do not restore the old
   ordering DAG, handoff buffers, conflict machinery, or prior-feedback ABI by
   default. Instead, state explicitly whether v1 supports or rejects:
   same-tick dependency between participants, bounded participant-owned event
   output to the consumer, and conflicts between participant effects. Ensure
   the selected minimal contract is sufficient for an external GPU-oriented
   physics/damage implementation to receive tick input, produce admitted
   effects, preserve its own state, and expose its own collision/simulation
   events without giving Moria a behavior vocabulary.

7. **State the performance-policy replacement.** The old TDD used P1-P10 hard
   architecture-feasibility gates. The current GDD appears to replace those
   with universal correctness, one named rollback qualification tier, and
   hardware-contextual benchmark reporting. Record that replacement explicitly
   and ensure the benchmark suite still measures sparse residency, mutation,
   query/collision, materialization, presentation, participant, checkpoint, and
   rollback paths. Do not silently imply the old gates remain satisfied.

8. **Mechanically clean the normative snippets.** For example,
   `MinimumVolumeRevision` currently declares `revision` twice. Inspect all
   normative Rust shapes for duplicate fields, unresolved named types, and
   missing facade connections.

After revision, update traceability and the review findings so approval asserts
both GDD coverage and implementation completeness. The desired result is the
current compact architecture with a complete consumer contract—not a return to
the 11,000-line design.
```

### Technical decision or clarification

The compact one-package architecture and the existing deterministic
publication, rollback, hashing, collision, persistence, and scope contracts
remain selected. The review requires implementation closure, not restoration
of the superseded large TDD.

The applied technical decisions are:

- add one complete normative facade surface and concrete receipt/cancellation
  behavior while retaining focused ownership contracts for each operation;
- make `ResourceBudgets` a closed nested schema with defaults, portable maxima,
  overload outcomes, and checked pre-genesis cross-limit equations;
- make base-source and store callback outputs producer-written into
  pre-reserved Moria-owned bounded completion cells;
- use a finite admitted observation membership and append-time immutable
  filter facts, with count/byte gaps and bounded frontier resnapshot/resume;
- retain a one-phase participant model: same-tick participant dependencies are
  rejected, bounded opaque participant events are supported for consumer
  delivery after confirmation, and effects use only ordinary deterministic
  command ordering/preconditions—no DAG, handoff, conflict subsystem, or
  prior-feedback ABI;
- explicitly supersede `P1`–`P10` with universal correctness gates, the one
  named 20-tick rollback tier, and hardware-contextual receipts covering all
  named performance paths; and
- require mechanical public-type/facade closure and distinguish approved GDD
  coverage from approved implementation completeness in the final gate.

### Unresolved question

None. The feedback resolves the consequential direction, and the remaining
field limits, lifecycle choices, participant event transport, and validation
mechanics are ordinary engineering decisions selected in the TDD.
