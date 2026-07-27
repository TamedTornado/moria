# Moria product design decisions

This record preserves human decisions made while reviewing the product design.
The approved `docs/product-vision.md` remains authoritative for product
identity, boundary, requirements, constraints, and non-goals. The standalone
design document states each current outcome in full; this record preserves the
review history rather than replacing design substance.

## Resolved decisions

### D1. Multi-target matter mutation completion

**Status:** Resolved by human product-design review.

**Question considered:** When one bounded matter-mutation command targets
multiple cells, may it report partial application, or must it commit as one
atomic public operation?

**Decision:** The command is atomic. All targeted matter changes become
committed together at one revision, or none of them commit. A consumer that
wants independently successful effects submits separate commands.

**Design consequences:** Admission and internal work may be staged, but
queries, collision, observations, persistence, and presentation never observe
a partially committed command. A failed admitted matter mutation has no
committed effect. Validation must force a multi-cell command to fail after
admission and confirm that no targeted cell, mutation revision, or intermediate
observation escaped.

**Boundary retained:** This selects consumer-visible semantics only. Staging,
coordination, rollback, and other mechanisms that provide atomicity remain
technical-design concerns.

## Open human questions

None.
