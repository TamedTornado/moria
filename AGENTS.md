# Moria agent instructions

## Commands

- Format (write): `cargo fmt --all`
- Format check: `cargo fmt --all -- --check`
- Check: `cargo check --all-targets --all-features`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Test: `cargo test --all-targets --all-features`
- Shader validation: `cargo run --bin moria-qualify -- shaders validate`
- Replay determinism: `cargo run --bin moria-qualify -- replay verify --fixture fixtures/replay/core-v1 --runs 8 --evidence target/moria-evidence/replay`
- Build: `cargo build --all-targets --all-features`
- Development scenario: `cargo run --bin moria-qualify -- scenario public-boundary --mode candidate --evidence target/moria-evidence/dev`
- Full local gate: run format check, check, lint, test, shader validation, replay determinism, then the development scenario in that order.

Replay determinism runs on the current machine and must use a real GPU for a replay-grade result. `UNAVAILABLE` is not `PASS`. There is no cross-vendor matrix or driver requalification command.

## Module and naming rules

- Keep `lib.rs`, `prelude.rs`, plugin `mod.rs` files, and the evidence binary entry point as wiring/facades.
- Organize by domain responsibility. Do not add root catch-alls named `types.rs`, `systems.rs`, `components.rs`, `resources.rs`, or `utils.rs`.
- A feature module owns its Bevy plugin wiring, systems, resources, messages, tests, and shader ABI.
- Public types use descriptive nouns. Fallible actions use verbs. State enums end in `State`, receipts in `Receipt`, stable identities in `Id`, immutable root references in `Root`, and configuration limits in `Limit` or `Budget`.
- Canonical wire structs end in `Wire`, are fixed-width, and live beside their WGSL ABI. Host-only structs must not be cast to a wire type implicitly.
- New public APIs require rustdoc examples and an error/bounds contract.

## Dependency rules

- The root package contains the public `moria` library crate and the `moria-qualify` binary crate. Do not introduce a Cargo workspace, another package, or another target without a concrete compile, reuse, or deliverable boundary recorded in the TDD.
- Pin Bevy and direct wgpu/Naga dependencies exactly. Do not create a second wgpu device in the Bevy path.
- Keep runtime/executor types out of the general public API. Do not add Tokio to the public contract; use pollable receipts and Bevy-driven progress.
- New dependencies need a written reason, compatible license, default-feature review, wasm assumptions review, and `cargo deny` policy update when that gate is introduced.
- Unsafe Rust is forbidden unless a reviewed module-level safety contract and Miri/test evidence are added. No unsafe code is needed for the baseline.

## Moria constraints

- No canonical state change outside a sealed numbered tick; genesis and exact retained-root restore are the only stated exceptions.
- No floating point, unordered map iteration, race-winner allocation, wall clock, OS entropy, callback order, or presentation readiness in canonical transitions.
- No unbounded channel, vector, retry loop, GPU probe, dispatch, readback, or retained history.
- Never expose internal storage buffers. GPU participants receive bounded source-hash-bound artifacts and fixed-capacity effect sinks.
- Submission is not completion. Do not recycle a GPU slot until its last queue use and mapping/decoding are complete.
- Treat unknown matter as unavailable, never empty.
- Physical slot IDs are not stable identities and never enter persistence.
- Every WGSL bounds check and overflow flag is part of the matching Rust ABI test. Pop every wgpu error scope.
- Implement the full TECH-070 facade; do not leave a prose-only capability or add a second callable path. Admission rejection returns owned requests, and every producer output reserves its TECH-036 count/byte capacity first.
- Observation filtering uses append-time filter facts, never current placement. Participant coordination remains one-phase with no same-tick DAG/handoff; opaque participant events are delivered only by confirmed tick receipts.
- Tests and tools use the public facade. CPU-oracle, mock, software-adapter, shader-compile, and rendered-frame evidence cannot be labeled real-GPU replay evidence.
- Canonical placement math lives only in `src/canonical/math/` and matching generated WGSL. Do not use floats, implicit numeric conversions, libm, or shader transcendental builtins in canonical code. Preserve the world placement format and participant representation-contract boundary.
- Preserve existing `TECH-###` meanings and `Implements:` requirement links. `TECH-063` is retired and must never be recreated or reused.
