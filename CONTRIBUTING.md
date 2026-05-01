# Contributing

## Prerequisites

Requires [Nix](https://nixos.org/) with flakes enabled.

```bash
# enter dev shell
direnv allow   # or: nix develop
```

## Build & test

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## Bazel

```bash
build   # alias: bazel build //...
test    # alias: bazel test //...
```

## Format

```bash
fmt     # alias: nix fmt
```

## Building specific bindings

```bash
cargo build -p stdbr-python
cargo build -p stdbr-wasm --target wasm32-unknown-unknown
cd bindings/python && maturin develop
cd bindings/wasm && wasm-pack build --target web
```

## Project structure

```
stdbr/
  core/                    # stdbr-core (no_std Rust library)
  bindings/
    ffi-c/                 # C/C++ FFI (cdylib + staticlib + cbindgen)
    nodejs/                # Node.js via napi-rs
    python/                # Python via PyO3 + maturin
    wasm/                  # WebAssembly via wasm-bindgen
  tools/                   # Bazel custom rules
  .github/workflows/       # CI (Nix-based)
  flake.nix                # Nix dev environment
  MODULE.bazel             # Bazel module config
```
