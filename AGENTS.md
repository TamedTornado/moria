# Contributor rules

Run all commands from the repository root:

```sh
# Format check
cargo fmt --all -- --check

# Type and target check
cargo check --all-targets

# Lint; warnings fail CI
cargo clippy --all-targets -- -D warnings

# Automated unit, property, headless plugin, and integration tests
cargo test

# Build all library/binary/test targets
cargo build --all-targets

# Develop the walkable demo
cargo run -p moria-demo

# Run benchmark scenarios (headed, release build)
cargo run --release -p moria-bench -- --scenario flythrough --output target/bench/flythrough.json
cargo run --release -p moria-bench -- --scenario mutation-workloads --output target/bench/mutation-workloads.json

# Rebuild or verify the deterministic curated manifest
cargo run -p moria-curate -- generate
cargo run -p moria-curate -- check

# Blocking feasibility gates (release build on the 32 GB M4 acceptance machine)
cargo run --release -p moria-curate -- prove-forest --output target/feasibility/forest.json
cargo run --release -p moria-bench -- --scenario feasibility-mutation --resolution 2560x1440 --forest-proof target/feasibility/forest.json --output target/feasibility/mutation.json
```

Normal development uses the dev profile, not `--release`; release is reserved for acceptance benchmarks. CI runs the first four commands in the shown order and also runs `moria-curate check`. GPU/platform acceptance is a separate headed job on the named machines.

## Module, naming, and imports

- Use the documented package/module tree. Add behavior beside the feature plugin that owns it; do not create repository-wide `systems`, `components`, `resources`, `types`, or `utils` buckets.
- Rust files and modules are `snake_case`; types, components, resources, messages, plugins, and system sets are `UpperCamelCase`; systems are verb-led `snake_case`; constants are `SCREAMING_SNAKE_CASE` and include units where ambiguity exists, such as `VOXEL_EDGE_METERS`.
- Suffix Bevy plugins with `Plugin`, public commands/results with an action/completed name (`WorldEditCommand`, `EditCommitted`), action enums with `Action`, and schedule sets with `Set`.
- Import through each package's public facade (`moria_world::{...}`) from other packages. `moria-demo` and `moria-bench` may not use path dependencies to private source modules, expose `WorldStore`, query arbitrary world entities, or construct authoritative voxel components.
- Within a feature, prefer `super` for the immediate module facade and `crate::feature` for cross-feature public APIs. Do not use glob imports except `bevy::prelude::*` inside implementation modules. Keep public exports explicit.
- A data type is either a singleton `Resource` or an entity `Component`, never both. Per-player, per-brick-render, per-object, and per-widget values are components; config, asset handle collections, queues, and the authoritative app/world singleton are resources.
- Public authoritative APIs use fixed-width integer coordinates and IDs. `usize` is limited to in-memory indexing. GPU-visible counters, allocation indices, and propagation labels are `u32`; shaders may use only 32-bit atomics.

## Project-specific constraints

- Authoritative generation/mutation must not depend on floating-point transcendental results, wall-clock time, thread order, render LOD, or hash-map iteration order. Sort coordinates before persistence and externally observed batch messages.
- Consumers mutate the world only through `WorldEditWrite`; all world inspection uses `WorldRead` (including bounded diagnostic pages) or public telemetry. Derived meshes, water meshes, dressing, and debug geometry are never serialized as truth.
- Simulation rules, collision, cooldowns, and edit commits belong in `FixedUpdate`; raw input edges, camera, UI, interpolation, lighting, task polling, and render installation belong in frame schedules. Convert input edges to durable intent before fixed ticks.
- Repeated vegetation and clutter share mesh/material handles. Do not allocate a material per instance. Add custom GPU rendering only after representative-scene diagnostics prove built-in batching, culling, visibility ranges, and LOD insufficient.
- Tests never sleep or open a window for logic. Use `MinimalPlugins`, seed only required state, and advance a controlled count of fixed ticks. Rendering correctness is verified with headed acceptance scenes and human visual review.
- Static water has no flow/pressure simulation; granular materials have no settling; reserved voxel state has no behavior; registered objects have no dynamics; excluded game systems must not acquire placeholder ECS types.
- Cargo dev profiles use `opt-level = 1` for project code and `opt-level = 3` for dependencies. Native release uses `lto = "thin"` and `codegen-units = 1`. No wasm profile is added because web is not a Product One target.
- Follow `docs/tdd/implementation-plan.md`: issues `G1`, `V1`, `T1`, `S1`, and `B1` cannot start until digest-matched F1 and F2 artifacts pass on the M4. Any gate failure immediately requires a reviewed TDD revision describing the measured bottleneck and redesign before more implementation; do not weaken workload/content/timing thresholds or substitute a partial pipeline.
