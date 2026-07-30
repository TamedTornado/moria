# Issue 474 — Encode and validate checkpoint replay chunks

References: `content-persistence.md` TECH-044/045 replay-chunk contract; issue M-042.

## Input validation

- `moria-checkpoint-replay-v1` chunks contain one to 64 exact length-prefixed tick records unless one oversized record is permitted by both request/log limits.
- Header first/last/count, every embedded tick/checksum/digest, exact continuity, uncompressed byte count, and blob digest must agree.

## Transformation correctness

- Encode/decode one-record, 64-record, partial-final, and branch-subrecord chunks byte-identically.
- Extract an active tick frame from a correction branch using its physical sequence/subrecord locator; chunk bytes must equal the standalone tick-record frame.
- Concatenating manifest-ordered decoded chunks reproduces the exact inclusive participant-required range without gaps/overlap.

## Edge and error paths

- Reject zero records, 65 records, reversed range, missing/reordered/duplicate/overlapping/gapped ticks, short/long framing, wrong embedded checksum/digest, wrong descriptor length/blob digest, truncation/trailing bytes, and count/offset arithmetic overflow.
- Bounds are checked before allocation. An oversized single record that exceeds either named limit fails checkpoint admission; it is never split or truncated.
- Failure cannot make a checkpoint manifest commit or expose partial replay lease data.

No rendering state is defined.
