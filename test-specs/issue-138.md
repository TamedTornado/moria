# Issue 138 — Wire persistence into the public world plugin

References: `docs/tdd/overview.md §Plugin composition`; `docs/tdd/api.md §Save/load protocol`; `docs/tdd/systems.md §Persistence systems`.

## Boundary contracts

- PersistencePlugin is installed in MoriaWorldPlugin for enabled/headless modes; public exports are exactly Save/Load request/result messages and read-only WorldTransactionState, never codec/store/staging/swap.
- Disk work occurs off fixed/render schedules; load rebuild acknowledgement precedes completion.

## Multi-system scenarios

- External consumer save then load through normal app schedules; observe Busy, Started, each Staging/SwapPending/Rebuilding phase and exactly one terminal.
- Enabled and presentation-disabled rebuilds; concurrent queries/movement in arbitrary consumer.

## Failure propagation

- Codec/I/O/identity/busy/rebuild failures propagate typed results without private access or partial truth.
- Compile-fail external codec/store/swap imports; task polling cannot block fixed ticks.

## Ordering guarantees

- Plugin/order follows facade contract; save polling frame schedule, swap fixed boundary, rebuild/install/ack before terminal.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

