# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**: a Rust crate (or small family of crates) that owns natural-looking, fully mutable volumetric worlds so downstream games can dig, build, stream, and query continuous 3D matter without embedding their own world engine.

## Purpose

Games and tools need a shared foundation where the world *is* material truth—not decorative terrain over a heightmap—and still reads as ordinary landscape. Moria exists so adventure, fortress, sandbox, and other consumers can share one mutable world layer, with game rules, content, and presentation policy living above it.

## Product boundary

**In this product:** the substrate that generates, stores, mutates, streams, and presents voxel worlds through public engine interfaces—matter, geology-first generation, smooth terrain views over voxel truth, mutation verbs and queries, and seams games need without implementing game systems.

**Not this product:** the eventual game(s). Combat, the System / LLM, spells, gas economy, AI, building-as-gameplay, authored campaigns, and game UX stay downstream. A walkable-world executable, if present, is an **adjacent consumer**: a validation harness that must call the same public APIs an external game would use, not a privileged game layer or the product identity itself.

**Harness vs substrate:** harness-owned means character control, camera, demo route, seed-region art direction, debug UX, and acceptance scenes. Substrate-owned means the reusable world capabilities those consumers exercise.

## Future products and enabling implications

Described future consumers include a System-driven ARPG, a Dwarf Fortress–style fortress/colony mode, a Moria-style descent experience, and pure sandbox tools. None are current deliverables.

**Enabling implications (substrate altitude only):** worlds that look natural yet remain mutable everywhere, including deep underground; continuous 3D play rather than a flat floor; clean layering so multiple games share matter, mutation, and queries without LLM or genre rules inside the crate.

## Non-goals

- Implementing game rules, entities-as-gameplay, combat, AI, spells, gas/pricing policy, or the System/LLM.
- Owning game content, controllers, cameras, or presentation policy beyond substrate views of matter.
- Treating a marketing demo, specific seed postcard, or benchmark theatre as the product definition.

## Confirmed vision constraints

- Identity and repo boundary: reusable substrate as Rust crate(s); the actual game is a separate downstream consumer and not part of this product’s scope.
- Any in-repo walkable-world binary is only a validation harness and must consume public substrate interfaces (workspace separation is required; crate split is design).
- Substrate stands alone with **zero LLM dependency**; System hooks are consumer-side if added later.
- Game rules live above the substrate; compatibility seams may be designed, not filled with those layers here.

## Assumptions proposed for approval

1. **Natural look is substrate responsibility at outcome level** (smooth, non-cube terrain over voxel truth; interactive matter including vegetation-scale objects where the substrate assigns them)—not deferred entirely to each game’s renderer.
2. **Long-horizon matter behaviors** motivated by fortress/ARPG fantasy (richer fluids, structural integrity, fire/weather ecology, full building/mechanism stacks) are **enabling implications**, not a committed current roadmap, until design prioritizes them after vision approval.

## Questions for human review

**Q1.** Is an in-repo walkable-world validation harness a **required current deliverable**, or only **permitted** adjacent to the substrate crate(s)?

- **Proposed safe answer:** Permitted and encouraged for proving public APIs, but not mandatory for product identity—the crate substrate is the product; harness controller, character, seed content, demo route, and performance theatre stay out of substrate scope.
- **If answered differently:** Making the harness mandatory keeps product identity on the substrate but expands **delivery boundary** to require a shippable walkable consumer; treating the walkable demo *as* the product would replace the crate-substrate identity with a single-player tech demo and conflict with the explicit project-boundary correction.

## Seed synthesis

- **README.md** — Named the product Moria; stated GPU-resident reusable substrate consumed as a Rust crate; framed the walkable-world executable as separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Binding boundary: product = substrate crate(s); game out of repo; harness only via public APIs; workspace consumer split required; game/System/LLM/spell/gas/combat/AI/building layers out of scope (seams allowed, implementation not).
- **docs/seeds/product-one-seed.md** — Motivated a first walkable proof (generated region, third-person run, dig/place as mutability proof) and many demo/performance/content details treated here as **harness or downstream design**, not vision inventory; reinforced non-goals aligning with boundary; contributed the tension settled in Q1 (demo-as-product-shaped delivery vs substrate-as-product).
- **docs/seeds/voxel-world-substrate.md** — Supplied substrate identity and purpose (natural-looking mutable worlds, deep-Z, substrate-not-game, multi-game reuse, GPU-resident matter); informed enabling implications and non-goals; design mechanisms, layers, milestones, and open technical questions deferred past vision.
