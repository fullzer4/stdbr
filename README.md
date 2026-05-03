# stdbr

[![CI](https://github.com/fullzer4/stdbr/actions/workflows/ci.yml/badge.svg)](https://github.com/fullzer4/stdbr/actions/workflows/ci.yml)
[![IBGE Sync](https://github.com/fullzer4/stdbr/actions/workflows/ibge-sync.yml/badge.svg)](https://github.com/fullzer4/stdbr/actions/workflows/ibge-sync.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

Standard library for Brazil.

One Rust codebase (`no_std` + `alloc`), five modules, four binding targets. Same behavior everywhere.

| Target | Package | Via |
|--------|---------|-----|
| Rust | `stdbr-core` | direct |
| Node.js | `@stdbr/stdbr` | napi-rs |
| Python | `stdbr` | PyO3 |
| WASM | `@stdbr/wasm` | wasm-bindgen |
| C/C++ | `stdbr-ffi` | `extern "C"` |

## License

Apache-2.0
