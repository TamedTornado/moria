# Issue 529 — Implement the post-admission atomicity scenario

References: `validation.md` TECH-064 scenario 4; `gpu-runtime.md` TECH-040; issue M-095.

## Universal properties

- For every injected prepublication candidate failure, the complete previously installed frontier bundle and all of its observable derivatives must remain unchanged.

## Multi-system sequence

- Configure public `ExecutionPolicy::Candidate` with exactly one `AfterBrickConstructionBeforePublication` fault for a named tick/order.
- Submit a valid multi-brick ordinary matter command that passes admission, resolves content/capacity, and constructs private changed bricks.
- Trigger the production diagnostic at validation step 9 and observe the normal no-advance cleanup.

## Properties

- Before/after failure, every targeted cell, volume revision, live root/hash, rollback deque, participant token/commitment, active log/replay position, observation ring, and presentation dirty set is byte-identical.
- Receipt is exact `FailedNoAdvance` with attempted tick/source frontier and `InjectedCandidateFailure`; no canonical outcome list is confirmed.
- Private allocations drain only after last GPU use and no artifact becomes current.

## Edge and error paths

- Fault plan in ReplayGrade, unknown tick/order, or unsupported stage rejects configuration/request.
- The one-shot fault does not fire on another command/tick and cannot write canonical bytes.
- Scenario imports only public candidate types; any storage mutation hook, private module, fake result, or authority-labeled candidate evidence fails.
