# Netcode Feasibility Ladder

Status: research / feasibility. This document does NOT belong to the Moria TDD and adds no Moria product scope. It defines the spikes that test the rollback-netcode thesis that motivated Moria's deterministic-simulation amendment. Rungs 0a/0b (kernel audit, cross-GPU kernel spike) live in the Moria amendment and are not duplicated here.

## Thesis under test

Deterministic fixed-point simulation with rollback (predict remote inputs, rewind to confirmed tick on misprediction, resimulate, blend corrections) makes the shared-physics-object problem — up to and including multi-player persistent constraint chains ("two grapples on one barrel", "barrel → helicopter → carrier") — a budgetable engineering problem rather than an unsolvable authority conflict.

Known-unproven rung in the industry: persistent multi-actor *constraints* under rollback. Impulse contact (Rocket League), rollback fighting games (GGPO), deterministic rollback over many bodies (Photon Quantum / Stumble Guys), and large deterministic sims (Factorio) are each proven separately; the conjunction is not.

## Dependencies

None on Moria. Rungs 1–2 are standalone: a deterministic fixed-point physics loop, a snapshot ring buffer, a simulated network layer, a hash function. No renderer, no engine, no real networking. The eventual production integration consumes Moria's deterministic contract (canonical TickBatch, rollback roots, hierarchical hash, replay artifact, rollback-participant contract), but the spikes consume nothing from Moria and can run in a parallel lane. Either track can fail without invalidating the other:

- Rung 1 fails → Moria determinism still pays rent (replay debugging, undo, conformance oracles).
- Rung 0b fails → rung 1 still answers whether the thesis survives on a CPU-authoritative variant.

## Common harness requirements

- Fixed-point (or qualified enhanced-determinism) physics; sim state fully snapshot/restorable.
- Tick-stamped inputs; totally ordered input stream; confirmed frontier = highest tick with all inputs.
- Prediction rule: remote input at tick t+1 = last confirmed input (document any smarter predictor separately; it changes the histograms).
- Fake network layer: per-peer one-way latency (asymmetric), jitter, and loss are scriptable per scenario.
- All thresholds and kill criteria declared in the scenario file BEFORE the run. No post-hoc goalposts.
- Every run emits a replay artifact (initial state + input log + expected hash sequence).

## Oracles (all machine-checkable, no LLM judges)

1. **Determinism / desync tripwire.** Bit-exact state hash match across peers at every confirmed tick. Any mismatch = correctness failure, run invalid, replay attached.
2. **Rollback replay identity.** Rewind N ticks, resimulate with identical inputs → hash sequence identical to original. Tested for several N each run.
3. **Resim cost.** Wall time to restore + resimulate N ticks, measured per N per scenario. Output: curve of achievable rollback depth per frame budget on declared hardware.
4. **Correction magnitude.** On each misprediction correction: position delta, velocity delta, and angular delta of every body, recorded as histograms per latency tier and per scenario. This is the feel proxy — the only oracle touching "feel," reduced to distributions.
5. **Constraint stress.** Peak constraint force, stretch distance (for compliant tethers), and break events per correction.

## Rung 1 — Two grapples, one body (thesis falsification)

**Setup.** One process, two simulated peers. One deterministic physics island: one shared dynamic body, two persistent grapple constraints (one per peer's avatar/anchor), scripted adversarial input streams (tug-of-war: sustained opposing forces, mid-pull direction reversals, attach/detach at worst-case ticks).

**Network scenarios.** Peer latencies {10/10, 50/100, 100/200, 150/300} ms one-way, each with 0 and 3% loss, ±20ms jitter.

**Measurements.** Oracles 1–5. Special attention: correction magnitude at the attach tick (impulse-heavy, worst case) vs. during sustained pull.

**Kill criteria (declare exact numbers before running; placeholders here):**
- Any oracle-1 or oracle-2 failure that survives debugging = harness invalid, fix before proceeding.
- At 150ms one-way, if p95 position correction on the shared body exceeds [X meters] with zero input delay AND still exceeds [X'] with 3 ticks of input delay on grapple actions, the naive thesis fails → finding: attach requires animation cover / input delay; record where the wall is.
- If restore+resim of 20 ticks exceeds one 60Hz frame interval on the reference machine for this single-island workload, the budget thesis fails at the simplest possible scale.

**Pass →** proceed to rung 2. **Fail →** publish the histogram and the wall coordinates; that is still the finding.

## Rung 2 — Adversarial constraint chains

**Setup.** Four simulated peers. Constraint graph fuzzing over:
- chain depth: 2 → 6 links (avatar → barrel → helicopter → carrier-class body)
- mass ratios across the chain: 1:1 up to 1:10⁴ (solver conditioning stress; also exercises fixed-point range + origin rebasing)
- player attachment count per island: 2 → 4
- latency tiers and loss as rung 1

**Mitigation knobs (the sweep dimensions — these are design levers, not hacks):**
- Compliant (XPBD-style) tethers with tunable stiffness: softness localizes misprediction into cable stretch instead of propagating rigidly through the chain.
- Break-force limits: designer-controlled island decomposition; chains exceeding a force budget snap (also the Just Cause fantasy).
- Tether-count caps per island (single-player Just Cause shipped with caps; precedent exists).
- Per-island adaptive input delay: 0 delay solo; +2–3 ticks of input delay for actors inside a multi-player constraint island (fighting-game hybrid), shrinking the misprediction window where corrections are most visible.
- Origin-rebased local coordinates per island (fixed-point dynamic range at carrier scale).

**Question the fuzzing answers.** At what (chain depth × mass ratio × player count × latency) does the correction histogram exceed what stretch-and-snap can absorb — with and without each mitigation knob? Output is a phase map, not a verdict.

**Kill criteria.** As rung 1, plus: if no knob combination brings p95 far-end correction under [Y] at 100ms with a 4-link chain, the full Just Cause fantasy is out of budget → finding: viable envelope is [recorded phase map]; game design must live inside it.

**Additional oracle for rung 2.** Resim cost now scales with island body count — record cost vs. island size; this number feeds the netcode layer's max-rollback clamp and Moria's rollback-budget qualification threshold (they should agree on the reference adversarial workload).

## Rung 3 (sketch only — do not plan yet)

Production integration: the rung-2 scenario running over Moria's deterministic contract with real physics-adapter rollback participation, plus deterministic interest management / simulation-domain membership at open-world scale. Blocked on: amendment sealed, rung 0b passed, rungs 1–2 passed. Session-scale player counts assumed; MMO-scale interest management over deterministic sim is explicitly out of scope for this ladder.

## Deliverables per rung

1. Scenario files with pre-declared thresholds (checked into repo before first run).
2. Replay artifacts for every failure.
3. Histograms + resim-cost curves per scenario.
4. A one-page findings note: pass/fail per kill criterion, wall coordinates if any.

## Sequencing

Rung 1 can start immediately in a spare lane; it needs no Moria code and no amendment outcome. Rung 2 starts only on rung-1 pass. Rung 3 is not planned until the Moria amendment path (0a/0b) and rungs 1–2 have all resolved.
