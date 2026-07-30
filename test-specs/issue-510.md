# Issue 510 — Implement the dynamic-volume lifecycle scenario

References: `validation.md` TECH-064 scenario 3; issue M-094.

## Properties

- For every stage of this scenario, authoritative identity, placement, cells, revisions, collision facts, and root bytes must equal the TDD-defined result for that exact frontier.
- For the scenario’s dynamic volume, identity remains stable across movement, local edit, checkpoint, process teardown, restore, and presentation rebuild.
- Collision/matter/root/revision changes derive only from admitted placement/matter ticks; no motion/physics behavior is inferred.

## Multi-system sequence

1. Publish Genesis; query/collide the initial volume and capture exact placement/cell/fact/root/revision bytes.
2. Submit one translation/rotation input; verify material local cells unchanged, world collision/contact changes by exact canonical transform, and revision/root advance once.
3. Apply a local matter edit; verify exact local/world facts and honest dirty/presentation state.
4. Checkpoint that confirmed frontier, destroy all live objects, cold restore from exact store/key, then rebuild derived presentation.
5. Compare restored identity, allocator, placement, cells, collision facts, revision, root, and next tick with the saved frontier.

## Failure paths

- Reject static-placement behavior, invalid transform/edit bounds, wrong checkpoint identity, or unavailable content without partial state.
- Presentation absence/staleness/failure never changes restored authoritative comparisons.
- Scenario outputs contain no physics, damage, player, or gameplay assertion.
