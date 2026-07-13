# Moria

Moria is a reusable, GPU-resident voxel-world substrate intended to be consumed
by the actual game as a Rust crate. Its walkable-world executable is a separate
consumer and validation harness for terrain generation, streaming, meshing,
editing, collision, persistence, and performance—not a game layer.

The substrate inputs are preserved in [`docs/seeds/`](docs/seeds/).
