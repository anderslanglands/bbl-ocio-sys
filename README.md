Rust-sys binding for [OpenColorIO](https://opencolorio.org/) using [babble](https://github.com/anderslanglands/babble).

Binding generation is automatic using the included `bbl-ocio` submodule (make sure to clone recursively).

# Building

## Prerequisites
- OpenColorIO 2.2 (other versions may work but are untested)
- babble 0.4
- CMake 3.15+

To build, just make sure OpenColorIO and babble are in your `CMAKE_PREFIX_PATH` and run `cargo build` or `cargo run`