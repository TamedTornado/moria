# Issue 468 — Encode and validate checkpoint manifests

References: `content-persistence.md` TECH-044; issue M-041.

## Input validation

- Decode only `moria-checkpoint-v1` with known contract/schema tags, bounded node/blob/depth/count/byte totals, valid checksum, and no trailing bytes.
- Require exact store ID/contract/key, world/genesis/configuration/placement/base lineage and roots, material/volume registries, revisions/root, simulation domain, allocator, participant/RNG/representation descriptors, and active-history locators.

## Transformation correctness

- Golden manifest encoding has fixed field order and exact descriptor lengths/digests; decode/re-encode is byte-identical.
- Active participant snapshot/replay descriptors cover exactly the strategy-required data. Completeness counts and total bytes equal independently enumerated descriptors.
- Derived mesh/dressing/cache/slot/entity/telemetry/timing/receipt data is absent and cannot affect the manifest digest.

## Edge and error paths

- Reject unknown required fields/version, wrong placement/config/contract, lineage-only match with wrong root, store/key mismatch, invalid volume identity, duplicate/unsorted descriptors, missing/overlap/gap in replay coverage, bad locator/checksum/digest, count/byte arithmetic overflow, and truncation/trailing bytes.
- Validate bounds before allocation/decompression and preserve no partial restore state.

No rendering states belong in persistence; their presence is a negative fixture.
