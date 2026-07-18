# Issue 344 — Recovery PR #314: Enforce canonical forest contracts

References: `docs/tdd/config.md` §Biome, object, and route constraints; `docs/tdd/assets.md` §Curated manifest; `docs/tdd/implementation-plan.md` §Gate F1; `docs/tdd/data-model.md` §Feasibility evidence.

## Properties that must hold

- For every accepted canonical forest manifest, forest area is at least 120,000 m²; tree/bush/prop counts meet the exact ceiling formulas; birch/pine meet 55/45 minima; every tree pair is at least 5 m apart; and every canopy radius is 2–4 m with at least 16 lower-bin and 16 upper-bin examples per species.
- For all accepted placements together, the 3 m-expanded capsule route through the qualifying forest corridor is free of registered-object solid cells, and no non-ruin raw solid shape overlaps another or any authored ruin-stamp coordinate.
- For every `generate`/`check`/`prove-forest` run from identical seed/config/stamp, canonical ordering/bytes, object index, first failure witnesses, and worst edit target selection are deterministic.

## Entity configurations to test

- Checked-in full manifest and focused fixtures failing exactly one of area, density count, species share, spacing, lower/upper canopy bin, route corridor, shape disjointness, index cap, or required-kind presence.
- Threshold fixtures at exact area/count/spacing/radius/bin/cap values and one failing unit below/above each.
- Multiple valid stress centers tied on broad candidates, exact IDs, dependency bricks, and changed bricks; assert lexicographically smallest voxel center wins after the documented tuple maximization.

## Edge cases

- Counts use `ceil` formulas and species minima use `floor`; fractional hectare/area inputs must exercise rounding exactly.
- Passing one forest constraint cannot weaken or omit another; aggregate counts cannot replace stable first-conflict/failing-ID evidence.

## Error paths

- `moria-curate check` rejects byte differences and any contract failure; `prove-forest` also rejects >1,000 ms validation, >250 ms index build, >16 MiB retained bytes, or nonzero dependency-coordinate allocation on acceptance evidence.
- Failure keeps `passed:false` with sorted reasons and typed stable witnesses; it never regenerates easier content or overwrites a failed artifact as passing.

