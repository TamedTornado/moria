# Rendering and Visual Presentation

## Rendering principle

Rendering is disposable output of `WorldRead` snapshots. No mesh vertex, render entity, texture selection, water patch, LOD, vegetation instance, or dressing anchor is world truth or appears in a save. Occupancy queries and player/camera collision always sample voxel density/material. The renderer can lag an accepted edit only within the explicit two-rendered-frame contract; it can never make collision follow old triangles.

The implementation uses Bevy 0.19's standard 3D renderer and wgpu abstraction. Terrain and water geometry are generated on CPU, uploaded as ordinary `Mesh` assets, and rendered with portable WGSL materials. This avoids GPU readback, vendor-specific allocation logic, and Apple-incompatible 64-bit atomics. GPU-visible allocation IDs, instance counts, indirect counts if later enabled, and propagation labels are `u32`; checked conversion rejects an oversized buffer before upload.

## Scene hierarchy

```text
WorldPresentationRoot
  TerrainLayer
    TerrainChunk (one entity per aligned render tile/LOD payload)
  WaterLayer
    WaterPatch (shared water material)
  ObjectLayer
    RegisteredObjectRoot (stable ObjectId)
      one of shared species/prop LOD scenes
    HorizonTreeCluster
  DressingLayer
    GrassBatch / SmallClutterBatch
  DebugLayer (optional brick/raw/streaming geometry)

PlayerRoot
  CharacterVisual
  UndergroundPointLight
  OrbitTarget

OrbitCamera
DirectionalSun
SkyEnvironment
DiagnosticHud
```

Layer roots allow visibility toggles and cleanup without broad unfiltered ECS queries. Every derived entity has a specific marker plus revision/LOD/source key; systems never use `Query<Entity>` to discover them, avoiding accidental interaction with Bevy 0.19 resource-backed entities.

## Terrain extraction

### Scalar field and chunk input

Each near chunk snapshots a 17 x 17 x 17 lattice of cell-corner samples plus a one-sample halo where gradient/material neighborhood needs it. Every sample owns the current `Voxel`, regenerated base `VoxelSource`, and the resulting `solid_presentation_owner`; values come from the same truth and deltas used by queries. The global solid scalar is density only for `CollisionClass::Solid` materials and empty for air/water; its isovalue is 128, exactly matching `solid_collision`. Topology is evaluated once from that global scalar so adjacent terrain and object solids do not create an artificial internal surface. A snapshot is immutable, owns all input needed by a task, and records brick, world revision, LOD, and a request token.

At coarser LOD, sampling stride is 2, 4, or 16 voxels for 0.5 m, 1 m, and 4 m representations. To avoid tens of thousands of tiny entities/draw inputs, aligned surface render tiles cover 1 x 1 authoritative bricks at Near, 4 x 4 at Middle (16 m square), 8 x 8 at Far (32 m square), and 16 x 16 at Horizon (64 m square). Vertical cave chunks remain brick-sized and exist only around underground focus. Density downsampling conservatively retains crossings and dominant exposed material separately within each presentation-owner partition; it cannot fold a `NonRuinObject(_)` sample into a terrain scalar. Horizon terrain is a surface-only mesh derived from provenance-carrying column summaries with the same exclusion because distant caves are occluded and need not allocate 3D detail; horizon object cards/clusters remain the exclusive non-ruin object presentation in that band. Downsampled values are never written back to truth.

### Material-aware dual contouring

CPU dual contouring is selected because it represents smooth density terrain while retaining Hermite/feature information for cliffs, cuts, and masonry. For every sign-changing cell:

1. Interpolate edge crossings from `u8` densities in deterministic rational form, converting to `f32` only for final vertex solution.
2. Estimate gradients from neighboring density samples.
3. Classify constraints by adjacent `SurfaceClass` and edit/stamp provenance.
4. Solve a bounded QEF for one cell vertex. Clamp the result to the cell with a small seam-safe margin; use the crossing centroid if the QEF is singular/non-finite.
5. Emit consistently wound quads/triangles around sign-changing grid edges. Each such edge has one solid endpoint at the isovalue crossing; both triangles receive that endpoint's `SolidPresentationOwner`. Terrain/ruin chunk payloads retain only `TerrainChunk` primitives, while a derived non-ruin payload retains only `NonRuinObject(id)` primitives matching its ID. Material weights use solid samples with the same owner, falling back to the solid edge endpoint, so an extractor cannot borrow a neighboring owner's appearance.

`Organic` terrain uses averaged gradients and smooth vertex normals. `Rock`/`Ore` retains features when gradient clusters differ beyond 35 degrees. `Masonry` and edit-created cut faces retain crease groups at material/normal discontinuities, duplicating presentation vertices when needed for hard normals. The same extractor therefore produces organic soil, legible strata/cliffs, smooth spherical tunnel boundaries, and defined cut-stone stairs without separate authoritative representations.

The edit operation records no permanent “cut face” voxel flag. Sharpness is inferred from current density gradients, material class, and the edited snapshot's dirty provenance while it is resident. After reload, the same scalar/material configuration still yields stable geometry; exact vertex bytes are not part of persistence truth.

### Material transitions

Each vertex carries up to four `u8` material IDs and normalized `u8` weights derived from surrounding solid samples. The terrain shader indexes shared albedo/normal/ORM texture arrays and uses world-space triplanar projection, avoiding UV seams on caves and vertical cuts. Material weights interpolate across natural soil/rock transitions; masonry/ore boundaries may use a sharper weight threshold based on surface class. Air and water are never terrain texture layers.

Normals are reconstructed from mesh attributes; tangent-space detail is triplanar and needs no authored UVs. Texture array layer dimensions/formats/mip counts must match at asset validation. One `TerrainMaterial` handle is shared by all terrain chunks; visual material IDs are buffer data, not unique Bevy material instances.

### Chunk seams and LOD

Adjacent same-LOD tiles and object jobs sample the same global lattice, owner tags, and canonical boundary constraints, so their border vertices match. Spatial ownership first assigns a candidate primitive to the tile whose cell minimum lies inside its aligned bounds; the provenance partition then assigns it to exactly one terrain/ruin chunk payload or one non-ruin object payload. No other payload may emit it. Near edit tiles and vertical cave chunks retain brick-level spatial ownership. Face-neighbor dirtying handles edits whose crossing moves across an authoritative brick or render-tile boundary; the extraction-stencil owner scan also dirties any derived object root whose exposed boundary can change.

Different LODs use a fine-side transition skirt generated from both boundary resolutions and tucked toward solid matter by at most 0.05 m. Skirts exist only in the visual mesh and are excluded from collision. LOD changes overlap for one frame with depth-safe dither/fade when supported by the standard material path; otherwise the old handle swaps after the new mesh is ready. No blank chunk is exposed during normal LOD transition.

Near, middle, far, and horizon distances are specified in [config.md](config.md). Frustum culling applies to all chunks; distance bands use 12 m hysteresis. Underground focus suppresses distant surface bands that are fully occluded and activates cave boundary chunks around player/camera. The full region is never rendered at 0.25 m.

## Edit update path

When `WorldEditWrite` stamps submission during rendered frame `F` and fixed tick `T` later commits revision `R`:

1. Queries/collision immediately read revision `R`.
2. Old affected meshes remain only as temporary derived presentation while revision-`R` snapshots queue at edit priority.
3. CPU tasks build all affected near render tiles/vertical cave chunks and required face seams in parallel, without locking `WorldStore`.
4. `install_chunk_meshes` rejects stale results and stages a single deferred swap/removal batch.
5. Dressing at old anchors is hidden as soon as the edit commits; regenerated dressing appears with the new mesh.
6. When all revision-`R` terrain, water, registered-object, and dressing payloads are available to render extraction, telemetry emits `EditSurfaceReady`. Its `ready_frame` must be no greater than submitted frame `F + 2` for the 3 m operation.

A 3 m sphere can intersect at most 27 4 m bricks; dirty face-neighbor seams enlarge the task set but not authoritative edit cells. The two reserved/preemptive worker lanes, immutable snapshots, shared terrain material, and 16 MiB/frame upload budget are sized for this path. Acceptance measures the actual target hardware. If evidence shows this design cannot meet two frames, the implementation must optimize measured extraction/upload or surface a Design Divergence for approval; it may not weaken the event definition.

Because collision updates before mesh installation, a player can enter a fresh void during the short allowed presentation lag. Debug tool input is frozen for that request until `EditSurfaceReady`, and the signature flow naturally resumes within two frames. The system never delays collision to match rendering because material truth is authoritative.

## Raw-voxel and brick views

Raw-voxel mode is a diagnostic replacement/overlay for near terrain. It creates instanced unit cubes at 0.25 m for `material_present` cells only in active detailed or explicitly inspected bricks, using one shared cube mesh and material-array shader. It obtains bounded pages through the public diagnostic snapshot, does not expand horizon/idle bricks, and rebuilds after each relevant revision. `solid_collision` cells use material colors, `water_volume` cells use a translucent shared diagnostic material, partial non-solid matter below the solid isovalue is outlined, and empty air has no cube.

Brick view draws one wire box per active brick using debug lines, colored by clean, dirty, edit-pinned, and task-pending status. Streaming view colors translucent chunk bounds by Near/Middle/Far/Horizon and includes focus-source markers. Debug draw counts and buffers are capped by `u32`, instrumented separately, and not enabled in performance acceptance runs.

## Water

Lake and river water use static horizontal surface meshes clipped to their generated `WaterBodyDef` footprints and carved banks. The lake may be one authored set of generated patches; the river is segmented into resident 4 m/LOD patches so streaming follows terrain. One shared portable water material provides normal-map motion, Fresnel, depth color, and restrained foam at bank intersections as a visual shader effect only. There is no particle splash, flow vector, pressure, level propagation, waterfall, drain, or simulation buffer.

`water_volume` remains true below the static surface for public water queries and paddling. Terrain edits may expose/cover parts of that predefined volume and rebuild intersecting clipped patches, but cannot alter surface level or cause water to move elsewhere. The renderer visually clips water against the latest terrain depth; it does not write voxel truth.

## Registered objects and forest scale

Two tree species, bushes, boulders, stumps, rocks, and the ruin are generated placements with voxel-backed material and solid collision. Non-ruin presentation uses shared meshes/materials from `WorldRenderAssets`; the ruin uses generated mesh payloads plus a shared masonry material:

- Near trees use one trunk plus leaf-cluster mesh per species with shared handles.
- Middle/far trees select authored lower-poly LODs by Bevy visibility range.
- Horizon trees use generated cluster/impostor cards per spatial cell only if representative profiling shows individual far LODs exceed budget; the underlying placements remain separate truth.
- Bushes and props use full/shared LODs in near/middle and cull by their configured visibility range.
- The ruin renders a dual-contoured cut-stone voxel stamp, optionally with a shared masonry detail material; its staircase geometry and collision both derive from the stamp.

Spawn systems clone asset handles and transforms; they never add to `Assets<StandardMaterial>` per placement. Built-in batching/instancing, frustum culling, visibility ranges, and LOD are measured in dense-forest scenes before any custom render pipeline is considered. Simulation object count stays independent from visible representation, so clusters/impostors do not alter IDs, material truth, or solid collision.

An accepted manifest has no solid voxel shared by two non-ruin raw shapes and no non-ruin raw shape intersecting an authored ruin-stamp coordinate. The curator retries deterministic placement candidates until required density/count contracts and this disjointness invariant both pass; a conflicting manifest fails before any render root is spawned. Consequently a non-ruin placement may use its complete shared authored visual—the configured base presentation of its analytic shape—as the sole presentation for every solid cell whose owner is `NonRuinObject(id)` only while every coordinate in its `ObjectSurfaceDependency` equals base; all terrain LOD extractors exclude primitives with that owner.

The dependency is the extractor-declared union of all object, adjacent terrain/ruin, gradient, material, provenance, and downsampling inputs that can affect that object's emitted primitives. It is represented by a fixed-size conservative AABB and exact lazy membership against the placement's analytic shape plus the shared finite extraction stencil, never by retained coordinates. Only an active object's bounded footprint probes sparse delta bricks. Any matching delta—regardless of base owner—removes the authored root and replaces it with a material-aware mesh retaining only globally extracted primitives tagged with that ID. Further dependency edits revision that mesh. The shared root returns only when the dependency has no delta. Thus an adjacent terrain-only place/dig can occlude or expose the object without leaving a stale GLB, and reverting object cells while such a boundary delta remains cannot restore the full authored visual. Placed/digged deltas still do not acquire the owner of a neighboring object: regenerated base provenance determines payload routing, including across a single edit sphere.

The ruin never takes this path: from initial activation through edits and exact reversion, its provenance maps to `TerrainChunk`, so stamped/current cut-stone surfaces are emitted only by revisioned voxel chunks, with no ruin GLB. The edit completion barrier includes each affected non-ruin root swap/rebuild and every dirty terrain/ruin chunk/seam handle through render extraction. This prevents duplicate or stale terrain/object geometry without adding felling or rigid-body behavior.

## Dressing

Grass and small clutter use deterministic surface anchors. Near grass is rendered as batched crossed cards or small blade clusters with a shared mesh/material and per-instance transforms. Middle density is 25% of near; far/horizon omit it. The shader may use presentation-only wind vertex motion but no growth/burning/wetness state.

Anchors require topsoil, upward normal `y >= 0.75`, current revision, and no water/object exclusion overlap. Edit commit immediately removes old-anchor batches in dirty chunks; regeneration uses the revised surface. Consequently grass never floats over a dug hole and never becomes saved independent scenery.

## Player, camera, and cave lighting

The player uses a simple authored third-person visual that is not authoritative for collision; the configured capsule is. The visual root interpolates between fixed simulation poses per rendered frame, while queries use current fixed pose. Animation needs only idle, run, sprint, jump/fall, and paddle clips; no combat/stat state exists.

The orbit camera targets the player's upper torso, applies yaw/pitch/zoom per frame, and sweeps a small probe from target to desired position against `solid_collision`. The final near-plane-safe position is chosen after smoothing. This prevents passing through terrain in caves or after edits even if a visual LOD seam differs slightly.

A warm point light is a player child and fades on when the local sky-exposure/depth test declares underground. The directional sun and sky remain present, but cave material receives negligible direct light through occlusion/shadows. Light range/intensity values make the route legible without dynamic torches or gameplay inventory.

## Fixed time of day and environment

The diagnostic slider maps 06:00–20:00 to a deterministic sun arc, directional color/intensity, sky/ambient color, and exposure preset. It has no running clock. Lighting is conventional PBR with one shadow-casting directional sun, ambient sky, distance fog, and the underground point light. Weather, seasons, dynamic ecology, and temporal simulation have no render resources or systems.

The default 14:00 view prioritizes landscape readability for milestone images. The geology cutaway uses raw/brick diagnostics and a curated camera position rather than a separate altered world.

## Graphics memory accounting

`GpuAllocationLedger` is updated whenever the application creates/destroys or resizes a counted GPU resource. It records checked byte estimates for:

- terrain vertex/index/material buffers;
- water buffers;
- registered object mesh/texture assets (counted once per shared handle);
- instance/dressing/debug buffers;
- material texture arrays and mip levels;
- shadow maps, depth/color render targets, and known post-process targets.

For each buffer, bytes are the requested allocation size. For textures, bytes include all mip levels, layers, sample count, and format block size. Shared resources are deduplicated by asset/allocation ID. The ledger total is a CPU `u64`; IDs and shader-visible category counts are `u32`. The report includes `untracked_driver_overhead: true`, adapter/backend fields, and category totals.

This ledger is a portable optimization/comparison proxy, not proof of driver/backend resident graphics memory. When an acceptance harness supplies a reviewed resident measurement, the report records provider, scope, sampling interval, peak bytes, and raw artifact reference separately. No cross-platform provider is mandated by the current implementation plan. Consequently the Product One `< approximately 2 GB resident graphics memory` requirement is not declared passed from the ledger alone; this is the explicit Design Divergence in [overview.md](overview.md) and requires product approval or a later resident-measurement amendment.

The benchmark asserts there is no monotonically growing allocation after repeated band crossings or carve operations. Chunk eviction decrements resident derived buffers; global shared assets remain counted.

## Portability and shader rules

- WGSL is the only custom shader source. It must validate under Bevy/wgpu's Metal, Vulkan, and DirectX backends.
- No `u64`/`i64` atomic, vendor subgroup assumption, Metal-only function, storage texture format without cross-backend support, or runtime-generated platform shader fork is allowed.
- Texture arrays and standard sampled textures are preferred to bindless/vendor-specific material paths.
- CPU mesh output uses finite `f32` positions and `u32` indices. Large global coordinates are converted to camera-relative/local chunk transforms to preserve precision.
- Render quality changes are explicit presentation config and benchmark metadata. Acceptance cannot silently lower bands/resolution/assets on one target.

## Visual acceptance checklist

Human headed review is required because rendering is not a unit-test target. A passing candidate demonstrates:

- an attractive smooth meadow, dense mixed forest, grass/bush/rock/stump dressing, river/lake in carved terrain, cliff with tilted strata, and the cut-stone ruin/staircase;
- one continuous camera/player journey from cliff top through shelves/stairs/cave mouth to about -40 m, with no level transition, camera clipping, or unreadable cave segment;
- visible aquifer and iron material at declared cave crossings;
- a 3 m hillside tunnel carve whose cut surface appears within two frames, whose old grass disappears, and which is immediately walkable;
- placed material with matching collision and visual material;
- raw voxel, brick boundary, streaming-band, and fixed-time diagnostics;
- no obvious LOD cracks, floating dressing, per-instance material explosion, water outside carved basins, or cube-field appearance in normal mode.

Milestone first-terrain, tunnel-carving, geology-cutaway, dressed-world, playable-run, and benchmark-result captures use the same shipping presentation and public operations. They are outputs, not authoritative assets.
