# Issue 512 — Prove genesis and query readiness boundaries headlessly

References: `validation.md` TECH-060 genesis/query fixture; issue M-117.

## Properties and lifecycle

- For every observed frontier or readiness state, the public encoding, next-tick eligibility, blocker, and terminal result must describe that exact state without a sentinel or empty substitute.
- Genesis ready is exactly `FrontierPosition::Genesis`, next tick zero, no confirmed-zero rollback/outcome, and replay header `{starting: Genesis,next_tick:0}`.
- Confirming batch zero yields `Confirmed(0)` and next tick one. Admission before/after classifications are exact on both sides.
- Encoding, hashing, query/collision facts, CPU/GPU participant metadata, and collider artifacts distinguish Genesis from Confirmed(0) byte-for-byte.

## Query state matrix

- For Complete queries, expose cold ranges, materializing ranges, unmet revisions, and resource pressure as exact pending blockers.
- Failed truth and retained-frontier age terminate availability; post-admission overflow terminates structured result-capacity failure.
- ReturnStale reports exact unmet pairs; Wait never pins a too-old retained frontier indefinitely.

## Pin/failure paths

- Withdraw interest while query/collision/artifact pins remain; truth stays live until final pin drains, then retires.
- Missing/cold/corrupt content is never reported empty/no-hit.
- Missing renderer yields `BackendUnavailable` for GPU-dependent work while pure headless frontier/admission tests remain valid.
