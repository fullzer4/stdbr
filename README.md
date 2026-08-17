# stdbr

[![CI](https://github.com/fullzer4/stdbr/actions/workflows/ci.yml/badge.svg)](https://github.com/fullzer4/stdbr/actions/workflows/ci.yml)
[![IBGE Sync](https://github.com/fullzer4/stdbr/actions/workflows/ibge-sync.yml/badge.svg)](https://github.com/fullzer4/stdbr/actions/workflows/ibge-sync.yml)
[![Version](https://img.shields.io/crates/v/stdbr-core.svg)](https://crates.io/crates/stdbr-core)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Standard library for Brazil.

One Rust codebase (`no_std` + `alloc`), five modules, four binding targets. Same behavior everywhere.

| Target | Package | Via |
|--------|---------|-----|
| Rust | [![crates.io](https://img.shields.io/crates/v/stdbr-core.svg?label=stdbr-core)](https://crates.io/crates/stdbr-core) | direct |
| Node.js | [![npm](https://img.shields.io/npm/v/@stdbr/stdbr.svg?label=@stdbr/stdbr)](https://www.npmjs.com/package/@stdbr/stdbr) | napi-rs |
| Python | [![PyPI](https://img.shields.io/pypi/v/stdbr.svg?label=stdbr)](https://pypi.org/project/stdbr/) | PyO3 |
| WASM | [![npm](https://img.shields.io/npm/v/@stdbr/wasm.svg?label=@stdbr/wasm)](https://www.npmjs.com/package/@stdbr/wasm) | wasm-bindgen |
| C/C++ | [![GitHub release](https://img.shields.io/github/v/release/fullzer4/stdbr.svg?label=stdbr-ffi)](https://github.com/fullzer4/stdbr/releases) | `extern "C"` |

## License

Apache-2.0
