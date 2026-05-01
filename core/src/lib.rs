//! `stdbr-core` - standard library for Brazil.
//!
//! `no_std` compatible (requires `alloc`). Enable the default `std` feature
//! for better RNG seeding in generation functions.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod cpf;
