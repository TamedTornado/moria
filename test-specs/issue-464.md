# Issue 464 — Traverse sparse matter and emit collision facts

References: `collision-presentation.md` TECH-052 and `interfaces.md` TECH-024; issue M-057.

## Properties

- For all emitted facts, the referenced cell is occupied under immutable material threshold/class and the fact binds exact source frontier/root, volume/revision, material, leaf hash, TOI, and world contact bytes.
- Traversal order is VolumeId, brick `(z,y,x)`, then cell; final output uses fixed slots, stable compaction, and TECH-024 ordering.
- Occupancy masks may skip only proven-empty cells and are valid only for the exact source leaf hash.

## Entity configurations

- Test uniform empty, uniform occupied, mixed dense, signed-density thresholds, `Never` matter, multiple overlapping volumes, sparse gaps, and translated/rotated dynamic volumes.
- Test Genesis and Confirmed(0) sources and require byte-distinct facts.
- Exercise DDA across negative coordinates, boundaries, multiple bricks, and capacity limits.

## Edge and error paths

- Stale mask, wrong leaf/root/revision, cold/corrupt/missing base, device loss, and output overflow yield pending/unavailable/no-advance as appropriate; they never yield empty/no-hit/truncated success.
- Mask false positives may increase work, but false negatives fail parity.
- Compare CPU/WGSL output bytes and exact order; no mesh/presentation data may enter traversal.
