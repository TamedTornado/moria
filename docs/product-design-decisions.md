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

---

## Human review entry

### Verbatim feedback

```text
Multi-target mutation completion: must a bounded mutation affecting multiple cells commit atomically, or may the contract support explicitly reported partial application? This design requires no unreported partial success but does not choose between those public semantics.

Atomic commits seems like the better idea.

Also, a somewhat vague guidance to keep in mind: The GPU residency requirements are about performance. Our physics and other "behaviour" engines will primarily be on the GPU as well, when that is feasible. This engine will be correct first, and then relentlessly optimised.
```

### Decision and clarification

The human selected atomic public completion for a bounded multi-target matter
mutation. One command commits all targeted changes together at one revision or
commits none; explicitly reported partial application is not a supported
outcome.

GPU residency is a performance requirement, not an end in itself. Correctness
and the public truth contract take priority, followed by continuous measured
optimization. The generic behavior-extension boundary must accommodate
physics and other external behavior engines running primarily on the GPU when
feasible, without making those engines part of Moria or granting them ownership
of Moria's voxel storage.

### Unresolved question

None at product-design altitude. The mechanisms and measured thresholds for an
efficient GPU-oriented extension path remain technical-design and validation
choices.
