# Issue 469 — Build source-bound canonical collider artifacts

References: `collision-presentation.md` TECH-053; issue M-058.

## Properties

- Every artifact is a deterministic lossless occupancy encoding bound to contract, source frontier/root, request digest, sorted volume revisions/placements, record counts, complete flag, and artifact hash.
- For all accepted artifacts, decoding spans reconstructs exactly the occupied cells/material/density selected by the request.
- Genesis and Confirmed(0) produce distinct header/hash bytes.

## Entity configurations

- Build empty, uniform occupied, mixed sparse, multi-volume, negative-coordinate, translated/rotated dynamic, and maximum-record artifacts.
- Compare rebuilds under different physical slots/cache layouts; bytes/hash remain identical.
- Feed the same artifact through CPU view and GPU read-only binding and compare metadata/count/bytes.

## Edge and error paths

- Insufficient record/byte capacity reports unavailable artifact, never truncation or `complete=true`.
- Reject wrong source root/frontier/revision, request digest, contract, volume count, record count, span order/range, checksum/hash, stale generation, or malformed bytes.
- Artifact disposal/rebuild cannot alter canonical matter/root/hash; participants may consume only an exact source-matching lease.
