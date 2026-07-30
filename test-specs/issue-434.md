# Issue 434 — Implement incremental Merkle commitments

References: `architecture.md` TECH-009; issue M-018.

## Properties

- For every hash domain, the digest is BLAKE3-256 over `"moria/v1/<domain>" || canonical_length || canonical_payload`.
- For all logically equal worlds, physical slot, cache, task, completion, mesh, telemetry, adapter, and driver differences cannot affect a canonical hash.
- For all canonical changes, the owning leaf and every required ancestor change unless the canonical payload remains byte-identical.

## Entity configurations

- Golden each domain: genesis, registries, base, brick, scar leaf, radix node, volume, simulation domain, allocator, participant/RNG, outcome, tick batch/state, and world root.
- Change matter, placement, IDs/allocator, simulation domain, participant/RNG commitment, placement format, or contract digest one at a time and require root sensitivity.
- Change surface assets/style, cache residency, presentation, timing, receipt ID, and adapter context one at a time and require root identity.

## Edge and error paths

- A one-brick change recomputes exactly the leaf plus its 26 radix ancestors and affected registry/world paths; unrelated volume hashes are not scheduled.
- Reject omitted configuration fields, unsorted registry children, wrong domain tags/lengths, digest type confusion, and map-order-dependent input.
- Use the maintained `blake3` implementation; no alternate hash or custom BLAKE3 is conforming.
