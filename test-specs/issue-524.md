# Issue 524 — Check normative Rust public closure mechanically

References: `validation.md` TECH-060 mechanical closure; `interfaces.md` public type index; issue M-124.

## Properties

- Enumerate every Rust-tagged fenced block in every approved TDD input with `pulldown-cmark = "=0.13.4"`; parse each extracted snippet with `syn = "=2.0.106"`.
- Every public capitalized name has exactly one owning contract/index row; every struct field is unique; every facade request/receipt is connected; every budget reference names an exact TECH-017 group/field.

## Configurations

- Positive corpus is the complete current approved TDD inventory, including snippets split across files and generic/associated/standard-library types.
- Retained negative corpus includes duplicate fields, unresolved names, missing owner row/facade connection, public tuple constructor, unchecked conversion, private/test-only import, and exact stale aliases `max_log_ticks`, `max_log_bytes`, `recovery_replay_cap`, `PresentationState::Failed`.

## Error paths

- Unreadable input, incomplete document/block inventory, malformed fence extraction, Rust parse failure, unknown budget ordinal/name, or ambiguous ownership fails closed.
- Standardized discovery/parsing must not use regex/custom Markdown scanner/home-grown Rust parser.
- The checker is limited to TDD public-closure rules and must not become a general validator or alter approved source documents/digests.
