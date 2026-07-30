# Issue 555 — Implement cold public-replay and post-correction restore scenarios

References: `validation.md` TECH-066 public replay/corrected restore; issue M-103.

## Universal properties

- For every accepted owned physical replay stream, folding and private replay must reproduce exactly one verified active semantic history before any public world exists.

## Multi-system scenarios

- Export a complete physical Genesis stream containing ordinary ticks and a correction branch; destroy all world/root/log objects; pass only owned header/records to a fresh builder.
- Separately replay a checkpoint-anchor stream using exact anchor restore limits/store/key/manifest.
- Take a checkpoint after correction, tear down, and restore only active corrected tick frames extracted from physical locators.

## Properties

- Public replay folds superseded bytes as branch evidence but returns only corrected outcomes/events/participant/RNG commitments/roots.
- After semantic verification, exact source physical bytes copy to fresh stream sequences 0..N; publication waits for all appends and first new tick uses N+1.
- Changing adapter context alone preserves identity/header/prefix; changing status/configuration/placement/arithmetic fails before transition/callback/destination sink.

## Error paths

- Poison root, outcome, participant, event, previous/corrected history digest, or branch frame; return earliest bounded divergence and publish nothing.
- Remove/reorder branch, supply superseded frames to post-correction restore, or use missing/extraneous anchor limits; fail exact validation.
- Cancel/fail destination sink before/after invocation; drain/retire stream as specified and publish no world.
