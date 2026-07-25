# Substrate interface reference

This reference supports the Moria program brief. It describes the reusable
surface that downstream consumers need and does not add deliverables.

- World identity combines format version, generation parameters, and seed.
- Queries return readiness and bounded authoritative material observations.
- Mutations are commands with explicit bounds, admission failures, and commit
  revisions.
- Streaming exposes requested, loading, resident, evicted, and failed states.
- Persistence records authoritative deltas rather than derived meshes.
- Registered objects can participate in queries without becoming game
  entities.
- Diagnostics report lifecycle and bounded work without exposing mutable
  internal handles.
