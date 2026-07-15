# Issue 166 — Add placeholder milestone capture and acceptance automation

References: `docs/tdd/assets.md §Milestone outputs`; `docs/tdd/rendering.md §Visual acceptance checklist`; `docs/tdd/benchmarks.md §Non-automated evidence`.

## Boundary contracts

- Automation drives tagged public cameras/time/debug toggles and WorldEditWrite operations, writing captures outside assets with seed/config/asset digest and camera/operation metadata.
- It reruns flythrough/mutation reports at named resolutions and labels placeholder output as placeholder, never final-art acceptance.

## Multi-system scenarios

- First terrain, tunnel clip, geology cutaway, dressed world, playable run and benchmark outputs; interactive primary/terminal and catastrophic progressive/terminal frames.
- Named machine profiles/resolutions and visual checklist template generation.

## Failure propagation

- Missing/mismatched metadata/digest, capture under private/special terrain scene, absent report or mislabeled placeholder fails bundle.
- Capture/report write failures leave runtime asset tree untouched and do not imply numerical/visual pass.

## Ordering guarantees

- World readiness -> tagged public setup -> operation progress markers -> capture -> report/checklist grouping; terminal captures cannot be taken from primary-only progress.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

