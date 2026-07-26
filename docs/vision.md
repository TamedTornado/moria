# Moria — Vision

*Proposal for human approval and handoff to downstream design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates.

It is an engine-layer foundation for matter worlds—not a game. Downstream titles consume it; they are not built in this repository.

The repository **may** include a walkable-world executable, but only as a **validation harness**. That harness must use the same public interfaces available to an external game. It must not own privileged or game-specific implementation paths.

**Product One (“The Walkable World”)** is the binding first implementation slice: substrate capabilities proven through one curated generated region and a third-person walkable harness, with dig/place as proof that the world is fully material—not a heightmap with decoration.

---

## Purpose

Make a single claim true and reusable:

> This is not a heightmap with props. It is a fully material voxel world that can look like a normal natural landscape, support continuous deep play, and remain mutable everywhere—exposed cleanly enough that multiple games can sit on it without forking the world layer.

Product One is the first proof of that claim: a demoable, benchmarked artifact that establishes crates, API discipline, and measured answers to open substrate questions. Future games (ARPG with a System/LLM layer, fortress/colony, descent/sandbox, or pure sandbox) are **motivating consumers**, not current deliverables.

The substrate must stand alone with **zero LLM dependency**.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Voxel matter storage, sparsity, streaming | Any full game (rules, progression, economy, modes) |
| Geology-first generation (Product One slice) | System / LLM authorship and adjudication |
| Smooth mesh/view derivation from voxel truth | Combat, stats, AI, entities beyond a harness player |
| Dig/place and queries as the public mutation surface | Spells, gas pricing, intent systems |
| Persistence as seed + edit deltas; streaming rings | Building-as-gameplay (UI, work orders, mechanisms, labor) |
| Walkable harness (controller, camera, debug tools) | Full CA ecology, fluid *flow*, structural integrity, tree felling |

**Consumer boundary is mandatory.** Prefer a Cargo workspace that separates substrate crates from the harness. The precise crate split is a technical-design decision; treating the harness as the product is not.

**Compatibility seams** may be designed where substrate outcomes require them (verb/query surface, reserved material-state fields). Higher layers must not be implemented here.

**Architecture reference vs. current slice.** The full substrate design describes a longer capability horizon. Only the portions selected by Product One are required for this milestone; the rest is context for why seams and layering matter.

---

## Required product-level outcomes

What the current product must make true—not a feature inventory:

1. **Voxel truth, natural look.** Terrain reads as continuous natural landscape while remaining fully material underneath. The render mesh is a *view* of voxel data, never the authority for collision or mutation.

2. **Mutable everywhere (proven).** Dig and place operate on real substrate matter; mid-traversal carves remesh cleanly and cut faces read as cut earth. Without this proof, the artifact is indistinguishable from ordinary terrain engines.

3. **Deep Z is first-class.** Continuous vertical play from surface into walkable underground—not a decorative floor under a heightmap. Geology is real enough that digging down is honest.

4. **Generated world, not hand-authored mesh.** One curated seed/parameter set produces a reliable demo region. Lazy materialization and sparsity are load-bearing: the region must not need to fit in memory as raw voxels.

5. **Public API boundary from day one.** Nothing outside the substrate touches voxels directly. Harness tools call the same dig/place and query surface external games will use.

6. **Credible performance and persistence.** Interactive frame rate on the stated dev class, low dig-to-remesh latency, cold start to walkable, bounded GPU memory with streaming, and seed + delta save/load that restores edits. Benchmarks (scripted scene + machine profile) are part of the deliverable.

7. **Reusable foundation.** Generation and matter capabilities shipped in this slice are assets for later consumers—not throwaway demo code. Product One answers open substrate questions (e.g. voxel size, object-layer scaling) with measurements where possible.

Concrete demo content (exact biome list, material palette, ruin stamp, scripted route) is **harness/design detail**, not product identity. Outcomes above must hold; authored postcard specifics are deferred to design.

---

## Non-goals

Explicitly out of current scope:

- Game rules, combat, stats, AI, and entities beyond the harness player
- The System, LLMs, spells, gas, pricing, and intent
- Building-as-gameplay (UI, blueprints-as-gameplay, work orders, mechanisms, room economics)—even where the substrate may later support construction
- Full matter simulation beyond the Product One slice: fluid flow, fire/CA ecology, structural integrity, granular settle, tree felling / rigid conversion (format may reserve fields; nothing runs these as product)
- Weather, seasons, and growth simulation (a fixed time-of-day control is enough for the demo)
- Multi-slot / versioned persistence; multiplayer
- Implementing any future game named only as motivation

Future products and the full substrate architecture are **context**: they justify mutability, deep Z, sparsity, meshing quality, and a clean API. Their gameplay, content, characters, assets, and implementation are **not** imported into current scope.

---

## Unresolved questions for humans

Seeds agree that the **substrate** is the product and the walkable world is **harness / first proof**, not a game. These items would still change delivery bar or success criteria if answered differently:

1. **Harness required vs. permitted.** `project-boundary.md` says the repository *may* include a walkable executable; Product One and the seed index treat that harness as part of the binding first slice. Is a walkable validation executable a **required** repository delivery for Product One, or only **permitted** if useful?

2. **Public artifact intent.** Product One describes milestone posts and a downloadable demo for an external audience. Is a shippable public demo required, or are internal harness + benchmarks enough?

3. **Binding acceptance platform.** Performance targets cite an M4 Mac Mini (wgpu/Metal) and provisional discrete-GPU numbers. Is M4 the binding acceptance platform for Product One, with discrete targets explicitly provisional?

No seed conflict blocks product identity: all sources name the substrate as Moria and keep games out of this repository.

---

## What each seed contributed

| Source | Contribution |
|---|---|
| **README.md** | Names Moria as a GPU-resident voxel-world substrate Rust crate; positions the walkable executable as harness, not game. |
| **docs/seeds/README.md** | Seed authority: Product One is binding for this milestone’s implementation and harness; substrate doc is architecture reference (only selected portions required); project-boundary is operator clarification that broader game/System/building intent is out of scope. |
| **project-boundary.md** | Locks consumer boundary (crate vs game), workspace separation, harness-through-public-API rule, and explicit exclusion of game/System/LLM/spell/gas/combat/AI/building layers. |
| **product-one-seed.md** | Defines the first product-shaped slice: material-world claim, dig-as-proof, deep traversal, generation/sparsity, API discipline, performance/persistence bar, and non-goals. Concrete seed content and milestone sequencing are design inputs, not re-stated here as vision inventory. |
| **voxel-world-substrate.md** | Long-horizon capability context: smooth meshing over voxel truth, brick sparsity, geology-first gen, deep Z, streaming/deltas, layered crate stack, verb/query discipline, multi-game reuse. Full layers (fluids tier 2+, integrity, building, entities/nav, weather, System hooks) remain reference for *why* seams exist—not current deliverables. |

---

## Summary for approval

**Build:** a reusable Rust voxel-world substrate (Product One slice) plus a public-API walkable validation harness.

**Prove:** a generated natural region that is continuous, deep, mutable, credible in look and performance, and clean enough at the API boundary to trust as the foundation for later games.

**Do not build:** those games, or the simulation and content layers that only they require.

Approve, amend the unresolved questions, or reject with a revised product identity before design proceeds.
