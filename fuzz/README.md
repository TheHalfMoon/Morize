# Morize fuzz harness

This directory is an isolated fuzz-only Cargo workspace for SG-000024. It is not a member of the Morize root workspace and does not add runtime, build, or test dependencies to normal Morize artifacts.

## Pinned toolchain

- Rust: `nightly-2026-09-18`
- cargo-fuzz: `0.13.2`
- cargo-fuzz Linux x86_64 musl executable SHA-256: `b5b704018b63e0f151c17a057ac53b5111e1db545d1b9f72fee79f08a545931c`
- libfuzzer-sys: exactly `0.4.13`
- fuzz target triple: `x86_64-unknown-linux-gnu`

The musl asset identifies only the prebuilt cargo-fuzz executable. Sanitizer-instrumented fuzz targets are explicitly compiled and run for GNU libc because AddressSanitizer is incompatible with statically linked musl libc.

## Targets

- `durable_id`: raw-byte durable-ID parser/canonical round-trip invariants for every durable ID type.
- `bounded_utf8`: raw-byte UTF-8 bound, preservation, and rejection-metadata invariants.
- `bounded_vec`: raw-byte collection bound, order, lossless-rejection, and failed-push invariants.

## Local bounded smoke

Install the pinned nightly and cargo-fuzz version, then run from the repository root:

```bash
export RUSTUP_TOOLCHAIN=nightly-2026-09-18
cargo fuzz build --fuzz-dir fuzz --target x86_64-unknown-linux-gnu
cargo fuzz run --fuzz-dir fuzz --target x86_64-unknown-linux-gnu durable_id -- -runs=5000 -seed=424242 -timeout=5 -rss_limit_mb=2048 -max_len=512
cargo fuzz run --fuzz-dir fuzz --target x86_64-unknown-linux-gnu bounded_utf8 -- -runs=5000 -seed=424243 -timeout=5 -rss_limit_mb=2048 -max_len=512
cargo fuzz run --fuzz-dir fuzz --target x86_64-unknown-linux-gnu bounded_vec -- -runs=5000 -seed=424244 -timeout=5 -rss_limit_mb=2048 -max_len=512
```

The CI smoke campaign is intentionally bounded. A passing run is evidence only for the executed budget; it is not a claim of exhaustive fuzzing, bug absence, complete security coverage, or coverage completeness.

Fuzz corpora and crash artifacts must contain generated/synthetic bytes only. Do not add real user data, secrets, credentials, PHI, or copied production fixtures.
