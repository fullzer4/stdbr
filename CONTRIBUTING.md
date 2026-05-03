# Contributing

## Prerequisites

Requires [Nix](https://nixos.org/) with flakes enabled.

```bash
direnv allow   # or: nix develop
```

## Build & test

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

## Bazel

```bash
build   # alias: bazel build //...
check   # alias: bazel test //...
```

## Parity tests

Every module must produce identical results across all bindings. The `tests/parity/` suite generates a `golden.json` from `stdbr-core` and validates Rust, Node.js, Python, FFI-C, and WASM against it.

```bash
bazel test //tests/parity/...
```

When adding a new function to core, add the corresponding test cases to `tools/parity_gen/src/main.rs` and update every `test_parity.*` file.

## Adding a new binding target

1. Create `bindings/<target>/` with `Cargo.toml` and source
2. Expose the same API surface as existing bindings (parse, validate, format, generate)
3. Add `test_parity.*` that reads `golden.json` and validates all cases
4. Add build + test rules in `tests/parity/BUILD.bazel`
5. Register the Cargo manifest in `MODULE.bazel`

## Adding a new BR module

1. Implement in `core/src/<module>.rs` (`no_std` + `alloc`)
2. Export from `core/src/lib.rs`
3. Add bindings in all targets: `ffi-c`, `nodejs`, `python`, `wasm`
4. Add golden test cases in `tools/parity_gen/src/main.rs`
5. Update all `test_parity.*` files to cover the new module
6. If the module needs external data (like municipio uses IBGE), add a sync workflow in `.github/workflows/`

## Project structure

```
stdbr/
  core/                    # stdbr-core (no_std Rust library)
  bindings/
    ffi-c/                 # C/C++ FFI (staticlib + cbindgen)
    nodejs/                # Node.js via napi-rs
    python/                # Python via PyO3 + maturin
    wasm/                  # WebAssembly via wasm-bindgen
  tools/
    parity_gen/            # Golden test data generator
    rules_rust_extras/     # Bazel custom rules (cbindgen)
  tests/parity/            # Cross-binding parity tests
  .github/workflows/       # CI + IBGE sync
  flake.nix                # Nix dev environment
  MODULE.bazel             # Bazel module config
```
