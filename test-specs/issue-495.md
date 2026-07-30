# Issue 495 — Bound active logs, replay, and checkpoint retention

References: `content-persistence.md` TECH-049; issue M-050.

## Valid transitions

- Confirmed ticks append exact active semantic records with physical sequence/subrecord locators.
- Correction atomically splices active entries after target; superseded entries leave active count but remain pinned for old readers until drain.
- Durable replay append/branch publication and compatible checkpoint coverage release only the pins each contract authorizes.

## Invalid transitions and guards

- Never evict a record required by rollback minimum, correction, reconstructible participant range, active replay, checkpoint, recovery, public sink durability, or existing reader.
- At count/byte boundary, `reserve_tick` returns `PersistenceBackpressure`; no only copy is dropped.
- A correction branch that cannot fit one physical sink record rejects before private work and is never split.

## Lifecycle and concurrency

- Hit every log tick/byte, replay in-flight, public replay, correction, private-root/result, divergence, and recovery quota at exact and one-over boundaries.
- Hold sink append and checkpoint chunk puts pending separately; verify which records remain pinned and telemetry counts exact bytes.
- Correct while old checkpoint/replay readers hold superseded suffix; active log shows corrected entries immediately on publication, old leases remain valid.
- After durable anchor, release older records only when no remaining declared range/pin exists.
