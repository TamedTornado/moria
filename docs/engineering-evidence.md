# Engineering evidence

This document states what the Moria repository demonstrates today and what it
does not. It is intended to keep implementation evidence, build-system
evidence, performance acceptance, and visual acceptance from being collapsed
into one optimistic status.

## Repository-scale evidence

The current public baseline has four workspace crates, 107 Rust source files,
approximately 22,800 lines of Rust, and 224 `#[test]`/`#[tokio::test]`
functions. These counts describe the review surface; they are not quality
claims by themselves.

The test surface includes:

- deterministic terrain, biome, object, and curated-manifest behavior;
- sparse storage coordinates, material truth, and collision ownership;
- bounded ray, overlap, sweep, sampling, and diagnostic queries;
- mutation admission, rejection, execution, and observation;
- focus-source and streaming lifecycle transitions;
- public-facade and headless Bevy integration;
- configuration, checked-in asset, and canonical-generation contracts;
- benchmark report schemas, cross-field invariants, and failure output.

The ordinary local gate is:

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo run -p moria-curate -- check
```

## Deterministic curation evidence

`moria-curate` regenerates the curated world manifest from the checked-in region
configuration and sparse ruin stamp. `moria-curate check` fails if the
checked-in manifest is not the canonical output.

The `prove-forest` command additionally validates the generated manifest and
writes a machine-readable report:

```sh
cargo run -p moria-curate -- \
  prove-forest --output target/feasibility/forest.json
```

This proves only the fields emitted and validated by the current command. It
does not substitute for every F1 acceptance field in
[`tdd/benchmarks.md`](tdd/benchmarks.md), and it does not produce visual
evidence.

## Rust/Bevy build evidence

Moria is the real dependency graph used to qualify
[`cargo-reapi`](https://github.com/TamedTornado/cargo-reapi). That matters
because the graph includes the build scripts, proc macros, native-tool
discovery, links, and relocatable test binaries that small synthetic fixtures
miss.

The public cargo-reapi qualification reports currently record:

| Host | One clean warm worktree | Five simultaneous | Ten simultaneous |
| --- | ---: | ---: | ---: |
| macOS/arm64 APFS | 8.302s | 14.264s | 25.016s |
| Linux/x86_64 XFS | 6.455s | 10.818s | 18.852s |

Every warm population reported zero physical actions and zero compiler/linker
executions under external OS observation. Both platform batches also covered
adversarial invalidation, poison propagation, flag/configuration changes,
concurrent miss coalescing, sandbox denial, and linked Bevy artifact parity.

Those are build-system results, not Moria runtime-performance results. Complete
methodology and pass matrices live in cargo-reapi:

- [macOS APFS record](https://github.com/TamedTornado/cargo-reapi/blob/main/benchmarks/results/2026-07-21-macos-apfs.md)
- [Linux XFS record](https://github.com/TamedTornado/cargo-reapi/blob/main/benchmarks/results/2026-07-21-linux-xfs-schema-v3.md)
- [Moria agent-fleet dogfood](https://github.com/TamedTornado/cargo-reapi/blob/main/docs/case-studies/moria-agent-fleet.md)

## Private-harness dogfood

We currently run Moria through a private agentic coding harness with five
logical agent slots and independent mechanical quality gates. Bro, the
orchestrator, is not public. The relevant cache statistics, revisions, and
methodology are published in cargo-reapi rather than asking readers to trust a
private link.

The dogfood run is useful because it exposed defects that a controlled benchmark
did not:

- orchestration-only environment variables accidentally became Rust build
  inputs;
- agent target mounts and cargo-reapi's declared target root diverged;
- Debian's `/etc/alternatives` indirection was hidden from a native Bevy build
  script by the strict sandbox;
- mutable target populations and cache garbage collection required independent
  storage policy and telemetry.

After the environment and target-root repair, one fresh agent-run action log
contained 31 cache hits, 10 coalesced hits, 21 producer misses, and six
non-cacheable capability probes across 74 wrapper records. A separate cold
sample showed five identical `bevy_pbr` callers become one producer and four
waiters.

These are promising field results. They do not make the private orchestrator
reproducible and do not qualify a live remote REAPI service.

## Acceptance still open

The following claims remain open and must not be inferred from the evidence
above:

- complete F1 forest/index feasibility under every written report contract;
- complete F2 interactive, colony, and catastrophic mutation workloads;
- the final resident graphics-memory product target;
- required milestone captures and human visual review;
- a released stable consumer API;
- a complete downstream game.

In particular, the current benchmark executable deliberately writes a failed
report when complete scenario evidence has not been captured. That is
fail-closed behavior, not a passing benchmark hidden behind an incomplete
runner.

## What this demonstrates

Moria and cargo-reapi together provide inspectable evidence for work involving:

- Rust and Bevy architecture with a deliberately bounded public API;
- deterministic world/content generation and canonical checked-in artifacts;
- sparse spatial data, bounded query contracts, streaming, and mutation;
- adversarial schema and acceptance-harness design;
- compiler-cache correctness across independent worktrees;
- Linux/macOS sandbox and filesystem behavior;
- resource diagnosis under parallel agent and CI-style workloads.

The strongest claim is not that every Moria milestone is finished. It is that
accepted, pending, and disproven claims are distinguished, measured, and made
reviewable—and that production dogfood failures become durable repairs and
tests.
