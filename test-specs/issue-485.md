# Issue 485 — Implement the GPU participant binding ABI

References: `collision-presentation.md` TECH-054; issue M-070.

## Input validation

- Validate exact 224-byte group-zero wire offsets, little-endian words, zero reserved word, six range records, effect/event capacities, option tags, and zero payloads for absent/unused fields.
- Enforce operation table: Genesis, Tick, RestoreSnapshot, Reconstruct, ExportSnapshot source/destination/attempted-tick combinations exactly as specified.

## Transformation correctness

- Encode/decode metadata views byte-for-byte for every operation; Genesis and Confirmed(0) remain distinct.
- All wrappers for one call share a private attempt/generation token. The designated primary wrapper binds only group zero; range/capacity accessors allocate nothing.
- Page-local ranges fit `u32`, effective bindings, usage and alignment; unused binding is the shared zero buffer with logical range zero.

## Edge and error paths

- Reject unknown tags, nonzero absent words, stale hashes, mixed attempts/generations, source/destination alias, unaligned/out-of-range buffers, count/byte overflow, missing status, and writes to unused slots.
- `bind_io` itself is infallible. Incompatible pipeline/layout before or after binding is caught by the balanced popped wgpu validation scope and publishes no output/token/frontier.
- Old generation closes wrappers and treats later status as late-generation cleanup.

No rendering state applies.
