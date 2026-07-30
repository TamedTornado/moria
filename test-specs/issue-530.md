# Issue 530 — Fuzz bounded canonical and persistence decoders

References: `validation.md` TECH-059 decoder fuzzing; issue M-113.

## Properties

- For every generated decoder input, execution must terminate within declared allocation/time bounds and either produce the exact valid value or the specified typed rejection.
- For arbitrary input within declared test bounds, canonical, checkpoint-manifest, checkpoint-replay, replay-record, and active-history public decoders terminate without panic/timeout and allocate no more than declared limits.
- Accepted data round-trips byte-identically; rejected data never partially publishes or mutates world/store/log state.

## Generated configurations

- With `proptest = "=1.7.0"`, mutate every byte boundary: truncation at all offsets, appended trailing bytes, invalid/unknown tags, count/length one below/equal/above available/max, duplicate/unsorted records, checksum/digest corruption, and allocation arithmetic wrap.
- Include compressed declarations that would exceed output caps and retained minimal regression cases for each decoder family.

## Error paths

- Each malformed family yields its specified bounded public error/layer; no silent field ignore, partial sequence, decompression bomb, or generic panic is accepted.
- The fixture uses existing public decoders/loaders only and does not create a common parser, alternate format, or general validation framework.
- Failure to discover/invoke one named decoder or loss of a retained regression invalidates the artifact.
