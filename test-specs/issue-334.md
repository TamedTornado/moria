# Issue 334 — Recovery PR #297: Harden Product One config validation

References: `docs/tdd/config.md` §Configuration ownership through §Benchmark configuration; `docs/tdd/systems.md` §`validate_world_identity`; `docs/tdd/api.md` §Startup and readiness.

## Properties that must hold

- For every present RON config, deserialization must deny unknown fields and startup must validate every scalar range and cross-field invariant before creating a queryable/editable `WorldStore`; invalid files never receive silent defaults.
- For all accepted Product One configs, bounds are exactly `[-500,500) x [-128,128) x [-500,500)`, voxel edge is 64 Q8, brick edge is 16, canonical material IDs/classes are preserved, species percentages sum to 100, and authoritative digests exclude presentation/input/player/camera/benchmark-only values.
- For all accepted runtime settings, distance bands are ordered through the 720 m horizon, hysteresis/budgets are representable and positive where work is required, player/camera capsules fit public query limits, mutation limits satisfy `min_radius <= debug_radius <= max_radius`, `max_atomic_bricks <= max_progressive_bricks`, nonzero queue/batch capacity, and benchmark thresholds equal the normative values.

## Entity configurations to test

- The complete checked-in files and defaults; then mutate each numeric/string/enum field independently to minimum, maximum, and one invalid value.
- Cross-field cases: inverted/overlapping bounds or bands, non-100 species mix, object cell sizes inconsistent with 32 m/4 m/64 m contracts, zero worker/job/batch capacity, atomic limit above progressive limit, invalid material classes/IDs, time default outside `[6,20]`, and camera/body dimensions beyond query ceilings.
- Unknown, duplicate, missing required, wrong-type, non-finite float, and present-but-empty path fields.

## Edge cases

- Exact inclusive maxima and exclusive bounds must be distinguished; checked integer multiplication/conversion must reject overflow rather than wrap.
- Presentation-only changes must leave `parameters_digest` unchanged; authoritative config or ruin-stamp changes must change it.

## Error paths

- Return the stable `InvalidConfig`/world-open error with field context, leave lifecycle non-ready, emit no `WorldReady`, and spawn no authoritative or authored object presentation.
- Validation must collect or deterministically select errors without allowing one invalid field to be masked by a substituted default.

