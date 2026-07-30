# Issue 483 — Hash canonical GPU candidates incrementally

References: `architecture.md` TECH-009/013; `gpu-runtime.md` TECH-034/035; issue M-064.

## Properties

- For every accepted canonical candidate, the mapped GPU digest and work-count record must equal the independent logical CPU expectation before publication.
- GPU BLAKE3 output for every canonical domain equals the independent CPU implementation byte-for-byte.
- Changed-leaf scheduling is based on stable logical keys; a one-brick edit hashes that leaf and its 26 ancestors plus required registry/world paths, not unrelated volumes.
- Candidate root/outcome/participant bytes cannot publish until GPU completion, successful mapping, exact decode, and diagnostic/count validation.

## Entity configurations

- Golden every domain at empty, one, multi-record, and maximum supported payload sizes; vary matter/placement/domain/allocator/participant/RNG influences independently.
- Perturb physical slots, workgroup scheduling, cache state, input staging, and completion order; root/output bytes remain identical.
- Hold a mapped comparison pending while the old live bundle remains readable.

## Edge and error paths

- Invalid count, unwritten padding, overflow flag, wrong domain/length, stale source/generation, mapping/decode error, and hash mismatch fail the candidate with no publication.
- Presentation/cache/adapter/timing changes leave hashes unchanged.
- No custom alternative hash, atomic append order, or render-frame success may stand in for mapped semantic parity.
