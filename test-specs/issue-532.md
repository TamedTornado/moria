# Issue 532 — Verify the participant RNG reference contract

References: `validation.md` TECH-059 RNG obligations; `architecture.md` TECH-016; issue M-115.

## Properties

- An independent published toy RNG contract defines seed decoding, full state bytes, next-state/output, rejection sampling, and exhaustion; oracle code does not import participant production evolution.
- For all generated steps, participant commitment contains stream ID, exact state length, and BLAKE3 state digest in stream order.

## Lifecycle configurations

- With `proptest = "=1.7.0"` and retained goldens, cover empty/max-64-byte seed, every state/output transition, snapshot bytes, reconstruction from genesis/log, rollback, checkpoint, cold restore, public replay, correction, and recovery.
- Verify all intermediate/final state digests after process teardown.
- Exercise multiple declared stream IDs including high-bit/max valid RngStreamId ordering.

## Error paths

- Undeclared stream, duplicate ID, wrong algorithm/version/contract/schema, malformed/oversized seed/state, exhaustion, missing snapshot/replay record, or divergent state produces exact participant/config failure and no publication.
- Instrument entropy sources to prove Moria requests no OS, wall-clock, thread, or hidden randomness.
- A 32-byte participant commitment alone cannot substitute for RNG descriptors/state commitments.
