# Issue 406 — Implement finite public owner types

References: `interfaces.md` TECH-070; issue M-004.

## Input validation

- `BoundedVec` and `BoundedBytes` accept capacities from zero through `u32::MAX` only when allocation succeeds; length greater than capacity returns the original `Vec`.
- `BoundedBytes64` accepts lengths 0 and 64 and rejects 65. `BoundedUtf8<N>` accepts valid UTF-8 of exactly `N` bytes and rejects invalid UTF-8 or `N+1` bytes without losing the original bytes.
- `OwnedBytes::try_from_vec` accepts exact `max_bytes`, rejects `max_bytes+1`, and exposes no spare growable capacity.

## Transformation correctness

- For all accepted values, `as_slice`, iteration, `len`, `capacity`, and consuming conversion preserve order and every byte/element.
- `try_push` at capacity returns exactly the rejected element. `try_extend_from_slice` that would overflow makes no partial append.
- Cloned/shared immutable bytes retain the same allocation semantics and exact length; no method can grow beyond admitted capacity.

## Edge and error paths

- Cover empty owners, zero capacity, exact capacity, one-over-capacity, allocation failure, and length/capacity arithmetic boundaries.
- Failed construction, push, or extension must not charge a Moria admission budget or mutate the source value.
- When an owner enters a facade/provider request, the accepting operation independently validates length and capacity and reserves its budget before ownership transfer.

These owners have no loading/rendering state; adding one would exceed TECH-070.
