# Issue 515 — Exercise store and replay completion boundaries headlessly

References: `validation.md` TECH-060 store/replay fixtures; issue M-120.

## Universal properties

- For every provider completion and replay-stream operation, only an exact identity/length/digest match at the legal lifecycle point may advance durability or publication.

## Boundary configurations

- External implementations of `ContentBlobStore`, `CheckpointStore`, and `ReplaySink` exercise exact/short/long/dropped/duplicate/cancelled/late-generation/wrong-identity completion.
- Known blob loads require exact cursor; initial manifest load accepts bounded actual length then full framing/checksum/trailing validation.
- Atomic manifest visibility returns exact complete bytes or `NotFound`.

## Replay lifecycle

- Genesis sequence-zero success/failure, accepted-construction tombstone, duplicate stream pair, retired-stream exhaustion, ordinary post-confirmation append failure, and correction prepublication branch failure expose exact distinct public records.
- Ordinary failure keeps tick receipt ready and committed frontier; branch failure keeps original frontier and committed none.
- Restore checkpoint-anchored header withholds publication until durable and orders first later tick at sequence one.

## Failure propagation

- Checkpoint, shutdown checkpoint, restore, and recovery call only the exact request store; no fallback occurs.
- Cancel before/after provider invocation according to family; callback bytes/cells/pins release only after last use.
- Shutdown repeats exact `ReplayExportFailure` metadata before raw record release; wrong/late success cannot recover world.
