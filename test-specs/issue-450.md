# Issue 450 — Apply atomic matter mutations on staged roots

References: `architecture.md` TECH-012; issue M-031.

## Properties

- For every matter command, all target cells/base bricks/capacities resolve before root construction; the command either commits all selected writes at one revision or none.
- A successful nonempty command advances its named volume revision exactly once, including multi-brick commands. Empty target is `NoOp` with no revision advance.
- Overlapping commands compose in canonical order; later command reads the earlier staged cell.

## Entity configurations

- Exercise Erase set-empty/subtract-density, Place, Patch, cell AABB, sphere, and stamp across one, 64, and 65 bricks and 1, 32,768, and 32,769 cells.
- Cover empty target/mask, negative local coordinates, base-equal scar removal, multiple materials/densities, overlapping direct/participant commands, and stale revision/hash preconditions.

## Edge and error paths

- Invalid cells/bounds/mask, absent/retired volume, static placement misuse where relevant, base unavailable, logical/physical capacity, arithmetic overflow, and revision `u64::MAX` produce the exact failed outcome/no writes.
- Inject failure after multi-brick construction but before publication; old cells, revision, root, observations, participant tokens, and log remain byte-identical.
- Concurrent readers retain the old immutable root until publication and never observe a partial edit.
