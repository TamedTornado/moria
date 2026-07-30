# Issue 428 — Encode and decode canonical wire records

References: `architecture.md` TECH-008; issue M-017.

## Input validation

- Decode only fixed-width little-endian integers, known `u8` tags, 32-byte digests, `u32`-count sequences, and canonical option tags 0/1.
- Reject platform-sized values, floats, strings/maps in canonical records, implicit padding, unknown enum tags, nonminimal option tags, excessive lengths, and arithmetic overflow before allocation.

## Transformation correctness

- For every canonical record named by the TDD, `decode(encode(x)) == x` and re-encoding decoded bytes is byte-identical.
- Signed extrema retain exact two’s-complement bytes; sequence order is preserved; Genesis and `Confirmed(0)` remain distinct.
- CPU encodings and WGSL ABI fixtures match field-for-field where a record crosses the GPU boundary.

## Edge and error paths

- Cover empty and maximum sequences, every one-byte truncation point, extra trailing bytes, length prefix one below/equal/above available bytes, invalid UTF-8 only where noncanonical labels are decoded, and checked allocation multiplication/addition overflow.
- A failed decode performs no partial registry/world publication and reports the named layer, not a generic panic.
- Fuzz arbitrary bytes under declared caps and prove bounded time/allocation.

Rendering states are not defined by TECH-008.
