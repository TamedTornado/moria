# Application States and Transitions

## Demo state map

```rust
#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DemoState {
    #[default]
    Boot,
    LoadingWorld,
    Playing,
    SuspendedForLoad,
    FatalError,
}
```

```text
Boot --assets/config requested--> LoadingWorld
LoadingWorld --WorldReady + active view ready--> Playing
LoadingWorld --fatal startup error-------------> FatalError
Playing --explicit Load action-----------------> SuspendedForLoad
SuspendedForLoad --load + active view ready----> Playing
SuspendedForLoad --load failure----------------> Playing (unchanged world)
any nonfatal state --unrecoverable world error-> FatalError
```

No title, account, character-creation, inventory, mission, narrative, pause, or settings states are added. Process/window exit is handled by Bevy's app exit event rather than a gameplay state.

### `Boot`

Entry creates the window, installs plugins, records the cold-start process timestamp, and requests the fixed region config, manifest, material assets, object assets, and ruin stamp. It renders the clear color plus a minimal `Loading world` label. Input collection may observe exit only; player/debug/world operations are disabled.

`Boot` must last no more than one app update after asset requests are registered, then transitions unconditionally to `LoadingWorld`. Failure to construct the app transitions to `FatalError` or exits before rendering with a diagnostic.

### `LoadingWorld`

Entry validates identity, constructs the private store, optionally loads the one save slot, selects the manifest spawn point, and begins high-priority activation. It renders the loading label and, once available, progressively derived world content; there is no percentage/progress bar requirement. Player entity/camera may be created, but movement/debug input remains disabled.

The state exits to `Playing` only when all are true:

- `WorldLifecycle::Ready` and `WorldReady` were observed;
- initial save is absent or successfully applied (a corrupt/mismatched existing save is fatal rather than silently discarded);
- the spawn capsule is valid and supported;
- collision query readiness covers the configured traversal neighborhood;
- initial camera-frustum surface/water meshes are installed at their requested LOD;
- registered object visuals have real or declared fallback assets; and
- elapsed process-to-control time has been recorded.

The transition's `OnExit` removes the loading label and resets input edge latches so a key pressed during loading cannot trigger an edit or jump.

### `Playing`

This is the only state in which player movement, camera control, debug edits, view toggles, time adjustment, and save requests run. The world continues streaming and presentation in this state. It renders the world, player, orbit camera, environmental light, optional visualizers, and minimal diagnostic HUD.

A load edge sends `LoadWorldRequest` and immediately moves the demo to `SuspendedForLoad`; the demo freeze is a UX choice, not a library precondition. Save is nonblocking and leaves the state in `Playing`; subsequent save/load requests while disk work is active receive typed `Busy` results.

### `SuspendedForLoad`

The request was sent on the transition edge. Entry clears movement velocity and all action latches and removes mutation/inspection focus. Camera/world remain visible, with a `Loading saved edits` label. The demo runs no movement or new edit submission, while the library-owned transaction finishes edits accepted before `LoadWorldStarted`, rejects later edit submissions, stages and atomically swaps without consulting `DemoState`, and discards revision-stale results normally.

`LoadWorldStarted` acknowledges acceptance. On `LoadWorldCompleted`, active collision and presentation already satisfy the new-revision ready barrier, so the state returns to `Playing`, resets latches again, and resumes interaction. On `LoadWorldFailed` (`Busy`, `NotFound`, checksum/config mismatch, decode, or I/O), the previous world is unchanged; the state returns to `Playing` and keeps a dismissible error string in the HUD. An internal invariant failure goes to `FatalError`. Tests drive this demo FSM separately from library tests that load while an arbitrary consumer continues querying/moving.

### `FatalError`

Entry disables all world, movement, and debug systems, cancels acceptance of new tasks, and displays the typed fatal error plus exit instruction. Existing background tasks may be dropped; their results are never installed. There is no retry flow in Product One.

## World lifecycle

`WorldLifecycle` is a resource owned by `moria-world`, not a Bevy `State`; a future consumer may integrate it with a different app FSM. Legal transitions are:

```text
Uninitialized -> Loading -> Ready
Uninitialized -> Loading -> Failed(error)
```

It never returns from `Ready` to `Loading` for ordinary streaming, edits, saves, or delta reload. A load is a transactional operation within the ready world. Fatal internal corruption changes `Ready -> Failed`; no other reverse transition is legal.

Within `WorldLifecycle::Ready`, the library-owned `WorldTransactionState` has the legal sequence `Idle -> Loading(Staging) -> Loading(SwapPending) -> Loading(Rebuilding) -> Idle`; failure from any loading phase returns to `Idle` without changing truth unless the atomic swap already completed, in which case only fatal invariant failure is legal. It has no dependency on `DemoState` and remains testable in a headless external-consumer app.

Tests assert that `WorldReady` is emitted once, queries return `NotReady` before readiness, edits are rejected before readiness, and invalid lifecycle transition requests are ignored with an invariant error.

## Player locomotion state

Locomotion is derived component state rather than a separate app FSM:

```rust
pub enum LocomotionMode { Grounded, Airborne, Paddling }
```

Transitions are evaluated once per fixed tick after collision detection:

- `Grounded -> Airborne`: a consumed jump edge applies configured impulse, or the downward support probe finds no solid collision.
- `Airborne -> Grounded`: downward sweep contacts a walkable solid-collision normal and vertical velocity is nonpositive.
- `Grounded|Airborne -> Paddling`: capsule overlaps a static water body and its surface meets the waist/head entry band.
- `Paddling -> Grounded`: solid support exists at/above the exit probe and water no longer meets entry criteria.
- `Paddling -> Airborne`: no qualifying water and no solid support exists.

At most one transition is committed per tick, using priority `Paddling entry`, `Grounded landing`, then `Airborne`. Entering paddling zeros downward velocity and places the capsule at the configured surface offset; exiting does not add an impulse. There is no underwater, climbing, crouching, falling-damage, or swimming-depth state.

Tests cover each legal transition, jump rejection while airborne/paddling, water-edge exit, and the invariant that paddling always references an actual `WaterSample`.

## Debug tool state

```rust
pub enum DebugOperation { Dig, Place }
pub struct DebugViewFlags {
    pub brick_bounds: bool,
    pub raw_voxels: bool,
    pub streaming_bands: bool,
}
```

Dig/place selection is mutually exclusive; view flags are independently combinable. Material cycling affects place only and skips non-placeable air/water. Entering `Playing` initializes dig, 3 m radius, topsoil selected, all views off, and the configured fixed time. Leaving `Playing` clears pending operation edges but retains toggles and time across in-session load. None of this state is persisted in the world save.

## Chunk lifecycle

Streaming chunks use a private lifecycle enum to prevent stale async installation:

```rust
enum ChunkPhase {
    Absent,
    Requested { token: u64 },
    Materializing { token: u64 },
    Meshing { token: u64, revision: u64, lod: Lod },
    Resident { revision: u64, lod: Lod },
    EvictPending,
}
```

The normal path is `Absent -> Requested -> Materializing -> Meshing -> Resident`. A compact uniform chunk with no surface may skip `Meshing` and become resident metadata-only. New focus/edits can supersede any requested/materializing/meshing token; results with old tokens are discarded. A resident edit moves back to `Meshing` while retaining the previous visual for no more than two frames. Loss of all focus plus hysteresis moves to `EvictPending -> Absent` unless pinned.

Properties:

- there is at most one installed resident render entity per `(brick, visual layer)`;
- only a result matching current token, revision, and desired LOD can enter `Resident`;
- pinned chunks cannot enter `EvictPending`;
- eviction does not remove a `BrickDelta`.

Horizon object cells use the parallel private lifecycle `Absent -> Requested { token, source_revision } -> Building { token, source_revision } -> Resident { token, source_revision } -> EvictPending -> Absent`. `Building` snapshots all tree IDs assigned to the cell and partitions them into intact aggregate cards versus owner-filtered derived payloads from one coherent delta revision. An edit or load affecting any member supersedes the token; an older result is discarded even if the cell key is unchanged. A Far/Horizon transition retains the prior logical presentation until the new cell batch is render-acknowledged. Edit-pinned cells cannot evict. Normal eviction removes aggregate/derived entities and GPU buffers but not deltas or immutable base descriptors, and reactivation always repartitions from current deltas instead of restoring a prior resident payload. `LoadWorldCompleted` waits for every active affected cell to become `Resident` at the post-swap source revision.

## Benchmark runner states

```rust
pub enum BenchmarkState {
    Boot,
    VerifyingFeasibilityInput,
    Loading,
    Warmup,
    QueryCostProbe,
    Running,
    Saving,
    RoundTrip,
    Reporting,
    Complete,
    Failed,
}
```

- `Boot` parses arguments and starts process/cold-start timing.
- `VerifyingFeasibilityInput` is used only by `feasibility-carve`; it verifies the F1 schema/pass bit, artifact hash, M4 identity, and exact git/world/manifest digest before any headed work. Failure writes a failed carve-feasibility report and never runs the edit.
- `Loading` uses the normal world readiness contract at the scenario start point.
- `Warmup` runs the scripted initial view for 300 rendered frames; warmup frames are excluded from FPS distributions but cold-start remains the earlier process-to-ready measurement.
- `QueryCostProbe` is used only by `feasibility-carve` after warmup. It runs the bounded active/cold query cases and representative per-frame bundle from [api.md](api.md), then enters `Running` only if limits, candidate counters, and M4 timing budgets pass.
- `Running` executes the selected feasibility-carve, flythrough, or carve-storm script and captures its contracted metrics.
- `Saving` is entered by carve storm and invokes the public save request; flythrough transitions directly to reporting and records the current slot size (zero for a clean isolated run).
- `RoundTrip` tells the headed app to return after saving; the process orchestrator runs a presentation-disabled headless app, loads through the public protocol, and appends exact-restore evidence.
- `Reporting` validates mandatory metrics/machine fields, atomically writes JSON, and computes pass/failure reasons. For carve storm it runs only after `RoundTrip`; for flythrough it follows `Running`.
- `Complete` returns process exit code 0. `Failed` attempts a report with error context, then exits 1. Argument failures exit 2 before app construction.

Scenario state advances only on explicit waypoint/edit/readiness/save completion events, never after arbitrary sleeps. The benchmark has a configurable watchdog solely to fail a stuck run; timeout never fabricates missing values.

For `feasibility-carve`, `Running` performs the clean-world signature and stress roles and then goes directly to `Reporting`; it does not save or round-trip. Reporting validates every named production trace stage and exact render-extraction barrier count against `CarveFeasibilityReport`. The final flythrough/carve-storm paths skip the two feasibility-only states and retain their lifecycle above.

## State-specific rendering summary

| State | World | Player/camera | HUD | Interaction |
|---|---|---|---|---|
| Boot | Clear only | None | Loading label | Exit only |
| LoadingWorld | Progressive initial view | Spawned but disabled when ready enough | Loading label | Exit only |
| Playing | Full streamed presentation | Active | Diagnostics/time control | Movement, camera, debug, save/load |
| SuspendedForLoad | Last valid/rebuilding view | Visible, frozen | Load label/error | Exit only |
| FatalError | Last safe frame or clear | Disabled | Typed fatal error | Exit only |

This table is a rendering contract, not a requirement for extra menus or polished UI.
