# Moria Product One — The Walkable World

## Overview

Moria Product One is a reusable voxel-world substrate demonstrated through a downloadable, third-person walkable world. It generates a natural-looking region containing hills, meadow, forest, a river and lake, cliffs, caves, geology, and a ruin. The visible landscape is a smooth presentation of an underlying material world rather than a heightmap decorated with disconnected props.

The product exists to prove one claim: the world is continuous, three-dimensional, and materially true. A user can travel from a canopy-level cliff top to a cave floor roughly 40 metres below the surface, then dig into or place material in the terrain and see the affected surfaces update immediately. The validation demo must use only the same supported world operations and queries available to an external consumer; it must not contain privileged or game-specific world behavior.

The primary users are:

- Creators of future games who need a measured, reusable foundation for mutable surface and underground worlds.
- The project team, which needs a benchmark bed for fidelity, distance rendering, streaming, persistence, and vegetation-scale decisions.
- A public audience evaluating the project through milestone images, clips, benchmark results, and the playable demo.

Product One is not itself a game. Combat, stats, non-player characters, AI, spells, the System or any language-model behavior, gas or pricing rules, construction gameplay, mechanisms, dynamic ecology, and broader game loops are outside its scope.

## Core Functionality

### Generate the curated seed region

The product generates one deterministic region from one authored seed and a curated set of generation parameters. The world itself is not hand-authored. Regenerating the seed must recreate the same base terrain, geology, cave, water bodies, vegetation placement, and point of interest.

The region is 1 km by 1 km by 256 m. Its typical surface is around 64 m, leaving about 64 m of playable sky above and about 190 m of geology below. The generated route must reliably include:

- A rolling meadow with grass dressing.
- A dense mixed forest with two tree species and bushes.
- A river occupying a genuinely carved channel and a lake with static water surfaces.
- A cliff or rocky outcrop that visibly exposes tilted geological strata.
- A karst cave that is walkable from a surface entrance to approximately -40 m.
- An aquifer band and one iron-ore vein crossing the cave route.
- Boulders, stumps, and scattered rocks.
- One small cut-stone ruin placed from point-of-interest metadata, with an intact staircase.

The generation process must define surface columns, strata, caves, ore, aquifer material, and point-of-interest placement for any requested location without requiring the entire region to be expanded into raw voxels in advance. The larger-scale continent context may be limited to the curated parameters needed by this region.

### Present smooth material truth

The world uses 25 cm voxels for Product One, grouped into 16 × 16 × 16 voxel bricks for diagnostics, updates, and streaming. Terrain, caves, cliffs, cuts, and the ruin must be displayed as smooth or sharp surfaces appropriate to their material: soil and natural terrain read organically, while cliff edges, newly cut faces, and masonry retain convincing definition. Material transitions use the underlying voxel materials, and the visible surface is always disposable, derived presentation rather than saved world truth.

Repeated grass and clutter are dressing anchored to eligible upward-facing soil or grass surfaces. Digging away their supporting surface removes the dressing with it. Trees, bushes, boulders, and stumps are registered material objects rather than disconnected scenery. Product One includes their placement and display at forest scale, but not felling, growth, burning, rolling, splitting, or conversion into moving bodies.

Only the parts of the region needed for nearby display, traversal, mutation, or inspection become fully active. Uniform untouched air, water, and geology remain compact so idle wilderness consumes negligible detailed-world memory. Moving through the region changes the active distance bands without changing the deterministic underlying world.

### Support traversal

The validation demo provides one third-person player who can run, sprint, jump, and paddle at the surface of water. Movement and collision are evaluated against voxel occupancy, not against the displayed terrain surface. This preserves the rule that the voxel world is authoritative even when the presentation is being rebuilt.

The camera freely orbits the player and avoids passing through terrain. Underground, a simple player-attached light makes the cave traversable. The curated route must exercise verticality through a cliff top, jumpable rock shelves, the ruin staircase, the cave mouth, and the cave floor in one continuous journey.

### Prove mutation through debug operations

Keyboard-driven debug operations provide:

- A spherical dig operation that erodes and removes matter.
- A spherical place operation using a selected material.
- A wireframe or brick-boundary view.
- A raw-voxel view toggle.
- A streaming-distance visualizer.
- A fixed time-of-day slider.

Digging and placement are proof and validation tools, not a player progression or construction system. A representative 3 m-radius hillside carve must expose credible cut earth, update only affected world areas, become traversable, and cause no visible hitch. The signature demonstration is a player running through the forest, stopping to carve a tunnel into a hillside, and then walking through it.

All world inspection and mutation, including the debug tools, uses defined world queries and dig/place operations. No consumer directly edits the voxel truth. Product One includes no embedded scripting language.

### Save and restore edits

The base seed is reproducible and is not duplicated in a save. A single save slot records only changes from the generated base world. Reloading the same seed plus its saved changes must restore every edit exactly. Save version migration, multiple slots, and persistence of future game systems are not included.

### Produce comparable benchmarks

The deliverable includes a scripted flythrough and a carve-storm benchmark. Each run reports frame rate, mutation-to-surface-update latency, cold-start time, active graphics memory, and save size together with a machine profile. Later substrate changes can be evaluated against the Product One baseline, and results from different machines remain identifiable rather than being compared without hardware context.

## Entities and Data Model

### World Seed

- **Attributes:** stable seed value; curated regional generation parameters.
- **Relationships:** deterministically produces one Base Region and all of its columns, geology, caves, objects, water bodies, and point-of-interest metadata.

### Base Region

- **Attributes:** 1 km × 1 km × 256 m bounds; typical surface elevation around 64 m; 25 cm voxel scale; active distance bands; generated feature locations.
- **Relationships:** contains Columns, Voxel Bricks, Water Bodies, Voxel Objects, Dressing, and the Ruin Point of Interest. It combines with an Edit Delta Set to produce the current World State.

### Column

- **Attributes:** horizontal location; surface height; ordered runs of air, soil, stone, water, and cave gaps; local strata and feature metadata.
- **Relationships:** summarizes the vertical composition of the Base Region and provides the source for surface placement and lazy creation of detailed matter.

### Voxel Brick

- **Attributes:** world coordinate; 16 × 16 × 16 voxel extent, equivalent to a 4 m cube at the Product One voxel scale; uniform or detailed state; dirty/clean surface status; active distance band.
- **Relationships:** contains Voxels when detail is required. A mutation dirties the affected brick and neighboring surface boundaries. Untouched uniform bricks represent air, water, or stone without allocating every voxel individually.

### Voxel

- **Attributes:** material; density from empty through full; reserved material-state value.
- **Relationships:** belongs to a detailed Voxel Brick. Its density and Material determine occupancy, collision, and the derived visible surface. Product One preserves the state field but does not run wetness, burning, growth, settling, or other state-driven behavior.

### Material

- **Attributes:** name/identifier; hardness; granular flag; visual blending and surface properties.
- **Seed palette:** air, water, topsoil, subsoil, sand, gravel, limestone, sandstone, shale, granite, iron ore, wood, leaf, and cut stone.
- **Relationships:** assigned to Voxels and Voxel Objects. Sand and gravel are identified as granular, but granular settling is not active in Product One.

### Geological Feature

- **Attributes:** type; three-dimensional extent; host material; depth; orientation.
- **Types in Product One:** topsoil and subsoil layers, tilted sedimentary/rock strata, karst cave, aquifer band, and iron-ore vein.
- **Relationships:** contributes material and voids to Columns and Voxels. The cave, aquifer, and ore vein intersect the curated traversal route.

### Water Body

- **Attributes:** kind (lake or river); surface level; occupied columns/volume; static state.
- **Relationships:** occupies real carved terrain rather than overlaying a heightmap. It supports surface paddling. Flow, flooding, draining, pressure, waterfalls, and fine splashes are absent.

### Voxel Object

- **Attributes:** object identity; object type; voxel-backed material shape; position and orientation; species where applicable.
- **Types in Product One:** two tree species, bushes, boulders, stumps, scattered rocks, and the cut-stone ruin stamp.
- **Relationships:** placed from terrain, biome, or point-of-interest data and displayed with the region. These objects do not gain felling, physics, growth, fire, or other simulation behavior in the required scope.

### Surface Dressing

- **Attributes:** dressing type; anchor position; source surface/material; density.
- **Relationships:** derived from eligible terrain surfaces and local vegetation density. It disappears or relocates when its supporting surface changes and is never independent world truth.

### Ruin Point of Interest

- **Attributes:** metadata location; hand-authored sparse cut-stone shape; intact staircase.
- **Relationships:** places one Voxel Object into the generated region. It demonstrates one reusable stamp path and provides sharp masonry beside organic terrain.

### Player

- **Attributes:** position; movement intent; run/sprint state; jump state; swim state; collision volume; attached underground light.
- **Relationships:** moves and collides against voxel occupancy, anchors nearby display/streaming activity, controls the Camera, and invokes Debug Operations.

### Camera

- **Attributes:** orbit angle; distance; collision-adjusted position.
- **Relationships:** follows the Player, determines the visible region, and must not pass through voxel terrain.

### Debug Operation

- **Attributes:** operation type; target position; spherical radius where applicable; selected placement material where applicable.
- **Types:** dig, place, brick/wireframe view, raw-voxel view, streaming visualization, and time-of-day adjustment.
- **Relationships:** reads or changes the World State through the supported operations. Dig and place create Edit Deltas and trigger surface and dressing refreshes.

### Edit Delta Set

- **Attributes:** changed brick coordinates; exact material/density differences from the seed; single-slot identity.
- **Relationships:** overlays the deterministic Base Region to reconstruct the current World State. Unchanged matter is not saved.

### Benchmark Run

- **Attributes:** scenario name; machine profile; resolution; frame rate; cold-start time; edit-to-surface-update latency; active graphics memory; saved-delta size.
- **Relationships:** executes against the curated Base Region and records comparable acceptance evidence for the product.

## Interactions

### World generation and activation

1. The World Seed establishes the region's broad terrain and climate parameters.
2. Columns define surface height, material bands, cave gaps, water, and placement metadata.
3. When the player approaches, inspects, or changes a location, the system derives the required detailed Voxels for that location.
4. Uniform areas remain compact; detailed areas produce the visible terrain surface and eligible Dressing.
5. As the player moves, nearby display areas become active and distant areas return to compact or saved representations without changing their truth.

### Dig or place

1. The user selects a debug dig or place operation and targets a spherical area.
2. The operation changes density and/or material in the affected Voxels. Digging erodes density before matter becomes empty; placement adds the chosen material into available space.
3. Affected brick areas and shared boundaries are marked as changed.
4. Collision reflects the new voxel occupancy, while the visible terrain and anchored dressing refresh from the changed truth.
5. The differences from the seed are added to the Edit Delta Set.
6. The visible update completes within two rendered frames for the representative 3 m-radius carve, without a hitch.

### Traversal

1. Player input produces run, sprint, jump, paddle, and camera-orbit intent.
2. The player collision volume tests the current voxel occupancy and resolves movement against solid matter.
3. The camera follows and orbits while avoiding terrain; the attached light illuminates underground traversal.
4. Player and camera position drive which portions of the world are displayed and held active.
5. A freshly carved opening becomes traversable because both movement and presentation derive from the changed voxel state.

### Save and load

1. Saving compares current changed matter with the deterministic base seed and records only the deltas in the single slot.
2. Loading regenerates the same base region, applies every saved delta at its coordinate, and restores the exact edited state.
3. Derived surfaces and dressing are recreated from the restored truth rather than stored as authoritative data.

### Benchmarking

1. The scripted flythrough exercises the meadow, dense forest, water, cliff, ruin, surface-to-cave route, and active distance bands.
2. The carve storm repeatedly changes material to stress surface refresh and persistence.
3. The run emits the required measurements and machine profile.
4. Results become the baseline used to decide voxel fidelity, distant-world presentation, and object-layer scaling.

## User Experience

### Entering the world

Launching the validation demo begins loading the fixed seed. Within five seconds, the player is in control in a walkable part of the generated region. No game account, character creation, mission, inventory, or narrative setup is present.

### Exploring the postcard world

The default view is free-orbit third person. The user runs or sprints through a meadow into a dense mixed forest, seeing grass, bushes, boulders, stumps, two tree species, the river and lake, exposed cliffs, and the ruin as parts of one coherent natural scene. The terrain should read as a conventional attractive world rather than a field of cubes.

The user can jump across climbable rock shelves and use the ruin staircase. At water, the player paddles on the surface; there is no dynamic current simulation or underwater swimming requirement.

### Making the continuous-depth run

The designed route takes the user from a canopy-level cliff top to the surface cave mouth and down to roughly -40 m without a level transition. Underground, the camera avoids cave walls and the player light keeps the route legible. Exposed strata, an aquifer band, and an iron-ore vein make the subterranean volume visibly geological rather than empty space under a surface shell.

### Proving material truth

At any point, the user can invoke debug dig or place shortcuts. The intended proof moment occurs at a hillside: a spherical carve opens a smooth tunnel with convincing cut faces, the view updates within two frames, and the user immediately walks through the new opening. Placement can add a sphere of a selected seed material and produces the corresponding surface and collision.

The user can switch to raw-voxel and brick-boundary views to reveal the underlying representation, turn on the streaming-distance visualizer while moving, and adjust a fixed time-of-day slider to inspect the world under different lighting. These are keyboard-driven diagnostics; Product One does not add a building palette, settings flow, or polished debug interface.

### Reviewing evidence

The scripted benchmark is a separate validation experience rather than gameplay. It flies through representative areas, performs a carve storm, and presents or records the required performance values with the machine profile. Milestone outputs also support a public sequence: a first terrain image, a tunnel-carving clip, a geology cutaway, a dressed-world shot, the playable run, and the benchmark results.

## Constraints and Requirements

### Acceptance targets

| Area | Requirement |
|---|---|
| Frame rate | 60 fps at 1440p on a 3060-class mid-range discrete graphics machine; 60 fps at 1080p–1440p on the 32 GB M4 Mac Mini development machine. |
| Mutation latency | Changed terrain surfaces update within two rendered frames; a 3 m-radius carve causes no hitch. |
| Cold start | The world becomes walkable in under 5 seconds. |
| Graphics memory | The full region remains below approximately 2 GB resident graphics memory with active distance bands; untouched uniform wilderness has near-zero detailed voxel cost. |
| Persistence | A heavily defaced world produces a delta save below 50 MB and reloads exactly. |
| Benchmark evidence | The flythrough and carve-storm scenarios output every listed metric plus a machine profile. |

The discrete-graphics frame-rate target is provisional until it can be verified on the Linux test machine and must then be re-baselined. Results without a machine profile are not considered comparable evidence.

### Platform and portability

- The primary development target is an M4 Mac Mini with 32 GB unified memory.
- Load-bearing graphics work must stay portable across Apple, Vulkan-class, and DirectX-class targets; Product One must not depend on an Apple-only graphics path.
- Graphics counters, allocation indices, and propagation labels must fit within 32-bit atomic operations because the Apple development target does not support the required 64-bit atomic operations.
- The design must treat memory traffic as the primary development-machine limit. Sparse untouched matter and compact uniform regions are required from the first milestone, not optional later optimizations.
- The world product and validation demo remain separate deliverables. The demo validates the same supported operations and observations available to a future external game and may not introduce privileged world access.

### Required scope boundaries

- Static lake and river surfaces are included; fluid flow, pressure, floods, draining, splashes, and other dynamic fluid tiers are excluded.
- Material data reserves state for future wetness, burning, growth, or damage, but Product One runs none of those behaviors.
- Sand and gravel are tagged as granular, but they do not settle or collapse.
- Voxel objects are placed, registered, and displayed, but they do not fall, grow, burn, break, or become dynamic bodies.
- No structural-integrity or cave-in simulation is included.
- No weather, seasons, growth, or ambient ecology is included. A manually controlled fixed time of day is sufficient.
- No dynamic navigation, agents, labor, room detection, blueprints, construction interface, mechanisms, or semantic building systems are included. The ruin uses one predefined stamp solely to validate generated placement.
- No combat, stats, autonomous characters, AI, System/language-model behavior, spells, gas, pricing, or intent systems are included.
- No multiplayer behavior is included.
- Persistence is one seed plus edit deltas in one slot, with no save versioning.

### Delivery sequence

The required product is demonstrated in this order:

1. **Hill that looks like a hill:** smooth terrain and material treatment over minimal generation.
2. **Carve a smooth tunnel:** dig/place behavior and incremental surface refresh.
3. **True geology:** strata, cave, ore, and aquifer visible in a cutaway diagnostic.
4. **Dressed world:** meadow, grass, forest objects, boulders, water, and ruin form the postcard scene.
5. **The run:** player and camera complete the continuous cliff-top-to-cave-floor route.
6. **Numbers:** streaming behavior, exact delta persistence, and benchmark output meet the acceptance targets.

A seventh **Timber** milestone—felling one tree and letting it fall as a moving body—is a stretch goal only and cannot delay or broaden the six required milestones. The source estimate is two to three weeks through milestone six, with the principal delivery risks concentrated in the two-frame terrain-refresh target and the memory target.

## Open Questions

1. **Final voxel scale.** Should later use retain 25 cm voxels or adopt 12.5 cm voxels for greater fidelity at approximately eight times the raw cost? The proposed Product One default is **25 cm throughout the seed region**. The benchmark must supply the measurements for the final decision; mixed per-region fidelity is only a possible later option.

2. **Distant terrain representation.** Should distant terrain use progressively simplified extracted surfaces or column-derived impostors, and how much can the intended camera distance conceal? **No final default is stated in the planning source.** Product One should keep this as a measured decision and use the simplest option that meets the visual and frame-rate targets.

3. **Voxel-object scaling.** At what number and density of trees and other voxel objects does the object layer need additional spatial acceleration? **No threshold or default is stated.** The curated dense two-species forest is the required measurement scene; added complexity is deferred until that scene demonstrates a need.

4. **Dynamic-fluid fidelity.** If a later product adds coarse fluid flow, will momentum-only behavior be adequate or will pressure behavior be required? The Product One default is **defer the decision and ship static lake and river bodies only**.

5. **Multiplayer scope statement.** Should multiplayer readiness remain an explicit design concern even though multiplayer is not being built? The proposed default is **preserve the controlled operation/query boundary and describe it as future-ready, but implement no multiplayer behavior in Product One**.

6. **Discrete-graphics baseline.** What performance does Product One achieve on the unavailable Linux/discrete-graphics test machine? The proposed default is **treat the 3060-class target as provisional, retain machine profiles with all results, and re-baseline when that machine is available**.

7. **Timber stretch milestone.** Is one tree-felling and rigid fall cheap enough to include after the required benchmark milestone? The proposed default is **exclude it from the committed scope unless milestones one through six are complete and the added coupling is demonstrably small**.
