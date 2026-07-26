# Moria — Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a
Rust crate (or a small family of tightly scoped crates). Games consume it; this
repository does not ship a game.

The product has two nested scopes that must not be collapsed:

1. **Enduring substrate mandate** — the full material world layer the crate exists
   to provide: normal-looking voxel truth, universal mutability, deep Z, matter
   behavior, physics coupling, queries, and mutation, with public verbs so nothing
   above the matter layer touches voxels directly.
2. **Product One (first delivery slice)** — the first milestone that *proves* that
   mandate under real pressure: one curated generated region, smooth mutable
   terrain, streaming and persistence, dig/place as mutation proof, and a
   third-person avatar that collides against voxel occupancy—not the render mesh.

**Product One’s walkable-world executable is required** for this first delivery.
It is a **validation harness and product-shaped demo**, not the product and not a
privileged game layer: it must use only the same public substrate APIs an
external game would use. The general project boundary allows a harness; Product
One *binds* the repository to ship that harness so the proof families below can
be demonstrated end-to-end.

Downstream games (sandbox, fortress/colony, ARPG, descent-style play) are
**consumers**, not deliverables here.

---

## Purpose

Make one claim undeniable: the world is **fully material voxel truth that still
reads as a normal natural landscape**, not a heightmap with props.

The substrate exists so those future products can start from a walkable, fully
mutable, deep-Z world instead of a whiteboard—and so every open fidelity/scale
tradeoff can be decided with measurements rather than guesses.

---

## Boundary

| In repository | Out of repository |
|---|---|
| Generation, matter representation, meshing/dressing, streaming, persistence | Any full game, campaign, or content IP |
| Public mutation/query API (verbs + mirror; nothing touches voxels directly) | Game rules, combat, stats, economy |
| Walkable Product One harness as *required* consumer of public APIs | LLM / “System”, spells, gas pricing, intent |
| Compatibility *seams* only where substrate requirements demand them | Building *gameplay* (UI, work orders, mechanisms, room economy) |
| Cargo workspace separating crates from harness | AI agents, multiplayer *product*, game-authored weather/growth content |

The harness is required to exercise, as substrate proofs: traversal, dig/place
mutation, natural dressing and voxel objects, static water, reusable
placement/stamping, deep geology, streaming, persistence, and performance. It
must not grow into fortress mode, ARPG mode, or any other game layer.

Thin ambient world behavior and voxel-object growth belong to the **substrate
mandate** (even when deferred past Product One). Only *game-authored* rules and
content for weather, seasons, growth, and similar systems are excluded by the
project boundary.

---

## Required product-level outcomes

Outcomes the product must make true—not an implementation checklist. Where
Product One narrows the first proof, that is called out explicitly; deferred
matter behaviors remain part of the current product mandate, not permanent
omissions.

### Substrate mandate (enduring)

1. **Reusable crate boundary** — External games and the in-repo harness share one
   public surface; no game-private paths inside the substrate.
2. **Looks normal, is material** — Continuous terrain that reads as hills,
   forest, water, rock, and underground space; the mesh is a regenerated view,
   never authoritative.
3. **Universal mutability** — Any voxel can be destroyed, moved, or placed,
   everywhere and all the way down. Nothing interactive is decorative geometry
   sitting outside the material world.
4. **Deep Z is first-class** — Continuous vertical play from surface into
   walkable underground; geology (strata, voids, materials) is real when cut or
   entered.
5. **Fused surface / object truth** — Interactive vegetation and objects remain
   voxel-backed. Non-voxel dressing (grass, clutter, similar surface read) is
   derived from and kept synchronized with voxel state—not an independent fake
   layer.
6. **Scale under sparsity** — Regions large enough that raw full-residency is
   impossible; streaming, homogeneous empty/solid bulk, and lazy materialization
   keep memory and cold-start honest.
7. **Truth vs. view separation** — Collision, queries, and edits run on voxel
   occupancy; rendering is derived and discardable.
8. **Matter, physics, queries, mutation** — The substrate provides material
   behavior and the public verb/query surface for games. Scalable matter
   behaviors—flowing fluids beyond static bodies, fire/CA, structural integrity,
   granular settle, dynamic object/rigid coupling, thin ambient weather, and
   voxel-object growth—are part of this mandate even when not first-shipped.
9. **Persistence of mutable world state** — World truth is generation plus edit
   deltas, including mutable object/entity state (not only terrain scars), with
   streaming and cross-run reuse of those deltas as the long-term model.
10. **Credible performance** — Sustained interactive frame rate and dig-to-remesh
    without hitch on stated mid-tier targets; memory and save-size discipline
    under streaming. Measurable benchmarks are part of what “done” means.
11. **Portable GPU stack** — Load-bearing GPU work stays on portable compute
    (wgpu/WGSL); design respects platforms without 64-bit buffer atomics and
    treats bandwidth as a first-class constraint.

### Product One (first delivery slice)

Product One proves the mandate under real sparsity and interaction pressure; it
does not redefine the product as a smaller world.

- **Required harness proofs** — A walkable third-person route demonstrating
  traversal (including continuous surface-to-underground Z), dig/place mutation
  as the mutability proof (not full destroy/move/place coverage), natural
  dressing and placed voxel objects, static water bodies, one reusable
  stamp/prefab placement path, deep geology honesty when cut, streaming,
  seed-plus-deltas persistence (single save slot), and the performance claims
  above.
- **Matter slice shipped first** — Generation full for the curated region;
  brick pool, homogeneous sentinels, lazy materialization, meshing, dressing,
  voxel-object placement/registration/render, static water, and dig/place verbs
  with mirror queries. Full fluids, fire/CA, integrity, granular settle, and
  object felling/rigid conversion do not run in Product One (format and seams
  may anticipate them).
- **Persistence slice** — Product One is limited to reload of the same seed +
  deltas; full object journals and cross-run multi-mode reuse remain later
  substrate delivery, still under the mandate above.

---

## Non-goals

- Implementing a game (rules, progression, combat, quests, multiplayer product).
- System / LLM features, spells, gas metering, or intent pipelines.
- Building-as-gameplay: blueprints-as-work-orders, mechanisms, room detection,
  labor agents (placement/stamp as *substrate* capability is in; fortress UI
  and economy are not).
- Authored open-world content, characters, factions, or story assets beyond a
  single curated **demo seed** used to prove the substrate.
- Native Metal (or other API) forks in load-bearing layers.
- Treating the walkable demo’s scenery, route, or debug keys as product IP
  rather than harness proofs.
- Shipping full fluids, fire/CA, integrity, granular settle, dynamic felling,
  ambient weather, or object growth **in Product One**—these are deferred
  substrate deliveries, not excluded from the product mandate (see outcomes).

---

## Unresolved human questions

Seeds agree on product identity, Product One as first slice, and the harness as
required proof for that slice. One boundary question remains for human approval
because it changes what “substrate complete enough for future games” means:

1. **Server-authoritative / multiplayer readiness as a compatibility
   constraint** — The shared verb/query boundary is already the sandbox and
   multiplayer-readiness seam by design. Implementing a multiplayer product
   stays out of scope. Should keeping the substrate **server-authoritative-ready
   by construction** (command/mirror architecture, no direct voxel writes above
   the matter layer) remain an explicit required compatibility constraint in
   scope statements, even though no multiplayer product is built here?

Downstream design or capability-depth choices (voxel size fidelity/cost,
distant LOD/impostor policy, object-registry scaling thresholds) are measurement
and design problems for Product One and later slices—not product-approval
questions. Discrete-GPU performance baselines are provisional delivery status
until re-baselined; they do not change product identity.

---

## What each seed contributed

| Seed | Role in this vision |
|---|---|
| **`README.md`** | Names the product (Moria), crate-first packaging, and harness-as-consumer stance. |
| **`project-boundary.md`** | Binding identity: substrate is the product; game is downstream; workspace boundary is mandatory; game/System/LLM/spell/gas/combat/AI/building layers stay out. Permits a harness; does not alone require Product One’s walkable artifact. |
| **`product-one-seed.md`** | Binding *first slice*: walkable demo required as proof, Product One non-goals, dig/place as proof-not-gameplay, which matter layers ship vs. wait, seed-plus-deltas save limit, and performance/portable-GPU pressure. Curated region/route/palette and M4/Linux validation details are harness or engineering specifics—capability pressure only, not product identity. |
| **`voxel-world-substrate.md`** | Enduring mandate: normal look, universal destroy/move/place, deep Z, matter/physics/queries/mutation, fused voxel objects vs. derived dressing, thin ambient behavior and object growth, persistence including object journals/streaming/cross-run reuse, and the multiplayer-readiness open question. Future modes (ARPG, fortress, System hooks) justify seams; their gameplay, content, and full implementation are not current scope. |
| **`docs/seeds/README.md`** | Manifest authority: Product One is binding for the milestone; substrate doc is the long-term design reference; operator clarification that Moria is substrate-only. |

**Deferred beyond Product One, still in product mandate:** flowing fluids, fire/CA,
integrity, granular settle, dynamic object coupling (e.g. felling), thin ambient
weather, voxel-object growth, full object-state persistence and cross-run reuse.

**Omitted from repository scope on purpose:** fortress/ARPG/Moria-descent game
designs; System-authored content loops; multiplayer as a shipped product;
game-authored weather/growth rules and content (as opposed to substrate ambient
behavior); treating harness scenery as game IP.

**No seed conflict requiring a product-identity decision:** README, boundary, and
Product One align—Product One narrows *when* substrate capabilities ship, not
*whether* the product is the substrate. The open multiplayer-readiness question
is the remaining human boundary call called out above.
