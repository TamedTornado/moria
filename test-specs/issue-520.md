# Issue 520 — Prove completion-bridge reservation and drain headlessly

References: `validation.md` TECH-060 bridge proof; `gpu-runtime.md` TECH-031/032/037; issue M-128.

## Valid lifecycle

- For every controlled job: reserve cell before extraction -> encoded -> submitted -> complete -> mapped -> decoded -> bridge-drained -> main acknowledged -> published/finalized -> resources returned.
- Matching envelope publishes exactly once through the real main-world path.

## Capacity and invalid transitions

- Saturate all job-usable cells while preserving two control cells; next admission returns retryable backpressure before callback/extraction/allocation and no envelope is dropped.
- Duplicate completion terminally fails generation, never republishes, and releases duplicate resources exactly once.
- Unknown/aborted completion performs lifetime cleanup only.

## Generation/concurrency

- Lose device with exhausted cells and jobs in every state. GenerationLost plus per-job terminal envelopes drain before recovery/shutdown removes bridge clones.
- Old-generation completions and consumer-cancelled completions release only old resources and cannot publish.
- Hold main acknowledgement to prove cell/job/staging/readback/pins remain reserved.
- All cases use controlled completions through actual facade/pool/publication/recovery paths; no fake GPU backend or completion bypass.
