# Moria current-product boundary

Moria is a reusable Rust voxel-world substrate for external consumers. This
repository delivers substrate crates and a minimal public-interface validation
harness.

The harness may use a free-fly camera and diagnostic overlays to exercise the
same API available to another repository. It is not a game layer. Game rules,
characters, controllers, animation, authored routes, production assets, and
consumer-specific content are outside the current product.

The current product must be complete enough to generate, stream, query, mutate,
mesh, save, and restore an authoritative voxel world through public interfaces.
