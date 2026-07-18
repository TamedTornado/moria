# Issue 329 — Recovery PR #264: Fix explorer skeleton scene hierarchy

References: `docs/tdd/assets.md` §Meshes and animation, §Production asset registries, and §Import and validation pipeline; `docs/tdd/rendering.md` §Player, camera, and cave lighting.

## Properties that must hold

- For every accepted `assets/player/explorer.glb`, the binary must be valid glTF 2.0 and its default scene must contain a reachable character mesh and skeleton; every skin joint and every animation channel target must resolve to a node in that scene hierarchy.
- For all accepted explorer meshes, indexed triangle data, finite positions/normals/UV0, required tangents, Q8 bounds/support origin, and the 40,000-triangle budget must satisfy the matching registry contract.
- For every accepted explorer asset, clips named exactly `Idle`, `Run`, `Sprint`, `Jump`, `Fall`, and `Paddle` must exist and target the usable skeleton.

## Entity configurations to test

- Checked-in GLB; orphan skeleton root; skin joint outside the scene; mesh and armature in disjoint scenes; animation channel targeting a missing node; invalid accessor/bufferView; missing each required clip one at a time.
- Static-pose development fallback, valid populated animated state, and corrupt/fatal asset state.

## Edge cases

- Optional transition clips may be absent; duplicate required clip names, empty clips, non-finite transforms, invalid indices, and out-of-budget geometry must fail.
- Exact declared file/triangle limits pass; one unit above either fails before installation.

## Error paths

- Missing required clips must produce an asset-validation error and may use only the documented development static-pose fallback.
- The TDD does not prescribe exact bone/node names or a single-root topology beyond a reachable mesh+skeleton hierarchy; tests must not invent such names, but must reject orphaned or unresolvable relationships.

