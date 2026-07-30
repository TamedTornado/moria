# Issue 554 — Implement participant snapshot and reconstruction restart scenarios

References: `validation.md` TECH-066 participant scenarios; issue M-101.

## Universal properties

- For every restored or reconstructed participant frontier, participant/RNG commitments and immutable token bindings must exactly match the saved frontier before installation.

## Entity configurations

- Register one `PerTickSnapshot` participant and one `ReconstructibleFromCanonicalStateAndLog` participant, each with declared RNG/representation contracts and recognizable per-frontier states.
- Drive genesis, several ticks, rollback frontiers, checkpoint, process teardown, cold restore, correction, device loss, and explicit recovery.

## Properties

- Snapshot export bytes/digest bind exact participant/contract/frontier/root/commitment/generation; restore produces a staged equal token.
- Replay chunks alone reconstruct every intermediate/final participant and RNG commitment after all in-memory log state is discarded.
- Installed source token never mutates; private/candidate tokens install only in a complete bundle.

## Failure/concurrency paths

- Hold snapshot export, snapshot blob put, replay chunk put, and manifest commit pending independently; manifest cannot commit until all required durable inputs.
- Corrupt/omit/reorder/gap/overlap/oversize snapshot or replay data; inject export/store/device/capacity failure and commitment divergence. No manifest/world/correction publishes.
- Make one participant fail after several private correction ticks; original tokens/root remain byte-identical.
- Failed staged CPU tokens drop after callback closure; GPU tokens reclaim only after last queue use. No locator substitution, stale-token reuse, fallback, or partial participant set.
