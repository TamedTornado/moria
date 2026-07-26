# Moria — Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a
Rust crate (or a small family of tightly scoped crates). Games consume it; this
repository does not ship a game.

A **walkable-world executable** may ship in-repo as a **validation harness and
product-shaped demo**. It must use only the same public substrate APIs an
external game would use. It is not a privileged game layer.

**Product One** is the first milestone that proves the substrate: one curated
generated region, smooth mutable terrain, streaming and persistence under real
sparsity pressure, dig/place as proof of voxel truth, and a third-person avatar
that collides against voxel occupancy—not the render mesh.

---

## Purpose

Make one claim undeniable: the world is **fully material voxel truth that still
reads as a normal natural landscape**, not a heightmap with props.

Downstream games (sandbox, fortress/colony, ARPG, descent-style play) are
**consumers**, not deliverables here. The substrate exists so those products can
start from a walkable, diggable, deep-Z world instead of a whiteboard—and so
every open fidelity/scale tradeoff can be decided with measurements.

---

## Boundary

| In repository | Out of repository |
|---|---|
| Generation, matter representation, meshing/dressing, streaming, persistence | Any full game, campaign, or content IP |
| Public mutation/query API (verbs + mirror; nothing touches voxels directly) | Game rules, combat, stats, economy |
| Walkable harness as consumer of public APIs | LLM / “System”, spells, gas pricing, intent |
| Compatibility *seams* only where substrate requirements demand them | Building gameplay (UI, work orders, mechanisms, room economy) |
| Cargo workspace separating crates from harness | AI agents, multiplayer product, weather/growth sims as gameplay |

The harness may exercise dig/place, static water bodies, a fixed time-of-day
control, and one stamp/prefab path as **substrate proofs**. It must not grow
into fortress mode, ARPG mode, or any other game layer.

---

## Required product-level outcomes

Outcomes the product must make true—not an implementation checklist.

1. **Reusable crate boundary** — External games and the in-repo harness share one
   public surface; no game-private paths inside the substrate.
2. **Looks normal, is material** — Continuous terrain that reads as hills,
   forest, water, rock, and underground space; the mesh is a regenerated view,
   never authoritative.
3. **Mutable everywhere that matters** — Dig and place change real voxel state;
   cut faces and scars remesh as cut earth, not decorative geometry.
4. **Deep Z is first-class** — Continuous vertical play from surface into
   walkable underground; geology (strata, voids, materials) is real when cut or
   entered.
5. **Scale under sparsity** — A region large enough that raw full-residency is
   impossible; streaming, homogeneous empty/solid bulk, and lazy materialization
   keep memory and cold-start honest.
6. **Truth vs. view separation** — Collision, queries, and edits run on voxel
   occupancy; rendering is derived and discardable.
7. **Persistence model** — World = generation seed + edit deltas; reload
   restores the same scarred world without saving untouched bulk.
8. **Credible performance** — Sustained interactive frame rate on the stated
   mid-tier / M4-class targets; dig-to-remesh without hitch; memory and
   save-size discipline under streaming. Benchmarks (scripted path + machine
   profile) are part of what “done” means for Product One.
9. **Portable GPU stack** — Load-bearing GPU work stays on portable compute
   (wgpu/WGSL); design respects platforms without 64-bit buffer atomics and
   treats bandwidth as a first-class constraint.

---

## Non-goals

- Implementing a game (rules, progression, combat, quests, multiplayer product).
- System / LLM features, spells, gas metering, or intent pipelines.
- Full cellular automata, fire ecology, flowing fluids beyond static bodies,
  structural integrity / cave-ins, or granular settle as shipped Product One
  behavior (format/hooks may anticipate them; they do not run here).
- Building-as-gameplay: blueprints-as-work-orders, mechanisms, room detection,
  labor agents.
- Authored open-world content, characters, factions, or story assets beyond a
  single curated **demo seed** used to prove the substrate.
- Native Metal (or other API) forks in load-bearing layers.
- Treating the walkable demo’s scenery, route, or debug keys as product IP
  rather than harness proofs.

---

## Unresolved human questions

Seeds agree on product identity and boundary. These remain for humans because
they affect Product One fidelity, not because the product is ambiguous:

1. **Voxel size** — 25 cm is the working assumption; 12.5 cm is the open
   fidelity/cost alternative (possibly region-varying). Product One is the
   measurement bed.
2. **Distant representation** — How far LOD / impostors may cheat vs. full
   meshing once a real camera policy exists (Diablo-lock is future-game, not
   harness).
3. **Object-layer scale** — At what vegetation/object count the object registry
   needs its own spatial acceleration (Product One places trees/rocks; capacity
   ceiling is still open).
4. **Discrete-GPU baselines** — Mid-desktop targets are provisional until
   re-baselined on the intended Linux box; M4 numbers are the enforceable
   development floor for now.

None of these change *what* Moria is—only how Product One is measured and
tuned.

---

## What each seed contributed

| Seed | Role in this vision |
|---|---|
| **`README.md`** | Names the product (Moria), crate-first packaging, and harness-as-consumer stance. |
| **`project-boundary.md`** | Binding identity: substrate is the product; game is downstream; workspace boundary is mandatory; game/System/LLM/spell/gas/combat/AI/building layers stay out. |
| **`product-one-seed.md`** | Binding *first slice*: walkable demo as proof, non-goals for Product One, performance/dev-platform outcomes, dig/place as proof-not-gameplay, and which substrate layers ship vs. wait. Curated region/route/palette details are harness content, not game scope—kept only as capability pressure (sparsity, geology, dressing, stamp path). |
| **`voxel-world-substrate.md`** | Architecture *reference*: long-term design goals (normal look, full mutability, deep Z, GPU-resident layering) that justify Product One outcomes. Future modes (ARPG, fortress, System hooks, full fluids/integrity/building/agents) define *why* seams and layering matter; their gameplay, content, and full implementation are **not** current scope. Only portions selected by Product One are required for this milestone. |
| **`docs/seeds/README.md`** | Manifest authority: Product One is binding for the milestone; substrate doc is reference; operator clarification that Moria is substrate-only. |

**Omitted from current scope on purpose:** fortress/ARPG/Moria-descent game
designs; System-authored content loops; fire/fluid/integrity milestones beyond
format readiness; tree felling as a required deliverable (stretch in Product
One only); multiplayer as a product; weather/season/growth simulation as
shipped behavior.

**No seed conflict requiring a product-identity decision:** README, boundary,
and seeds README align; Product One narrows the substrate reference rather than
redefining the product as a game.
