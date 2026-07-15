# Issue 155 — Prove the continuous playable route with placeholders

References: `docs/tdd/overview.md §Verification strategy`; `docs/tdd/api.md §Player and camera action contracts`; `docs/tdd/states.md §Demo state map`.

## Boundary contracts

- Thin demo uses semantic intent plus public WorldRead/WorldEditWrite/focus/save-load lifecycle only; no private store/generator/mesh privilege.
- Continuous route and signature operation use same collision, streaming and staged mutation pipeline as future consumers.

## Multi-system scenarios

- Controlled fixed ticks traverse every ordered route tag from cliff/ruin/cave to -40±2 m with no teleport; assert capsule/camera validity at each.
- Submit public 3 m signature dig/place, observe atomic committed truth, walk clear opening, and wait for terminal owner/dressing reconciliation; startup to Playing <5000 ms.

## Failure propagation

- Any route gap, private access, fallback, stale owner/dressing, premature completion or timing miss fails.
- Excluded gameplay systems/types (combat, inventory, quests, AI, spells, dynamic fluid) remain absent.

## Ordering guarantees

- WorldReady -> Playing -> scripted semantic intent; edit Accepted -> batches -> primary -> traversal -> terminal. Zero/multiple fixed ticks remain deterministic.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

