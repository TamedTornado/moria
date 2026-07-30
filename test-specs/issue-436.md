# Issue 436 — Encode and validate replay identities and records

References: `content-persistence.md` TECH-047; issue M-045.

## Input validation

- `ReplayIdentityV1` is exactly 33 bytes: authority tag 0/1 plus configuration fingerprint; every other tag is invalid.
- Sequence zero accepts only `Header`; later sequences accept tick records or a valid correction branch. Header `next_tick` equals `starting.next_tick()`.
- V1 tick range requires count 1 and identical first/last tick. Branch validates target, superseded/corrected present, record count, framed contiguous records, and active-history identities.

## Transformation correctness

- Encode/decode Genesis and checkpoint anchors, ordinary tick records, and correction branches byte-for-byte with fixed framing/checksum.
- Adapter/backend/driver/process/fault-plan context does not change replay identity/header/record/prefix bytes; placement/arithmetic/configuration change does.
- Genesis and `Confirmed(0)` headers/facts remain byte-distinct.

## Edge and error paths

- Cover tick/sequence/count/length overflow, unknown tags/version, status mismatch, wrong anchor restore limits, gapped/reordered/overlapping embedded frames, incorrect digest/checksum, truncation, and trailing bytes.
- Compatibility mismatch is rejected before participant callback, GPU submission, canonical transition, or destination sink invocation.
- Decoding remains bounded by request limits and never silently ignores physical branch records or regenerates bytes.

No rendering state applies.
