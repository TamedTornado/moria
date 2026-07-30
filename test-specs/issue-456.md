# Issue 456 — Materialize base content with scar overlays

References: `content-persistence.md` TECH-042; issue M-038.

## Properties

- For every request, materialization pins one immutable root, verifies exact base identity/payload, then overlays at most one complete scar leaf from that root.
- A resident cache entry is keyed by source root/volume/brick/content digest and becomes ready only after GPU upload and directory publication.
- A new complete brick byte-identical to verified base removes the scar; any different complete brick replaces it.

## Entity configurations

- Test uniform empty/nonempty and full bricks; no scar, uniform scar, full scar; static/dynamic volumes; cold/cache-hit paths; old and current rollback roots.
- Withdraw interest and evict resident detail while dirty scars/rollback roots remain; rematerialization must recover exact matter.

## Edge and error paths

- Missing, cold, failed, corrupt, wrong-digest, invalid-cell, or wrong-root base content remains unavailable and never becomes empty.
- Cancellation/failure before upload leaves no ready directory entry; failure after private upload but before directory publication rolls back the reservation.
- Base-cache eviction cannot remove a scar leaf, dirty journal reference, pinned root, or admitted use.
- Concurrent old-root readers and new-root materialization receive source-matching bytes without cross-wiring.
