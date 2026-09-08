//! `stdbr-core` - standard library for Brazil.
//!
//! `no_std` compatible (requires `alloc`). Enable the default `std` feature
//! for better seeding in generation functions. Generated values use a small
//! non-cryptographic PRNG and must not be used as secrets.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod rand;
pub(crate) mod util;

pub mod cep;
pub mod cnpj;
pub mod cpf;
#[cfg(feature = "municipio")]
pub mod municipio;
pub mod rg;
pub mod uf;
