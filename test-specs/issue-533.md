# Issue 533 — Lint canonical Rust source boundaries

References: `validation.md` TECH-060 Rust source audit and `architecture.md` TECH-071; issue M-125.

## Properties

- For every discovered canonical Rust module and transitive in-scope expression, only the approved integer fixed-math boundary may influence canonical arithmetic.
- Discover every in-scope canonical Rust module and parse with `syn = "=2.0.106"`; fail closed on unreadable/module-discovery/parse errors.
- Reject float types/literals, implicit scalar casts, libm/transcendental calls, and canonical arithmetic paths bypassing `moria-fixed-v1`.

## Positive and negative configurations

- Positive fixtures include approved fixed integer types/helpers and the separately named one-way presentation conversion that requires explicit format and cannot return canonical type.
- Negative fixtures place `f32/f64`, float suffixes/literals, transcendental calls, alternate fixed helpers, implicit conversions, and forbidden calls in direct and transitive canonical modules.
- Include aliases, generic types, macro-expanded/qualified paths to the extent the selected Rust AST exposes them; uninspectable generated code fails closed unless its checked generated artifact is in the approved inventory.

## Scope/error paths

- Custom logic implements only TDD boundary rules; no regex/tokenizer/home-grown parser/general validator.
- A lint pass cannot substitute for arithmetic oracle or GPU execution evidence.
- Adding a canonical module without inventory inclusion or an allowed-call entry without TECH authority fails.
