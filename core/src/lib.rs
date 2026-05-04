//! `stdbr-core` - standard library for Brazil.
//!
//! `no_std` compatible (requires `alloc`). Enable the default `std` feature
//! for better RNG seeding in generation functions.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod rand;
pub(crate) mod util;

pub mod cep;
pub mod cnpj;
pub mod cpf;
pub mod municipio;
pub mod rg;
pub mod uf;
