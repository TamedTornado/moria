# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate
or a small family of tightly scoped Rust crates. It is an engine-layer world
foundation for external games—not a game, not a System/LLM product, and not a
presentation or character product. A walkable-world executable may exist in this
repository only as an adjacent validation harness (see Q1); it is not the
product identity.

## Purpose

Moria exists so multiple game products can share one diggable, continuous-3D
natural world whose material truth is fully mutable—including deep
underground—without each game reimplementing world matter, generation,
mutation, and queries. The substrate must stand alone with no LLM dependency;
game rules live above it.

## Product boundary

**In product:** the reusable voxel-world substrate and its public consumer
interfaces for matter, world generation, mutation, queries, and the
engine-layer world services those outcomes require.

**Adjacent, not identity:** a walkable-world executable, if present, is only a
validation harness and must use the same public interfaces available to an
external game. Whether that harness is a required current delivery is open
(Q1).

**Out of product / separate consumers:** the actual game (not in this
repository); game rules; System/LLM, spell, gas, combat, AI, and building
layers; harness- or game-owned controllers, cameras, characters, authored demo
routes and content, presentation, and acceptance scenarios. Compatibility seams
may be designed where substrate requirements demand them; those upper layers
must not be implemented here.

## Required product outcomes

- Consumers get a continuous 3D voxel world that can read as a natural surface
  world (terrain, forests, rivers, cliffs, caves) while the voxel grid remains
  authoritative material truth—not decorative geometry beside a heightmap.
- Matter is mutable everywhere, all the way down: dig, destroy, place, and
  query against voxel truth; deep Z (strata, caves, underground material) is
  first-class content space, not a false floor.
- The substrate provides engine-layer matter, physics-capable world services,
  generation, mutation, and queries so games above do not own privileged voxel
  access; mesh and surface dressing are views derived from matter, not
  authoritative saves.
- Consumers can generate diggable geology, run large continuous regions with
  activity-centered residency, and persist truth as generation plus edits so a
  scarred world reloads as the same material world.
- Any in-repo harness and any external game share one non-privileged public
  integration surface at the Rust crate boundary.
- The substrate has zero LLM dependency and remains usable as a pure sandbox
  foundation.

## Future products and enabling implications

Future consumers (not this product): a System/LLM ARPG, a DF-style
fortress/colony game, a Moria-style descent/adventure game, and pure sandbox
uses. The walkable-world harness is an adjacent validation consumer of the
substrate under test, not a game product.

Enabling implications supported by the seeds: clean public seams for upper
layers; diggable geology and deep Z for descent and fortress fantasies;
mutable matter and mutation verbs as the basis for later game-owned building
and combat policy—without implementing those game layers here.

## Non-goals

- Implementing the actual game or game rules in this repository
- System/LLM features, spells, gas policy, combat, AI, or building layers
- Owning player controllers, cameras, characters, HUD, or authored campaign
  content as product scope
- Treating a validation harness’s demo route, performance gates, or device
  profile as the substrate’s product identity
- Privileged or game-specific implementation paths available only to in-repo
  code

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates
  for external game consumption.
- If a validation harness exists, it must consume only the public interfaces
  available to an external game; the consumer boundary is not optional.
- Game rules and future System, LLM, spell, gas, combat, AI, and building
  layers are out of scope for implementation here.
- The substrate is GPU-resident and must stand alone without LLM dependency.

## Deferred design decisions

- Precise crate split within the allowed family; internal layering and APIs
- Depth and sequence of matter/physics services (fluids, structural integrity,
  vegetation objects, ambient sim) versus any first harness slice
- Voxel scale, LOD/streaming policy, meshing approach, persistence encoding
- Harness content, controls, presentation, workloads, and performance gates if
  a harness is delivered
- Supported hardware/OS targets and portable performance promises for the
  substrate itself

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a required current delivery of
this repository, or only a permitted adjacent artifact?

*Proposed safe answer:* Permitted and valuable for proving the substrate, but
not part of product identity; schedule and “done” criteria for any harness are
downstream of substrate outcomes.

*If answered “required”:* the repository must plan harness delivery as a
current obligation while still excluding harness controls, content,
presentation, and acceptance detail from substrate identity. *If “permitted
only”:* substrate crates and public interfaces alone define current success;
any harness is optional adjacent work.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world
  substrate (Rust crate) and separates the walkable-world executable as
  consumer/harness, not game layer.
- **docs/seeds/project-boundary.md** — Binding repository boundary: substrate
  product, external game out of repo, public-interface harness rule, and
  explicit exclusion of System/LLM/spell/gas/combat/AI/building layers.
- **docs/seeds/product-one-seed.md** — First walkable-world demo consumer
  motivating mutability proof, dig/place, geology, and harness-style
  validation; its controllers, seed content, milestones, and device/performance
  gates stay consumer-adjacent, not product identity.
- **docs/seeds/voxel-world-substrate.md** — Long-horizon substrate purpose and
  outcome family (natural look over voxel truth, full mutability, deep Z,
  matter/physics/queries/mutation, multi-game reuse, GPU-resident, no LLM);
  mechanisms and open technical questions deferred to design.
