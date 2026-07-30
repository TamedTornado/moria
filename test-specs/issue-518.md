# Issue 518 — Compile the complete external facade contract

References: `validation.md` TECH-060 external facade slice; `interfaces.md` TECH-070; issue M-123.

## Universal properties

- For every normative public callable/type, an external consumer must be able to construct, invoke, inspect, or pattern-match it exactly as defined, while every nonpublic capability remains unreachable.

## Public compile/use configurations

- A separate external-style crate imports only `moria` and constructs/calls every TECH-070 method, request/result/error, receipt, subscription, provider, CPU participant, and GPU participant public shape.
- Construct/fill/read/iterate/consume every bounded owner at empty/exact/overflow capacities and prove value recovery on every failure.
- Round-trip all private-field IDs/digests/keys/lineages/counters/placement types through their exact constructors/accessors.

## Typed outcome assertions

- Pattern-match every `AdmissionCode` with only its legal context and exact fields.
- Observe every query blocker/unavailability/capacity path, three telemetry variants, and all structured `FailedNoAdvance` causes/identities/retry policies.
- Compile-fail fieldless/prose surrogate error use.

## Closure/error paths

- Reject zero/high-bit boundaries per ID type, including full nonzero RngStreamId; reject all-zero stream key and invalid placement format.
- Compile-fail tuple constructors, unchecked/cross-domain/float conversions, private imports, test-only features, internal buffers/storage handles, unbounded owners, and missing named-type ownership.
- Provider traits can inspect only their declared bounded views/bytes and cannot obtain Moria allocation/storage internals.
