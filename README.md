# stdbr

Standard library for Brazil. A single Rust codebase (`no_std` + `alloc`) with bindings for Node.js, Python, WebAssembly and C/C++.

## Modules

| Module | Description |
|--------|-------------|
| **CPF** | Validation, formatting, masking, generation (modulo-11, Receita Federal) |
| **CNPJ** | Numeric & alphanumeric CNPJ — validation, formatting, generation |
| **CEP** | Postal codes — validation, formatting, generation by region/state |
| **UF** | 27 states + Federal District — abbreviation, name, region lookup |
| **Municipio** | 5 571 municipalities — IBGE code lookup, capital, search by name/state |

## Bindings

All five modules are available in every target:

| Language | Package | Mechanism |
|----------|---------|-----------|
| Rust | `stdbr-core` | Direct dependency |
| Node.js | `@stdbr/stdbr` | napi-rs |
| Python | `stdbr` | PyO3 + maturin |
| WebAssembly | `@stdbr/wasm` | wasm-bindgen |
| C / C++ | `stdbr-ffi` | FFI (`extern "C"`) |

One implementation, one test suite, every language.

## Quick examples

### Rust

```rust
use stdbr_core::{cpf, cep, cnpj, municipio, uf};

// CPF
let cpf = cpf::generate_cpf();
assert!(cpf::is_valid(cpf.as_str()));

// CEP
let cep = cep::generate_for_state(uf::State::SP);
println!("{}", cep.formatted()); // "01310-100"

// Municipio
let capital = municipio::Municipio::capital_of(uf::State::RJ);
println!("{} ({})", capital.name, capital.ibge_code);
```

### Node.js

```js
import { Cpf, Cep, municipioCapitalOf, State } from '@stdbr/stdbr'

const cpf = Cpf.generate()
console.log(cpf.formatted())

const cep = Cep.generateForState(State.SP)
console.log(cep.formatted())

const capital = municipioCapitalOf(State.RJ)
console.log(capital.name)
```

### Python

```python
import stdbr

cpf = stdbr.Cpf.generate()
print(cpf.formatted())

cep = stdbr.Cep.generate_for_state(stdbr.State.SP)
print(cep.formatted())

capital = stdbr.municipio_capital_of(stdbr.State.RJ)
print(capital.name)
```

## Development

Requires [Nix](https://nixos.org/) (recommended) or Rust 1.85+.

```bash
# Enter dev shell
nix develop

# Check everything
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

## License

MIT OR Apache-2.0
