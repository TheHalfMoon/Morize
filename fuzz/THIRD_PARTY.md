# Fuzz-only third-party components

SG-000024 isolates all fuzz tooling and dependencies from Morize runtime artifacts.

## cargo-fuzz

- Project: rust-fuzz/cargo-fuzz
- Version: 0.13.2
- Release commit: 984c861c8dfea28055254c5f1d2659ab2cd63f76
- License: MIT OR Apache-2.0
- CI executable: cargo-fuzz-0.13.2-x86_64-unknown-linux-musl.tar.gz
- Pinned SHA-256: b5b704018b63e0f151c17a057ac53b5111e1db545d1b9f72fee79f08a545931c
- Use: fuzz-only development/CI frontend; never linked into Morize runtime artifacts.

## libfuzzer-sys

- Project: rust-fuzz/libfuzzer
- Crate: libfuzzer-sys
- Version: exactly 0.4.13
- Release commit: 719e4efb9b8857ebaa782ae59376c8cbb78fed0f
- Wrapper license: MIT OR Apache-2.0
- Combined crate expression: (MIT OR Apache-2.0) AND NCSA
- Bundled libFuzzer source license: NCSA
- Use: fuzz-only dependency in the independent `fuzz/` workspace; never part of the root dependency graph or production artifacts.

The committed `fuzz/Cargo.lock` closes the exact fuzz-only transitive dependency graph. Root `Cargo.toml` and root `Cargo.lock` remain independent and unchanged.
