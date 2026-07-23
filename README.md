# Moria

Moria is a reusable voxel-world substrate written in Rust and Bevy. It is
designed to be consumed as a crate by a downstream game, while its executables
exist to curate, exercise, benchmark, and visually validate the substrate.

It is not a game layer. Player controllers, characters, skeletal animation,
and game-specific presentation are outside the accepted substrate boundary.

## Why this repository exists

Voxel worlds force several difficult systems to agree: deterministic
generation, sparse storage, bounded queries, streaming lifecycles, edits,
collision truth, persistence, GPU representation, and measurable performance.
Moria treats those as explicit contracts rather than hiding them behind a
single demo.

The workspace currently contains:

| Crate | Responsibility |
| --- | --- |
| `moria-world` | Public world facade, deterministic generation, storage, queries, mutation admission, streaming, objects, and telemetry |
| `moria-curate` | Deterministic manifest generation and forest/index feasibility evidence |
| `moria-bench` | Versioned benchmark evidence schemas and scenario capture |
| `moria-demo` | Downstream consumer and visual-validation harness |

The public `moria-world` facade can be installed into a headless Bevy app.
Implementation modules stay private unless they are part of the consumer
contract, and unsafe Rust is forbidden workspace-wide.

## Current evidence

At the current public baseline:

- the workspace contains 107 Rust source files and approximately 22,800 lines
  of Rust;
- 224 unit and integration tests cover the public facade, deterministic
  generation, bounded query behavior, mutation admission, lifecycle
  transitions, asset/config contracts, report schemas, and failure cases;
- deterministic curation produces a checked-in manifest and a machine-readable
  forest/index feasibility report;
- Moria's real Bevy dependency graph is the production dogfood workload for
  [cargo-reapi](https://github.com/TamedTornado/cargo-reapi), our shared
  cross-worktree Rust build cache.

In a controlled Linux/XFS run, five simultaneous clean Moria worktrees completed
their full warm quality gates in 24.695 seconds with zero OS-observed compiler
or linker work. In our private agentic coding harness, five worktrees reaching
the same cold `bevy_pbr` action produced one physical action and four coalesced
waiters. The public
[agent-fleet dogfood case study](https://github.com/TamedTornado/cargo-reapi/blob/main/docs/case-studies/moria-agent-fleet.md)
records the measurements, defects, repairs, and limitations.

See [Engineering evidence](docs/engineering-evidence.md) for the exact boundary
between implemented contracts, reproducible build evidence, and work that is
not yet accepted.

## Status

Moria is active engineering work, not a released engine and not a finished
visual demo.

| Claim | Status |
| --- | --- |
| Rust workspace and bounded public facade | Implemented and tested |
| Deterministic curation and manifest validation | Implemented and tested |
| Query, lifecycle, mutation-admission, and telemetry contracts | Implemented and tested |
| Real Bevy graph used in cross-worktree cache qualification | Passed on macOS/APFS and Linux/XFS |
| Final F1/F2 feasibility evidence | Not yet accepted |
| Final graphics-memory product target | Not yet proven |
| Required visual captures and human visual review | Not yet complete |
| Released reusable voxel engine | Not claimed |

Performance numbers do not prove visual quality, and a passing unit suite does
not prove final feasibility. The repository keeps these gates separate on
purpose.

## Run the current checks

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo run -p moria-curate -- check
```

To regenerate the current forest/index evidence:

```sh
cargo run -p moria-curate -- \
  prove-forest --output target/feasibility/forest.json
```

That report is one input to the feasibility process; its existence alone is not
a final F1 pass.

## Engineering documents

- [Project boundary](docs/seeds/project-boundary.md)
- [Voxel-world substrate seed](docs/seeds/voxel-world-substrate.md)
- [Design document](docs/design-document.md)
- [Technical design overview](docs/tdd/overview.md)
- [Public API contract](docs/tdd/api.md)
- [Benchmark and acceptance contract](docs/tdd/benchmarks.md)
- [Engineering evidence](docs/engineering-evidence.md)

Together, the repository and its cargo-reapi dogfood record demonstrate
contract-driven Rust/Bevy systems work, deterministic content pipelines,
adversarial acceptance design, and diagnosis of build/resource failures under
a real parallel agent workload without weakening correctness to obtain a fast
benchmark.
