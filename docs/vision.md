# Project vision

## What we are building now

**Moria** is a reusable, **GPU-resident voxel-world substrate** exposed as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation: generation, matter, physical and ambient material behavior, mutation, queries, derived presentation of voxel truth, and persistence for downstream games. It is **not** a game, not a demo product identity, and not a System/LLM stack.

## Purpose

Moria exists so multiple games can share one continuous material world that **reads as a natural surface world** while remaining **fully diggable voxel truth**—including deep underground—without each title reimplementing geology, matter simulation, ambient world behavior, streaming, edit persistence, and the public command/observation boundary. The substrate stands alone with **zero LLM dependency**. Game rules, presentation policy, and gameplay live above it; derived meshing, material look of voxel truth, and voxel-responsive dressing remain substrate-owned.

## Product boundary

**This product owns**

- The reusable voxel-world substrate and its public Rust consumer surface.
- Generation, material truth, active-region physical and ambient matter behavior, dig/place-class mutation and engine-level stamps, collision against voxel truth, derived presentation of matter, streaming, and persistence that regenerates baseline, restores edit deltas, and journals substrate-owned object/entity state for cross-run reuse.
- Compatibility seams where substrate requirements demand them—not game-layer or building-layer implementations.

**Adjacent or downstream (not this product’s identity)**

- Actual games (ARPG/System, fortress/colony, descent, sandbox) and their rules, UX, controllers, cameras, characters, demo routes/content, and **presentation policy**.
- System/LLM, spells, gas/pricing, combat, AI, and the **building layer** (labor, economy, designations, work-order blueprints, fortress semantics, scripted mechanisms such as doors/gates/pumps)—not the policy-neutral dig/place and stamp primitives those layers consume. Seams may keep future mechanisms compatible; mechanisms themselves are not required substrate facilities.
- A walkable-world executable is only an **adjacent validation consumer**, not the product. Whether shipping it is current repository delivery is **unresolved (Q1)**. While open, it **may** exist and must use only public interfaces—no privileged substrate paths.

## Required product outcomes

1. **Natural, fully material world with dressing contract.** Outdoor terrain that reads as ordinary (hills, forest, water, cliffs, meadows) while everything that can burn, break, or block is mutable voxel matter—not props outside the material world, and not a cube-aesthetic primary look. Interacting vegetation and micro-objects participate as voxel-backed matter objects; passive dressing need not have individual voxel identity but must stay anchored to and responsive to voxel state.
2. **Mutable everywhere, deep Z, placement first-class.** Any volume can be destroyed, altered, or placed; underground is content, not a false floor. Placement is a first-class engine verb; stamps (prefab multi-brick shapes) are substrate facilities. Scripted mechanism objects are building-layer consumers of seams, not required product facilities.
3. **Deterministic geology-first generation.** Layered ground and voids so digging reveals honest materials; stages are pure functions of coordinates and world seed so regions materialize independently and lazily.
4. **Active-region matter and ambient world behavior.** Materials support environmental and physical behavior in active regions—fluid response, structural failure/integrity, granular or other dynamic matter, and a thin ambient world family (day/night, seasons, weather fronts, growth, snow-line, wetness, water-level, ignition, drought, fire ecology)—so the surface world behaves as shared matter over time. First-harness slice omissions do not drop these substrate outcomes; delivery depth and sequence are design’s problem.
5. **Derived presentation as view; asymmetric command/observation.** Collision and truth run on voxels; mesh and material presentation are derived, non-authoritative, and not save truth. Consumers issue **commands in** and observe via **stale mirror + events**—verbs and queries only, no direct or synchronously exact voxel access. Games and any harness share that public surface with no privileged access.
6. **Streamable, restorable world; GPU-resident.** Untouched volume stays cheap; saved truth is generation plus edit deltas that restore player scars. Object and entity journals and cross-run persistence support reuse of moved objects and related substrate-owned state; that journal path is not claimed as exact bit-for-bit restore of every lifecycle. Active regions stream without always-resident dense truth. The substrate lives as a GPU-resident engine layer under the command/mirror contract.

## Future products and enabling implications

Future **consumers** (not current scope) include a System/LLM ARPG, fortress/colony, Moria-style descent, and pure sandbox. They own gameplay, content, controls, economy, agents, building-layer mechanisms, and presentation policy.

**Enabling implications:** the same matter, generation, mutation, object, ambient, and query foundation is reusable across modes; gas/pricing and LLM attach as **game-layer policy/clients**; semantic fortress tooling, work orders, and mechanism entities sit above compatible seams. First-consumer slice limits (deferred weather/seasons/growth, static-only fluids, deferred felling) do not narrow these product outcomes.

## Non-goals

- The actual game, rules, combat, stats, AI, or player-facing building/crafting UX.
- System/LLM, spells, gas metering/pricing, or LLM dependency in the substrate.
- Demo character, camera, seed route, debug keys, trailer content, harness save/size/timing gates, or consumer platform performance targets as product requirements.
- Building-layer implementation (fortress tooling, economy, designations, work-order blueprints, doors/gates/pumps and related scripted mechanisms) beyond compatibility seams.
- Primary “Minecraft cube world” aesthetic for the surface look.

## Confirmed vision constraints

- **Rust library:** crate or small family of tightly scoped crates.
- **GPU-resident** substrate with **commands in / stale mirror + events out**; observation is not direct synchronous voxel access.
- **Consumer isolation:** harness or external game uses only public interfaces.
- **Standalone:** zero LLM dependency; System is not a substrate feature.
- **Out of implementation here:** game rules and System, spell, gas, combat, AI, and building layers (seams only if required). Engine dig/place and stamp facilities stay in product scope as policy-neutral primitives; mechanism objects do not.
- **Presentation ownership:** substrate owns derived meshing, material presentation of voxel truth, and voxel-responsive dressing; games own presentation policy.

## Deferred design decisions

- Exact crate split and internal module boundaries (consumer boundary is fixed).
- Delivery depth and sequence for matter behaviors, ambient sim richness, generation, streaming, and persistence encodings.
- Voxel resolution, LOD/impostors, object-layer scaling, and meshing technique choices.
- Performance budgets, benchmark scenes, and target hardware baselines.
- Multiplayer deployment depth beyond the required command/observation boundary.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current repository delivery**, or only a **permitted adjacent artifact**?

- **Proposed safe answer:** Permitted adjacent artifact only—not part of Moria’s product identity; if present, it must consume solely public substrate interfaces.
- **If answered differently:** A “required delivery” answer keeps product identity as the substrate but adds a settled obligation to ship a harness beside it; a “not in repo” answer allows substrate-only delivery with validation entirely outside this repository.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world Rust substrate; walkable-world executable is a separate validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Substrate crate(s) only; game out of repo; harness if any is public-API-only; game/System/spell/gas/combat/AI/building layers out of scope (seams only).
- **`docs/seeds/product-one-seed.md`:** First consumer demo slice and its exact-load defacement benchmark; partial first-slice limits (including deferred weather/seasons/growth) do not redefine substrate identity; retains command/mirror as product-level contract.
- **`docs/seeds/voxel-world-substrate.md`:** Engine responsibilities—natural look, mutability, deep Z, geology, matter physics, thin ambient time/weather/growth/fire ecology, voxel-backed objects/dressing, dig/place and stamps, deterministic gen + delta/journal persistence, GPU command/mirror, derived presentation—without elevating building-layer mechanisms or making future games current product.
