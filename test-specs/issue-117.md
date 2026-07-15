# Issue 117 — Map physical controls to durable semantic intent

References: `docs/tdd/api.md §Player and camera action contracts`; `docs/tdd/systems.md §Input systems`; `docs/tdd/config.md §Input defaults`.

## Properties that must hold

- For every physical binding, the centralized ActionMap emits the documented semantic action/dead-zone/sensitivity value; gameplay/debug logic contains no device code.
- For every edge, zero fixed ticks preserves its latch, the first eligible tick consumes it once, and additional ticks cannot repeat it; continuous axes use current frame value.

## Entity configurations to test

- Keyboard/mouse/gamepad mappings; dead zone just below/at/above 0.15; mouse and full-stick orbit; zero/one/multiple fixed ticks.
- Slider unfocused, focused, pointer-dragged and focus exit with pending movement/jump/edit/save/load latches.

## Edge cases and type boundaries

- Unknown/malformed/duplicate binding fails config validation; a physical edge maps at most once per semantic action.

## Error paths

- Focused UI suppresses and clears gameplay/debug/camera capture while time/Tab remain; no latent action fires after focus ends.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

