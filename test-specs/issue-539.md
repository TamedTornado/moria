# Issue 539 — Lint canonical WGSL source boundaries

References: `validation.md` TECH-060 WGSL source audit; issue M-126.

## Properties

- For every discovered canonical WGSL entry point and transitive helper, canonical arithmetic must use only the approved integer `moria-fixed-v1` path.
- Consume matching Naga 29 parsed/validated/reflected module graphs for every in-scope canonical WGSL module; fail closed on discovery, parse, validation, or reflection failure.
- Reject floating types/literals, transcendental builtins, and any canonical arithmetic/helper path bypassing `moria-fixed-v1`.

## Positive and negative configurations

- Positive fixtures cover approved integer/wide helpers, CORDIC tables, bounded scans, and explicitly noncanonical presentation modules outside the canonical boundary.
- Negative fixtures inject float scalar/vector fields, float literals/conversions, trig/sqrt/transcendental builtin alternatives, duplicate math helpers, and direct canonical arithmetic in both entry points and transitive helpers.
- Add a new undiscovered canonical module and require failure.

## Scope/error paths

- Custom analysis implements only TDD boundary rules and uses no regex/tokenizer/home-grown WGSL parser/general validator.
- A lint pass cannot stand in for Naga validity, contamination dataflow, or real-GPU semantic parity.
- Any exception must be named by current TDD authority; undocumented allowlists fail.
