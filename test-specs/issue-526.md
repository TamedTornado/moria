# Issue 526 — Validate all WGSL with matching Naga

References: `validation.md` TECH-061; issue M-084.

## Properties

- Discover every WGSL module referenced by the crate and parse/validate it with matching Naga 29 capabilities; the inventory is complete and fail-closed.
- Reflected bindings, workgroups, ranges, layouts, and ABI tables match exactly. Every pushed wgpu validation scope is popped; unexpected uncaptured errors fail the run.
- Regenerated TECH-071 CPU/WGSL fixed-math sources and CORDIC tables for all splits byte-match checked-in sources.

## Positive and negative configurations

- Positive fixtures cover every canonical and presentation shader under its declared capabilities.
- Named negative fixtures independently target malformed syntax, semantic invalidity, mismatched bindings, undersized ranges, workgroup dimensions/storage, nonuniform barriers, missing over-dispatch guard, scan/counter/output overflow, indirect args, unsupported features, mapping/decode, sparse reservation/deletion, and stale generation.

## Error paths

- Each negative passes only when rejected at its named layer with expected error; crash, timeout, unrelated validation error, or silent no-op fails.
- Missing module/entry point/reflection record, unpopped scope, regeneration mismatch, or unavailable required capability invalidates the command.
- Naga success alone must not be labeled real-GPU semantic evidence.
