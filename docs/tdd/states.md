# Application States and Transitions

## Top-level state

```rust
pub enum AppState {
    Boot,
    LoadingWorld,
    Exploring,
    Benchmarking,
    BootFailed,
}

pub enum BenchmarkScenario { Flythrough, CarveStorm }
```

Launch without `--benchmark` follows `Boot -> LoadingWorld -> Exploring`. Launch with a valid benchmark argument follows `Boot -> LoadingWorld -> Benchmarking(scenario)`. A manifest, asset, renderer, or save compatibility failure from Boot/Loading enters `BootFailed`. There is no main menu, account, character creation, mission, inventory, narrative, or gameplay pause state.

| State | Entry work | What renders | Exit condition |
|---|---|---|---|
| `Boot` | parse CLI, install plugins/config, start metric clock | clear window and product title | plugin construction succeeds -> `LoadingWorld`; otherwise `BootFailed` |
| `LoadingWorld` | generate/validate manifest, read save if present, activate spawn | simple loading text; no progress bar | `ControlReady` -> requested interactive/benchmark state; failure -> `BootFailed` |
| `Exploring` | capture mouse, enable player/debug intent | third-person world, minimal control/material/radius/time status | process/window exit only |
| `Benchmarking` | disable live movement/debug input, start one scripted scenario and recorder | benchmark camera/world plus compact metric/scenario label | scenario writes report -> clean process exit; error -> `BootFailed` after failed report |
| `BootFailed` | release cursor, stop world/benchmark mutation | plain fatal error and cause | process/window exit |

`ControlReady` is emitted in under five seconds on acceptance hardware under the readiness definition in `api.md`. Loading has no progress indicator because generation work is deliberately compact; the text avoids presenting an invented percentage.

## Exploring substates

These are orthogonal resources/components, not mutually exclusive app states:

- Locomotion: `Grounded`, `Airborne`, or `Paddling`. Ground contact enters Grounded; valid jump leaves it; fluid surface contact enters Paddling; leaving fluid returns to Airborne/Grounded based on a solid probe. No underwater state exists.
- Cursor: `Captured` or `Released`. Escape toggles this. Released cursor suppresses camera orbit/dig/place but does not pause world simulation.
- Diagnostic flags: brick bounds, raw voxels, and streaming bands are independently on/off. Their toggles are legal only after world readiness.
- Time adjustment: hidden or visible. While visible, Left/Right adjust fixed solar time and do not redirect movement keys.
- Mutation: `Idle -> Accepted(command, revision) -> CollisionReady -> SurfaceCommitted -> Idle`, or submission remains Idle with a rejection status. The interactive tool cannot accept a new operation before terminal commit.
- Persistence: `Idle -> Saving(snapshot_revision) -> Idle` and `Idle -> Loading -> RebuildingVisible -> Idle`. Save permits continued exploration/editing; load suppresses mutation and player movement until replacement collision truth is installed, then retains the player at its prior pose only if the capsule is clear. Otherwise it relocates to the validated meadow spawn and reports that relocation.

## Exact render contracts by mode

- Smooth default: material-aware terrain/object surfaces, static water, vegetation/dressing, normal lighting and third-person character.
- Raw voxels: within near/inspect activation, smooth solid terrain/object representation is hidden only where a raw replacement is ready; occupied voxel cubes display material IDs. Water remains a distinct translucent voxel/material representation. Collision stays unchanged.
- Brick bounds: 4 m wire boxes overlay current representations, colored dirty/clean.
- Streaming bands: active render entities are tinted/outlined by near/mid/far/interaction classification and the focus radii are visible from an elevated diagnostic camera angle when relevant.
- Fixed time slider: a small value/track appears, although adjustment remains keyboard driven; the sun stays fixed at the chosen value.

Mode transitions never mutate truth or persist. Toggling off removes only diagnostic entities/material overrides and reveals the latest regular revision.

## Invalid transitions

- Query/mutation/save/load/diagnostic actions before `ControlReady` return/notate `WorldNotReady`; they do not queue invisibly across a failed boot.
- Live debug/player input is ignored in `Benchmarking`, preventing user input from making runs incomparable.
- Load while a mutation or load/save replacement is in its critical swap is rejected `Busy`. Save may snapshot while normal streaming jobs run because those jobs are derived.
- A benchmark cannot switch scenario mid-run. A second process invocation is required.
- `BootFailed` has no recovery path because Product One has one fixed preset/save slot and no settings flow.

## State tests

Headless tests assert every listed transition, entry side effect, invalid-command rejection, load relocation rule, and that `Exploring`/`Benchmarking` can only begin after `ControlReady`. A terminal benchmark event must produce exactly one report and exit request.
