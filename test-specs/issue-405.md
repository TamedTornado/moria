# Issue 405 — Implement canonical identity and digest types

References: `architecture.md` TECH-005; issue M-002.

## Properties and transformations

- For all `MaterialId`/`VolumeId` raw values, zero is rejected; every nonzero value round-trips exactly.
- For all `ParticipantId` and `InputSourceId` values, only `1..=0x7fff_ffff` is accepted. For all `RngStreamId` values, only zero is rejected, including acceptance of `0x8000_0000` and `u32::MAX`.
- For every 16- or 32-byte identity/digest input, `from_bytes -> as_bytes/to_bytes` preserves every bit; equal wire widths do not permit cross-type conversion.
- For all counter types, zero and extrema round-trip. No physical slot, task, entity, or submission identity is constructible as a canonical/public identity.

## Boundary configurations

- Exercise zero, one, maximum valid, and first invalid raw values for every constrained scalar type.
- Exercise all-zero/all-one/alternating/high-bit byte patterns for `WorldId` and each digest; exercise all-zero rejection and nonzero acceptance for `ReplayStreamKey` through its owning TECH-041 contract.
- Allocate explicit genesis volume IDs at low and maximum occupied values; verify `next_volume_serial` is one above the maximum and exhaustion is typed.

## Invalid and error paths

- Duplicate genesis IDs, absent/retired references, zero-reserved values, range violations, and exhausted counters fail with their exact typed error and do not alter a registry.
- Compile-fail fixtures reject tuple construction, unchecked/lossy constructors, and implicit conversion between digest or ID types.
- Constructors/accessors allocate nothing and do not panic at any scalar/byte boundary.

Rendering states are outside TECH-005 and must not be introduced by this issue.
