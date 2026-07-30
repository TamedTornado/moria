# Issue 482 — Checkpoint an isolated confirmed frontier

References: `content-persistence.md` TECH-045; issue M-043.

## Valid transitions

- `Queued -> Pinning -> Reading/Exporting -> BytesVerified -> BlobPutPending -> BlobDurable -> CommittingManifest -> Ready`.
- Replay chunks follow LogPinned/BytesReserved/Encoded/Verified/Put/Durable/Referenced; manifest commit occurs only after every scar, metadata, snapshot, and required replay blob is durable.

## Invalid transitions and guards

- Request must name a retained `Confirmed(t)` frontier/root and exact registered store; Genesis/unknown/reclaimed/wrong-root requests reject unchanged.
- Enforce all request/world store, blob, node, count, staging, mapped/store byte, participant snapshot, and replay-range bounds before work.

## Lifecycle and concurrency

- Start checkpoint at tick t, confirm later ticks, and prove result/manifest describe only pinned t while later truth stays dirty.
- Hold each blob class pending in turn; manifest cannot be called or durability reported.
- Cancel before first read/store versus after submission; later cancel stops new batches and drains, never commits manifest.
- Inject mapping/device/export/digest/length/store/chunk/manifest failure. No manifest becomes visible; orphan blobs may remain; root/tokens/log stay pinned through cleanup.
- Route only to requested store and report exact volume revisions, participant commitments, counts, bytes, and manifest digest.
