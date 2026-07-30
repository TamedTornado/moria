# Issue 514 — Exercise base-content and provider registration headlessly

References: `validation.md` TECH-060 base/provider fixtures; issue M-119.

## Properties

- For every admitted provider request, capacity and lifetime ownership must be reserved before invocation and released exactly once after its terminal path.
- Before every source callback, exact request, 2,048 payload bytes, callback cell, diagnostic, and operation permits are reserved.
- Exactly one terminal completion is accepted; every terminal path eventually releases all fixed resources.

## Callback configurations

- Exercise exact brick in one/many writes, short finish, long write, valid/invalid uniform, invalid cell/digest, bounded failure, producer drop, panic, cancellation during active copy, duplicate, late, and old-generation completion.
- Prove no provider-owned `Vec`, writer, error chain, panic payload, map, or unbounded diagnostic is stored by Moria.

## Registry configurations and errors

- Call every builder registration method with valid descriptors.
- Within each registry, duplicate IDs reject without replacement. Freeze rejects missing/wrong-kind source, base authority, content store, input source, checkpoint store, replay sink, participant kind, and provider descriptor identity/contract/bounds.
- No callback/page allocation occurs for freeze failures.
- Explicit retry uses a fresh request/cell/ID; timing never selects content and failure never becomes empty matter.
