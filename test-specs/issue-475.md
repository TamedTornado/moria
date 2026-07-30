# Issue 475 — Implement and verify the host/WGSL ABI

References: `gpu-runtime.md` TECH-034; issue M-062.

## Input validation

- For every wire type, verify the declared field order, byte offset, alignment, total size, array stride, effective binding range, and logical record count.
- Reject Rust bool/enums/pointers/`usize`/implicit padding, WGSL `vec3` durable fields, runtime arrays not last, and bindings whose effective range/count disagrees with the request.

## Transformation correctness

- CPU encoder and shader fixture write/read every scalar, packed `CellWire`, digest word, counter, flag, padding word, and unused slot byte exactly; all padding/unused output starts zero.
- Checked dispatch count equals `count/width + (count%width != 0)` and every over-dispatched invocation guards the logical count.
- Indirect records are exactly `[x,y,z]` as three packed u32 words at four-byte-aligned range.

## Edge and error paths

- Cover empty/one/exact/partial workgroup counts, last legal buffer range/alignment, one-byte/range overflow, invalid array count, workgroup dimension/product/storage limit, global-index overflow, invalid indirect offset/range/dimension, and decode length/alignment failure.
- Each suspect operation is enclosed by a balanced error scope and fails at its named layer; no transmute, uncaptured panic, or silent no-op is accepted.

Rendering states are outside this ABI component.
