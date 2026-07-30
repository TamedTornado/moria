# Issue 461 — Implement gap-aware observation subscriptions

References: `interfaces.md` TECH-025; issue M-027.

## Valid transitions

- Subscription starts at Now, OldestRetained, or a valid After cursor; `poll` yields Items, honest Gap, or Closed.
- After Gap, `request_observation_resnapshot` pins one retained root plus current tail, returns bounded summaries/query and `resume_at`; successful `resume` changes the cursor, then polling yields suffix or another Gap.
- `close`/drop is idempotent and permanently yields Closed.

## Invalid transitions and guards

- Reject empty kind filter, invalid/nonunique/oversized membership, unknown volume, wrong-stream/beyond-tail/backward/unproduced cursor, query outside membership/spatial filter, and resnapshot bounds beyond budgets.
- Failed/cancelled resnapshot leaves cursor unchanged.

## Lifecycle, filtering, and concurrency

- Move and retire a volume, reclaim old directory state, then poll historical records: matching uses stored append-time IDs/bounds/revisions only.
- Empty membership means no volume facts; future creation does not expand it. World events require `include_world_events`.
- Trigger independent count and byte overwrite; gap exposes last trustworthy/oldest/resnapshot frontier and correlation loss.
- Append while resnapshot runs; resume at captured cursor returns exact suffix or a second explicit gap, never hidden loss/duplication.
- Shutdown freezes tail, permits retained polling while draining, then closes all subscriptions without waiting for consumers.
