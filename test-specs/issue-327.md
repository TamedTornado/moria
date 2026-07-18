# Issue 327 — Recovery PR #234: Handle long controlled fixed-tick runs

References: `docs/tdd/systems.md` §Schedule model and §Tests and fixed-time helper; `docs/tdd/overview.md` §Verification strategy.

## Properties that must hold

- For every caller-supplied tick count that the test process can represent and complete, `moria_world::testing::run_fixed_ticks(app, count)` must return only after exactly `count` `FixedUpdate` executions, regardless of how many `app.update()` calls Bevy requires.
- For all runs, fixed systems must observe 60 Hz virtual/manual time, rendered-frame scheduling may execute zero, one, or several fixed ticks per update, and no result may depend on wall-clock time or sleeping.
- For all long runs, tick accounting must not lose ticks through accumulated-time clamping, rounding, counter wrap, or a hidden one-update-per-tick assumption.

## Entity configurations to test

- A headless `MinimalPlugins` app with a fixed-tick counter at counts `0`, `1`, `2`, `60`, `61`, and `10_000`; the last value is an adversarial regression fixture, not a new product limit.
- A fixed system that records sequential tick numbers and another that mutates state once per tick; assert the sequence has no gaps/duplicates and final state equals the requested count.
- Preload virtual time so one app update can run multiple ticks, and separately require multiple app updates to reach the requested total.

## Edge cases

- `count == 0` must execute no fixed systems while leaving the app usable for a later nonzero call.
- Consecutive calls must add exactly their individual counts rather than resetting or double-counting the observer.

## Error paths

- If the observed counter deviates from the requested count, the helper must fail immediately with expected and observed totals; it must not silently return partial progress.
- The test must prove no use of `sleep`, real elapsed-time thresholds, or an opened window.

