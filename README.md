# stdbr

> **Work in progress** - this project is under active development.

Standard library for Brazil. A single Rust codebase with `no_std` compatible core and bindings for Node.js, Python, WebAssembly and C.

Currently implements **CPF** (Cadastro de Pessoas Fisicas): validation, formatting, masking and generation using the modulo-11 algorithm specified by Receita Federal. More modules (CNPJ, CEP, etc.) are planned.

## How it works

Everything starts from `stdbr-core`, a `no_std` Rust library that implements the actual logic. Each binding wraps the core directly, so the behavior is identical across every platform. One implementation, one test suite, every language.

## Bindings

| Language | Crate / Package | Mechanism |
|----------|----------------|-----------|
| Rust | `stdbr-core` | Direct dependency |
| Node.js | `@stdbr/stdbr` | napi-rs |
| Python | `stdbr` | PyO3 + maturin |
| WebAssembly | `@stdbr/wasm` | wasm-bindgen |
| C / C++ | `stdbr-ffi` | FFI + cbindgen |

Full API documentation will live on a dedicated docs site.

## License

MIT OR Apache-2.0
