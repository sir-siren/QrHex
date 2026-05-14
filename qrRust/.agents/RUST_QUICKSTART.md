# QUICKSTART

Everything you need to run, build, test, and ship this Rust project.

---

## Requirements

```bash
# check rust -- need 1.85+
rustc --version
cargo --version

# if missing
# https://rustup.rs

# update to latest stable
rustup update stable
```

If your rust lives in WSL, prefix every command below with `wsl`.

---

## Install / setup

```bash
# nothing to install for pure rust projects
# just make sure rust is on your PATH

# verify the workspace builds
cargo check
```

---

## Run (dev)

```bash
# run the binary
cargo run

# run with arguments
cargo run -- --port 8080 --config ./config.toml

# run a specific binary in a workspace
cargo run -p cli

# with logging
RUST_LOG=debug cargo run
RUST_LOG=my_crate=debug cargo run
```

---

## Run tests

```bash
# run all tests across the whole workspace
cargo test --workspace

# run a specific test by name
cargo test health_does_not_underflow

# run tests in a specific crate
cargo test -p core

# run with stdout printed (useful for debugging)
cargo test --workspace -- --nocapture

# run only ignored tests
cargo test --workspace -- --ignored
```

---

## Lint

```bash
# clippy -- all warnings treated as errors
cargo clippy --all-targets -- -D warnings

# clippy on a specific crate
cargo clippy -p core -- -D warnings

# auto-fix what clippy can fix
cargo clippy --fix
```

---

## Format

```bash
# format all code
cargo fmt

# check formatting without changing files (CI use)
cargo fmt --check
```

---

## Compile check (fast, no binary output)

```bash
# fastest way to check for errors
cargo check

# check all targets including tests and benches
cargo check --all-targets

# check a specific crate
cargo check -p core
```

---

## Build (dev)

```bash
# debug build -- fast compile, slow runtime, debug symbols included
cargo build

# build a specific crate
cargo build -p server
```

---

## Build (release)

```bash
# optimized build -- slow compile, fast runtime
cargo build --release

# output lands here
ls -lh target/release/
```

---

## Run release binary

```bash
cargo build --release
./target/release/my-binary

# or in one step
cargo run --release
```

---

## Documentation

```bash
# generate and open docs in browser
cargo doc --open

# include private items
cargo doc --document-private-items --open

# for a specific crate
cargo doc -p core --open
```

---

## Benchmarks

```bash
# run benchmarks (requires bench targets in Cargo.toml)
cargo bench

# run a specific bench
cargo bench throughput
```

---

## Full pre-ship pipeline

```bash
# run all of these in order before committing

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --workspace
cargo build --release
```

---

## WSL users

Prefix all `cargo` and `rustc` commands with `wsl`:

```bash
wsl cargo check
wsl cargo test --workspace
wsl cargo build --release
wsl RUST_LOG=debug cargo run
```

For best performance, keep project files inside WSL home:

```
~/projects/my-project/   # fast
/mnt/c/Users/.../        # slower, cross-filesystem overhead
```

---

## Common errors and fixes

| Error | Fix |
|---|---|
| `error: toolchain 'stable' not installed` | run `rustup install stable` |
| `error[E0432]: unresolved import` | check `mod` declarations and file names match |
| `error[E0502]: cannot borrow as mutable` | restructure ownership or clone intentionally |
| `cargo: command not found` | rust not on PATH -- source `$HOME/.cargo/env` |
| `wsl: command not found` on Windows | enable WSL in Windows features |
| clippy warning treated as error in CI | fix the warning, do not suppress it |
| tests pass locally, fail in CI | check for environment-dependent state in tests |
