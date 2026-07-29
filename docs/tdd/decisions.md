# Technical design decisions

This record preserves human feedback received during technical-design review
and the technical interpretation applied to the TDD. Verbatim feedback is kept
separate from interpretation. Product authority remains in
[`../design-document.md`](../design-document.md).

## Human review entry

### Verbatim feedback

```text
Is this as simple as it can be while still satisfying the requirements? If yes, leave the TDD unchanged. If no, revise the TDD to make it the simplest sufficient design.
```

### Technical decision or clarification

The TDD was not yet as simple as it could be. The two-package Cargo workspace
did not provide a necessary isolation boundary: one Cargo package can build a
public `moria` library crate and a separate `moria-qualify` binary crate. Rust's
target privacy still prevents that binary from accessing the library's private
or `pub(crate)` implementation, so it remains an external-style consumer of
the public facade.

The repository contract is therefore simplified to one root package with one
library target and one qualification binary target. No canonical, GPU,
rollback, persistence, collision, participant, or validation mechanism is
removed: those mechanisms are the minimum selected implementation of explicit
approved requirements rather than optional product scope.

### Unresolved question

None.
