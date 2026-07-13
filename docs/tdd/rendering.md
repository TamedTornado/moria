# Rendering Design

## Visual authority and portability

Rendering is a disposable projection of `WorldQuery` at a captured truth revision. No render mesh, depth buffer, collider mesh, texture, object instance, or dressing anchor is saved or used as authoritative occupancy. The load-bearing renderer is Bevy 0.19’s cross-platform wgpu path and standard PBR pipeline. Product One does not introduce an Apple-only path or require 64-bit GPU atomics.

All GPU-visible allocation IDs, counters, draw/instance indices, cluster labels, and propagation labels are `u32`/32-bit atomic operations. CPU diagnostics add per-resource byte sizes into `u64`. A custom terrain material/shader may sample material arrays and blending weights, but custom compute culling/meshing is out of scope until shared handles, Bevy batching, frustum culling, LOD, and measured representative scenes prove inadequate.

Mesh/chunk allocation rejects any vertex, index, instance, or draw count that would exceed `u32`; subdivision occurs before upload. No shader relies on a 64-bit counter even if a desktop backend offers one.

## Scene structure

```text
WorldRoot
  DirectionalSun + environment/ambient lighting
  TerrainRoot
    near terrain-owner brick fragments (25 cm composite source)
    mid chunk meshes (50 cm source)
    far surface chunks (1 m source)
  WaterRoot
    lake surface sections
    river surface sections
  ObjectRoot
    shared tree species/variant/support-signature instances
    shared bush/boulder/stump/rock instances
    shared ruin mesh or edited instance sections
  DressingRoot
    grass/clutter batches
  DiagnosticsRoot
    raw voxels / brick lines / streaming bands
PlayerRoot
  character presentation
  UndergroundLight
CameraRig
```

Render entities include a source key, LOD, and source revision. Installation is atomic per artifact: keep the old version visible until the new GPU asset is ready, then swap and despawn the old. A stale job can never replace a newer revision.

## Material-aware terrain extraction

Near extraction uses deterministic manifold dual contouring over solid density at iso-value 128. Each 16-cube brick job reads a one-voxel halo from the composite world query and immutable base-provenance owners. It computes crossings in fixed traversal order and solves a bounded QEF per active cell. QEF output is clamped to the cell; degenerate cases fall back to the average crossing. Brick boundary ownership belongs to the lexicographically lower brick/cell and halo samples guarantee matching positions/normals, preventing cracks.

This is one composite topology pass with an owner-filtered output, not independent terrain/object isosurfaces. A current solid/non-solid crossing is emitted only by the solid sample's `SurfaceOwner` from `data-model.md`; current solid/solid interfaces emit no exterior cap. A mixed-owner cell shares its one QEF vertex and assigns each primitive by the deterministic contributing-crossing rule. Terrain jobs retain only terrain-owned primitives for their brick; an object-base/unique job retains only primitives for its `ObjectId`. Therefore an unedited tree or ruin is absent from the terrain fragment even though it influenced composite topology, and its object artifact supplies that boundary exactly once. Object/terrain contacts have neither a gap nor coincident faces.

Surface class controls feature preservation:

- Organic soil/subsoil uses smoothed density gradients and normal averaging to read as continuous terrain.
- Rock/geological transitions retain creases when gradient angle or material boundary exceeds `WorldRenderConfig.terrain_crease_angle_deg = 38`, making tilted strata and fresh cuts legible.
- Cut stone/masonry uses feature-preserving QEF constraints and flat/crease normals aligned to stamp faces, keeping stair treads, risers, and ruin corners sharp.
- Wood/leaf object templates use the same material truth but object-template extraction/cached meshes.

Vertices carry position, normal, dominant material ID, secondary material ID, blend weight, and source diagnostic value. The terrain shader selects texture-array layers and blends only adjacent underlying solid materials; it never paints biome color independent of voxel material. Newly cut faces automatically reveal the materials sampled at those crossings.

The output must be manifold within the supported generated/edit configurations, contain no NaN/Inf vertices, keep all vertices within brick plus halo bounds, and have matching seam samples. A property test keys primitives by canonical lattice crossing/topology ID and proves that concatenating all owner partitions equals the unpartitioned composite output one-for-one. Cross-owner dig/place/revert and all brick-corner seam fixtures must have neither missing nor duplicate keys. Mesh property tests cover these invariants without treating screenshots as unit tests.

## Level of detail and distance continuity

- Near (`<=96 m`): full 25 cm source sampling, caves/overhangs, full material blend and shadows.
- Mid (`<=256 m`): 50 cm conservative density/material reduction. It retains large overhang/cliff silhouettes and cave mouths selected by portal visibility; small underground voids are omitted.
- Far (to all region bounds, maximum radius 720 m): 1 m surface-only chunks generated from column/base surface queries, large river/lake/cliff silhouette, low-detail tree representation, no hidden underground geometry.

LOD chunks use authored transition rings/transition cells over an 8 m overlap and geomorph between compatible boundary samples. If a transition is not ready, the higher-detail artifact remains and the lower-detail artifact stays hidden; cracks are not accepted. Frustum culling, Bevy visibility ranges, and horizon/occlusion opportunities apply before draw. Band hysteresis is in `config.md`.

Simulation count is decoupled from render count: a distant generated object record does not imply a high-detail mesh entity. Unedited repeated objects share mesh/material handles by template, orientation variant, and validated support-contact signature so Bevy batching/instancing can apply. Tree near/mid/far variants are shared per that key. An instance receives a unique derived mesh whenever its extraction stencil differs from base, including a neighboring terrain edit that changes its boundary. It returns to the shared base mesh only after its entire stencil equals base and the base key is restored.

## Water

Lake and river surfaces are deterministic meshes derived from `WaterBody` polygons/segments and occupied water columns. They sit at fixed generated levels, clip to the genuinely carved banks, use shared translucent PBR water materials, and receive static normal-map motion only as a visual effect. The shader may animate normals from render time but cannot move the surface boundary or affect queries.

There are no flow vectors, waterfalls, pressure, draining, flooding, displaced volumes, or fine splash particles. A terrain edit can expose/hide existing water cells in the composite query; since water itself is not editable, the water surface artifact only refreshes where visibility against edited solid matter changed.

## Vegetation, objects, and dressing

Procedural voxel templates produce a bounded set of shared meshes for two tree species and variants, bushes, boulders, stumps, and rocks. Each template declares a bounded set of support-contact signatures; generation rejects an unsupported placement rather than silently creating arbitrary unedited mesh variants. Placement uses transform instances from the manifest. Materials are shared by kind/material palette; no per-instance material is created. Dense forest representative scenes are the performance baseline.

Grass/clutter dressing uses shared crossed-card or low-poly blade clusters batched by material/variant and activation chunk. Near band uses full configured density; mid uses deterministic thinning; far has no grass. Anchors require eligible material and normal Y `>= WorldRenderConfig.dressing_min_normal_y` (`0.82`) and regenerate from surface revision. Dig acceptance hides invalidated anchors immediately, so grass never floats above a removed supporting surface.

The ruin is the object-owner partition of the same composite extraction used around its foundation. Its feature-preserving normals keep blocks/stairs crisp beside organic terrain, while the common crossing/QEF rule prevents a second terrain copy or a foundation gap. It is not a semantic building, construction entity, or special collision mesh.

## Camera and lighting presentation

The free-orbit perspective camera follows the kinematic player. Its position is collision-corrected with voxel queries before Bevy transform propagation, so it does not enter terrain/cave walls. Near plane and field of view use the concrete `WorldRenderConfig` defaults (0.05 m and 65 degrees vertical); `render_fingerprint` includes them and is a fixed member of benchmark `acceptance_config_fingerprint`.

A directional sun and environment/ambient term provide conventional outdoor lighting. `FixedSolarTime` maps 6:00–20:00 to a deterministic sun direction/intensity curve; it never advances. The player point light enables underground under the classifier contract in `config.md`. Fog may be used only as distance composition, not to hide missing geometry within the configured visible bands.

## Diagnostics rendering

- Brick boundaries reuse one wire cube mesh and shared clean/dirty/band materials; transforms size it to 4 m.
- Raw voxel mode reuses one cube mesh and one material per palette entry, with batched/instanced transforms. It is restricted to near/inspect activation to avoid expanding the region.
- Streaming visualization uses per-band shared material overrides/overlay colors and displays focus radii.
- A compact Bevy UI text block shows active controls, selected material/radius, fixed time, lifecycle/save result, and active diagnostic flags.

Diagnostics do not allocate unique material per brick/voxel and are included in tracked GPU bytes when active.

## Graphics memory accounting

Every app-owned GPU buffer, texture, mesh payload, texture-array level, diagnostic instance buffer, and retained staging allocation registers an allocation label, `u32` ID, size, creation frame, and active/retired state in `GraphicsAllocationTracker`. Shared handles count once. The benchmark’s portable `active_graphics_allocated_bytes` is the peak sum of active tracked allocations during the measured interval.

On the discrete Linux acceptance machine, the benchmark harness also samples process/device-local resident graphics memory through the approved Vulkan/driver telemetry available on that machine and records its tool/source; this `active_graphics_resident_bytes` high-water mark is required for re-baselining. On M4 unified memory there is no honest dedicated-VRAM residency figure, so the report requires tracked GPU allocation plus process physical-footprint high-water and marks memory model `unified`. Unsupported platform residency is `null`, never fabricated from tracked allocation.

The acceptance target is below 2,000,000,000 active graphics bytes with the full far region and normal active bands: tracked allocation must pass everywhere, discrete Linux resident high-water must also pass when establishing its baseline, and M4 reports tracked allocation within the unified process footprint. CPU voxel/delta/cache memory is reported separately.

## Visual validation checklist

Human release/testbed validation must cover: attractive non-cubic meadow/forest vista; both tree species and required clutter; carved river/lake banks; tilted strata; cliff/sharp cuts; intact traversable ruin stairs; connected cave to ~40 m local depth; visible aquifer and ore crossing; no camera clipping; no seam/LOD cracks; immediate dressing removal; smooth carve and material-appropriate place; raw/brick/band toggles; fixed time range; and no stale surface after mutation or load.
