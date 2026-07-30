# Issue 550 — Implement checkpoint and cold-restore scenarios

References: `validation.md` TECH-066 checkpoint/restore scenarios; issue M-100.

## Universal properties

- For every checkpointed frontier, a cold restore using only its complete manifest-referenced bytes must reproduce exactly that frontier’s canonical state and no later or derived state.

## Multi-system sequence

1. Edit a static volume and edit/move a dynamic volume; begin a checkpoint at confirmed frontier t.
2. While t is pinned, confirm t+1 and prove it remains dirty relative to the checkpoint.
3. Hold and then durably complete scar/metadata/participant/replay blobs and manifest for t.
4. Destroy every live world/root/log/cache/presentation object and restore only from exact manifest-referenced bytes.
5. Rebuild cache/presentation and compare authoritative behavior.

## Properties

- Manifest/result report only t’s root/revisions/participants; later t+1 is neither included nor cleared dirty.
- Restored cells, placements, IDs, allocator, simulation domain, participant/RNG commitments, root hash, frontier, and next tick equal t exactly.
- Saved canonical data contains scars/continuation, no mesh/dressing/cache/slots/telemetry, and is smaller than raw untouched fixture domain.

## Error paths

- Wrong lineage with correct label but wrong exact root, wrong placement/config/store/key, missing/corrupt/short/long blob/chunk, root mismatch, or participant divergence publishes no world.
- Derived rebuild failure leaves authoritative restored state correct but presentation noncurrent/failed.
- No surviving in-memory object may satisfy a cold-restore dependency.
