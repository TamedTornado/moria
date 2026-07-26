# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate. It is delivered as public crate interfaces for multiple downstream consumers (games and validation tools). This repository delivers the substrate, not any particular game.

## Purpose

Moria exists so many independent consumers can create, stream, observe, edit, extract surfaces from, persist, and diagnose voxel worlds through one shared public contract—without reimplementing world machinery or depending on privileged internals.

## Product boundary

**In product.** World create and identity; deterministic seed-based generation; sparse voxel material truth; bounded region request and streaming; readiness and lifecycle observation; material query; bounded mutation; surface extraction as derived (non-authoritative) output; persistence of authoritative material state; read-only diagnostics. All of this is exposed only through public crate interfaces.

**Adjacent, not product identity.** Headless fixtures and a minimal validation executable are program deliveries that exercise those same public interfaces only. They may demonstrate render-and-edit through the public API and may include harness-local presentation (for example a free-fly camera), but they are not a game prototype and own no privileged world path. Their specific controllers, content, presentation, workloads, platforms, and performance gates are not product scope.

**Out of product / later consumers.** Games, demos, and Product One (a separate future repository) own gameplay, rules, characters, animation, authored levels, production content, UX, and game-specific policy. Interface pressure from possible consumers does not move that ownership into Moria.

## Required product outcomes

- Multiple consumers integrate only through public Rust crate interfaces; no consumer reaches storage, meshing, or scheduler internals.
- A consumer can create a world whose identity combines format version, generation parameters, and seed; request bounded regions; observe readiness and streaming lifecycle; query readiness and bounded authoritative material observations (including registered objects that participate in queries without becoming game entities); submit bounded edit commands; and persist material deltas.
- Generation is deterministic for the same versioned parameters and seed. Mutations are commands with explicit bounds; admission failures and commit revisions are exposed on the public surface. Failures are typed and observable to public consumers.
- Streaming bounds resident work, exposes lifecycle states (requested, loading, resident, evicted, failed), and tags background results with generation identities so stale work cannot replace newer truth.
- Persistence restores the same authoritative material state. Derived meshes and diagnostics never become authoritative world state.
- Diagnostics report lifecycle and bounded work without exposing mutable internal handles.
- The substrate remains useful across multiple downstream consumers, not tuned to a single game’s content or controls.
- Headless fixtures cover generation, query, mutation, persistence, and lifecycle behavior. A small visual fixture shows that a relocated external consumer can render and edit through the public API. Performance is reported with machine identity; this vision establishes no machine-specific correctness threshold.

## Future products and enabling implications

After the substrate ships, a separate Product One repository may host a third-person explorer demo in a generated region. That game-facing work—controller, character, animation, curated routes, terrain/content presentation—is future-consumer scope only.

**Enabling implication (already covered by current outcomes):** Moria must remain a multi-consumer substrate that games can drive solely through public interfaces for generation, streaming, query, edit, surface derivation, and persistence. No Product One gameplay, assets, or presentation is in current scope.

## Non-goals

- Game rules, combat, inventory, AI, narrative systems, characters, animation, authored levels, or production content.
- Shipping any particular game, demo slice, or Product One content inside this repository.
- Privileged validation or game paths into substrate internals.
- Machine-specific performance pass/fail thresholds as product correctness.

## Confirmed vision constraints

- Delivery form is a reusable Rust substrate consumed through public crate interfaces.
- Correctness: deterministic generation for the same versioned parameters and seed; atomic commit of admitted mutations with public commit revisions; persistence restores authoritative material state; derived meshes and diagnostics are never authoritative.
- Isolation: external consumers, including validation, use only the public surface; no privileged world path.
- Streaming and failure: resident work is bounded; lifecycle is observable; background results carry generation identities; mutation admission failures and other failures are typed and observable to public consumers.
- Diagnostics: report lifecycle and bounded work without exposing mutable internal handles.
- Validation and reporting: headless and small visual fixtures exercise the public API; performance reports include machine identity without a machine-specific correctness threshold set by this product.

## Deferred design decisions

- Concrete public API shapes, encodings for world identity (format version, parameters, seed), command schemas, commit-revision representations, and readiness/lifecycle representations.
- Internal algorithms and layouts for generation, sparse storage, streaming policy, surface extraction, and persistence encoding.
- Crate and package structure (beyond the outcome that consumers have no privileged access).
- Fixture presentation details, workloads, and any numeric performance budgets or target environments.
- Implementation depth beyond the stated product outcomes, and delivery sequencing among the binding capability families (generation, storage, streaming, mutation, surface extraction, persistence, diagnostics). Whether those outcome families are delivered is not deferred.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md` — Names Moria as the reusable voxel-world substrate whose only repository deliverables are current substrate commitments; marks the interface reference as non-scoping support.
- `docs/seeds/mixed-project-brief.md` — Binding current-product identity, public boundary, correctness and validation commitments, non-goals, and explicit separation of the later Product One consumer vision from Moria scope.
- `docs/seeds/substrate-interface-reference.md` — Supporting public-surface outcomes (composite world identity, bounded material query, mutation bounds/admission/commit revisions, streaming states, delta persistence, registered objects, diagnostics lifecycle without mutable handles) without adding deliverables or expanding product scope.
