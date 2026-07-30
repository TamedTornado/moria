# Issue 429 — Define canonical inputs and command outcomes

References: `interfaces.md` TECH-020; issue M-021.

## Input validation

- Construct every `CanonicalInput`, `MutationShapeQ`, `EraseMode`, `ParticipantInput`, `CommandOutcome`, and `TickConfirmed` variant at minimum and maximum bounds.
- Stamp masks have exactly `ceil(cell_count/8)` bytes in x-major/y/z bit order and zero unused high bits. `SubtractDensity.amount_q8_8` is positive.
- Patch cells are bounded, sorted, unique, and within the named volume domain; participant input schema/source/size must match registration.

## Transformation correctness

- Correlation sidecars associate with the unique canonical input key and resulting `CanonicalOrder` but never change canonical bytes, digest, sorting, outcome, hash, or participant input.
- `Applied`, `Failed`, and `NoOp` retain submitted tick/order; applied results carry exact affected bounds/revision/root; failed results carry stable reason and no writes.
- Participant events are sorted by `(ParticipantId, local_sequence)` and their digest binds exact schema/payload bytes.

## Edge and error paths

- Reject invalid cells, zero/negative sphere radius where forbidden, malformed masks, duplicate patch cells, unknown source/schema, oversized payload/correlation, and invalid shapes without partial normalization.
- Replayed outcomes carry `correlation: None`; ring expiry/gap may lose correlation but never canonical fields.

No rendering states belong to this record component.
